use core::fmt;
use std::{ops::Add, collections::HashSet, fs};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    type Output = Self;
}

static DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Guard {
    position: Point,
    facing: usize,
    directions: [Point; 4],
}

impl Guard {
    fn walk(&mut self) {
        self.position = self.position + self.directions[self.facing];
    }

    fn rotate(&mut self) {
        self.facing = (self.facing + 1) % 4;
    }

    fn preview(&mut self) -> Self {
        let mut guard = self.clone();
        guard.walk();
        guard
    }
}

struct NorthPoleProtoTypeManufacturingLab {
    max_x: isize,
    max_y: isize,
    guard: Guard,
    obstructions: HashSet<Point>,
    visited: HashSet<Point>,
    orientations: HashSet<Guard>,
}

impl NorthPoleProtoTypeManufacturingLab {
    fn new() -> Self {
        Self {
            max_x: 1,
            max_y: 2,
            guard: Guard {
                position: Point { x: 0, y: 0 },
                facing: 0,
                directions: DIRECTIONS,
            },
            obstructions: HashSet::new(),
            visited: HashSet::new(),
            orientations: HashSet::new(),
        }
    }

    fn move_guard(&mut self) {
        let mut next = self.guard.preview();
        while self.obstructions.contains(&next.position) {
            self.guard.rotate();
            next = self.guard.preview();
        }
        self.guard.walk();
        if self.guard_is_in_area() {
            self.visited.insert(self.guard.position);
        }
    }

    fn guard_is_in_area(&mut self) -> bool {
        let point = self.guard.position;
        point.x >= 0 && point.y >= 0 && point.x < self.max_x && point.y < self.max_y
    }

    fn is_loop(&mut self) -> bool {
        self.find_guard_route();
        self.orientations.contains(&self.guard)
    }

    fn find_guard_route(&mut self) {
        while self.guard_is_in_area() && !self.orientations.contains(&self.guard) {
            self.orientations.insert(self.guard);
            self.move_guard();
        }
    }
}

impl fmt::Debug for NorthPoleProtoTypeManufacturingLab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let point = Point { x, y };
                if self.obstructions.contains(&point) {
                    result.push('#');
                } else if point == self.guard.position {
                    result.push('^');
                } else if self.visited.contains(&point) {
                    result.push('X');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

fn get_lab(s: &str) -> NorthPoleProtoTypeManufacturingLab {
    let mut lab = NorthPoleProtoTypeManufacturingLab::new();
    for (y, row) in s.lines().enumerate() {
        if y == 0 {
            lab.max_x = row.len() as isize;
        }
        lab.max_y = (y + 1) as isize;
        for (x, char) in row.chars().enumerate() {
            match char {
                '#' => {
                    lab.obstructions.insert(Point {
                        x: x as isize,
                        y: y as isize,
                    });
                }
                '^' => {
                    lab.guard.position = Point {
                        x: x as isize,
                        y: y as isize,
                    };
                    lab.visited.insert(lab.guard.position);
                }
                _ => (),
            };
        }
    }
    lab
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lab = get_lab(&contents);
    lab.find_guard_route();
    println!("Part 1: {}", lab.visited.len());

    let mut count = 0;
    for position in lab.visited.iter() {
        let mut new_lab = get_lab(&contents);
        new_lab.obstructions.insert(*position);
        if new_lab.is_loop() {
            count += 1;
        }
    }
    println!("Part 2: {}", count);
}
