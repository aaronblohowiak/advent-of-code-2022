use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};


use num::rational::Ratio;


type Numeric = Ratio<isize>;

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
    let mut id_interner = StringInterner::default();

    let mut monkeys: HashMap<usize, MonkeyNumber> = HashMap::default();

    std::fs::read_to_string("./21.test")
        .expect("read file")
        .lines()
        .map(|s| parse_monkey(s, &mut id_interner))
        .for_each(|(id, number)| {
            monkeys.insert(id, number);
        });

    let root_id = id_interner.get_index("root");
    println!("Part 1: {}", resolve(&root_id, &monkeys));

    {
        let root = monkeys.get_mut(&root_id).unwrap();
        match root {
            MonkeyNumber::Formulae(f) => {
                *root = MonkeyNumber::Formulae(Formulae {
                    lhs: f.lhs,
                    op: '-',
                    rhs: f.rhs,
                });
            }
            _ => {
                unreachable!("root should be a formula")
            }
        }
    }

    let humn_id = id_interner.get_index("humn");

    let mut stride: isize = 10;
    let mut current: isize = 0;

    let mut guess = attempt(&humn_id, &root_id, &mut monkeys, current);

    let mut next_guess = attempt(&humn_id, &root_id, &mut monkeys, current + stride);

    if next_guess.err.abs() > guess.err.abs() {
        //in the beginning i might be going the wrong way!
        stride *= -1;
    }

    for _i in 0..1000 {
        next_guess = attempt(&humn_id, &root_id, &mut monkeys, current + stride);

        if next_guess.err == 0.0 {
            println!("Part 2: {}", next_guess);
            return;
        }

        if next_guess.err.signum() == guess.err.signum() && next_guess.err.abs() < guess.err.abs() {
            //try doubling stride

            let double_guess = attempt(&humn_id, &root_id, &mut monkeys, current + stride * 2);
            if double_guess.err.signum() == guess.err.signum()
                && double_guess.err.abs() < guess.err.abs()
            {
                stride *= 2;
                guess = double_guess;
                current += stride;
            } else {
                current += stride;
                guess = next_guess;
            }

            continue;
        }

        //we know that the value is between current and stride.
        //just reset stride
        stride = stride.signum();
    }
}

#[derive(Copy, Clone)]
struct Check {
    value: Numeric,
    err: f64,
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value: {}, Err: {}", self.value.to_integer(), self.err)
    }
}

fn attempt(
    humn_id: &usize,
    root_id: &usize,
    monkeys: &mut HashMap<usize, MonkeyNumber>,
    guess: isize,
) -> Check {
    let guess = Ratio::from_integer(guess);
    update_humn(humn_id, monkeys, guess);

    let result = resolve(root_id, monkeys);
    let err = *result.numer() as f64 / *result.denom() as f64;
    Check {
        value: guess,
        err,
    }
}

fn update_humn(humn_id: &usize, monkeys: &mut HashMap<usize, MonkeyNumber>, guess: Numeric) {
    let humn = monkeys.get_mut(humn_id).unwrap();
    *humn = MonkeyNumber::Constant(guess);
}

fn resolve(id: &usize, monkeys: &HashMap<usize, MonkeyNumber>) -> Numeric {
    let monkey = monkeys.get(id).unwrap();

    match monkey {
        MonkeyNumber::Constant(x) => *x,
        MonkeyNumber::Formulae(f) => {
            let left = resolve(&f.lhs, monkeys);
            let right = resolve(&f.rhs, monkeys);

            match f.op {
                '+' => left + right,
                '*' => left * right,
                '/' => left / right,
                '-' => left - right,
                _ => {
                    unreachable!("only valid operators")
                }
            }
        }
    }
}

fn parse_monkey(s: &str, interner: &mut StringInterner) -> (usize, MonkeyNumber) {
    let mut parts = s.split(": ");
    let id = parts
        .next()
        .map(|id_str| interner.get_index(id_str))
        .unwrap();

    let brain = parts.next().unwrap();

    let number: MonkeyNumber;

    if let Ok(val) = brain.parse() {
        number = MonkeyNumber::Constant(val);
    } else {
        let (left, op, right) = brain
            .split(' ')
            .tuples()
            .next()
            .expect("not a constant should be a formulae.");
        let op = op.chars().next().unwrap();
        let (left, right) = (interner.get_index(left), interner.get_index(right));

        number = MonkeyNumber::Formulae(Formulae {
            lhs: left,
            op,
            rhs: right,
        });
    }

    (id, number)
}

#[derive(Debug)]
enum MonkeyNumber {
    Constant(Numeric),
    Formulae(Formulae),
}

#[derive(Debug)]

struct Formulae {
    lhs: MonkeyId,
    op: char,
    rhs: MonkeyId,
}

type MonkeyId = usize;
