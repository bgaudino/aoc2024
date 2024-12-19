use std::{collections::HashSet, fs, usize};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut segments = contents.split("\n\n");
    let towels: HashSet<&str> = segments.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = segments.next().unwrap().lines().collect();

    let (possible_designs, total_combinations) =
        designs
            .iter()
            .fold((0usize, 0usize), |(count, total), design| {
                let ways = count_combinations(design, &towels);
                (count + if ways > 0 { 1 } else { 0 }, total + ways)
            });

    println!("Part 1: {}", possible_designs);
    println!("Part 2: {}", total_combinations);
}

fn count_combinations(design: &str, towels: &HashSet<&str>) -> usize {
    let mut ways = vec![0; design.len() + 1];
    ways[0] = 1;

    for i in 1..=design.len() {
        for towel in towels {
            if i >= towel.len() {
                let start = i - towel.len();
                if &design[start..i] == *towel {
                    ways[i] += ways[start];
                }
            }
        }
    }

    ways[design.len()]
}
