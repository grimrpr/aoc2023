use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn count_possible_seqs(
    seq: &[u8],
    group_size_desc: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if seq.is_empty() {
        return 0;
    }

    if group_size_desc.is_empty() {
        if seq.contains(&b'#') {
            return 0;
        }
        return 1;
    }

    //println!(
    //    "Called count_possible_seqs with {} and {:?}",
    //    std::str::from_utf8(seq).unwrap(),
    //    group_size_desc
    //);

    let start_spring_entry = seq[0];
    let mut cache_updates = Vec::new();
    let mut branch_queue: VecDeque<(usize, usize, usize, u8)>;
    if start_spring_entry != b'?' {
        branch_queue = VecDeque::from([(0, 0, 0, start_spring_entry)]);
    } else {
        branch_queue = VecDeque::from([(0, 0, 0, b'.'), (0, 0, 0, b'#')]);
    }
    let mut seq_counter = 0;
    while let Some((spring_idx, broken_idx, broken_count, spring_entry)) = branch_queue.pop_front()
    {
        //println!(
        //    "Current rest {} and {:?} broken_count {} value {}",
        //    std::str::from_utf8(&seq[spring_idx..]).unwrap(),
        //    group_size_desc[broken_idx..].to_vec(),
        //    broken_count,
        //    seq_counter,
        //);

        while let Some((cache_seq_count, cache_spring_idx, cache_broken_idx)) = cache_updates.last()
        {
            if *cache_spring_idx <= spring_idx {
                break;
            }
            //println!(
            //    "Insert into cache {} and {:?} value {}",
            //    std::str::from_utf8(&seq[*cache_spring_idx..]).unwrap(),
            //    group_size_desc[*cache_broken_idx..].to_vec(),
            //    seq_counter - cache_seq_count,
            //);

            cache.insert(
                (
                    std::str::from_utf8(&seq[*cache_spring_idx..])
                        .unwrap()
                        .to_owned(),
                    group_size_desc[*cache_broken_idx..].to_vec(),
                ),
                seq_counter - cache_seq_count,
            );
            cache_updates.pop();
        }
        let mut next_broken_count = broken_count;
        let mut next_broken_idx = broken_idx;
        let spring_is_broken = spring_entry == b'#';
        if spring_is_broken {
            if broken_count >= group_size_desc[broken_idx] {
                continue;
            }
            next_broken_count += 1;
        } else {
            if broken_count != 0 {
                if broken_count != group_size_desc[broken_idx] {
                    continue;
                }
                if broken_idx < (group_size_desc.len() - 1) {
                    next_broken_idx += 1;
                    next_broken_count = 0;
                }
            }
        }

        let end_is_reached = spring_idx == seq.len() - 1;
        if end_is_reached {
            if broken_idx != (group_size_desc.len() - 1) {
                continue;
            }

            if group_size_desc[broken_idx] != next_broken_count {
                continue;
            }

            seq_counter += 1;
            //println!("Reached end seq_counter {}", seq_counter);
            continue;
        }

        let next_spring_idx = spring_idx + 1;
        let next_spring_entry = seq[next_spring_idx];

        if !spring_is_broken && (broken_count == 0) {
            if let Some(cached_count_val) = cache.get(&(
                std::str::from_utf8(&seq[next_spring_idx..])
                    .unwrap()
                    .to_owned(),
                group_size_desc[next_broken_idx..].to_vec(),
            )) {
                //println!(
                //    "Cache hit {} and {:?} with value {}",
                //    std::str::from_utf8(&seq[next_spring_idx..])
                //        .unwrap()
                //        .to_owned(),
                //    group_size_desc[next_broken_idx..].to_vec(),
                //    cached_count_val
                //);
                seq_counter += cached_count_val;
                continue;
            } else {
                //println!(
                //    "Queue cache update counter {} for spring_idx {} and broken_idx {}",
                //    seq_counter, next_spring_idx, next_broken_idx
                //);
                cache_updates.push((seq_counter, next_spring_idx, next_broken_idx));
            }
        }

        match next_spring_entry {
            b'?' => {
                branch_queue.push_front((
                    next_spring_idx,
                    next_broken_idx,
                    next_broken_count,
                    b'#',
                ));
                branch_queue.push_front((next_spring_idx, next_broken_idx, next_broken_count, b'.'))
            }
            _ => branch_queue.push_front((
                next_spring_idx,
                next_broken_idx,
                next_broken_count,
                next_spring_entry,
            )),
        }
    }
    while let Some((cache_seq_count, cache_spring_idx, cache_broken_idx)) = cache_updates.last() {
        //println!(
        //    "Insert into cache {} and {:?} value {}",
        //    std::str::from_utf8(&seq[*cache_spring_idx..]).unwrap(),
        //    group_size_desc[*cache_broken_idx..].to_vec(),
        //    seq_counter - cache_seq_count,
        //);

        cache.insert(
            (
                std::str::from_utf8(&seq[*cache_spring_idx..])
                    .unwrap()
                    .to_owned(),
                group_size_desc[*cache_broken_idx..].to_vec(),
            ),
            seq_counter - cache_seq_count,
        );
        cache_updates.pop();
    }

    seq_counter
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day12").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut result_cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    let sum_of_seqs = lines
        .iter()
        .filter_map(|line| line.split_once(' '))
        .map(|(seq_str, groups_str)| {
            let groups_parsed = groups_str
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            count_possible_seqs(seq_str.as_bytes(), &groups_parsed, &mut result_cache)
        })
        //.inspect(|c| println!("{}", c))
        .sum::<usize>();

    println!("Sum of possible sequences {}", sum_of_seqs);

    let sum_of_unfolded_seqs = lines
        .iter()
        .filter_map(|line| line.split_once(' '))
        .map(|(seq_str, groups_str)| {
            let unfolded_seq_str = std::iter::once(seq_str)
                .chain(std::iter::once("?"))
                .cycle()
                .take(9)
                .collect::<String>();
            let groups_parsed =
                std::iter::repeat(groups_str.split(',').map(|c| c.parse::<usize>().unwrap()))
                    .take(5)
                    .flatten()
                    .collect::<Vec<_>>();

            count_possible_seqs(
                &unfolded_seq_str.as_bytes(),
                &groups_parsed,
                &mut result_cache,
            )
        })
        //.inspect(|c| println!("{}", c))
        .sum::<usize>();

    println!(
        "Sum of unfolded possible sequences {}",
        sum_of_unfolded_seqs
    );
}
