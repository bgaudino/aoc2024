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

fn find_space(disk_map: &Vec<String>, size: &usize) -> Option<usize> {
    let mut start: Option<usize> = None;
    for (i, char) in disk_map.iter().enumerate() {
        if char == "." {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(s) = start {
                if i - s >= *size {
                    return Some(s);
                } else {
                    start = None
                }
            } else {
                start = None
            }
        }
    }
    None
}

fn move_files(disk_map: &mut Vec<String>) {
    let mut current: Option<&str> = None;
    let mut end = disk_map.len() - 1;
    for (i, ch) in disk_map.clone().iter().enumerate().rev() {
        if let Some(id) = current {
            if ch != id {
                let start = i + 1;
                let l = end - start;
                if let Some(space) = find_space(&disk_map, &l) {
                    if space < start {
                        disk_map.splice(space..space + l, vec![id.to_string(); l]);
                        disk_map.splice(start..end, vec![".".to_string(); l]);
                    }
                }
                current = {
                    if ch == "." {
                        None
                    } else {
                        end = i + 1;
                        Some(ch)
                    }
                }
            }
        } else if ch != "." {
            end = i + 1;
            current = Some(ch)
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut disk_map = expand(&contents);
    move_blocks(&mut disk_map);
    println!("Part 1: {}", calculate_checksum(disk_map));

    disk_map = expand(&contents);
    move_files(&mut disk_map);
    println!("Part 2: {}", calculate_checksum(disk_map));
}
