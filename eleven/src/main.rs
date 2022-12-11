//Goal: Count the total number of times each monkey inspects items
use std::{fs, collections::VecDeque};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    op: Operation,
    test: u32,
    t: u32,
    f: u32,
    inspection_count: u32
}

#[derive(Debug, PartialEq)]
struct Operation {
    operator: Operator,
    operand: Operand
}

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Old,
    Number(u32)
}

impl Operand {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self>{
        alt((
            value(Operand::Old, tag("old")),
            map(nom::character::complete::u32, Self::Number)
        ))(i)
    }
}


impl Operation {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Operator::parse,
                preceded(space1, Operand::parse),
            )),
            |(operator, operand)| Self { operator, operand },
        )(i)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Plus,
    Times
}

impl Operator {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self>{
        alt((
            value(Operator::Plus, tag("+")),
            value(Operator::Times, tag("*"))
        ))(i)
    }
}

fn turn(m: &mut Monkey, troop: &mut HashMap<u32, Monkey>){
    while let Some(item) = m.items.pop_front() {
        let mut item = item;
        m.inspection_count += 1;

        let operand = match m.op.operand {
            Operand::Old => item,
            Operand::Number(n) => n
        };

        item = match m.op.operator {
            Operator::Plus => item + operand,
            Operator::Times => item * operand
        };

        item /= 3;

        let catcher = match item % m.test {
            0 => m.t,
            _ => m.f
        };

        troop
            .get_mut(&catcher)
            .expect("catcher monkey by id not present.")
            .items.push_back(item);
    }
}


fn round(troop: &mut HashMap<u32, Monkey>){
    for i in 0..troop.len(){
        let mut current = troop.remove(&(i as u32)).unwrap();
        turn(&mut current, troop);
        troop.insert(current.id, current);
    }
}


fn debug(troop: &HashMap<u32, Monkey>){
    for i in 0..troop.len(){
        if let Some(m) = troop.get(&(i as u32)) {
            println!("Monkey {} {}: {:?}", m.id, m.inspection_count, m.items);
        }
    }

    println!("\n");
}

trait VecExt {
    fn sorted_rev(self) -> Self;
}

impl<T> VecExt for Vec<T>
where
    T: std::cmp::Ord,
{
    fn sorted_rev(mut self) -> Self {
        self.sort();
        self.reverse();
        self
    }
}

fn monkey_business(troop: &HashMap<u32, Monkey>) -> u32{
    troop.values()
        .map(|s| s.inspection_count)
        .collect::<Vec<_>>()
        .sorted_rev().iter()
        .take(2)
        .fold(1, |acc, x| acc * *x)
}

fn main() {    
    let mut troop = parse_file("11.input");

    for _ in 0..20 {
        round(&mut troop);
        debug(&troop);
    }

    println!("monkey business: {} ", monkey_business(&troop));
}

use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{space1},
    combinator::{map, opt, value},
    multi::many1,
    sequence::{preceded, terminated, delimited, tuple},
    IResult
};

fn parse_monkey_id(i: &str) -> IResult<&str, u32>{
    delimited(tag("Monkey "), nom::character::complete::u32,tag(":\n"))(i)
}


fn parse_starting_items(i: &str) -> IResult<&str, Vec<u32>>{
    delimited(tag("  Starting items: "),
        many1(
                terminated( nom::character::complete::u32, opt(tag(", ")))
        ),
        tag("\n"))(i)
}

fn parse_operation(i : &str) -> IResult<&str, Operation>{
    delimited(tag("  Operation: new = old "), Operation::parse, tag("\n"))(i)
}

fn parse_divisible_by(i: &str) -> IResult<&str, u32>{
    delimited(tag("  Test: divisible by "), nom::character::complete::u32,tag("\n"))(i)
}

fn parse_branch(i: &str) -> IResult<&str, u32>{
    //    If true: throw to monkey 2
    // return 2
    //    If false: throw to monkey 3
    // return 3
    delimited(
        preceded(
            tag("    If "), 
        alt((tag("true"),tag("false")))),
        preceded(tag(": throw to monkey "), 
            nom::character::complete::u32),
    opt(tag("\n")))(i)
}

fn parse_monkey(i: &str) -> Monkey{
 
    let (rest, id) = parse_monkey_id(i).unwrap();

    let (rest, items) = parse_starting_items(rest).unwrap();

    let (rest, op) = parse_operation(rest).unwrap();

    let (rest, test) = parse_divisible_by(rest).unwrap();

    let (rest, t) = parse_branch(rest).unwrap();
    let (_rest, f) = parse_branch(rest).unwrap();

    Monkey{
        id,
        op,
        test,
        t,
        f,
        items: items.into(),
        inspection_count: 0
    }
}

fn parse_file(fname: &str) -> HashMap<u32, Monkey> {
    let input = fs::read_to_string(fname).expect("Error while reading");

    let result : HashMap<u32, Monkey>= input.split("\n\n").map(parse_monkey).map(|m| (m.id, m)).collect();
    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parse(){
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        assert_eq!(parse_monkey_id(input).unwrap().1, 0);

        assert_eq!(parse_starting_items("  Starting items: 79, 98\n").unwrap().1, vec![79u32, 98u32]);

        assert_eq!(parse_operation("  Operation: new = old * 19\n").unwrap().1, Operation{operator: Operator::Times, operand: Operand::Number(19)});

        assert_eq!(parse_divisible_by("  Test: divisible by 23\n").unwrap().1, 23);

        assert_eq!(parse_branch("    If false: throw to monkey 3\n").unwrap().1, 3);

        assert_eq!(parse_monkey(input), Monkey{
            id:0,
            items: vec![79,98].into(),
            op: Operation{operator: Operator::Times, operand: Operand::Number(19)},
            test: 23,
            t: 2,
            f:3,
            inspection_count: 0
        });


    }
}