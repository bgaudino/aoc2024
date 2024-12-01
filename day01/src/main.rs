use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("day01.txt").unwrap();
    let mut list_1: Vec<usize> = vec![];
    let mut list_2: Vec<usize> = vec![];
    for line in contents.lines() {
        let location_ids: Vec<usize> = line
            .split_whitespace()
            .map(|id| id.parse::<usize>().unwrap())
            .collect();
        list_1.push(location_ids[0]);
        list_2.push(location_ids[1]);
    }
    list_1.sort_unstable();
    list_2.sort_unstable();

    let total_distance: usize = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Part 1: {total_distance}");

    let mut occurences: HashMap<usize, usize> = HashMap::new();
    for id in list_2 {
        *occurences.entry(id).or_insert(0) += 1;
    }

    let similarity: usize = list_1
        .iter()
        .map(|id| id * occurences.get(id).unwrap_or(&0))
        .sum();
    println!("Part 2: {similarity}");
}
