use std::fs;

type Point = (i32, i32);

static DIRECTIONS: [Point; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

struct WordSearch {
    grid: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
}

impl WordSearch {
    fn from_string(s: String) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|s| s.chars().collect()).collect();
        Self {
            max_x: grid[0].len() as i32,
            max_y: grid.len() as i32,
            grid,
        }
    }

    fn out_of_bounds(&self, point: Point) -> bool {
        point.0 < 0 || point.0 >= self.max_x || point.1 < 0 || point.1 >= self.max_y
    }

    fn get(&self, point: Point) -> char {
        if self.out_of_bounds(point) {
            '.'
        } else {
            self.grid[point.1 as usize][point.0 as usize]
        }
    }

    fn find_xmas(&self, point: Point, direction: Point) -> bool {
        let mut x = point.0;
        let mut y = point.1;

        for char in "XMAS".chars() {
            if self.get((x, y)) != char {
                return false;
            }
            x += direction.0;
            y += direction.1;
        }

        true
    }

    fn find_all_xmas(&self, point: Point) -> usize {
        DIRECTIONS
            .iter()
            .filter(|d| self.find_xmas(point, **d))
            .count()
    }

    fn find_x_mas(&self, point: Point) -> bool {
        if self.get(point) != 'A' {
            return false;
        }

        let check_diagonal = |a: Point, b: Point| -> bool {
            let chars = (self.get(a), self.get(b));
            chars == ('M', 'S') || chars == ('S', 'M')
        };

        let top_left = (point.0 - 1, point.1 - 1);
        let top_right = (point.0 - 1, point.1 + 1);
        let bottom_right = (point.0 + 1, point.1 + 1);
        let bottom_left = (point.0 + 1, point.1 - 1);

        check_diagonal(top_left, bottom_right) && check_diagonal(top_right, bottom_left)
    }
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let word_search = WordSearch::from_string(contents);

    let mut xmas_count = 0;
    let mut x_mas_count = 0;
    for y in 0..word_search.max_y {
        for x in 0..word_search.max_x {
            xmas_count += word_search.find_all_xmas((x, y));
            x_mas_count += word_search.find_x_mas((x, y)) as usize;
        }
    }

    println!("Part 1: {}", xmas_count);
    println!("Part 2: {}", x_mas_count);
}
