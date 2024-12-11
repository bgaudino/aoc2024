use std::{
    collections::{HashMap, HashSet},
    fs,
};

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn trail_score(map: &HashMap<(i32, i32), i32>, head: (i32, i32)) -> (usize, usize) {
    let mut stack = vec![(head, vec![head])];
    let mut unique = HashSet::new();
    let mut all_paths = 0;

    while let Some((location, path)) = stack.pop() {
        let next_height = path.len() as i32;
        for direction in DIRECTIONS {
            let point = (location.0 + direction.0, location.1 + direction.1);
            if let Some(&height) = map.get(&point) {
                if height == next_height {
                    let mut new_path = path.clone();
                    new_path.push(point);
                    if height == 9 {
                        unique.insert(point);
                        all_paths += 1;
                    } else {
                        stack.push((point, new_path));
                    }
                }
            }
        }
    }
    (unique.len(), all_paths)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut map = HashMap::new();
    let mut trail_heads = Vec::new();

    for (y, row) in contents.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let point = (x as i32, y as i32);
            let height = ch.to_digit(10).unwrap() as i32;
            map.insert(point, height);
            if height == 0 {
                trail_heads.push(point);
            }
        }
    }

    let (part_1, part_2) = trail_heads
        .iter()
        .map(|&head| trail_score(&map, head))
        .fold((0, 0), |(sum_unique, sum_all), (unique, all)| {
            (sum_unique + unique, sum_all + all)
        });

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
