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

fn priority(c: char) -> i32 {
    return if c.is_lowercase() {
        c as i32 - 'a' as i32 + 1
    } else {
        c as i32 - 'A' as i32 + 27
    };
}

fn main() {
    let input = fs::read_to_string("./3.input").expect("Error while reading");

    let total = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<&str>>())
        .map(|triad| {
            let overlap: Vec<char> = triad[0]
                .chars()
                .unique()
                .chain(triad[1].chars().unique())
                .duplicates()
                .unique()
                .chain(triad[2].chars().unique())
                .duplicates()
                .collect();

            return priority(overlap[0]);
        })
        .sum::<i32>();

    dbg!(total);
}
