use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;

struct Galaxy(usize, usize);

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day11").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let width = lines[0].len();
    let height = lines.len();
    let galaxies: Vec<Galaxy> = lines
        .iter()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            repeat(row_idx).zip(line.match_indices('#'))
        }).map(|(row_idx, (col_idx, _))| Galaxy(row_idx, col_idx))
        .collect();
}
