use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn is_arithmetic_seq(seq: &[i64]) -> bool {
    if seq.len() < 2 {
        return true;
    }

    let diff = seq[1] - seq[0];
    seq.iter()
        .zip(seq.iter().skip(1))
        .all(|(first, second)| (*second - *first) == diff)
}

fn history_completion(seq: &[i64], seq_reverse: bool) -> i64 {
    let mut diff_seq = if seq_reverse {
        Vec::from_iter(seq.iter().rev().copied())
    } else {
        seq.to_owned()
    };

    for dx in 1..(seq.len() - 1) {
        if is_arithmetic_seq(&diff_seq[0..(seq.len() - dx + 1)]) {
            let diff = diff_seq[1] - diff_seq[0];
            return diff + diff_seq[(seq.len() - dx)..seq.len()].iter().sum::<i64>();
        }
        let mut last_val = diff_seq[diff_seq.len() - dx];
        for val in diff_seq[0..(seq.len() - dx)].iter_mut().rev() {
            let tmp = *val;
            *val = last_val - *val;
            last_val = tmp;
        }
        //println!("{:?}", diff_seq);
    }
    0
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day9").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let seqs: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|number| number.trim().parse::<i64>())
                .collect()
        })
        .collect();

    let extrapol_sum = seqs
        .iter()
        .map(|seq| history_completion(seq.as_slice(), false))
        //.inspect(|res| println!("{}", res))
        .sum::<i64>();

    println!("Sum of extrapolations: {}", extrapol_sum);

    let extrapol_reversed = seqs
        .iter()
        .map(|seq| history_completion(seq.as_slice(), true))
        //.inspect(|res| println!("{}", res))
        .sum::<i64>();

    println!("Part 2 Sum of extrapolations: {}", extrapol_reversed);
}
