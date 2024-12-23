use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut network: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in contents.lines() {
        let mut computers = line.split("-");
        let a = computers.next().unwrap();
        let b = computers.next().unwrap();
        network.entry(a).or_insert(HashSet::new()).insert(b);
        network.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let mut triangles: HashSet<Vec<&str>> = HashSet::new();
    for (c, connections) in &network {
        for combo in connections.iter().combinations(2) {
            let (a, b) = (combo[0], combo[1]);
            if !(a.starts_with("t") || b.starts_with("t") || c.starts_with("t")) {
                continue;
            }
            if let Some(conn) = network.get(b) {
                if conn.contains(a) {
                    let mut triangle = vec![*a, *b, *c];
                    triangle.sort_unstable();
                    triangles.insert(triangle);
                }
            }
        }
    }
    println!("Part 1: {}", triangles.len());

    let max_neighbors = network.values().map(|v| v.len()).max().unwrap_or(0);
    for i in (1..=max_neighbors).rev() {
        for (computer, connections) in network.iter() {
            if connections.len() < i + 1 {
                continue;
            }
            for subset in connections.iter().copied().combinations(i) {
                if subset
                    .iter()
                    .all(|&n| subset.iter().all(|&m| n == m || network.get(m).unwrap().contains(n)))
                {
                    let mut computers: HashSet<String> = HashSet::new();
                    for n in connections.iter() {
                        computers.insert(n.to_string());
                    }
                    computers.insert(computer.to_string());
                    let mut party: Vec<String> = vec![];
                    for c in computers {
                        party.push(c);
                    }
                    party.sort_unstable();
                    let password = party.join(",");  
                    println!("Part 2: {}", password);
                    return;
                }
            }
        }
    }


}
