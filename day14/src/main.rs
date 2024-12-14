use std::{collections::HashMap, fs};

use regex::Regex;

static MAX_X: i32 = 101;
static MAX_Y: i32 = 103;

type Point = (i32, i32);
type Floor = HashMap<Point, Vec<Point>>;

fn plot(floor: &Floor) -> String {
    let mut s = String::new();
    for y in 0..MAX_Y {
        for x in 0..MAX_X {
            if let Some(robots) = floor.get(&(x, y)) {
                s.push_str(&robots.len().to_string());
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    s
}

fn parse_robot(line: &str) -> (Point, Point) {
    let re = Regex::new(r"-?\d+").unwrap();
    let mut digits = re
        .find_iter(line)
        .map(|m| m.as_str().parse::<i32>().unwrap());
    let px = digits.next().unwrap();
    let py = digits.next().unwrap();
    let vx = digits.next().unwrap();
    let vy = digits.next().unwrap();
    ((px, py), (vx, vy))
}

fn move_robot(mut pos: Point, vel: Point) -> Point {
    pos = ((pos.0 + vel.0) % MAX_X, (pos.1 + vel.1) % MAX_Y);
    if pos.0 < 0 {
        pos.0 = MAX_X + pos.0;
    }
    if pos.1 < 0 {
        pos.1 = MAX_Y + pos.1;
    }
    pos
}

fn move_robots(floor: &Floor) -> Floor {
    let mut new_floor: Floor = HashMap::new();
    for (pos, velocities) in floor {
        for velocity in velocities {
            let robot = move_robot(*pos, *velocity);
            new_floor
                .entry(robot)
                .or_insert_with(Vec::new)
                .push(*velocity);
        }
    }
    new_floor
}

fn safety_factor(floor: &Floor) -> usize {
    let mid_x = MAX_X / 2;
    let mid_y = MAX_Y / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_right = 0;
    let mut bottom_left = 0;

    for ((x, y), robots) in floor {
        if *x < mid_x {
            if *y < mid_y {
                top_left += robots.len();
            } else if *y > mid_y {
                bottom_left += robots.len();
            }
        } else if *x > mid_x {
            if *y < mid_y {
                top_right += robots.len();
            } else if *y > mid_y {
                bottom_right += robots.len();
            }
        }
    }
    top_left * bottom_left * bottom_right * top_right
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut floor: Floor = HashMap::new();

    for line in contents.lines() {
        let robot = parse_robot(line);
        floor.entry(robot.0).or_insert_with(Vec::new).push(robot.1);
    }

    // let mut seen: HashMap<String, usize> = HashMap::new();
    let mut seconds = 1;
    loop {
        floor = move_robots(&floor);
        if seconds == 100 {
            println!("Part 1: {}", safety_factor(&floor));
        }

        // let s = plot(&floor);
        if seconds == 8280 {
            println!("Part 2: {}", seconds);
            println!("{}", plot(&floor));
            break;
        }
        // if seen.get(&s).is_some() {
        //     panic!("Oops, missed it");
        // }
        // seen.insert(s, seconds);
        seconds += 1;
    }
}
