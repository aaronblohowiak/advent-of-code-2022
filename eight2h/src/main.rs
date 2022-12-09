use std::fs;
use std::iter;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        return Position { x: x, y: y };
    }
}

//shorthand.
fn p(x: isize, y: isize) -> Position {
    Position::new(x, y)
}

#[derive(Debug)]
struct CarteseanWalker {
    pos: Position,
    stride: Position,
    bounds_x: RangeInclusive<isize>,
    bounds_y: RangeInclusive<isize>,
}

impl CarteseanWalker {
    //creates a new walker that has positive bounds inclusive of max
    // if stride is negative, starts at max
    fn new(max: Position, stride: Position) -> CarteseanWalker {
        let mut start = Position::new(0, 0);
        if stride.x < 0 {
            start.x = max.x;
        }
        if stride.y < 0 {
            start.y = max.y;
        }

        CarteseanWalker {
            pos: start,
            stride: stride,
            bounds_x: RangeInclusive::new(0, max.x),
            bounds_y: RangeInclusive::new(0, max.y),
        }
    }
}

impl iter::Iterator for CarteseanWalker {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.bounds_x.contains(&self.pos.x) && self.bounds_y.contains(&self.pos.y) {
            let result = Some(self.pos);
            self.pos.x += self.stride.x;
            self.pos.y += self.stride.y;
            result
        } else {
            None
        };
    }
}

type Forest = Vec<Vec<isize>>;

//from the internet. thank you, internet.
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

//go from a row-oriented file of digits to a column-oriented [[isize]]
fn parse_forest(s: &str) -> Forest {
    transpose(
        s.trim()
            .split("\n")
            .map(|row| {
                row.split_inclusive(|_x| true)
                    .map(|a| {
                        return a.parse::<isize>().unwrap();
                    })
                    .collect()
            })
            .collect(),
    )
}

//return the inclusive bounds of the forest's indices
fn forest_bounds(forest: &Forest) -> Position {
    Position::new((forest[0].len() - 1) as isize, (forest.len() - 1) as isize)
}

//TODO make Vec<Vec<isize>> a type and add this as a trait on that type?
fn get(forest: &Forest, pos: &Position) -> isize {
    forest[pos.x as usize][pos.y as usize]
}

fn score_direction(forest: &Forest, candidate: &Position, direction: Position) -> isize {
    let mut walker = CarteseanWalker::new(forest_bounds(forest), direction);
    walker.pos.x = candidate.x;
    walker.pos.y = candidate.y;

    walker.next().unwrap(); //skip starting location.

    let height = get(forest, candidate);
    let mut score = 0;

    while let Some(pos) = walker.next() {
        score += 1;
        if get(forest, &pos) >= height {
            break;
        }
    }

    return score;
}

fn score(forest: &Forest, candidate: &Position) -> isize {
    let left = score_direction(forest, candidate, p(-1, 0));
    let right = score_direction(forest, candidate, p(1, 0));
    let up = score_direction(forest, candidate, p(0, -1));
    let down = score_direction(forest, candidate, p(0, 1));

    return left * right * up * down;
}

fn score_forest(forest: &Forest) -> isize {
    let bounds = forest_bounds(&forest);

    return (0..bounds.y)
        .map(|row| {
            let mut walker = CarteseanWalker::new(bounds, p(1, 0));
            walker.pos.y = row;

            walker
                .map(|tree| score(forest, &tree))
                .max()
                .expect("row should have at least one tree to score")
        })
        .max()
        .expect("forest should have trees to score");
}

fn main() {
    let input = fs::read_to_string("./8.input").expect("Error while reading");

    let forest = parse_forest(&input);

    println!("{:?}", score_forest(&forest));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_stride() {
        let down = CarteseanWalker::new(Position::new(4, 3), Position::new(0, 1));

        let result: Vec<Position> = down.collect();

        let should = vec![p(0, 0), p(0, 1), p(0, 2), p(0, 3)];
        assert_eq!(result, should);

        let right = CarteseanWalker::new(Position::new(4, 3), Position::new(1, 0));
        let result: Vec<Position> = right.collect();
        let should = vec![p(0, 0), p(1, 0), p(2, 0), p(3, 0), p(4, 0)];
        assert_eq!(result, should);

        let left = CarteseanWalker::new(Position::new(4, 3), Position::new(-1, 0));

        let result: Vec<Position> = left.collect();
        let should = vec![p(4, 0), p(3, 0), p(2, 0), p(1, 0), p(0, 0)];
        assert_eq!(result, should);
    }

    #[test]
    fn test_input() {
        let forest = parse_forest(&PROVIDED_INPUT);
        println!("{:?}", forest);

        let left = CarteseanWalker::new(forest_bounds(&forest), Position::new(-1, 0));
        let result: Vec<isize> = left
            .map(|pos| forest[pos.x as usize][pos.y as usize])
            .collect();

        println!("{:?}", result);

        assert_eq!(result, vec![3, 7, 3, 0, 3]);
    }

    #[test]
    fn test_view_score() {
        let forest = parse_forest(&PROVIDED_INPUT);

        let view_score = score(&forest, &Position::new(2, 1));
        assert_eq!(view_score, 4);

        let view_score = score(&forest, &Position::new(2, 3));
        assert_eq!(view_score, 8);
    }

    #[test]
    fn test_score_forest() {
        let forest = parse_forest(&PROVIDED_INPUT);
        assert_eq!(score_forest(&forest), 8);
    }

    const PROVIDED_INPUT: &str = include_str!("../8.test");
}
