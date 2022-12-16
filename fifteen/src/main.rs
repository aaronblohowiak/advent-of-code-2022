use std::collections::btree_map::Range;
use std::fs;
use std::ops::RangeInclusive;

use rustc_hash::FxHashMap;
use peg;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Sensor {
    pos: Coord,
    beacon: Coord
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

struct Field {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    positions: FxHashMap<Coord, char>,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: 0,
            positions: FxHashMap::default()
        }
    }
}

impl Field {
    fn debug(&self) {
        println!(
            "Showing X: {} to {} , Y: 0 to {}",
            self.min_x - 1,
            self.max_x + 1,
            self.max_y + 2
        );
        for y in RangeInclusive::new(self.min_y, self.max_y + 2) {
            print!("{:>5} ", y);
            for x in RangeInclusive::new(self.min_x - 1, self.max_x + 1) {
                let pos = Coord { x, y };
                if let Some(c) = self.positions.get(&pos) {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn upsert(&mut self, pos: Coord, c: char) {

        self.min_x = self.min_x.min(pos.x);
        self.min_y = self.min_y.min(pos.y);

        self.max_x = self.max_x.max(pos.x);
        self.max_y = self.max_y.max(pos.y);

        self.positions.insert(pos, c);
    }

    fn paint_line(&mut self, from: Coord, to: Coord, c: char) {
        let x_step = to.x.cmp(&from.x) as isize;
        let y_step = to.y.cmp(&from.y) as isize;

        let mut pos = from;

        self.upsert(pos, c);
        loop {
            pos.x += x_step;
            pos.y += y_step;
            self.upsert(pos, c);

            if pos == to {
                return;
            }
        }
    }

    fn fill_manhattan_ball_line(&mut self, center: Coord, to: Coord, c: char, line: isize) {
        //https://en.wikipedia.org/wiki/Taxicab_geometry
        let distance : isize = (center.x - to.x).abs() + (center.y - to.y).abs();

        if !RangeInclusive::new(center.y - distance, center.y + distance).contains(&line) {
            return //dont bother
        }

        let x = distance - (center.y - line).abs();

        self.paint_line(Coord{x: center.x-x, y: line}, Coord{x: center.x + x, y: line}, c);
    }
}


fn main() {
    let input = fs::read_to_string("./15.input").expect("could not read file");

    let res: Vec<Sensor> = input.lines().flat_map( sensor_parser::sensor).collect();

    let mut f = Field::default();

    let line = 2000000;

    for sensor in res.iter() {
        println!("filling for {:?}", sensor);
        f.fill_manhattan_ball_line(sensor.pos, sensor.beacon, '#', line);
    }

    for sensor in res.iter() {
        f.upsert(sensor.pos, 'S');
        f.upsert(sensor.beacon, 'B')
    }

    // f.debug();

    let no_beacon_guaranteed = (f.min_x..(f.max_x+1))
        .flat_map(|x| f.positions.get(&Coord{x, y:line}))
        .filter(|c| **c != 'B').count();
    
        println!("{}", no_beacon_guaranteed);
}

peg::parser!{
    grammar sensor_parser() for str {

    rule number() -> isize
        = n:$(['-']? ['0'..='9']+) {? n.parse().or(Err("isize")) }

    pub rule sensor() -> Sensor
        = "Sensor at x=" sx:number() ", y=" sy:number() ": closest beacon is at x=" bx:number() ", y=" by:number() {
            Sensor {
                pos: Coord{x: sx, y:sy},
                beacon: Coord{x: bx, y: by}
            }
        }
    }
}