/* i will represent the chamber with the left wall being the 7th bit. */

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use std::io::{stdout, Write};
use std::{thread, time};

type Shape = Vec<u8>;
type Chamber = Vec<u8>;

fn main() -> Result<()> {
    // assert_eq!(chamber_space_to_screen_space(0, 4), chamber_space_to_screen_space(1, 4) );

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, EnterAlternateScreen)?;

    let mut chamber = Chamber::default();

    let shapes = shape_bits();

    let input_str = std::fs::read_to_string("./17.input").expect("file should be present");

    let mut input_cycle = input_str.chars().enumerate().cycle();
    let mut shapes_cycle = shapes.iter().enumerate().cycle();

    for i in 0..2022 {
        //appear!
        let shape_pair = shapes_cycle.next().unwrap();
        let mut shape = shape_pair.1.clone();
        let mut distance_from_top = -3;

        loop {
            print_chamber_top(&chamber, Some(&shape), distance_from_top, i)?;

            //push!
            let (input_idx, input) = input_cycle
                .next()
                .expect("endless cycle of input should be endless");

            match input {
                '<' => {
                    if !shape.iter().any(|r| *r & 0b01000000u8 > 0) {
                        shape.iter_mut().for_each(|r| *r <<= 1);
                        if hit_detector(&chamber, &shape, distance_from_top) {
                            //could avoid doing this work if i had another shape to play with, but that seems annoying.
                            shape.iter_mut().for_each(|r| *r >>= 1);
                        }
                    }
                }
                '>' => {
                    if !shape.iter().any(|r| *r & 1 > 0) {
                        shape.iter_mut().for_each(|r| *r >>= 1);
                        if hit_detector(&chamber, &shape, distance_from_top) {
                            //could avoid doing this work if i had another shape to play with, but that seems annoying.
                            shape.iter_mut().for_each(|r| *r <<= 1);
                        }
                    }
                }
                _ => {
                    unreachable!("input should only have left and right moves");
                }
            }

            //fall!
            if hit_detector(&chamber, &shape, distance_from_top + 1) {
                place_block(&mut chamber, &shape, distance_from_top as usize);
                print_chamber_top(&chamber, None, 0, i)?;
                break;
            } else {
                distance_from_top += 1;
            }
        }
    }

    // let ten_millis = time::Duration::from_millis(2000);
    // thread::sleep(ten_millis);

    execute!(stdout, LeaveAlternateScreen);
    println!("{}", chamber.len());
    Ok(())
}

//used to add block to a chamber
fn place_block(chamber: &mut Chamber, shape: &Shape, distance_from_top: usize) {
    //if distance from top is 0 then i need to grow the vector by shape.len()

    let starting_y = chamber.len() - distance_from_top;
    let ending_y = starting_y + shape.len() - 1;

    if ending_y >= chamber.len() {
        chamber.resize(ending_y + 1, 0); //fill in top with zeros if needs-be
    }

    (&mut chamber[starting_y..=ending_y])
        .iter_mut()
        .zip(shape.iter())
        .for_each(|(c, s)| *c |= s);
}

//detects if a shape hits the floor or an existing rock, NOT THE WALLS.
//distance from top is how "deep" from the top the shape is sent down. if it is 0 or less, not hits possible.
//  if it is 1, then the bottom row of the shape is compared to the top row of the chamber
// if it is 2, then the bottom row is compred to the top-1 row of the chamber and if present,
//          the second from bottom row in the shape is compared to the top row of the chamber
fn hit_detector(chamber: &Chamber, shape: &Shape, distance_from_top: isize) -> bool {
    if distance_from_top <= 0 {
        //if we're above the top, no worries.
        return false;
    }

    if distance_from_top > chamber.len() as isize {
        //we cant go through the floor of the chamber!
        return true;
    }

    chamber
        .iter()
        .skip(chamber.len() - distance_from_top as usize) //set offset correctly.
        .zip(shape.iter())
        .any(|(a, b)| *a & *b > 0)
}

