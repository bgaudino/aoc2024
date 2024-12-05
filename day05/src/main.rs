use std::cmp::Ordering;
use std::fs;

struct Rule {
    before: usize,
    after: usize,
}

impl Rule {
    fn validate(&self, update: &[usize]) -> bool {
        match (
            update.iter().position(|&n| n == self.before),
            update.iter().position(|&n| n == self.after),
        ) {
            (Some(bi), Some(ai)) => ai > bi,
            _ => true,
        }
    }
}

fn parse_rule(s: &str) -> Rule {
    let numbers: Vec<usize> = s.split('|').map(|n| n.parse().unwrap()).collect();
    Rule {
        before: numbers[0],
        after: numbers[1],
    }
}

fn is_valid(update: &[usize], rules: &[Rule]) -> bool {
    rules.iter().all(|rule| rule.validate(update))
}

fn find_middle(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn to_sorted(update: &[usize], rules: &[Rule]) -> Vec<usize> {
    let mut sorted = update.to_vec();
    sorted.sort_by(|a, b| {
        for rule in rules {
            if rule.before == *a && rule.after == *b {
                return Ordering::Less;
            }
            if rule.before == *b && rule.after == *a {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
    sorted
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut segments = contents.split("\n\n");

    let rules: Vec<Rule> = segments.next().unwrap().lines().map(parse_rule).collect();
    let updates: Vec<Vec<usize>> = segments
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut part_1: usize = 0;
    let mut part_2: usize = 0;

    for update in updates.iter() {
        if is_valid(update, &rules) {
            part_1 += find_middle(update);
        } else {
            part_2 += find_middle(&to_sorted(update, &rules));
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
