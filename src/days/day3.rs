// mod day2

use std::cmp::min;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day3").unwrap());
    let file_input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    println!("Sum of part IDs: {}", sum_of_part_nums(&file_input));
}
