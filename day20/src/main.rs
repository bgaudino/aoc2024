use std::{collections::HashSet, fs};

type Point = (i32, i32);

struct RaceTrack {
    walls: HashSet<Point>,
    max_x: i32,
    max_y: i32,
    start: Point,
    finish: Point,
}

impl RaceTrack {
    fn is_out_of_bounds(&self, point: Point) -> bool {
        point.0 < 0 || point.0 > self.max_x || point.1 < 0 || point.1 > self.max_y
    }

    fn find_track(&mut self) -> Option<Vec<Point>> {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: Vec<(Point, Vec<Point>)> = vec![(self.start, vec![self.start])];
        while !queue.is_empty() {
            let (point, path) = queue.remove(0);
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            if point == self.finish {
                return Some(path);
            }
            let (x, y) = point;
            let next_points = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            for next_point in next_points {
                if self.is_out_of_bounds(next_point) || self.walls.contains(&point) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(next_point);
                queue.push((next_point, new_path));
            }
        }
        return None;
    }
}

fn get_race_track(s: &str) -> RaceTrack {
    let mut walls = HashSet::new();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in s.lines().enumerate() {
        max_y = y as i32;
        for (x, c) in line.chars().enumerate() {
            max_x = x as i32;
            let point = (x as i32, y as i32);
            match c {
                '#' => {
                    walls.insert(point);
                }
                'S' => {
                    start = point;
                }
                'E' => {
                    finish = point;
                }
                _ => {}
            }
        }
    }
    RaceTrack {
        walls,
        max_x,
        max_y,
        start,
        finish,
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut race_track = get_race_track(&contents);
    let track = race_track.find_track().unwrap();

    let mut part_1 = 0;
    let mut part_2 = 0;
    let min_savings = 100;
    for i in 0..track.len() - min_savings {
        for j in i + min_savings + 1..track.len() {
            let (p1, p2) = (track[i], track[j]);
            let distance = manhattan_distance(p1, p2) as usize;
            let savings = j - i - distance;
            if savings >= min_savings {
                if distance == 2 {
                    part_1 += 1;
                }
                if distance <= 20 {
                    part_2 += 1;
                }
            }
        }
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
