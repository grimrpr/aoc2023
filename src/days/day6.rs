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

/// s = v * t_up ms [mm]
///
/// s = t_h mm/ms * t_up ms [mm]
///
/// t = t_h + t_up [ms]
///
/// t_up = t - t_h [ms]
///
/// s = t_h mm/ms * (t - t_h) ms [mm]
///
/// s = t*t_h - t_h²  [mm]
///
/// 0 = -t_h² + time*t_h - record_dist
fn prod_num_victory(races: &[BoatRace]) -> usize {
    races
        .iter()
        .map(|race| {
            let inner_sqrt = (((race.time.as_millis() * race.time.as_millis()) as u64
                - 4 * race.distance) as f64)
                .sqrt();
            let x_1 = (((race.time.as_millis() as f64) + inner_sqrt) / 2.0).floor() as u64;
            let x_0 = (((race.time.as_millis() as f64) - inner_sqrt) / 2.0).ceil() as u64;
            (x_0..=x_1).count()
        })
        .product()
}

fn parse_race(input: &[String]) -> BoatRace {
    input
        .iter()
        .take(1)
        .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
        .zip(
            input
                .iter()
                .skip(1)
                .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>()),
        )
        .map(|(time_str, dist_str)| BoatRace {
            time: Duration::from_millis(time_str.parse().unwrap()),
            distance: dist_str.parse().unwrap(),
        }).next().unwrap()
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day6").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let boat_races = parse_races(&lines.as_slice());

    //println!("{:?}", boat_races);
    println!("Num product {}", prod_num_victory(&boat_races));
    let boat_race = parse_race(&lines.as_slice());
    println!("Part 2 product {}", prod_num_victory(&[boat_race]));
}
