use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Point = (i32, i32);
type Side = (Point, Point);

static HORIZONTAL: [Point; 2] = [(0, -1), (0, 1)];
static VERTICAL: [Point; 2] = [(1, 0), (-1, 0)];
static DIRECTIONS: [Point; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn measure(map: &HashMap<Point, char>, point: Point, ch: char) -> (HashSet<Point>, HashSet<Side>, usize) {
    let mut plot: HashSet<Point> = HashSet::new();
    let mut fence: HashSet<Side> = HashSet::new();
    let mut stack: Vec<Point> = vec![point];
    let mut perimeter = 0;
    while !stack.is_empty() {
        let point = stack.pop().unwrap();
        if plot.contains(&point) {
            continue;
        }
        plot.insert(point);
        for direction in DIRECTIONS {
            let next_point = (point.0 + direction.0, point.1 + direction.1);
            let next_ch = map.get(&next_point).unwrap_or(&' ');
            if *next_ch == ch {
                if !plot.contains(&next_point) {
                    stack.push(next_point);
                }
            } else {
                fence.insert((point, direction));
                perimeter += 1;
            }
        }
    }
    (plot, fence, perimeter)
}

fn follow_side(fence: &HashSet<Side>, side: Side) -> HashSet<Side> {
    let mut sides: HashSet<Side> = HashSet::new();
    let mut stack: Vec<Side> = vec![side];
    while !stack.is_empty() {
        let (side, direction) = stack.pop().unwrap();
        if sides.contains(&(side, direction)) {
            continue;
        }
        sides.insert((side, direction));
        let directions = {
            if direction.0 == 0 {
                VERTICAL
            } else {
                HORIZONTAL
            }
        };
        for new_direction in directions {
            let new_side = ((side.0 + new_direction.0, side.1 + new_direction.1), direction);
            if fence.contains(&new_side) {
                stack.push(new_side);
            }
        }
    }
    sides 
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut map: HashMap<Point, char> = HashMap::new();

    for (y, row) in contents.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let point = (x as i32, y as i32);
            map.insert(point, ch);
        }
    }

    let (mut part_1, mut part_2) = (0, 0);
    let mut seen: HashSet<Point> = HashSet::new();
    for (point, ch) in &map {
        if seen.contains(point) {
            continue;
        }
        let (plot, fence, perimeter) = measure(&map, *point, *ch);
        seen.extend(&plot);
        part_1 += plot.len() * perimeter;

        let mut sides: HashSet<Side> = HashSet::new();
        let mut s = 0;
        for segment in &fence {
            if sides.contains(&segment) {
                continue;
            }
            let side = follow_side(&fence, *segment);
            sides.extend(&side);
            s += 1;
        }
        part_2 += plot.len() * s;
    }
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
