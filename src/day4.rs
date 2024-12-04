use std::{fs, usize};

fn part1(crossword: &[Vec<char>]) -> i64 {
    let mut count = 0;

    for row in 0..crossword.len() {
        for col in 0..crossword[row].len() {
            count += get_xmas_count_for_position(crossword, (row, col));
        }
    }

    count
}

fn part2(crossword: &[Vec<char>]) -> i64 {
    let mut count = 0;

    for row in 0..crossword.len() {
        for col in 0..crossword[row].len() {
            if is_middle_of_x_mas(&crossword, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn get_xmas_count_for_position(crossword: &[Vec<char>], position: (usize, usize)) -> i64 {
    match crossword[position.0][position.1] {
        'X' => [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ]
        .into_iter()
        .filter(|dir| is_start_of_xmas(crossword, position, dir))
        .count() as i64,
        _ => 0,
    }
}

fn is_start_of_xmas(crossword: &[Vec<char>], pos: (usize, usize), dir: &(isize, isize)) -> bool {
    "XMAS"
        .chars()
        .enumerate()
        .map(|(idx, ch)| {
            (
                pos.0 as isize + idx as isize * dir.0,
                pos.1 as isize + idx as isize * dir.1,
                ch,
            )
        })
        .all(|(row, col, ch)| {
            row >= 0
                && col >= 0
                && row < crossword.len() as isize
                && col < crossword[0].len() as isize
                && crossword[row as usize][col as usize] == ch
        })
}

fn is_middle_of_x_mas(crossword: &[Vec<char>], row: usize, col: usize) -> bool {
    if crossword[row][col] != 'A'
        || row == 0
        || row + 1 >= crossword.len()
        || col == 0
        || col + 1 >= crossword[0].len()
    {
        return false;
    }

    let corners = [
        crossword[row - 1][col - 1],
        crossword[row - 1][col + 1],
        crossword[row + 1][col - 1],
        crossword[row + 1][col + 1],
    ];

    let valid_patterns = [
        ['M', 'M', 'S', 'S'],
        ['S', 'S', 'M', 'M'],
        ['M', 'S', 'M', 'S'],
        ['S', 'M', 'S', 'M'],
    ];

    valid_patterns.iter().any(|&pattern| corners == pattern)
}

pub fn solution() {
    let read_result = fs::read_to_string("input/day4.txt");

    match read_result {
        Ok(input) => {
            let crossword: Vec<Vec<char>> =
                input.lines().map(|line| line.chars().collect()).collect();
            println!("Part 1: {}", part1(&crossword));
            println!("Part 2: {}", part2(&crossword));
        }
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
        }
    }
}
