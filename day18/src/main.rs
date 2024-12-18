use std::{collections::HashSet, fs};

type Point = (i32, i32);

const SIZE: i32 = 70;

fn parse_byte(s: &str) -> Point {
    let mut segments = s.split(",");
    let x: i32 = segments.next().unwrap().parse().unwrap();
    let y: i32 = segments.next().unwrap().parse().unwrap();
    (x, y)
}

fn is_out_of_bounds(point: Point) -> bool {
    point.0 < 0 || point.0 > SIZE || point.1 < 0 || point.1 > SIZE
}

fn shortest_path(memory: &HashSet<Point>) -> Option<usize> {
    let start: Point = (0, 0);
    let end: Point = (SIZE, SIZE);

    let mut queue: Vec<Vec<Point>> = vec![vec![start]];
    let mut seen: HashSet<Point> = HashSet::new();
    while !queue.is_empty() {
        let path = queue.remove(0);
        let location = path.last().unwrap();
        if seen.contains(location) {
            continue;
        }
        seen.insert(*location);
        if location == &end {
            return Some(path.len() - 1);
        }

        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (location.0 + direction.0, location.1 + direction.1);
            if is_out_of_bounds(next) || memory.contains(&next) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(next);
            queue.push(new_path);
        }
    }
    None
}

fn first_blocking_byte(memory: &mut HashSet<Point>, bytes: Vec<Point>) -> Point {
    let mut low: usize = 0;
    let mut high: usize = bytes.len() - 1;
    while low <= high {
        memory.clear();
        let mid = (low + high) / 2;
        memory.extend(&bytes[..=mid]);
        let path = shortest_path(&memory);
        if path.is_none() {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    } 

    bytes[low]
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let bytes: Vec<Point> = contents.lines().map(|s| parse_byte(s)).collect();

    let mut memory: HashSet<Point> = HashSet::new();
    memory.extend(&bytes[..1024]);
    println!("Part 1: {}", shortest_path(&memory).unwrap());

    let byte = first_blocking_byte(&mut memory, bytes);
    println!("Part 2: {},{}", byte.0, byte.1);
}
