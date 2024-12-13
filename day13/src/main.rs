use regex::Regex;
use std::fs;

type Point = (isize, isize);

struct ClawMachine {
    a: Point,
    b: Point,
    prize: Point,
}

fn parse_machine(s: &str) -> ClawMachine {
    let mut lines = s.lines();
    let a = parse_point(lines.next().expect("Missing A button"));
    let b = parse_point(lines.next().expect("Missing B button"));
    let prize = parse_point(lines.next().expect("Missing prize"));
    ClawMachine { a, b, prize }
}

fn parse_point(line: &str) -> Point {
    let re = Regex::new(r"\d+").expect("Invalid regex");
    let mut digits = re
        .find_iter(line)
        .map(|m| m.as_str().parse::<isize>().expect("Invalid number"));
    let x = digits.next().expect("Missing X coordinate");
    let y = digits.next().expect("Missing Y coordinate");
    (x, y)
}

fn lowest_cost(machine: &ClawMachine, extra: isize) -> isize {
    let (ax, ay) = machine.a;
    let (bx, by) = machine.b;
    let (px, py) = (machine.prize.0 + extra, machine.prize.1 + extra);

    let d = ax * by - ay * bx;
    let a = by * px - bx * py;
    let b = ax * py - ay * px;

    if a % d != 0 || b % d != 0 {
        return 0;
    }

    let a_presses = a / d;
    let b_presses = b / d;
    let cost = 3 * a_presses + b_presses;

    cost
}

fn solve(machines: &[ClawMachine], extra: isize) -> isize {
    machines.iter().map(|m| lowest_cost(m, extra)).sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let machines: Vec<ClawMachine> = contents.split("\n\n").map(|s| parse_machine(s)).collect();

    println!("Part 1: {}", solve(&machines, 0));
    println!("Part 2: {}", solve(&machines, 10_000_000_000_000));
}
