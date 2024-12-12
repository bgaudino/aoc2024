use std::{collections::HashMap, fs};

type Counter = HashMap<usize, usize>;

fn blink(stones: &Counter) -> Counter {
    let mut new_stones: Counter = HashMap::new();
    for (stone, count) in stones {
        if *stone == 0 {
            increment(&mut new_stones, 1, *count);
        } else {
            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let (a, b) = s.split_at(s.len() / 2);
                increment(&mut new_stones, a.parse().unwrap(), *count);
                increment(&mut new_stones, b.parse().unwrap(), *count);
            } else {
                increment(&mut new_stones, stone * 2024, *count);
            }
        }
    }
    new_stones
}

fn increment(stones: &mut Counter, stone: usize, count: usize) {
    *stones.entry(stone).or_insert(0) += count;
}

fn num_stones(stones: &Counter) -> usize {
    stones.values().sum()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut stones: Counter = HashMap::new();
    contents
        .split_whitespace()
        .map(|stone| stone.parse::<usize>().unwrap())
        .for_each(|stone| {
            increment(&mut stones, stone, 1);
        });

    for i in 1..=75 {
        stones = blink(&stones);
        if i == 25 {
            println!("Part 1: {}", num_stones(&stones));
        }
    }
    println!("Part 2: {}", num_stones(&stones));
}
