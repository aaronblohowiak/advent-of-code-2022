use anyhow::Result;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::BinaryHeap;

/* Each line contains the calories contained in an item in an elves' inventory.
the inventories are separated by a blank line.

I need to print out the calorie total for the elf that has the most calories, and the sum of the top three elves.
*/

fn main() -> Result<(), anyhow::Error> {
    let mut current_tally = 0;
    let mut heap = BinaryHeap::new();

    for line in read_lines("./input/1.input")? {
        let calories = line?;

        match calories.parse::<i32>() {
            Ok(result) => current_tally += result,
            Err(_) => {
                if current_tally > 0 {
                    heap.push(current_tally);
                }
                current_tally = 0;
            }
        }
    }

    let mut top_calories;
    let mut sum = 0;
    for _ in 0..3 {
        top_calories = heap.pop().unwrap_or(0);
        sum += top_calories;
        println!("{} {}", top_calories, sum);
    }

    return Ok(());
}

//this code copy-pasted from the internet...
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
