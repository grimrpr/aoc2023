use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day13").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
}
