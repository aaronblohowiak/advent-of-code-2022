/* A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. */

use std::fs;
use std::str::FromStr;

#[derive(Default, Debug)]
struct Tree {
    height: isize,
    visibility: u8,
}

impl Tree {
    fn new(height: isize) -> Tree {
        let mut tree = Tree::default();
        tree.height = height;
        return tree;
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<isize>() {
            Ok(height) => Ok(Tree::new(height)),
            Err(err) => { panic!("Could not parse tree height! {:?} ", err); }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./8.input").expect("Error while reading");

    let forest: Vec<Vec<Tree>> = parse_forest(&input);
    let total = mark_forest(forest);

    println!("{:?}", total);
}

fn parse_forest(s : &str) -> Vec<Vec<Tree>> {
    s.trim().split("\n").map(|row| {
        row.split_inclusive(|_x| true).map(|a|{
             return a.parse::<Tree>().unwrap();
        }).collect()
    }).collect()
}

fn mark_forest(mut forest : Vec<Vec<Tree>>) -> isize{
    sweep(&mut forest, 1);
    reverse(&mut forest);
    sweep(&mut forest, 1);
    reverse(&mut forest); //not needed but nice to put things back for debugging :D

    forest = transpose(forest);

    sweep(&mut forest, 1);
    reverse(&mut forest);
    sweep(&mut forest, 1);
    reverse(&mut forest); //not needed but nice to put things back for debugging :D

    forest = transpose(forest);

    let total = forest.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |mut acc, x| { if x.visibility > 0 {acc += 1}; acc })
    });

    return total;
}

fn sweep(forest: &mut Vec<Vec<Tree>>, mask : u8){
    for line in forest {
        let mut hieghest = -1;
        for mut tree in line {
            if tree.height > hieghest {
                tree.visibility += mask;
                hieghest = tree.height;
            }
        }
    }
}

fn reverse(forest : &mut Vec<Vec<Tree>>){
    for line in forest {
        line.reverse();
    }
}

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


fn test_input(s: &str, expected: isize ){
    let forest: Vec<Vec<Tree>> = parse_forest(s);
    let total = mark_forest(forest);
    assert_eq!(total, expected);
}

const PROVIDED_INPUT: &str = include_str!("../8.test");

#[test]
fn test_scoring(){
    let all_visible = "123\n456\n789\n\n";

    let short_middle = "222\n212\n222\n";

    test_input(all_visible, 9);
    test_input(short_middle, 8);
    test_input(PROVIDED_INPUT, 21);

}