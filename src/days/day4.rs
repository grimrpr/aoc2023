// mod day2

use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    chosen: HashSet<u32>,
}

impl Card {
    fn new() -> Self {
        Card {
            winning: HashSet::new(),
            chosen: HashSet::new(),
        }
    }
}

fn parse_cards(lines: Vec<String>) -> Vec<Card> {
    lines
        .iter()
        .filter_map(|l| match l.split_once(':') {
            Some((_, rstr)) => Some(rstr),
            _ => None,
        })
        .filter_map(|l| l.split_once('|'))
        .map(|(set_win, set_chosen)| {
            let mut c = Card::new();
            c.winning.extend(
                set_win
                    .trim()
                    .split_whitespace()
                    .filter_map(|num: &str| num.parse::<u32>().ok()),
            );
            c.chosen.extend(
                set_chosen
                    .trim()
                    .split_whitespace()
                    .filter_map(|num: &str| num.parse::<u32>().ok()),
            );
            c
        })
        .collect()
}

fn get_total_points(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card: &Card| {
            card.winning
                .intersection(&card.chosen)
                .fold(0u32, |points, _| match points {
                    0 => 1,
                    _ => 2 * points,
                })
        })
        .sum()
}

fn get_num_scratchcards(cards: &Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card: &Card| card.winning.intersection(&card.chosen).count())
        .enumerate()
        .fold(
            HashMap::from_iter((0..cards.len()).zip(repeat(1))),
            |mut card_counts: HashMap<usize, u32>, (id, num_hits)| {
                let instances = card_counts.get(&id).unwrap().clone();
                let won_card_ids = (id..min(id + num_hits + 1, cards.len())).skip(1);
                for i in won_card_ids {
                    if let Some(won_count) = card_counts.get_mut(&i) {
                        *won_count += instances;
                    }
                }
                card_counts
            },
        )
        .values()
        .sum()
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day4").unwrap());
    let file_input: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let cards = parse_cards(file_input);
    println!("Total points: {}", get_total_points(&cards));
    println!("Part2 Num scratchcards: {}", get_num_scratchcards(&cards));
}
