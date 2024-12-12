use std::{
    collections::{HashMap, HashSet},
    fs,
};

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn measure(
    map: &HashMap<(i32, i32), char>,
    point: (i32, i32),
    ch: char,
) -> (HashSet<(i32, i32)>, i32) {
    let (mut area, mut perimeter) = (0, 0);
    let mut plot: HashSet<(i32, i32)> = HashSet::new();
    let mut stack: Vec<(i32, i32)> = vec![point];
    while !stack.is_empty() {
        let point = stack.pop().unwrap();
        if plot.contains(&point) {
            continue;
        }
        plot.insert(point);
        area += 1;
        for direction in DIRECTIONS {
            let next_point = (point.0 + direction.0, point.1 + direction.1);
            let next_ch = map.get(&next_point).unwrap_or(&' ');
            if *next_ch == ch {
                if !plot.contains(&next_point) {
                    stack.push(next_point);
                }
            } else {
                perimeter += 1;
            }
        }
    }
    (plot, area * perimeter)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut map: HashMap<(i32, i32), char> = HashMap::new();

    for (y, row) in contents.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let point = (x as i32, y as i32);
            map.insert(point, ch);
        }
    }

    let mut part_1 = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    for (point, ch) in &map {
        if seen.contains(point) {
            continue;
        }
        let (plot, price) = measure(&map, *point, *ch);
        seen.extend(plot);
        part_1 += price;
    }
    println!("Part 1: {}", part_1);
}
