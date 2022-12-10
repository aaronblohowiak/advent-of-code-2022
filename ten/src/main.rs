use std::{fs, ops::RangeInclusive};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
    Noop,
    Addx(i32),
}

use crate::Command::*;

#[derive(Clone, Copy, Debug)]
struct ComputerState {
    X: i32,
    processing: Option<Command>,
}

fn process(hist: &mut Vec<ComputerState>, cmd: Command) {
    let end = hist.len() - 1;

    let head = hist
        .get_mut(end)
        .expect("can not process an empty state, please.");
    //set the current tick to be procesing the command
    head.processing = Some(cmd);
    let reg = head.X;
    let cpy = head.clone();

    match cmd {
        Noop => {
            hist.push(ComputerState {
                X: reg,
                processing: None,
            }); //push next state.
        }
        Addx(addend) => {
            hist.push(cpy); //duplicate state.
            hist.push(ComputerState {
                X: reg + addend,
                processing: None,
            }); //record state after complete
        }
    }
}

fn parse(s: &str) -> impl Iterator<Item = Command> + '_ {
    return s.lines().map(|line| {
        if line == "noop" {
            Noop
        } else {
            //get the second element in line and parse as a signed int.
            let x: i32 = line
                .split(" ")
                .skip(1)
                .next()
                .expect("Addx should have a value")
                .parse()
                .expect("Addx param should parse to i32");
            Addx(x)
        }
    });
}

fn debug(hist: &Vec<ComputerState>) {
    for (i, x) in hist.iter().enumerate() {
        println!("IDx: {} Value: {} Processing: {:?}", i, x.X, x.processing);
    }
}

#[test]
fn test_process_simple() {
    let mut hist = Vec::new();
    hist.push(ComputerState {
        X: 1,
        processing: None,
    });

    process(&mut hist, Noop);
    process(&mut hist, Addx(3));
    process(&mut hist, Addx(-5));

    assert_eq!(hist[1].X, 1);
    assert_eq!(hist[3].X, 4);
    assert_eq!(hist[5].X, -1);

    println!("{:?}", hist);
    // assert_eq!(1, -1);
}

fn signal_strength(hist: &Vec<ComputerState>, nth: usize) -> isize {
    return hist[nth - 1].X as isize * nth as isize;
}

fn process_file(s: &str) -> Vec<ComputerState> {
    let input = fs::read_to_string(s).expect("Error while reading");
    let mut hist = Vec::new();
    hist.push(ComputerState {
        X: 1,
        processing: None,
    });

    for cmd in parse(&input) {
        process(&mut hist, cmd);
    }

    return hist;
}

/*
    the sprite is 3 pixels wide, and the X register sets the horizontal position of the middle of that sprite.
    (In this system, there is no such thing as "vertical position": if the sprite's horizontal position puts
        its pixels where the CRT is currently drawing, then those pixels will be drawn.

*/
fn render(hist: &Vec<ComputerState>) -> String {
    let mut result = "".to_string();

    for (i, state) in hist.iter().enumerate() {
        let x = i % 40;

        if x == 0 {
            result += "\n";
        }

        result += if RangeInclusive::new(state.X - 1, state.X + 1).contains(&(x as i32)) {
            "#"
        } else {
            "."
        };
    }

    return result.trim().to_string();
}

fn main() {
    let hist = process_file("10.input");

    //Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. What is the sum of these six signal strengths?
    let total: isize = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|nth| signal_strength(&hist, *nth))
        .sum();
    println!("Part One: {}", total);

    println!("{}", render(&hist));
}

#[test]
fn test_parse() {
    let input = "noop\naddx 3\naddx -5\n";
    let parsed: Vec<Command> = parse(input).collect();

    assert_eq!(parsed, vec![Noop, Addx(3), Addx(-5)]);
}

#[test]
fn test_file() {
    /*
    The interesting signal strengths can be determined as follows:

        During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
        During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
        During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
        During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
        During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
        During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.

    */
    let mut hist = process_file("10.test");

    assert_eq!(hist[20 - 1].X, 21);
    assert_eq!(signal_strength(&hist, 20), 420);
    assert_eq!(hist[60 - 1].X, 19);
    assert_eq!(signal_strength(&hist, 60), 1140);
    assert_eq!(hist[100 - 1].X, 18);
    assert_eq!(signal_strength(&hist, 100), 1800);
    assert_eq!(hist[140 - 1].X, 21);
    assert_eq!(hist[180 - 1].X, 16);
    assert_eq!(hist[220 - 1].X, 18);

    let rendered = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    hist.pop(); //we dont need last state. this is a hack to get it to work

    assert_eq!(&render(&hist), rendered);
}
