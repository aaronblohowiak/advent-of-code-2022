/*
https://adventofcode.com/2022/day/19

goals for today: elegant code, use rayon to parallelize, correct answer, done by 3pm. */

/*
    The goal is to collect the most geodes.
    Geodes are collected by robots that have a cost to make given by different resources.
    The decision is: what is the right order to build robots to maximize ending geode sum.
    We can only build one robot at a time.
    Excess resources have no value.

    Basic loop:
        create a candidate list of robots i can build eventually with current resources being produced
        filter candidate list:
            avoid over-production
            avoid if time to build exceeds time remaining

        pick candidate as target // recurse.
        let time pass until:
            can afford to build
                build candidate
                adjust production
            hit time limit
                return total geodes captured
*/

#[derive(Default, Clone, Copy, Debug)]
pub struct Blueprint {
    id: usize,
    robots: [RobotSpec; 4], //input and output for each robot type
    max_production: ResourceList,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct RobotSpec {
    costs: ResourceList,
    gives: ResourceList,
}

use derive_more::{Add, Sub};
#[derive(Default, Clone, Copy, Add, Sub, Debug, Hash, Eq, PartialEq)]
pub struct ResourceList {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geode: isize,
}

use std::ops::Mul;
impl Mul<isize> for ResourceList {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self {
        ResourceList {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

//represents the state at the _end_ of the minute.
#[derive(Debug)]
struct State<'a> {
    minute: usize,            //starts at 1 because aoc
    production: ResourceList, //how much we are producing per turn.
    balance: ResourceList,    //how much we have at the END of the minute
    parent: Option<&'a State<'a>>,
}

impl Default for State<'_> {
    fn default() -> State<'static> {
        State {
            minute: 1,
            production: ResourceList {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            balance: ResourceList {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            parent: None,
        }
    }
}

impl<'a> State<'a> {
    fn candidates(
        &'a self,
        blueprint: &'a Blueprint,
        max_time: isize,
    ) -> impl Iterator<Item = State> + '_ {
        CandidateIterator {
            state: self,
            blueprint,
            max_time,
            index: 0,
        }
    }

    //returns how long it will be until i can build this. I can build it one day AFTER i can afford it, since we track balance as end of day balance..
    fn time_until_build(&self, spec: &RobotSpec) -> Option<usize> {
        fn time_for(costs: isize, production: isize, balance: isize) -> Option<isize> {
            if costs > 0 && production == 0 {
                return None;
            }

            if costs == 0 || balance >= costs {
                return Some(1);
            }

            Some(num::Integer::div_ceil(&(costs - balance), &production) + 1)
        }

        let times = [
            time_for(spec.costs.ore, self.production.ore, self.balance.ore),
            time_for(spec.costs.clay, self.production.clay, self.balance.clay),
            time_for(
                spec.costs.obsidian,
                self.production.obsidian,
                self.balance.obsidian,
            ),
        ];

        if times.iter().any(Option::is_none) {
            return None;
        }

        times.iter().map(|t| t.unwrap() as usize).max()
    }
}

struct CandidateIterator<'a> {
    state: &'a State<'a>,
    blueprint: &'a Blueprint,
    index: usize,
    max_time: isize,
}

impl<'a> Iterator for CandidateIterator<'a> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        //i can always build another robot OR let time run out
        // index < len is for robot evaluation.
        // index = len is let time run out.
        //      after that, we're done.
        if self.index > self.blueprint.robots.len() {
            return None;
        }

        //find first candidate robot
        //if none, just burn remaining time.
        if let Some((index, state)) = self
            .blueprint
            .robots
            .iter()
            .enumerate()
            .skip(self.index)
            .filter_map(|(index, robot_spec)| {
                self.state.time_until_build(robot_spec).map(|time| {
                    (
                        index,
                        State {
                            minute: self.state.minute + time,
                            production: self.state.production + robot_spec.gives,
                            balance: self.state.balance + (self.state.production * time as isize)
                                - robot_spec.costs,
                            parent: Some(self.state),
                        },
                    )
                })
            })
            .filter(|(_, state)| state.minute <= self.max_time as usize)
            .filter(|(_, state)| {
                state.production.ore <= self.blueprint.max_production.ore
                    && state.production.clay <= self.blueprint.max_production.clay
                    && state.production.obsidian <= self.blueprint.max_production.obsidian
            })
            .next()
        {
            self.index = index + 1;
            return Some(state);
        }

        self.index = self.blueprint.robots.len() + 1;

        Some(State {
            minute: self.max_time as usize,
            production: self.state.production,
            balance: self.state.balance
                + (self.state.production * (self.max_time - self.state.minute as isize)),
            parent: Some(self.state),
        })
    }
}

