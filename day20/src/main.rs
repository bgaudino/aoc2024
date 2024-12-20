use std::{
    collections::{HashMap, HashSet},
    f32::INFINITY,
    fs,
};

type Point = (i32, i32);
type State = (Point, usize);
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

    fn bfs(&mut self, best_time: usize, brick: Option<Point>) -> Option<usize> {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: Vec<State> = vec![(self.start, 0)];
        let mut route_costs: Vec<usize> = Vec::new();
        let mut walls = self.walls.clone();
        if let Some(brick) = brick {
            walls.remove(&brick);
        }
        while !queue.is_empty() {
            let (point, steps) = queue.remove(0);
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            if steps > best_time {
                continue;
            }
            if point == self.finish {
                route_costs.push(steps);
                return Some(steps);
            }
            let (x, y) = point;
            let next_points = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
            for next_point in next_points {
                if self.is_out_of_bounds(next_point) || walls.contains(&point) {
                    continue;
                }
                queue.push((next_point, steps + 1));
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

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut race_track = get_race_track(&contents);
    let steps = race_track.bfs(INFINITY as usize, None).unwrap();
    let mut counter: HashMap<usize, usize> = HashMap::new();
    for brick in race_track.walls.clone() {
        if let Some(cheat) = race_track.bfs(steps - 100, Some(brick)) {
            *counter.entry(steps - cheat).or_insert(0) += 1;
        }
    }
    let part_1: usize = counter.iter().map(|(_, v)| v).sum();
    println!("Part 1: {}", part_1);
}
