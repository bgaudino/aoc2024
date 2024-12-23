use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

    let mut all_secrets: Vec<Vec<isize>> = Vec::new();
    for buyer in &buyers {
        let mut secret = *buyer;
        let mut secrets: Vec<isize> = vec![secret];
        for _ in 0..2000 {
            secret = evolve(secret);
            secrets.push(secret);
        }
        all_secrets.push(secrets);
    }

    let part_1: isize = all_secrets
        .iter()
        .map(|secrets| secrets.last().unwrap())
        .sum();

    let mut price_maps: Vec<HashMap<[isize; 4], isize>> = Vec::new();
    let mut unique_changes: HashSet<[isize; 4]> = HashSet::new();
    for secrets in all_secrets.iter() {
        let mut price_map: HashMap<[isize; 4], isize> = HashMap::new();
        let mut changes: Vec<isize> = vec![];
        for i in 1..secrets.len() {
            let (prev, curr) = (secrets[i - 1], secrets[i]);
            let p = prev % 10;
            let c = curr % 10;
            changes.push(c - p);
            if i < 4 {
                continue;
            }
            let ch = [
                changes[i - 4],
                changes[i - 3],
                changes[i - 2],
                changes[i - 1],
            ];
            let price = price_map.get(&ch);
            if price.is_none() {
                price_map.insert(ch, secrets[i] % 10);
            }
        }
        unique_changes.extend(price_map.keys());
        price_maps.push(price_map);
    }
    println!("Part 1: {}", part_1);

    let part2: isize = unique_changes
        .iter()
        .map(|change| {
            price_maps
                .iter()
                .map(|price_map| price_map.get(change).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap();
    println!("Part 2: {}", part2);
}