#[rustfmt::skip]
fn shape_bits() -> Vec<Shape> {

    let horiz = vec![
        0b0011110u8];

    let plus = vec![
        0b0001000u8,
        0b0011100u8,
        0b0001000u8];

    let j = vec![
        0b0000100u8,
        0b0000100u8,
        0b0011100u8];

    let i = vec![
        0b0010000u8,
        0b0010000u8,
        0b0010000u8,
        0b0010000u8];

    let o = vec![
        0b0011000u8,
        0b0011000u8];

    //these shapes are created visually which has an inverted index from how computers think 
    //  (we want the bottom of the shape to be index 0)
    //  so after creation, we need to reverse them.
    let mut res = vec![horiz, plus, j, i, o];
    res.iter_mut().for_each(|v| v.reverse());
    res
}

/*
    DISPLAY CODE FOLLOWS, YOU CAN IGNORE IT.
*/

const DISPLAY_HEIGHT: usize = 40;
const ROWS_TO_SHOW: usize = 30;

fn display_row(mut n: u8) -> String {
    let mut s: String = "".to_string();
    let mask = 0b01000000u8;
    for _ in 0..7 {
        if n & mask > 0 {
            s += "█";
        } else {
            s += ".";
        }
        n <<= 1;
    }
    s
}

//we want to draw the top N lines of the chamber at a given position.
// we want the bottom of the display to be at the same position (for the aesthetics)
fn chamber_space_to_screen_space(y: isize, chamber_height: usize) -> u16 {
    let top_buffer = (DISPLAY_HEIGHT - ROWS_TO_SHOW) as isize;

    //if the chamber is shorter than our window, shift its top further down so we grow up before scrolling down.
    if chamber_height <= ROWS_TO_SHOW {
        return ((ROWS_TO_SHOW as isize - y) + top_buffer) as u16;
    }

    // if the chamber is taller than our window, "grow down"
    ((chamber_height as isize - y) + top_buffer) as u16
}

fn print_chamber_top(
    chamber: &Chamber,
    shape: Option<&Shape>,
    distance_from_top: isize,
    rock_idx: usize,
) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        style::PrintStyledContent(("Rock Num:".to_string() + &rock_idx.to_string()).white())
    )?;

    queue!(
        stdout,
        cursor::MoveTo(0, 1),
        style::PrintStyledContent(("Height: ".to_string() + &chamber.len().to_string()).white())
    )?;

    for y in 6..=DISPLAY_HEIGHT {
        queue!(
            stdout,
            cursor::MoveTo(19, y as u16),
            style::PrintStyledContent("|.......|".red())
        )?;
    }

    for (y, row) in chamber.iter().enumerate().rev().take(ROWS_TO_SHOW as usize) {
        let screen_y = chamber_space_to_screen_space(y as isize, chamber.len());

        queue!(
            stdout,
            cursor::MoveTo(20, screen_y),
            style::PrintStyledContent(display_row(*row).dark_blue())
        )?;

        queue!(
            stdout,
            cursor::MoveTo(0, screen_y),
            style::PrintStyledContent(y.to_string().yellow())
        )?;
    }

    if let Some(shape) = shape {
        let mask = 0b01000000u8;

        for row in 0..shape.len() {
            let mut n = shape[row];
            let y = chamber_space_to_screen_space(
                (chamber.len() + row) as isize - distance_from_top,
                chamber.len(),
            );

            for x in 0..7 {
                if n & mask > 0 {
                    queue!(
                        stdout,
                        cursor::MoveTo(20 + x, y),
                        style::PrintStyledContent("█".red())
                    )?;
                }
                n <<= 1;
            }
        }
    };

    queue!(stdout, cursor::MoveTo(0, 40))?;
    stdout.flush()?;

    // let sleep_millis = time::Duration::from_millis(15);
    // thread::sleep(sleep_millis);

    Ok(())
}
