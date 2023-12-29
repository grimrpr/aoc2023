use std::collections::BTreeSet;
use std::collections::HashMap;
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
    let mut galaxies: Vec<Galaxy> = lines
        .iter()
        .enumerate()
        .flat_map(|(row_idx, line)| repeat(row_idx).zip(line.match_indices('#')))
        .map(|(row_idx, (col_idx, _))| Galaxy(row_idx, col_idx))
        .collect();

    let remap_rows: HashMap<usize, usize> = HashMap::from_iter(
        BTreeSet::<usize>::from_iter(galaxies.iter().map(|Galaxy(row, _)| *row))
            .into_iter()
            .zip(0..height)
            .map(|(row_galaxy, row_idx)| (row_galaxy, row_galaxy + row_galaxy - row_idx)),
    );
    let remap_cols: HashMap<usize, usize> = HashMap::from_iter(
        BTreeSet::<usize>::from_iter(galaxies.iter().map(|Galaxy(_, col)| *col))
            .into_iter()
            .zip(0..width)
            .map(|(col_galaxy, col_idx)| (col_galaxy, col_galaxy + col_galaxy - col_idx)),
    );

    // expand the universe
    for galaxy in &mut galaxies {
        galaxy.0 = remap_rows[&galaxy.0];
        galaxy.1 = remap_cols[&galaxy.1];
    }

    // sum of shortest paths between galaxy pairs
    let mut sum_of_lengths = 0usize;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            sum_of_lengths += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1)
        }
    }

    println!("Sum of lengths {}", sum_of_lengths);

}
