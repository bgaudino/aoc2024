use std::fs;

fn evolve(secret: usize) -> usize {
    let mut secret = secret;

    secret = mix(secret, secret * 64);
    secret = prune(secret);

    secret = mix(secret, secret / 32);
    secret = prune(secret);

    secret = mix(secret, secret * 2048);
    secret = prune(secret);

    secret
}

fn mix(secret: usize, value: usize) -> usize {
    value ^ secret
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn simulate(secret: usize, days: usize) -> usize {
    let mut secret = secret;
    for _ in 0..days {
        secret = evolve(secret);
    }
    secret
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let part_1: usize = contents
        .lines()
        .map(|buyer| {
            let secret = buyer.parse::<usize>().unwrap();
            simulate(secret, 2000)
        })
        .sum();
    println!("Part 1: {}", part_1);
}
