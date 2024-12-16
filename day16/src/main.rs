use std::{
    collections::{HashMap, HashSet},
    f32::INFINITY,
    fs,
};

type Point = (i32, i32);
type State = (Vec<Point>, Point, usize);

const DIRECTIONS: [Point; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut walls: HashSet<Point> = HashSet::new();
    let (mut start, mut end) = ((-1, -1), (-1, -1));
    for (y, row) in contents.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let point: Point = (x as i32, y as i32);
            match ch {
                '#' => {
                    walls.insert(point);
                }
                'S' => start = point,
                'E' => end = point,
                _ => {}
            }
        }
    }

    let mut queue: Vec<State> = vec![(vec![start], (1, 0), 0)];
    let mut seen: HashMap<(Point, Point), usize> = HashMap::new();
    let mut min_cost = INFINITY as usize;
    let mut paths: HashMap<usize, Vec<Vec<Point>>> = HashMap::new();
    while !queue.is_empty() {
        let (path, direction, cost) = queue.remove(0);
        let position = path[path.len() - 1];
        if let Some(c) = seen.get(&(position, direction)) {
            if &cost > c {
                continue;
            }
        }
        seen.insert((position, direction), cost);

        if cost > min_cost {
            continue;
        }
        if position == end {
            paths.entry(cost).or_insert_with(Vec::new).push(path);
            min_cost = cost;
            continue;
        }

        let next_position = (position.0 + direction.0, position.1 + direction.1);

        if walls.get(&next_position).is_none() {
            let mut next_path = path.clone();
            next_path.push(next_position);
            queue.push((next_path, direction, cost + 1));
        }

        for d in DIRECTIONS {
            if d != direction {
                queue.push((path.clone(), d, cost + 1000));
            }
        }
    }
    let best_paths = paths.get(&min_cost).unwrap();

    let mut best_spaces: HashSet<Point> = HashSet::new();
    for path in best_paths {
        best_spaces.extend(path);
    }
    println!("Part 1: {}", min_cost);
    println!("Part 2: {}", best_spaces.len());
}
