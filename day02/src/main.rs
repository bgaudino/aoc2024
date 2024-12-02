use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut safe_count = 0;
    let mut safe_count_with_problem_dampener = 0;

    for line in contents.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();

        if is_safe(&report) {
            safe_count += 1;
            safe_count_with_problem_dampener += 1;
        } else {
            for i in 0..report.len() {
                let new_report = dampen_problem(&report, i);
                if is_safe(&new_report) {
                    safe_count_with_problem_dampener += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1: {safe_count}");
    println!("Part 2: {safe_count_with_problem_dampener}");
}

fn is_safe(report: &[i32]) -> bool {
    let prev_increasing = report[1] > report[0];
    for i in 1..report.len() {
        let diff = (report[i - 1] - report[i]).abs();
        let increasing = report[i] > report[i - 1];
        if diff == 0 || diff > 3 || increasing != prev_increasing {
            return false;
        }
    }
    true
}

fn dampen_problem(report: &[i32], index: usize) -> Vec<i32> {
    let mut new_report = report.to_vec();
    new_report.remove(index);
    new_report
}
