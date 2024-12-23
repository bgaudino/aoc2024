use std::{collections::{HashMap, HashSet}, fs};
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap(); 

    let mut network: HashMap::<&str, HashSet<&str>> = HashMap::new();
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

    println!("{:?}", triangles.len());
}
