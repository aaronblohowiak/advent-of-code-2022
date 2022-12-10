use std::{collections::HashMap, fs};

type Coord = (i32, i32);

#[derive(Debug)]
struct Item {
    pos: Coord,
    label: char,
}

impl Item {
    fn update(&mut self, dir: Coord) {
        self.pos.0 += dir.0;
        self.pos.1 += dir.1;
    }

    /*
        If the head is ever two steps directly up, down, left, or right from the tail, the tail must also move one step in that direction so it remains close enough:

        Otherwise, if the head and tail aren't touching and aren't in the same row or column, the tail always moves one step diagonally to keep up:

        You just need to work out where the tail goes as the head follows a series of motions. Assume the head and the tail both start at the same position, overlapping.
    */

    fn chase(&mut self, other: Coord) {
        let mut diffx = other.0 - self.pos.0;
        let mut diffy = other.1 - self.pos.1;

        //if we are less than 2 steps in any direction, we are touching.
        if diffx.abs() < 2 && diffy.abs() < 2 {
            return;
        }

        //we only move 0 or 1 steps towards our target, but we dont know the direction, so normalize by abs value.
        if diffx != 0 {
            diffx = diffx / diffx.abs();
        }

        if diffy != 0 {
            diffy = diffy / diffy.abs();
        }

        self.pos.0 += diffx;
        self.pos.1 += diffy;
        return;
    }
}

fn debug(knots: &Vec<Item>) {
    let mut positions: HashMap<Coord, &Item> = HashMap::new();
    for knt in knots {
        positions.insert(knt.pos, knt);
    }

    for y in (-15..30).rev() {
        for x in (-15..27) {
            if let Some(knt) = positions.get(&(x, y)) {
                print!("{}", knt.label);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn unit_vector(s: &str) -> Coord {
    match s {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("unknown direction"),
    }
}

const NUM_KNOTS: u32 = 10;
fn main() {
    println!("Hello, world!");

    let input = fs::read_to_string("./9.input").expect("Error while reading");

    let lines = input.lines();

    let mut knots: Vec<Item> = Vec::new();

    for i in 0..(NUM_KNOTS) {
        knots.push(Item {
            pos: (0, 0),
            label: i.to_string().chars().next().unwrap(), //beter way to get "1" ?
        });
    }

    let tail_pos = knots.len() - 1;

    if let Some(h) = knots.get_mut(0) {
        h.label = 'H';
    }

    if let Some(t) = knots.get_mut(tail_pos) {
        t.label = 'T';
    }

    let v = Item {
        pos: (0, 0),
        label: '#',
    };

    let mut visited: HashMap<Coord, &Item> = HashMap::new();

    for line in lines {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let cnt = parts.next().unwrap().parse::<i32>().unwrap();

        println!("== {} {} == ", dir, cnt);

        let unit = unit_vector(dir);
        for _ in 0..cnt {
            let mut c: Coord = (0, 0);

            if let Some(h) = knots.get_mut(0) {
                h.update(unit);
                c = h.pos;
            }

            for k in &mut knots {
                k.chase(c);
                c = k.pos;
            }

            if let Some(t) = knots.get(tail_pos) {
                visited.insert(t.pos, &v);
            }
        }

        debug(&knots);
    }

    println!("{}", visited.len());
}
