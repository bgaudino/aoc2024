use std::fs;

const SIZE: usize = 5;
type Schematic = [usize; SIZE];

fn possibly_fit(lock: &Schematic, key: &Schematic) -> bool {
    lock.iter().zip(key.iter()).all(|(lock, key)| lock + key <= SIZE)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut keys: Vec<Schematic> = Vec::new();
    let mut locks: Vec<Schematic> = Vec::new();

    for part in contents.split("\n\n") {
        let mut lines = part.lines();
        let is_key = lines.next().unwrap().contains("#");
        let lines: Vec<&str> = {
            if is_key {
                lines.collect()
            } else {
                let mut lines = lines.rev();
                lines.next().unwrap();
                lines.collect()
            }
        };

        let mut schematic: Schematic = [0; SIZE];
        for line in lines {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    schematic[x] += 1;
                }
            }
        }

        if is_key {
            keys.push(schematic);
        } else {
            locks.push(schematic);
        }
    }

    let mut possible_fits = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if possibly_fit(lock, key) {
                possible_fits += 1;
            }
        }
    }
    println!("Part 1: {}", possible_fits);
}
