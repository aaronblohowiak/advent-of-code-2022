use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let (_, rounds) = part1("./14.input");
    println!("{}", rounds);

    let (_, rounds) = part2("./14.input");
    println!("{}", rounds+1);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
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
const DOWN: Coord = Coord { x: 0, y: 1 };
const DOWN_LEFT: Coord = Coord { x: -1, y: 1 };
const DOWN_RIGHT: Coord = Coord { x: 1, y: 1 };
const FALLING_DIRECTIONS: [Coord; 3] = [DOWN, DOWN_LEFT, DOWN_RIGHT];

const SOURCE_COORD: Coord = Coord { x: 500, y: 0 };

struct Field {
    min_x: isize,
    max_x: isize,
    max_y: isize,
    positions: HashMap<Coord, char>,
    lock_y: bool //HACK HACK HACK for part1 and part2 support...
}

impl Default for Field {
    fn default() -> Field {
        Field {
            min_x: isize::MAX,
            max_x: isize::MIN,
            max_y: 0,
            positions: HashMap::new(),
            lock_y: false
        }
    }
}

impl Field {
    fn debug(&self, extra: Coord, extra_c: char) {
        println!(
            "Showing X: {} - {} , Y: 0 - {}",
            self.min_x - 1,
            self.max_x + 1,
            self.max_y + 2
        );
        for y in RangeInclusive::new(0, self.max_y + 2) {
            for x in RangeInclusive::new(self.min_x - 1, self.max_x + 1) {
                let pos = Coord { x, y };
                if pos == extra {
                    print!("{}", extra_c);
                } else if let Some(c) = self.positions.get(&pos) {
                    print!("{}", c);
                } else if y > self.max_y + 1{
                    print!("_");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn upsert(&mut self, pos: Coord, c: char) {
        if pos.x < self.min_x {
            self.min_x = pos.x;
        }

        if pos.x > self.max_x {
            self.max_x = pos.x;
        }

        if !self.lock_y && pos.y > self.max_y {
            self.max_y = pos.y;
        }

        self.positions.insert(pos, c);
    }

    fn paint_range(&mut self, from: &Coord, to: &Coord, c: char) {
        let x_step = to.x.cmp(&from.x) as isize;
        let y_step = to.y.cmp(&from.y) as isize;

        let mut pos = *from;

        self.upsert(pos, c);
        loop {
            pos.x += x_step;
            pos.y += y_step;
            self.upsert(pos, c);
            println!("{:?} {:?} {:?}", from, to, pos);

            if pos == *to {
                return;
            }
        }
    }

    fn part_2_hack_get(&mut self, pos: &Coord) -> char {
        if pos.y == self.max_y + 2 {
            return '#'; //virtual floor
        }

        *self.positions.get(pos).unwrap_or(&' ')
    }


    fn next_falling_position(&mut self, falling: Coord) -> Option<Coord> {
        FALLING_DIRECTIONS
            .iter()
            .find(|dir| self.part_2_hack_get(&(falling + **dir)) == ' ')
            .map(|c| falling + *c)
    }

    fn next_resting_location(&mut self, failed: fn(pos: &Coord, f: &Field)->bool) -> Result<Coord, Vec<Coord>> {
        let mut pos = SOURCE_COORD;
        let mut err = Vec::new();

        loop {
            match self.next_falling_position(pos) {
                Some(n) => {
                    pos = n;

                    err.push(pos);
                    //i could debug here?

                    if failed(&pos, self){
                        return Err(err);
                    }
                }
                None => {
                    if failed(&pos, self){
                        return Err(err);
                    }

                    return Ok(pos);
                }
            }
        }
    }
}

fn parse_input(fname: &str) -> Vec<Vec<Coord>> {
    let input = fs::read_to_string(fname).expect("could not read file");

    input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|c| {
                    let (x, y) = c
                        .split(',')
                        .map(|s| s.parse::<isize>().unwrap())
                        .tuples()
                        .next()
                        .unwrap();
                    Coord { x, y }
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Vec<Coord>>>()
}

fn calculate_rounds(fname: &str, failed: fn(pos: &Coord, f: &Field)->bool ) -> (Field, usize) {
    let splines = parse_input(fname);

    let mut f = Field::default();

    for spline in splines {
        let mut coords = spline.iter();
        let mut curr = coords.next().expect("at least two coords");
        for next in coords {
            f.paint_range(curr, next, '#');
            curr = next;
        }
    }

    f.lock_y = true;

    f.debug(SOURCE_COORD, '+');

    let mut rounds = 0;
    while let Ok(pos) = f.next_resting_location(failed) {
        f.upsert(pos, 'o');
        rounds += 1;

        if rounds % 1000 == 0 {
            f.debug(SOURCE_COORD, '+');

        }
    }

    f.debug(SOURCE_COORD, '+');


    (f, rounds)
}

fn part1(fname: &str) -> (Field, usize){
    let failed = |pos: &Coord, f: &Field| pos.y > f.max_y;

    calculate_rounds(fname, failed)
}

fn part2(fname: &str) -> (Field, usize){
    let failed = |pos: &Coord, f: &Field| *pos == SOURCE_COORD;

    calculate_rounds(fname, failed)
}


#[cfg(test)]
mod test {

    use crate::{part1, SOURCE_COORD};

    #[test]
    fn test_input_file() {
        let (f, rounds) = part1("./14.test");

        f.debug(SOURCE_COORD, '+');

        assert_eq!(24, rounds);
    }
}
