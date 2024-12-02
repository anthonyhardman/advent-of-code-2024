use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn part1(reports: &Vec<Vec<i64>>) -> i64 {
    reports.iter().filter(|report| is_safe(&report)).count() as i64
}

fn part2(reports: &Vec<Vec<i64>>) -> i64 {
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(&report))
        .count() as i64
}

fn get_reports(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solution() {
    let read_result = fs::read_to_string("input/day2.txt");

    match read_result {
        Ok(input) => {
            let reports = get_reports(&input);
            println!("Part 1: {}", part1(&reports));
            println!("Part 2: {}", part2(&reports));
        }
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
        }
    }
}

fn is_safe(report: &[i64]) -> bool {
    let differences: Vec<i64> = report.windows(2).map(|w| w[0] - w[1]).collect();

    differences.iter().copied().all(|d| (1..=3).contains(&d))
        || differences.iter().copied().all(|d| (-3..=-1).contains(&d))
}

fn is_safe_with_dampener(report: &[i64]) -> bool {
    fn is_bad_window(a: i64, b: i64, c: i64) -> bool {
        !(a < b && b < c && b - a <= 3 && c - b <= 3)
            && !(a > b && b > c && a - b <= 3 && b - c <= 3)
    }

    let bad_windows: Vec<HashSet<usize>> = report
        .iter()
        .copied()
        .enumerate()
        .tuple_windows::<(_, _, _)>()
        .filter(|&((_, a), (_, b), (_, c))| is_bad_window(a, b, c))
        .map(|((i1, _), (i2, _), (i3, _))| HashSet::from([i1, i2, i3]))
        .collect();

    match bad_windows.len() {
        0 => return is_safe(report),
        1 | 2 => bad_windows
            .into_iter()
            .reduce(|a, b| &a & &b)
            .unwrap_or_default()
            .iter()
            .any(|&idx| {
                let mut modified_report = report.to_vec();
                modified_report.remove(idx);
                is_safe(&modified_report)
            }),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example_input() -> String {
        fs::read_to_string("input/day2_example.txt").expect("Failed to read example input file")
    }

    #[test]
    fn test_example_input_for_part_1() {
        let reports = get_reports(&load_example_input());
        assert_eq!(part1(&reports), 2);
    }

    #[test]
    fn test_example_input_for_part_2() {
        let reports = get_reports(&load_example_input());
        assert_eq!(part2(&reports), 4);
    }
}
