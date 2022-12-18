use bit_iter::BitIter;
use std::{collections::HashMap, hash::Hash, rc::Rc}; //TODO: FxHashmap

use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    nodes_open: u64,
    time_remaining: u8,
    pressure_being_released: u16,
    pressure_released_so_far: u16,
    me_position: u8, //need a me me_position and a dumbo me_position
    me: Task,
    dumbo_position: u8,
    dumbo: Task,
    prev: Option<Rc<State>>,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Task {
    Open,
    Walk { to: u8, time_left: u8 },
    Fin,
    Unknown, //waiting for work. intermediary value should never be seen when not being processed. maybe i should make task an Option<> rather than the sentinal value..
}

#[derive(Default)]
struct StringInterner {
    s_to_i: HashMap<String, u8>,
    i_to_s: HashMap<u8, String>,
}

impl StringInterner {
    fn get_index(&mut self, s: &str) -> u8 {
        if let Some(idx) = self.s_to_i.get(s) {
            return *idx;
        }
        let idx = self.s_to_i.len() as u8;
        self.s_to_i.insert(s.to_owned(), idx);
        self.i_to_s.insert(idx, s.to_owned());
        idx
    }
}

struct Context {
    interner: StringInterner,
    starting_moves: Vec<(u8, u8)>,
    distance_matrix: HashMap<u8, HashMap<u8, u8>>,
    flow_rates: HashMap<u8, u16>,
}

