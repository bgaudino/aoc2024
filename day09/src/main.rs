use std::fs;

fn expand(disk_map: &String) -> Vec<String> {
    let mut blocks: Vec<String> = vec![];
    let mut id: usize = 0;
    for (i, char) in disk_map.chars().enumerate() {
        let n: usize = char.to_string().parse().unwrap();
        let s = {
            if i % 2 == 0 {
                let s = id.to_string();
                id += 1;
                s
            } else {
                ".".to_string()
            }
        };
        blocks.extend(vec![s; n]);
    }
    blocks
}

fn move_blocks(disk_map: &mut Vec<String>) {
    let mut i = 0;
    'out: for (j, s) in disk_map.clone().iter().enumerate().rev() {
        if s == "." {
            continue;
        }
        for k in i..disk_map.len() {
            if disk_map[k] == "." {
                if k < j {
                    disk_map[k] = s.to_string();
                    disk_map[j] = ".".to_string();
                    i = k;
                    continue 'out;
                }
                return;
            }
        }
        return;
    }
}

fn calculate_checksum(disk_map: Vec<String>) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(|(position, id)| id.parse::<usize>().unwrap_or(0) * position)
        .sum()
}

fn main() {
    let disk_map = fs::read_to_string("input.txt").unwrap();
    let mut expanded = expand(&disk_map);
    move_blocks(&mut expanded);
    println!("Part 1: {}", calculate_checksum(expanded));
}
