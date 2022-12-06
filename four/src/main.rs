//use itertools::Itertools;
use std::fs;

//returns true IFF "me" fully contains "you", but not the opposite
fn fully_contains(me: &Vec<i32>, you: &Vec<i32>) -> bool {
    return me[0] <= you[0] && me[1] >= you[1];
}

//returns true if the two 2-ary vecs overlap at all. no need to check the flip.
fn overlaps_at_all(me: &Vec<i32>, you: &Vec<i32>) -> bool {
    /* things overlap if either the start or end is contained in the other range. */

    //i do NOT love the manual +1 because of range's upper-bound exclusivity :(
    let me_r = std::ops::Range {
        start: me[0],
        end: me[1] + 1,
    };
    let you_r = std::ops::Range {
        start: you[0],
        end: you[1] + 1,
    };
    return me_r.contains(&you[0])
        || me_r.contains(&you[1])
        || you_r.contains(&me[0])
        || you_r.contains(&me[1]);
}

fn main() {
    let input = fs::read_to_string("./4.input").expect("Error while reading");

    let total = input
        .lines()
        .map(|pair| {
            pair.split(",")
                .map(|elf| {
                    elf.split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .map(|elves| {
            let fully_overlapped =
                if fully_contains(&elves[0], &elves[1]) || fully_contains(&elves[1], &elves[0]) {
                    1
                } else {
                    0
                };

            let partial_overlap = if overlaps_at_all(&elves[0], &elves[1]) {
                println!("{:?} {:?}", elves[0], elves[1]);
                1
            } else {
                0
            };

            return (fully_overlapped, partial_overlap);
        })
        .reduce({
            |mut results, current| {
                results.0 += current.0;
                results.1 += current.1;
                results
            }
        });

    println!("{:?}", total);
}