fn main() {
    let ctx = load("./16.test");

    let mut flow_nodes_m: u64 = 0;
    for id in ctx.flow_rates.keys() {
        flow_nodes_m |= 1 << id;
    }

    let flow_nodes = flow_nodes_m;

    let starting_position = ctx.interner.s_to_i["AA"];

    let mut frontier: &mut Vec<Rc<State>> = &mut ctx
        .starting_moves
        .iter()
        .combinations(2)
        .map(|mut combo| {
            combo.sort_by(|a, b| b.0.cmp(&a.0)); //THIS CODE ONLY WORKS WHEN THIS IS SORTED THIS WAY. WHYYYYYYYYYY

            Rc::new(State {
                nodes_open: 0,
                time_remaining: 26,
                pressure_being_released: 0,
                pressure_released_so_far: 0,
                me_position: starting_position,
                me: Task::Walk {
                    to: combo[0].0,
                    time_left: combo[0].1,
                },
                dumbo_position: starting_position,
                dumbo: Task::Walk {
                    to: combo[1].0,
                    time_left: combo[1].1,
                },
                prev: None,
            })
        })
        .collect();

    let mut processed: &mut Vec<Rc<State>> = &mut vec![];

    let mut done: Vec<Rc<State>> = vec![];

    for s in frontier.iter() {
        match s.me {
            Task::Walk { to, time_left } => {
                print!("to: {:?} ", to)
            }
            _ => {}
        }
    }
    println!();

    let _prev = SystemTime::now();
    for i in 0..26 {
        println!("Time: {}", i);
        while let Some(prev) = frontier.pop() {
            let mut s = (*prev).clone();
            s.prev = Some(prev);

            s.pressure_released_so_far += s.pressure_being_released;
            s.time_remaining -= 1;

            if s.time_remaining == 0 {
                done.push(Rc::new(s.clone()));
                continue;
            }

            match s.dumbo {
                Task::Unknown => {
                    unreachable!()
                }
                Task::Walk { to, mut time_left } => {
                    time_left -= 1;
                    if time_left == 0 {
                        s.dumbo_position = to;
                        s.dumbo = Task::Open;
                    } else {
                        s.dumbo = Task::Walk { to, time_left };
                    }
                }
                Task::Open => {
                    s.nodes_open |= 1 << s.dumbo_position;
                    s.pressure_being_released += ctx.flow_rates[&s.dumbo_position];
                    s.dumbo = Task::Unknown;
                }
                Task::Fin => {}
            }

            //copy pasta
            match s.me {
                Task::Unknown => {
                    unreachable!()
                }
                Task::Walk { to, mut time_left } => {
                    time_left -= 1;
                    if time_left == 0 {
                        s.me_position = to;
                        s.me = Task::Open;
                    } else {
                        s.me = Task::Walk { to, time_left };
                    }
                }
                Task::Open => {
                    s.nodes_open |= 1 << s.me_position;
                    s.pressure_being_released += ctx.flow_rates[&s.me_position];
                    s.me = Task::Unknown;
                }
                Task::Fin => {}
            }

            match (s.me, s.dumbo) {
                (Task::Unknown, Task::Unknown) => {
                    //get all the combos of 2 potential places to go, and enqueue them all for processing.
                    let potentials = flow_nodes & !s.nodes_open; //unneccesary performant way to find nodes left to visit?
                    if potentials == 0 {
                        s.me = Task::Fin;
                        s.dumbo = Task::Fin;
                        processed.push(Rc::new(s.clone()));
                    }

                    if BitIter::from(potentials).count() == 1 {
                        //could go to me OR dumbo

                        //first, to me
                        s.dumbo = Task::Fin;

                        let to = BitIter::from(potentials).next().unwrap() as u8;
                        let time_left = ctx.distance_matrix[&s.me_position][&to];
                        if time_left < s.time_remaining {
                            s.me = Task::Walk { to, time_left };
                        } else {
                            s.me = Task::Fin;
                        }

                        processed.push(Rc::new(s.clone()));

                        //then to dumbo. copy+paste. this code is so gross

                        s.me = Task::Fin;
                        let time_left = ctx.distance_matrix[&s.dumbo_position][&to];
                        if time_left < s.time_remaining {
                            s.dumbo = Task::Walk { to, time_left };
                        } else {
                            s.dumbo = Task::Fin;
                        }

                        processed.push(Rc::new(s.clone()));
                    }

                    for pair in BitIter::from(potentials).permutations(2) {
                        //create work for both of us
                        let (mut mine, mut theirs);
                        unsafe {
                            mine = *pair.get_unchecked(0) as u8;
                            theirs = *pair.get_unchecked(1) as u8;
                        }

                        let me_time_left = ctx.distance_matrix[&s.me_position][&mine];
                        if me_time_left < s.time_remaining {
                            s.me = Task::Walk {
                                to: mine,
                                time_left: me_time_left,
                            };
                        } else {
                            s.me = Task::Fin
                        }

                        let dumbo_time_left = ctx.distance_matrix[&s.dumbo_position][&theirs];
                        if dumbo_time_left < s.time_remaining {
                            s.dumbo = Task::Walk {
                                to: theirs,
                                time_left: dumbo_time_left,
                            };
                        } else {
                            s.dumbo = Task::Fin
                        }

                        processed.push(Rc::new(s.clone()))
                    }
                }
                (Task::Unknown, _) => {
                    let mut potentials = flow_nodes & !s.nodes_open; // TODO: remove what dumbo is doing from potentials!

                    match s.dumbo {
                        Task::Walk { to, time_left } => {
                            potentials &= !(1 << to);
                        }
                        Task::Open => {
                            potentials &= !(1 << s.dumbo_position);
                        }
                        _ => {}
                    }

                    if potentials == 0 {
                        s.me = Task::Fin;
                        processed.push(Rc::new(s.clone()));
                    }

                    for to_usize in BitIter::from(potentials) {
                        let to = to_usize as u8;
                        let time_left = ctx.distance_matrix[&s.me_position][&to];
                        s.me = Task::Walk { to, time_left };
                        processed.push(Rc::new(s.clone()))
                    }
                }
                (_, Task::Unknown) => {
                    let mut potentials = flow_nodes & !s.nodes_open; // TODO: remove what me is doing from potentials!
                    match s.me {
                        Task::Walk { to, time_left } => {
                            potentials &= !(1 << to);
                        }
                        Task::Open => {
                            potentials &= !(1 << s.me_position);
                        }
                        _ => {}
                    }

                    if potentials == 0 {
                        s.dumbo = Task::Fin;
                        processed.push(Rc::new(s.clone()));
                    }

                    for to_usize in BitIter::from(potentials) {
                        let to = to_usize as u8;

                        let time_left = ctx.distance_matrix[&s.dumbo_position][&to];
                        s.dumbo = Task::Walk { to, time_left };
                        processed.push(Rc::new(s.clone()))
                    }
                }
                _ => {
                    processed.push(Rc::new(s.clone()));
                }
            }
        }

        (frontier, processed) = (processed, frontier);
    }

    (done).sort_by(|a, b| b.pressure_released_so_far.cmp(&a.pressure_released_so_far));

    let top = (&done).iter().next().unwrap();

    fn display(state: &State, ctx: &Context) {
        if let Some(prev) = &state.prev {
            display(&prev, &ctx);
        }

        let open_valves: Vec<String> = BitIter::from(state.nodes_open)
            .map(|f| ctx.interner.i_to_s[&(f as u8)].to_owned())
            .collect();

        println!("== Minute {} ==", 26 - state.time_remaining);
        println!(
            "Valves {:?} are open, releasing {} pressure",
            open_valves, state.pressure_being_released
        );

        match state.me {
            Task::Fin | Task::Unknown => {}
            Task::Open => {
                print!(" You open {}. ", ctx.interner.i_to_s[&state.me_position])
            }
            Task::Walk { to, time_left } => print!(
                "You are walking to {}, {} left. ",
                ctx.interner.i_to_s[&to], time_left
            ),
        }

        match state.dumbo {
            Task::Fin | Task::Unknown => {}
            Task::Open => {
                print!(
                    " Dumbo opens {}. ",
                    ctx.interner.i_to_s[&state.dumbo_position]
                )
            }
            Task::Walk { to, time_left } => print!(
                "Dumbo is walking to {}, {} left. ",
                ctx.interner.i_to_s[&to], time_left
            ),
        }

        print!(
            "    total pressure: {}. My position {}, dumbo: {}. ",
            state.pressure_released_so_far,
            ctx.interner.i_to_s[&state.me_position],
            ctx.interner.i_to_s[&state.dumbo_position],
        );

        println!("\n");
    }

    display(&top, &ctx);

    println!("top: {:?} ", top)
}

