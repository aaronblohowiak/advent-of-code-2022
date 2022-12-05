/* The winner of the whole tournament is the player with the highest score.
 Your total score is the sum of your scores for each round.
 The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
 plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
*/
use std::fs;

#[derive(Debug)]
enum HandShape {
    ROCK,
    PAPER,
    SCISSORS,
}

impl HandShape {
    fn vs(&self, other: &HandShape) -> i32 {
        match (self, other) {
            (HandShape::ROCK, HandShape::PAPER) => 0,
            (HandShape::ROCK, HandShape::SCISSORS) => 6,
            (HandShape::PAPER, HandShape::SCISSORS) => 0,
            (HandShape::PAPER, HandShape::ROCK) => 6,
            (HandShape::SCISSORS, HandShape::ROCK) => 0,
            (HandShape::SCISSORS, HandShape::PAPER) => 6,
            _ => 3,
        }
    }

    fn throw_value(&self) -> i32 {
        return match self {
            HandShape::ROCK => 1,
            HandShape::PAPER => 2,
            HandShape::SCISSORS => 3,
        };
    }
}

fn main() {
    let input = fs::read_to_string("./2.input").expect("Error while reading");

    let mut line_count = 0;

    let mut total: i32 = 0;
    let mut opponant_total: i32 = 0;

    input.lines().for_each(|line| {
        let mut round = line.split(" ").map({
            |shape| {
                return match shape {
                    "A" | "X" => HandShape::ROCK,
                    "B" | "Y" => HandShape::PAPER,
                    "C" | "Z" => HandShape::SCISSORS,
                    _ => panic!("Invalid input {}", shape),
                };
            }
        });

        let you = round.next().unwrap();
        let me = round.next().unwrap();

        line_count += 1;
        total = total + me.throw_value() + me.vs(&you);
        opponant_total = opponant_total + you.throw_value() + you.vs(&me);
        println!(
            "{} {:?} {:?} {} {} {} {}",
            line_count,
            me,
            you,
            me.throw_value(),
            me.vs(&you),
            total,
            opponant_total
        );
    });

    println!("{}", total)
}
