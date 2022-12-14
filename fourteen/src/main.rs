use std::fs;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use itertools::Itertools;


fn main() {
    println!("Hello, world!");
}

fn sand_until_abyss() -> usize {
    0
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

const SOURCE_COORD : Coord = Coord{x: 500, y:0};


struct Field {
    min_x: isize,
    max_x: isize,
    max_y: isize,
    positions: HashMap<Coord, char>
}

impl Default for Field {
    fn default() -> Field {
        Field {
            min_x: isize::MAX,
            max_x: isize::MIN,
            max_y: 0,
            positions: HashMap::new()
        }
    }
}


impl Field {

    fn debug(&self, extra: Coord, extra_c: char){
        println!("Showing X: {} - {} , Y: 0 - {}", self.min_x - 1, self.max_x + 1, self.max_y+1);
        for y in RangeInclusive::new(0, self.max_y+1) {
            for x in RangeInclusive::new(self.min_x - 1, self.max_x + 1) {
                let pos = Coord{x, y};
                if pos == extra{
                    print!("{}", extra_c);
                } else if let Some(c) = self.positions.get(&pos) {
                    print!("{}", c);
                } else if y > self.max_y {
                    print!("_");
                }else{
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }

    fn upsert(&mut self, pos: Coord, c: char){
        if pos.x < self.min_x {
            self.min_x = pos.x;
        }

        if pos.x > self.max_x{
            self.max_x = pos.x;
        }

        if pos.y > self.max_y {
            self.max_y = pos.y;
        }

        self.positions.insert(pos, c);
    }

    fn paint_range(&mut self, from: &Coord, to: &Coord, c: char){
        let x_step = to.x.cmp(&from.x) as isize;
        let y_step = to.y.cmp(&from.y) as isize;

        let mut pos = from.clone();

        let mut x = 0;
        
        loop {
            pos.x += x_step;
            pos.y += y_step;
            self.upsert(pos, c);
            println!("{:?} {:?} {:?}", from, to, pos);

            x +=1;

            if pos == *to || x > 100 {
                return
            }
        };
    }
}

fn parse_input(fname: &str) -> Vec<Vec<Coord>> {
    let input = fs::read_to_string("./14.test").expect("could not read file");

    input.lines().map(|l| {
        l.split(" -> ")
        .map(|c| {
            let (x, y) = c.split(",")
                .map(|s| s.parse::<isize>().unwrap())
                .tuples().next().unwrap();
            Coord{x, y}
        }).collect::<Vec<Coord>>()
    }).collect::<Vec<Vec<Coord>>>()
}

mod test {
    use crate::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_input_file(){

        let splines = parse_input("14.test");
        
        let mut f = Field::default();

        for spline in splines {
            let mut coords = spline.iter();
            let mut curr = coords.next().expect("at least two coords");
            while let Some(next) = coords.next() {
                f.paint_range(curr, next, '#');
                curr = next;
            }
        }

        f.debug(SOURCE_COORD, 'c');


        assert_eq!(24, sand_until_abyss())
    }
}