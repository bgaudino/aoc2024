use std::{collections::HashMap, fs};

type Point = (i32, i32);
type Warehouse = HashMap<Point, char>;

fn get_warehouse(s: &str) -> (Warehouse, Point) {
    let mut location: Point = (-1, -1);
    let mut warehouse: Warehouse = Warehouse::new();
    for (y, row) in s.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            warehouse.insert((x as i32, y as i32), ch);
            if ch == '@' {
                location = (x as i32, y as i32);
            }
        }
    }
    (warehouse, location)
}

fn get_directions(s: &str) -> Vec<Point> {
    s.chars()
        .filter(|c| c != &'\n')
        .map(|ch| match ch {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => {
                panic!("Invalid direction")
            }
        })
        .collect()
}

fn gps(warehouse: &Warehouse) -> i32 {
    warehouse
        .iter()
        .map(|((x, y), ch)| {
            if ch == &'O' || ch == &'[' {
                100 * y + x
            } else {
                0
            }
        })
        .sum()
}

fn expand(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '@' => "@.",
            '\n' => "\n",
            _ => "..",
        })
        .collect()
}

fn _print_warehouse(warehouse: &Warehouse) {
    for y in 0..50 {
        for x in 0..100 {
            print!("{}", warehouse.get(&(x, y)).unwrap());
        }
        println!()
    }
    println!()
}

fn try_move(
    warehouse: &mut Warehouse,
    location: &Point,
    direction: &Point,
    recurse: bool,
) -> Point {
    let space = *warehouse.get(location).unwrap();
    let next_location = (location.0 + direction.0, location.1 + direction.1);
    let next_space = warehouse.get(&next_location).unwrap();
    if next_space == &'#' {
        return *location;
    } else if next_space != &'.' {
        if next_location == try_move(warehouse, &next_location, direction, true) {
            return *location;
        }
    }

    if direction.1 != 0 && recurse {
        if space == '[' {
            let partner_location = (location.0 + 1, location.1);
            if partner_location == try_move(warehouse, &partner_location, direction, false) {
                return *location;
            }
        } else if space == ']' {
            let partner_location = (location.0 - 1, location.1);
            if partner_location == try_move(warehouse, &partner_location, direction, false) {
                return *location;
            }
        }
    }

    warehouse.insert(*location, '.');
    warehouse.insert(next_location, space);
    next_location
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut segments = contents.split("\n\n");

    let (mut warehouse, mut location) = get_warehouse(segments.next().unwrap());
    let directions = get_directions(segments.next().unwrap());
    for direction in &directions {
        location = try_move(&mut warehouse, &location, &direction, true);
    }
    println!("Part 1: {}", gps(&warehouse));

    segments = contents.split("\n\n");
    (warehouse, location) = get_warehouse(&expand(segments.next().unwrap()));

    for direction in &directions {
        location = try_move(&mut warehouse, &location, &direction, true);
    }
    println!("Part 2: {}", gps(&warehouse));
}