fn load(fname: &str) -> Context {
    let input = std::fs::read_to_string(fname).expect("could not read file");

    let res: Vec<(String, u16, Vec<String>)> =
        input.lines().flat_map(valve_parser::valve).collect();

    let mut interner = StringInterner::default();

    let mut input_map: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut nonzero_flow_rates: HashMap<u8, u16> = HashMap::new(); //from id to flow rate

    for v in res {
        let id = interner.get_index(&v.0);
        let flow: u16 = v.1;

        if flow > 0 {
            nonzero_flow_rates.insert(id, flow);
        }

        let adjacency_ids: Vec<u8> = v.2.iter().map(|s| interner.get_index(s)).collect();
        input_map.insert(id, adjacency_ids);
    }

    let successors = |&n: &u8| -> Vec<(u8, u8)> { input_map[&n].iter().map(|f| (*f, 1)).collect() };

    //id of valve to a map of the other non-zero valves and the cost to get there.
    let mut valve_distances: HashMap<u8, HashMap<u8, u8>> = HashMap::new();

    for starting in nonzero_flow_rates.keys() {
        let distances = dijkstra_all(starting, successors);
        valve_distances.insert(*starting, HashMap::new());

        for id in nonzero_flow_rates.keys() {
            if id == starting {
                continue;
            }
            let (_, cost) = distances[id];
            valve_distances.get_mut(starting).unwrap().insert(*id, cost);
        }
    }

    let aa_idx = interner.get_index("AA");

    let mut starting_moves: Vec<(u8, u8)> = vec![];

    let aa_distances = dijkstra_all(&aa_idx, successors);
    for id in nonzero_flow_rates.keys() {
        starting_moves.push((*id, aa_distances[id].1));
    }

    println!("Starting moves: ");
    for (id, cost) in &starting_moves {
        println!("{} {}", interner.i_to_s[id], cost)
    }

    Context {
        interner,
        starting_moves,
        distance_matrix: valve_distances,
        flow_rates: nonzero_flow_rates,
    }
}

peg::parser! {
    grammar valve_parser() for str {

    rule number() -> u16
        = n:$(['0'..='9']+) {? n.parse().or(Err("usize")) }

    rule valve_id() -> String
        = id:$(['A'..='Z']['A'..='Z']) { id.to_string() }

    pub rule list() -> Vec<String>
        = l:(valve_id() ** ", ") { l }

    pub rule valve() -> (String, u16, Vec<String>)
        = "Valve " id:valve_id() " has flow rate=" flow:number() "; tunnel" "s"? " lead" "s"? " to valve" "s"?  " " adjacencies:list() {
            (id, flow, adjacencies)
        }
    }
}
