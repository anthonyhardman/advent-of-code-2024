use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn part1(list1: &[u32], list2: &[u32]) -> u32 {
    list1
        .iter()
        .sorted()
        .zip(list2.iter().sorted())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn part2(list1: &[u32], list2: &[u32]) -> u32 {
    let weights: HashMap<u32, u32> = list2.iter().copied().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += num;
        acc
    });

    list1
        .iter()
        .map(|num| weights.get(&num).unwrap_or(&0))
        .sum()
}

pub fn solution() {
    let read_result = fs::read_to_string("input/day1.txt");

    match read_result {
        Ok(input) => {
            let (list1, list2) = partition_input(&input);
            println!("Part 1: {}", part1(&list1, &list2));
            println!("Part 2: {}", part2(&list1, &list2));
        }
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
        }
    }
}

fn partition_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n";

    #[test]
    fn test_exaample_input_for_part_1() {
        let (list1, list2) = partition_input(EXAMPLE_INPUT);
        assert_eq!(part1(&list1, &list2), 11);
    }

    #[test]
    fn test_exaample_input_for_part_2() {
        let (list1, list2) = partition_input(EXAMPLE_INPUT);
        assert_eq!(part2(&list1, &list2), 31);
    }
}
