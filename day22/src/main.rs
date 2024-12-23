use std::{collections::HashSet, fs};

fn evolve(secret: isize) -> isize {
    let mut secret = secret;

    secret = mix(secret, secret * 64);
    secret = prune(secret);

    secret = mix(secret, secret / 32);
    secret = prune(secret);

    secret = mix(secret, secret * 2048);
    secret = prune(secret);

    secret
}

fn mix(secret: isize, value: isize) -> isize {
    value ^ secret
}

fn prune(secret: isize) -> isize {
    secret % 16777216
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let buyers: Vec<isize> = contents
        .lines()
        .map(|buyer| buyer.parse::<isize>().unwrap())
        .collect();

    let mut secrets_: Vec<Vec<isize>> = Vec::new();
    for buyer in &buyers {
        let mut secret = *buyer;
        let mut secrets: Vec<isize> = vec![secret];
        for _ in 0..2000 {
            secret = evolve(secret);
            secrets.push(secret);
        }
        secrets_.push(secrets);
    }

    let part_1: isize = secrets_.iter().map(|secrets| secrets.last().unwrap()).sum();

    let mut all_changes: Vec<Vec<[isize; 4]>> = Vec::new();
    let mut unique: HashSet<[isize; 4]> = HashSet::new();
    for secrets in secrets_.iter() {
        let mut changes: Vec<isize> = vec![];
        let mut change: Vec<[isize; 4]> = Vec::new();
        for i in 1..secrets.len() {
            let (prev, curr) = (secrets[i - 1], secrets[i]);
            let p = prev % 10;
            let c = curr % 10;
            changes.push(c - p);
            if i >= 4 {
                change.push([
                    changes[i - 4],
                    changes[i - 3],
                    changes[i - 2],
                    changes[i - 1],
                ]);
            }
        }
        unique.extend(change.clone());
        all_changes.push(change.clone());
    }
    println!("Part 1: {}", part_1);

    let part2 = unique.iter().map(|change| {
        let mut price = 0;
        for (i, changes) in all_changes.iter().enumerate() {
            for (j, c) in changes.iter().enumerate() {
                if change == c {
                    let secret = secrets_[i][j + 4];
                    price += secret % 10;
                    break;
                }
            }
        }
        price
    }).max().unwrap();
    println!("Part 2: {}", part2);
}
