use itertools::Itertools;
use std::{collections::{HashMap, HashSet}, iter::repeat};

type Point = (i32, i32);

fn is_valid(path: &[char], mut pos: Point, keypad: &HashMap<char, Point>) -> bool {
    for &direction in path {
        match direction {
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => (),
        }
        if !keypad.values().any(|&p| p == pos) {
            return false;
        }
    }
    true
}

fn generate_paths(
    presses: &[char],
    location: Point,
    keypad: &HashMap<char, Point>,
) -> HashSet<Vec<char>> {
    presses
        .iter()
        .cloned()
        .permutations(presses.len())
        .filter(|perm| is_valid(&perm, location, keypad))
        .collect()
}

fn find_sequences(code: &str, keypad: &HashMap<char, Point>) -> Vec<Vec<char>> {
    let mut location = keypad[&'A'];
    let mut sequences = vec![Vec::new()];

    for n in code.chars() {
        let button = keypad[&n];
        let dx = button.0 - location.0;
        let dy = button.1 - location.1;
        let mut presses = Vec::new();

        if dx > 0 {
            presses.extend(repeat('>').take(dx as usize));
        } else {
            presses.extend(repeat('<').take((-dx) as usize));
        }

        if dy > 0 {
            presses.extend(repeat('v').take(dy as usize));
        } else {
            presses.extend(repeat('^').take((-dy) as usize));
        }

        let paths = generate_paths(&presses, location, keypad);
        let mut new_sequences = Vec::new();

        for seq in &sequences {
            for path in &paths {
                let mut new_seq = seq.clone();
                new_seq.extend(path);
                new_seq.push('A');
                new_sequences.push(new_seq);
            }
        }

        sequences = new_sequences;
        location = button;
    }

    sequences
}

fn complexity(
    code: &str,
    numeric: &HashMap<char, Point>,
    directional: &HashMap<char, Point>,
) -> i32 {
    let mut codes = find_sequences(code, numeric);

    for _ in 0..2 {
        codes = codes
            .iter()
            .flat_map(|seq| find_sequences(&seq.iter().collect::<String>(), directional))
            .collect();
    }

    let seq = codes.iter().min_by_key(|s| s.len()).unwrap();
    seq.len() as i32 * code[..code.len() - 1].parse::<i32>().unwrap()
}

fn main() {
    let numeric = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let directional = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let contents = std::fs::read_to_string("input.txt").unwrap();
    let codes: Vec<&str> = contents.lines().collect();

    let part1: i32 = codes
        .iter()
        .map(|&code| complexity(code, &numeric, &directional))
        .sum();

    println!("{}", part1);
}
