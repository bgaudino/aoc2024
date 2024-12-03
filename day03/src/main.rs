use regex::Regex;
use std::fs;

fn main() {
    let instructions = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", process(&instructions));

    let enabled = filter_disabled(&instructions);
    println!("Part 2: {}", process(&enabled));
}

fn process(instructions: &str) -> i32 {
    static MUL_PATTERN: &str = r"mul\(\d+,\d+\)";
    let re = Regex::new(MUL_PATTERN).unwrap();

    re.find_iter(instructions)
        .map(|m| multiply(m.as_str()))
        .sum()
}

fn multiply(instruction: &str) -> i32 {
    static DIGITS_PATTERN: &str = r"\d+";
    let re = Regex::new(DIGITS_PATTERN).unwrap();

    re.find_iter(instruction)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .product()
}

fn filter_disabled(instructions: &str) -> String {
    instructions
        .split("don't()")
        .enumerate()
        .flat_map(|(i, segment)| {
            let skip = if i == 0 { 0 } else { 1 };
            segment.split("do()").skip(skip)
        })
        .collect()
}
