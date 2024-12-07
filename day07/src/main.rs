use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

trait Operate {
    fn operate(&self, x: usize, y: usize) -> usize;
}

impl Operate for Operator {
    fn operate(&self, x: usize, y: usize) -> usize {
        match self {
            Operator::Add => x + y,
            Operator::Multiply => x * y,
            Operator::Concatenate => (x.to_string() + &y.to_string()).parse().unwrap(),
        }
    }
}

struct Equation {
    test_value: usize,
    initial: usize,
    numbers: Vec<usize>,
}

impl Equation {
    fn get_operators(&self, can_concatenate: bool) -> Vec<Vec<Operator>> {
        let base_ops = if can_concatenate {
            vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
        } else {
            vec![Operator::Add, Operator::Multiply]
        };
        (0..self.numbers.len())
            .map(|_| base_ops.clone())
            .multi_cartesian_product()
            .collect()
    }

    fn is_possible(&self, can_concatenate: bool) -> bool {
        self.get_operators(can_concatenate)
            .into_iter()
            .any(|op| self.calculate(op).is_ok_and(|v| v == self.test_value))
    }

    fn calculate(&self, operators: Vec<Operator>) -> Result<usize, usize> {
        operators
            .into_iter()
            .zip(self.numbers.iter())
            .try_fold(self.initial, |acc, (op, n)| {
                let result = op.operate(acc, *n);
                if result > self.test_value {
                    Err(result)
                } else {
                    Ok(result)
                }
            })
    }
}

fn get_equation(s: &str) -> Equation {
    let mut segments = s.split(": ");
    let target: usize = segments.next().unwrap().parse().unwrap();
    let numbers: Vec<usize> = segments
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Equation {
        test_value: target,
        initial: numbers[0],
        numbers: numbers[1..].to_vec(),
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let results: (usize, usize) = contents
        .par_lines()
        .map(|line| {
            let equation = get_equation(line);
            let part_1 = {
                if equation.is_possible(false) {
                    equation.test_value
                } else {
                    0
                }
            };
            let part_2 = {
                if part_1 == 0 && equation.is_possible(true) {
                    equation.test_value
                } else {
                    part_1
                }
            };
            (part_1, part_2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
