use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

#[derive(Debug)]
struct BoatRace {
    time: Duration,
    distance: u64,
}

fn parse_races(input: &[String]) -> Vec<BoatRace> {
    input
        .iter()
        .take(1)
        .flat_map(|l| {
            l.trim_start_matches(|c: char| !c.is_whitespace())
                .trim()
                .split_whitespace()
        })
        .zip(input.iter().skip(1).flat_map(|l| {
            l.trim_start_matches(|c: char| !c.is_whitespace())
                .trim()
                .split_whitespace()
        }))
        .map(|(time_str, dist_str)| BoatRace {
            time: Duration::from_millis(time_str.parse().unwrap()),
            distance: dist_str.parse().unwrap(),
        })
        .collect()
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day6").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let boat_races = parse_races(&lines.as_slice());

    println!("{:?}", boat_races);
}
