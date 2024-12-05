use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn part1(orderings_set: &HashSet<(i64, i64)>, updates_list: &[Vec<i64>]) -> i64 {
    updates_list
        .iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows::<(_, _)>()
                .all(|order| orderings_set.contains(&(*order.0, *order.1)))
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(orderings_set: &HashSet<(i64, i64)>, updates_list: &[Vec<i64>]) -> i64 {
    updates_list
        .iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows::<(_, _)>()
                .any(|order| !orderings_set.contains(&(*order.0, *order.1)))
        })
        .map(|update| {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| {
                if orderings_set.contains(&(*a, *b)) {
                    std::cmp::Ordering::Less
                } else if orderings_set.contains(&(*b, *a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            sorted_update[sorted_update.len() / 2]
        })
        .sum()
}

fn get_orderings_set(orderings: &str) -> HashSet<(i64, i64)> {
    orderings
        .lines()
        .filter_map(|line| {
            line.split_once('|')
                .map(|(left, right)| (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()))
        })
        .collect()
}

fn get_updates_list(updates: &str) -> Vec<Vec<i64>> {
    updates
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect()
}

pub fn solution() {
    let read_result = fs::read_to_string("input/day5.txt");

    match read_result {
        Ok(input) => {
            let (ordering, updates) = input.split_once("\n\n").unwrap();

            let orderings_set = get_orderings_set(ordering);
            let updates_list = get_updates_list(updates);

            println!("Part 1: {}", part1(&orderings_set, &updates_list));
            println!("Part 2: {}", part2(&orderings_set, &updates_list));
        }
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
        }
    }
}
