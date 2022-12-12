use std::fs;

use pathfinding::directed::dijkstra::dijkstra;

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub trait Plane {
    fn width(&self) -> isize;
    fn height(&self) -> isize;

    fn get(&self, p: Coord) -> Option<char>;
    fn set(&mut self, p: Coord, c: char);
}

#[derive(Clone, Debug)]
struct OneDPlane {
    storage: Vec<char>,
    cols: isize,
    rows: isize,
}

impl Plane for OneDPlane {
    fn width(&self) -> isize {
        self.cols
    }
    fn height(&self) -> isize {
        self.rows
    }

    fn get(&self, p: Coord) -> Option<char> {
        if p.x < 0 || p.y < 0 {
            None
        } else {
            self.storage.get((self.cols * p.y + p.x) as usize).copied()
        }
    }
    fn set(&mut self, p: Coord, c: char) {
        self.storage[(self.cols * p.y + p.x) as usize] = c
    }
}

impl OneDPlane {
    fn coord_from_idx(&self, idx: usize) -> Coord {
        let idx = idx as isize;
        Coord {
            x: idx % self.cols,
            y: idx / self.cols,
        }
    }
}

#[derive(Clone, Debug)]
struct Topo {
    plane: OneDPlane,
    visited: HashSet<Coord>,
}

const UP: Coord = Coord { x: 0, y: -1 };
const DOWN: Coord = Coord { x: 0, y: 1 };
const LEFT: Coord = Coord { x: -1, y: 0 };
const RIGHT: Coord = Coord { x: 1, y: 0 };

const COMPASS: [Coord; 4] = [UP, DOWN, LEFT, RIGHT];

impl Topo {
    fn neighbors_uphill(&mut self, pt: Coord) -> impl IntoIterator<Item = (Coord, isize)> {
        let cur = self.plane.get(pt).unwrap();

        COMPASS
            .iter()
            .filter_map(|dir| {
                let lookup = pt + *dir;

                if let Some(n) = self.plane.get(lookup) {
                    self.visited.insert(lookup);
                    if n <= cur || (n as isize - cur as isize).abs() < 2 {
                        return Some((lookup, 1));
                    }
                }

                None
            })
            .collect::<Vec<(Coord, isize)>>()
    }

    fn neighbors_downhill(&mut self, pt: Coord) -> impl IntoIterator<Item = (Coord, isize)> {
        let cur = self.plane.get(pt).unwrap();

        COMPASS
            .iter()
            .filter_map(|dir| {
                let lookup = pt + *dir;

                if let Some(n) = self.plane.get(lookup) {
                    self.visited.insert(lookup);
                    if n >= cur || (n as isize - cur as isize).abs() < 2 {
                        return Some((lookup, 1));
                    }
                }

                None
            })
            .collect::<Vec<(Coord, isize)>>()
    }
}

fn main() {
    let input = fs::read_to_string("./12.input").expect("could not read file");
    let cols = input.lines().next().unwrap().len() as isize;
    let rows = input.lines().count() as isize;

    let storage: Vec<char> = input.lines().flat_map(str::chars).collect();
    let mut plane = OneDPlane {
        storage,
        cols,
        rows,
    };
    let start = plane.coord_from_idx(plane.storage.iter().position(|x| x == &'S').unwrap());
    let goal = plane.coord_from_idx(plane.storage.iter().position(|x| x == &'E').unwrap());

    plane.set(start, 'a');
    plane.set(goal, 'z');

    let mut topo = Topo {
        plane,
        visited: HashSet::new(),
    };

    if let Some(result) = dijkstra(&start, |x| topo.neighbors_uphill(*x), |x| *x == goal) {
        println!("shortest path pt 1 is {:?}", result.0.len() - 1); //number of steps, not the total points which includes the first.
    } else {
        println!("Oh noes!");
        for y in 0..topo.plane.height() {
            for x in 0..topo.plane.width() {
                if topo.visited.contains(&Coord { x, y }) {
                    print!("#");
                } else {
                    let c = topo.plane.get(Coord { x, y }).unwrap();
                    let color = c as isize - 'a' as isize + 1;
                    print!("\u{001b}[38;5;{}m{}\u{001b}[0m", color, c);
                }
            }
            println!();
        }
        println!();
    }

    println!();
    println!();
    println!();

    topo.visited.clear();

    let uuuuugh = topo.clone();

    if let Some(result) = dijkstra(
        &goal,
        |x| topo.neighbors_downhill(*x),
        |x| uuuuugh.plane.get(*x).unwrap() == 'a',
    ) {
        println!("Least path to 'a' is {:?}", result.0.len() - 1); //number of steps, not the total points which includes the first.
    } else {
        println!("Oh noes!");
        for y in 0..topo.plane.height() {
            for x in 0..topo.plane.width() {
                if topo.visited.contains(&Coord { x, y }) {
                    print!("#");
                } else {
                    let c = topo.plane.get(Coord { x, y }).unwrap();
                    let color = c as isize - 'a' as isize + 1;
                    print!("\u{001b}[38;5;{}m{}\u{001b}[0m", color, c);
                }
            }
            println!();
        }
        println!();
    }
}
