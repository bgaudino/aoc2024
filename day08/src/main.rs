use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs,
};

type Point = (isize, isize);

struct City {
    map: HashMap<char, Vec<Point>>,
    max_x: usize,
    max_y: usize,
}

impl City {
    fn in_bounds(&self, point: Point) -> bool {
        point.0 >= 0
            && point.1 >= 0
            && point.0 < self.max_x as isize
            && point.1 < self.max_y as isize
    }

    fn find_all_antinodes(&self, in_line: bool) -> HashSet<Point> {
        let mut antinodes: HashSet<Point> = HashSet::new();
        for (frequency, antennas) in &self.map {
            for antenna in antennas {
                antinodes.extend(self.find_antinodes(antenna, frequency, in_line));
            }
        }
        if in_line {
            for (_, antennas) in &self.map {
                antinodes.extend(antennas);
            }
        }
        antinodes
    }

    fn find_antinodes(&self, antenna: &Point, frequency: &char, in_line: bool) -> HashSet<Point> {
        let mut antinodes: HashSet<Point> = HashSet::new();
        for other in self.map.get(frequency).unwrap() {
            if other == antenna {
                continue;
            }
            let mut a = *antenna;
            let mut b = *other;
            loop {
                let antinode = self.find_antinode(a, b);
                if self.in_bounds(antinode) {
                    antinodes.insert(antinode);
                    if in_line {
                        b = a;
                        a = antinode;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        antinodes
    }

    fn find_antinode(&self, a: Point, b: Point) -> Point {
        (a.0 - (b.0 - a.0), a.1 - (b.1 - a.1))
    }
}

fn parse_city(input: &str) -> City {
    let mut map = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = max(max_y, y);

        for (x, ch) in line.chars().enumerate() {
            max_x = max(max_x, x);

            if ch != '.' {
                let point = (x as isize, y as isize);
                map.entry(ch).or_insert_with(Vec::new).push(point);
            }
        }
    }

    City {
        map,
        max_x: max_x + 1,
        max_y: max_y + 1,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let city = parse_city(&contents);

    let part_1 = city.find_all_antinodes(false).len();
    let part_2 = city.find_all_antinodes(true).len();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
