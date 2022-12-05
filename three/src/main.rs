/*
Rucksack

Rucksack has two compartments

All items of a given type are meant to go into exactly one of the two compartments.
The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The list of items for each rucksack is given as characters all on a single line.
A given rucksack always has the same number of items in each of its two compartments,
  so the first half of the characters represent items in the first compartment,
while the second half of the characters represent items in the second compartment.

To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.

*/
use itertools::Itertools;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Rucksack {
    left: String,
    right: String,
}

impl FromStr for Rucksack {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let compartment_length = s.len() / 2;
        return Ok(Rucksack {
            left: s[..compartment_length].to_string(),
            right: s[compartment_length..].to_string(),
        });
    }
}

impl Rucksack {
    fn duplicated_type_score(&self) -> i32 {
        let mut total: i32 = 0;

        println!("new rucksack");
        let dupes: Vec<char> = self
            .left
            .chars()
            .unique()
            .chain(self.right.chars().unique())
            .duplicates()
            .dedup()
            .collect();

        for c in dupes {
            total += if c.is_lowercase() {
                c as i32 - 'a' as i32 + 1
            } else {
                c as i32 - 'A' as i32 + 27
            }
        }
        return total;
    }
}

fn main() {
    let input = fs::read_to_string("./3.input").expect("Error while reading");

    let total = input
        .lines()
        .map(|line| {
            let rucksack = line.parse::<Rucksack>().unwrap();
            println!("{:?}", rucksack);
            return rucksack.duplicated_type_score();
        })
        .sum::<i32>();

    dbg!(total);
}
