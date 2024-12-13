use std::{collections::HashSet, fs};

use regex::Regex;

type Point = (usize, usize);
type State = (Point, usize);

#[derive(Debug)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_machine(s: &str) -> ClawMachine {
    let mut lines = s.lines();
    let a = parse_point(lines.next().unwrap());
    let b = parse_point(lines.next().unwrap());
    let prize = parse_point(lines.next().unwrap());
    ClawMachine{
        a, b, prize
    }
}

fn parse_point(line: &str) -> Point {
    let re = Regex::new(r"\d+").expect("Invalid regex");
    let mut digits = re
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap());
    let x = digits.next().unwrap();
    let y = digits.next().unwrap();
    (x, y)
}

fn lowest_cost(machine: &ClawMachine) -> usize {
    let mut queue: Vec<State> = vec![((0, 0), 0)];
    let mut lowest_cost: Option<usize> = None;
    let mut seen: HashSet<State> = HashSet::new();
    while !queue.is_empty() {
        let state = queue.remove(0);
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        let (claw, cost) = state;
        if claw.0 > machine.prize.0 || claw.1 > machine.prize.1 {
            continue;
        }
        if let Some(lowest) = lowest_cost {
            if cost >= lowest {
                continue;
            } 
        }
        if claw == machine.prize {
            lowest_cost = Some(cost);
        }
        let push_b = ((claw.0 + machine.b.0, claw.1 + machine.b.1), cost + 1);
        let push_a = ((claw.0 + machine.a.0, claw.1 + machine.a.1), cost + 3);
        queue.extend(vec![push_b, push_a]);
    }
    lowest_cost.unwrap_or(0)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let part_1: usize = contents.split("\n\n").map(|s| lowest_cost(&parse_machine(s))).sum();
    println!("Part 1: {}", part_1);
}
