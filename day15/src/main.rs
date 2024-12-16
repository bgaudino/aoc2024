use std::{collections::HashMap, fs};

type Point = (i32, i32);
type Warehouse = HashMap<Point, char>;

const ROBOT: char = '@';
const WALL: char = '#';
const EMPTY: char = '.';
const BOX: char = 'O';
const BOX_LEFT_SIDE: char = '[';
const BOX_RIGHT_SIDE: char = ']';

const UP: char = '^';
const DOWN: char = 'v';
const LEFT: char = '<';
const RIGHT: char = '>';

fn get_warehouse(s: &str) -> (Warehouse, Point) {
    let mut location: Point = (-1, -1);
    let mut warehouse: Warehouse = Warehouse::new();
    for (y, row) in s.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            warehouse.insert((x as i32, y as i32), ch);
            if ch == ROBOT {
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
            UP => (0, -1),
            DOWN => (0, 1),
            LEFT => (-1, 0),
            RIGHT => (1, 0),
            _ => {
                panic!("Invalid direction")
            }
        })
        .collect()
}

fn gps(warehouse: &Warehouse) -> i32 {
    warehouse
        .iter()
        .map(|((x, y), ch)| match ch {
            &BOX | &BOX_LEFT_SIDE => 100 * y + x,
            _ => 0,
        })
        .sum()
}

fn expand(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            WALL => format!("{}{}", WALL, WALL),
            BOX => format!("{}{}", BOX_LEFT_SIDE, BOX_RIGHT_SIDE),
            ROBOT => format!("{}{}", ROBOT, EMPTY),
            '\n' => "\n".to_string(),
            _ => format!("{}{}", EMPTY, EMPTY),
        })
        .collect()
}

fn can_move(warehouse: &mut Warehouse, location: &Point, direction: &Point, recurse: bool) -> bool {
    let space = *warehouse.get(location).unwrap();
    let next_location = (location.0 + direction.0, location.1 + direction.1);
    let next_space = warehouse.get(&next_location).unwrap();

    if next_space == &WALL {
        return false;
    }

    if next_space != &EMPTY {
        if !can_move(warehouse, &next_location, direction, true) {
            return false;
        }
    }

    if direction.1 != 0 && recurse {
        if space == BOX_LEFT_SIDE {
            let partner_location = (location.0 + 1, location.1);
            assert_eq!(warehouse.get(&partner_location).unwrap(), &BOX_RIGHT_SIDE);
            if !can_move(warehouse, &partner_location, direction, false) {
                return false;
            }
        } else if space == BOX_RIGHT_SIDE {
            let partner_location = (location.0 - 1, location.1);
            assert_eq!(warehouse.get(&partner_location).unwrap(), &BOX_LEFT_SIDE);
            if !can_move(warehouse, &partner_location, direction, false) {
                return false;
            }
        }
    }

    true
}

fn mv(warehouse: &mut Warehouse, location: &Point, direction: &Point, recurse: bool) -> Point {
    let space = *warehouse.get(location).unwrap();
    let next_location = (location.0 + direction.0, location.1 + direction.1);
    let next_space = warehouse.get(&next_location).unwrap();

    if next_space != &EMPTY {
        mv(warehouse, &next_location, direction, true);
    }

    if direction.1 != 0 && recurse {
        if space == BOX_LEFT_SIDE {
            let partner_location = (location.0 + 1, location.1);
            assert_eq!(warehouse.get(&partner_location).unwrap(), &BOX_RIGHT_SIDE);
            mv(warehouse, &partner_location, direction, false);
        } else if space == BOX_RIGHT_SIDE {
            let partner_location = (location.0 - 1, location.1);
            assert_eq!(warehouse.get(&partner_location).unwrap(), &BOX_LEFT_SIDE);
            mv(warehouse, &partner_location, direction, false);
        }
    }

    warehouse.insert(*location, '.');
    warehouse.insert(next_location, space);
    next_location
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut segments = contents.split("\n\n");

    let (mut warehouse, mut robot) = get_warehouse(segments.next().unwrap());
    let directions = get_directions(segments.next().unwrap());
    for direction in &directions {
        if can_move(&mut warehouse, &robot, &direction, true) {
            robot = mv(&mut warehouse, &robot, direction, true);
        }
    }
    println!("Part 1: {}", gps(&warehouse));

    segments = contents.split("\n\n");
    (warehouse, robot) = get_warehouse(&expand(segments.next().unwrap()));
    for direction in &directions {
        if can_move(&mut warehouse, &robot, &direction, true) {
            robot = mv(&mut warehouse, &robot, direction, true);
        }
    }
    println!("Part 2: {}", gps(&warehouse));
}