type SafeHash = Arc<Mutex<FxHashMap<(usize, ResourceList, ResourceList), usize>>>;
fn most_geodes(state: &State, blueprint: &Blueprint, max_time: isize, cache: SafeHash) -> usize {
    let balance_key = ResourceList{
        ore: blueprint.max_production.ore.min(state.balance.ore),
        clay: blueprint.max_production.clay.min(state.balance.clay),
        obsidian: blueprint.max_production.obsidian.min(state.balance.obsidian),
        geode: blueprint.max_production.geode.min(state.balance.geode),
    };

    let key = (state.minute, state.production, balance_key);

    {
        let mut c = cache.lock().unwrap();
        if let Some(res) = c.get(&key) {
            return *res
        }
    }

    let best = state
        .candidates(blueprint, max_time)
        .collect::<Vec<State>>()
        .par_iter()
        .map(|s| {
            if s.minute == max_time as usize {
                s.balance.geode as usize
            } else {
                most_geodes(&s, blueprint, max_time, cache.clone())
            }
        })
        .max()
        .expect("should have some amount even if zero from burning remaining time");

    {
        let mut c = cache.lock().unwrap();
        c.insert(key, best);
        return best;
    }
}

use rustc_hash::FxHashMap;
use std::sync::Arc;
use std::sync::Mutex;

fn highest_geode_count(blueprint: &Blueprint, max_time: isize) -> usize {
    let initial = State::default();


    let mut cache : FxHashMap<(usize, ResourceList, ResourceList), usize> = FxHashMap::default();
    let mtx = Arc::new(Mutex::new(cache));

    let highest_geode_count = most_geodes(&initial, blueprint, max_time, mtx);

    println!(
        "Heighest geodes {} for blueprint {}",
        highest_geode_count, blueprint.id
    );

    highest_geode_count
}

fn parse_blueprint(input: &str) -> Blueprint {
    let mut bp = blueprint_parser::blueprint(input).unwrap();

    //since we can only build one robot per turn,
    //  it doesnt make sense to ever produce more than the materials required to build any robot every turn,
    //  but there's no limit to the amount of geodes we want to build.
    bp.max_production = ResourceList {
        ore: bp.robots.iter().map(|r| r.costs.ore).max().unwrap(),
        clay: bp.robots.iter().map(|r| r.costs.clay).max().unwrap(),
        obsidian: bp.robots.iter().map(|r| r.costs.obsidian).max().unwrap(),
        geode: isize::MAX,
    };

    bp
}

use rayon::prelude::*;

fn part1(path: &str) -> usize {
    std::fs::read_to_string(path)
        .expect("open input file")
        .lines()
        .map(parse_blueprint)
        .collect::<Vec<Blueprint>>()
        .par_iter()
        .map(|bp| bp.id * highest_geode_count(bp, 24))
        .sum()
}

fn part2(path: &str) -> usize {
    std::fs::read_to_string(path)
        .expect("open input file")
        .lines()
        .take(3)
        .map(parse_blueprint)
        .collect::<Vec<Blueprint>>()
        .par_iter()
        .map(|bp| highest_geode_count(bp, 32))
        .product()
}

fn main() {
    println!("Part 1: {}", part1("./19.input"));
    println!("Part 2: {}", part2("./19.input"));
}

peg::parser! {
    grammar blueprint_parser() for str {

    rule number() -> isize
        = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

        //Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 3 ore and 18 obsidian.

    pub rule blueprint() -> (Blueprint)
        = "Blueprint " id:number() ": Each ore robot costs " ore_ore_cost:number() " ore. Each clay robot costs " clay_ore_cost:number() " ore. Each obsidian robot costs " obs_ore_cost:number() " ore and " obs_clay_cost:number() " clay. Each geode robot costs " geode_ore_cost:number() " ore and " geode_obs_cost:number() " obsidian." {
            Blueprint{
                id: id as usize,
                robots: [
                    RobotSpec{
                        gives: ResourceList{ore: 0, clay: 0, obsidian: 0, geode: 1},
                        costs: ResourceList{ore: geode_ore_cost, clay: 0, obsidian: geode_obs_cost, geode: 0}
                    },
                    RobotSpec{
                        gives: ResourceList{ore: 0, clay: 0, obsidian: 1, geode: 0 },
                        costs: ResourceList{ore: obs_ore_cost, clay: obs_clay_cost, obsidian: 0, geode: 0}
                    },
                    RobotSpec{
                        gives: ResourceList{ore: 0, clay: 1, obsidian: 0, geode: 0 },
                        costs: ResourceList{ore: clay_ore_cost, clay: 0, obsidian: 0, geode: 0}
                    },
                    RobotSpec{
                        gives: ResourceList{ore: 1, clay: 0, obsidian: 0, geode: 0 },
                        costs: ResourceList{ore: ore_ore_cost, clay: 0, obsidian: 0, geode: 0}
                    },
                ],
                max_production : ResourceList::default()
            }
        }
    }
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn test_input_file() {
        let bp = parse_blueprint("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");

        let state = State::default();
        assert_eq!(9, most_geodes(&state, &bp, 24));

    }
}
