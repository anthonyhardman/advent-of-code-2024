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
        || differences
            .iter()
            .copied()
            .all(|d| (-3..=-1).contains(&d))
}


fn is_safe_with_dampener(report: &[i64]) -> bool {
    (0..report.len()).any(|i| {
        let sub_report: Vec<_> = report
            .iter()
            .enumerate()
            .filter_map(|(j, &level)| if j != i { Some(level) } else { None })
            .collect();
        is_safe(&sub_report)
    })
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
