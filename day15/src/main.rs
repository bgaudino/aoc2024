use std::{collections::HashMap, fs};

type Point = (i32, i32);
type Warehouse = HashMap<Point, char>;

static LANTERNFISH: char = '@';
static SPACE: char = '.';
static WALL: char = '#';
static BOX: char = 'O';

fn get_direction(ch: char) -> Point {
    match ch {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => {
            panic!("Invalid direction")
        }
    }
}

fn _print_warehouse(warehouse: &Warehouse) {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", warehouse.get(&(x, y)).unwrap());
        }
        println!()
    }
    println!()
}

fn try_move(warehouse: &mut Warehouse, location: &Point, direction: &Point) -> Point {
    let space = *warehouse.get(location).unwrap();
    let next_location = (location.0 + direction.0, location.1 + direction.1);
    let next_space = warehouse.get(&next_location).unwrap();
    if *next_space == WALL {
        return *location;
    } else if *next_space != SPACE {
        if next_location == try_move(warehouse, &next_location, direction) {
            return *location;
        }
    }
    warehouse.insert(*location, SPACE);
    warehouse.insert(next_location, space);
    next_location
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut segments = contents.split("\n\n");

    let mut location: Point = (-1, -1);
    let mut warehouse: Warehouse = Warehouse::new();
    for (y, row) in segments.next().unwrap().lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            warehouse.insert((x as i32, y as i32), ch);
            if ch == LANTERNFISH {
                location = (x as i32, y as i32);
            }
        }
    }

    let directions: Vec<Point> = segments
        .next()
        .unwrap()
        .chars()
        .filter(|c| c != &'\n')
        .map(|ch| get_direction(ch))
        .collect();

    for direction in directions {
        location = try_move(&mut warehouse, &location, &direction);
    }

    let part_1: i32 = warehouse.iter().map(|((x, y), ch)| {
        if *ch == BOX {
            100 * y + x
        } else {
            0
        }
    }).sum();

    println!("Part 1: {}", part_1);
}
