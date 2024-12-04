use regex::Regex;
use std::fs;

fn part1(program: &str) -> i64 {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    mul_regex
        .captures_iter(program)
        .map(parse_and_multiply)
        .sum()
}

fn part2(program: &str) -> i64 {
    let dont_block = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    part1(dont_block.replace_all(program, "").as_ref())
}

fn parse_and_multiply(captures: regex::Captures) -> i64 {
    let x = captures[1]
        .parse::<i64>()
        .expect("Failed to parse x in mul(x,y)");
    let y = captures[2]
        .parse::<i64>()
        .expect("Failed to parse y in mul(x,y)");
    x * y
}

pub fn solution() {
    let read_result = fs::read_to_string("input/day3.txt");

    match read_result {
        Ok(input) => {
            println!("Part 1: {}", part1(&input));
            println!("Part 2: {}", part2(&input));
        }
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
        }
    }
}
