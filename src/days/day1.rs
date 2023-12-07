// mod day1

use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day1").unwrap());
    println!("Sum of calibration : {}", read_calibration(reader));
    println!(
        "Part 2 sum of calibration : {}",
        read_calibration_spelled(BufReader::new(File::open("data/input_day1").unwrap()))
    );
}

fn read_calibration<R>(reader: R) -> u32
where
    R: BufRead,
{
    reader
        .lines()
        .map(|line| {
            let line_unwrap = line.unwrap();
            let line_str = line_unwrap
                .trim_start_matches(char::is_alphabetic)
                .trim_end_matches(char::is_alphabetic);
            let d_1 = line_str.chars().next().unwrap().to_digit(10).unwrap();
            let d_2 = line_str.chars().last().unwrap().to_digit(10).unwrap();
            10 * d_1 + d_2
        })
        .sum()
}

fn read_calibration_spelled<R>(reader: R) -> i32
where
    R: BufRead,
{
    let parse_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    reader
        .lines()
        .map(|line| {
            let line_unwrap = line.unwrap();

            let mut d_1 = 0;
            'outer_d1: for offset in 0..line_unwrap.len() {
                for slice_end in offset..min(offset + 6, line_unwrap.len()) {
                    if let Some(value) = parse_map.get(&line_unwrap[offset..=slice_end]) {
                        d_1 = *value;
                        break 'outer_d1;
                    }
                }
            }
            let mut d_2 = 0;
            'outer_d2: for offset in (0..line_unwrap.len()).rev() {
                for slice_begin in (offset.saturating_sub(6)..=offset).rev() {
                    if let Some(value) = parse_map.get(&line_unwrap[slice_begin..=offset]) {
                        d_2 = *value;
                        break 'outer_d2;
                    }
                }
            }

            10 * d_1 + d_2
        })
        .sum()
}
