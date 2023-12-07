// mod day2

use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

fn check_for_adjacent_part(input: &Vec<String>, row_idx: usize, match_idx: usize) -> bool {
    let num_cols = input[row_idx].len();
    let check = |c: char| c != '.' && !c.is_numeric();
    let check_upper = &input[row_idx.saturating_sub(1)]
        [match_idx.saturating_sub(1)..min(match_idx + 2, num_cols)];
    let check_middle = &input[row_idx][match_idx.saturating_sub(1)..min(match_idx + 2, num_cols)];
    let check_lower = &input[min(row_idx + 1, input.len() - 1)]
        [match_idx.saturating_sub(1)..min(match_idx + 2, num_cols)];

    //println!("{} {} {}", check_upper, check_middle, check_lower);
    check_upper.contains(check) || check_middle.contains(check) || check_lower.contains(check)
}

fn sum_of_part_nums(input: &Vec<String>) -> u32 {
    let mut part_nums: Vec<u32> = Vec::new();
    for row_idx in 0..input.len() {
        let line = &input[row_idx];
        let (part_num, _, adjacent) = line.match_indices(char::is_numeric).fold(
            (0u32, 0usize, false),
            |(part_num, position, is_adjacent), (match_idx, digit)| {
                if (match_idx - position) <= 1 {
                    return (
                        10 * part_num + digit.parse::<u32>().unwrap(),
                        match_idx,
                        is_adjacent || check_for_adjacent_part(input, row_idx, match_idx),
                    );
                } else if is_adjacent {
                    part_nums.push(part_num);
                }
                (
                    digit.parse::<u32>().unwrap(),
                    match_idx,
                    check_for_adjacent_part(input, row_idx, match_idx),
                )
            },
        );

        if adjacent {
            part_nums.push(part_num);
        }
    }

    part_nums.iter().sum()
}

fn get_enclosing_range_from_subrange(
    row: &str,
    begin_idx: usize,
    end_idx: usize,
) -> (usize, usize) {
    let mut new_begin = begin_idx;
    // move left border
    if row.chars().nth(begin_idx).unwrap().is_numeric() {
        for _ in row[0..begin_idx]
            .chars()
            .rev()
            .take_while(|c| c.is_numeric())
        {
            new_begin -= 1;
        }
    }

    let mut new_end = end_idx;

    // move right border
    if row.chars().nth(end_idx).unwrap().is_numeric() {
        for _ in row[(end_idx + 1)..row.len()]
            .chars()
            .take_while(|c| c.is_numeric())
        {
            new_end += 1;
        }
    }

    (new_begin, new_end)
}

fn sum_of_gear_ratios(input: &Vec<String>) -> u32 {
    let mut gear_ratios: Vec<u32> = Vec::new();
    for row_idx in 0..input.len() {
        gear_ratios.extend(
            input[row_idx]
                .match_indices(|c: char| c == '*')
                .map(|(match_idx, _)| get_gear_ratio(input, row_idx, match_idx)),
        );
    }

    gear_ratios.iter().sum()
}

fn get_gear_ratio(input: &Vec<String>, row_idx: usize, match_idx: usize) -> u32 {
    let num_cols = input[row_idx].len();
    let mut part_nums: Vec<u32> = Vec::new();
    if row_idx > 0 {
        let (up_l, up_r) = get_enclosing_range_from_subrange(
            &input[row_idx - 1],
            match_idx.saturating_sub(1),
            min(match_idx + 1, num_cols),
        );

        part_nums.extend(
            input[row_idx - 1][up_l..=up_r]
                .split(|c: char| !c.is_numeric())
                .filter_map(|s: &str| s.parse::<u32>().ok()),
        );
    }

    let (m_l, m_r) = get_enclosing_range_from_subrange(
        &input[row_idx],
        match_idx.saturating_sub(1),
        min(match_idx + 1, num_cols),
    );

    part_nums.extend(
        input[row_idx][m_l..=m_r]
            .split(|c: char| !c.is_numeric())
            .filter_map(|s: &str| s.parse::<u32>().ok()),
    );

    if row_idx < (input.len() - 1) {
        let (low_l, low_r) = get_enclosing_range_from_subrange(
            &input[row_idx + 1],
            match_idx.saturating_sub(1),
            min(match_idx + 1, num_cols),
        );

        part_nums.extend(
            input[row_idx + 1][low_l..=low_r]
                .split(|c: char| !c.is_numeric())
                .filter_map(|s: &str| s.parse::<u32>().ok()),
        );
    }

    if part_nums.len() == 2 {
        return part_nums[0] * part_nums[1];
    }

    0
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day3").unwrap());
    let file_input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    println!("Sum of part IDs: {}", sum_of_part_nums(&file_input));
    println!("Sum of gear ratios: {}", sum_of_gear_ratios(&file_input));
}
