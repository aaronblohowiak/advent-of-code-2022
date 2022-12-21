use itertools::Itertools;
use std::{collections::HashMap};

#[derive(Default)]
struct StringInterner {
    s_to_i: HashMap<String, usize>,
    i_to_s: HashMap<usize, String>,
}

impl StringInterner {
    fn get_index(&mut self, s: &str) -> usize {
        if let Some(idx) = self.s_to_i.get(s) {
            return *idx;
        }
        let idx = self.s_to_i.len();
        self.s_to_i.insert(s.to_owned(), idx);
        self.i_to_s.insert(idx, s.to_owned());
        idx
    }
}

fn main() {
    println!("Hello, world!");

    let mut id_interner = StringInterner::default();

    let mut monkeys: HashMap<usize, MonkeyNumber> = HashMap::default();

    std::fs::read_to_string("./21.input")
        .expect("read file")
        .lines()
        .map(|s| parse_monkey(s, &mut id_interner))
        .for_each(|(id, number)| {
            monkeys.insert(id, number);
        });

    let root_id = id_interner.get_index("root");

    println!("{}", resolve(root_id, &monkeys));
}

fn resolve(id: usize, monkeys: &HashMap<usize, MonkeyNumber>) -> f64 {
    let monkey = monkeys.get(&id).unwrap();

    match monkey {
        MonkeyNumber::Constant(x) => *x,
        MonkeyNumber::Formulae(f) => {
            let left = resolve(f.Lhs, monkeys);
            let right = resolve(f.Rhs, monkeys);

            let res = match f.Operator {
                '+' => left + right,
                '*' => left * right,
                '/' => left / right,
                '-' => left - right,
                _ => {
                    unreachable!("only valid operators")
                }
            };

            res
        }
    }
}

fn parse_monkey(s: &str, interner: &mut StringInterner) -> (usize, MonkeyNumber) {
    let mut parts = s.split(": ");
    let id = parts
        .next()
        .map(|id_str| interner.get_index(id_str))
        .unwrap();

    let mut brain = parts.next().unwrap();

    let mut number: MonkeyNumber;

    if let Ok(val) = brain.parse() {
        number = MonkeyNumber::Constant(val);
    } else {
        let (left, op, right) = brain
            .split(" ")
            .tuples()
            .next()
            .expect("not a constant should be a formulae.");
        let op = op.chars().next().unwrap();
        let (left, right) = (interner.get_index(left), interner.get_index(right));

        number = MonkeyNumber::Formulae(Formulae {
            Lhs: left,
            Operator: op,
            Rhs: right,
        });
    }

    return (id, number);
}

#[derive(Debug)]
enum MonkeyNumber {
    Constant(f64),
    Formulae(Formulae),
}

#[derive(Debug)]

struct Formulae {
    Lhs: MonkeyId,
    Operator: char,
    Rhs: MonkeyId,
}

#[derive(Debug)]

enum MonkeyRef {
    Unresolved(MonkeyId),
    Resolved(f64),
}

type MonkeyId = usize;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::{map, opt, value},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
