use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Terrain {
    rows: Vec<u64>,
    width: usize,
}

impl Terrain {
    fn transpose(&self) -> Self {
        let mut rows = Vec::new();
        let width = self.rows.len();
        rows.reserve_exact(self.width);

        for width_idx in (0..self.width).rev() {
            let mut col_val = 0;
            for row_idx in 0..self.rows.len() {
                col_val |= ((self.rows[row_idx] & (1 << width_idx)) >> width_idx)
                    << (self.rows.len() - row_idx - 1);
            }
            rows.push(col_val);
        }

        Terrain { rows, width }
    }

    fn reflection(&self) -> Option<usize> {
        if self.rows.len() < 2 {
            return None;
        }

        for idx in 1..(self.rows.len()) {
            if self.rows[0..idx]
                .iter()
                .rev()
                .zip(self.rows[idx..].iter())
                .all(|(a, b)| a == b)
            {
                return Some(idx);
            }
        }
        None
    }

    fn reflection_with_smudge(&self) -> Option<usize> {
        if self.rows.len() < 2 {
            return None;
        }

        let mut smudged = true;

        for idx in 1..(self.rows.len()) {
            if self.rows[0..idx]
                .iter()
                .rev()
                .zip(self.rows[idx..].iter())
                .all(|(a, b)| match a ^ b {
                    0 => true,
                    c if smudged && (c & (c - 1) == 0) => {
                        smudged = false;
                        return true;
                    }
                    _ => false,
                })
                && !smudged
            {
                return Some(idx);
            } else {
                smudged = true;
            }
        }
        None
    }
}

#[test]
fn test_transpose() {
    // 1 0    1 0
    // 0 1 -> 0 1
    let terrain = Terrain {
        rows: vec![2, 1],
        width: 2,
    };
    assert_eq!(terrain.transpose().rows, vec![2, 1]);

    // 1 0 0    1 0 0
    // 0 1 0 -> 0 1 0
    // 0 0 1    0 0 1
    let terrain_id = Terrain {
        rows: vec![4, 2, 1],
        width: 3,
    };
    assert_eq!(terrain_id.transpose().rows, vec![4, 2, 1]);

    // 1 0 0    1 0 1
    // 0 0 1 -> 0 0 1
    // 1 1 0    0 1 0
    let terrain_four_one_six = Terrain {
        rows: vec![4, 1, 6],
        width: 3,
    };
    assert_eq!(terrain_four_one_six.transpose().rows, vec![5, 1, 2]);
    assert_eq!(
        terrain_four_one_six.transpose().transpose().rows,
        vec![4, 1, 6]
    );
}

#[test]
fn test_reflection() {
    // 1 0 0    1 0 1
    // 0 0 1 -> 0 0 1
    // 1 1 0    0 1 0
    let terrain_one = Terrain {
        rows: vec![4, 1, 6],
        width: 3,
    };

    assert_eq!(terrain_one.reflection(), None);
    assert_eq!(terrain_one.transpose().reflection(), None);

    // 1 0 0    1 0 0 1 1
    // 0 0 1 -> 0 0 0 0 1
    // 0 0 1    0 1 1 0 0
    // 1 0 0
    // 1 1 0
    let terrain_two = Terrain {
        rows: vec![4, 1, 1, 4, 6],
        width: 3,
    };
    assert_eq!(terrain_two.reflection(), Some(2));
    assert_eq!(terrain_two.transpose().reflection(), None);
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day13").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let (mut terrains, (rows, width)) = lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                return None;
            } else {
                return Some((
                    line.trim().len(),
                    line.chars()
                        .rev()
                        .enumerate()
                        .map(|(idx, chr)| match chr {
                            '#' => 1 << idx,
                            _ => 0,
                        })
                        .sum::<u64>(),
                ));
            }
        })
        .fold(
            (Vec::<Terrain>::new(), (Vec::<u64>::new(), 0usize)),
            |(mut terrains, (mut rows, width)), row| match row {
                Some((row_width, row_value)) => {
                    rows.push(row_value);
                    (terrains, (rows, row_width))
                }
                None => {
                    terrains.push(Terrain { rows, width });
                    (terrains, (Vec::<u64>::new(), width))
                }
            },
        );
    terrains.push(Terrain { rows, width });

    let reflection_notes_total: usize = terrains
        .iter()
        .map(|terrain| {
            100 * terrain.reflection().unwrap_or(0) + terrain.transpose().reflection().unwrap_or(0)
        })
        .sum();
    println!("Sum of reflections: {}", reflection_notes_total);

    let reflection_notes_total_part2: usize = terrains
        .iter()
        .map(|terrain| {
            100 * terrain.reflection_with_smudge().unwrap_or(0)
                + terrain.transpose().reflection_with_smudge().unwrap_or(0)
        })
        .sum();
    println!(
        "Sum of reflections with smudge: {}",
        reflection_notes_total_part2
    );
}
