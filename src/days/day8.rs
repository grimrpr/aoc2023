use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Waypoint<'a>(&'a str, usize, usize);

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day8").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let map_points: Vec<_> = lines
        .iter()
        .skip(2)
        .filter_map(|line| line.split_once('='))
        .map(|(key, left_right_pair)| (key, left_right_pair.split_once(',').unwrap()))
        .map(|(key, (l_next_key, r_next_key))| {
            (
                key.trim(),
                (
                    l_next_key.trim().trim_start_matches('('),
                    r_next_key.trim().trim_end_matches(')'),
                ),
            )
        })
        .collect();
    let waypoints: Vec<Waypoint> = map_points
        .iter()
        .map(|(key, (left_str, right_str))| {
            Waypoint(
                *key,
                map_points
                    .iter()
                    .position(|find_str| find_str.0 == *left_str)
                    .unwrap(),
                map_points
                    .iter()
                    .position(|find_str| find_str.0 == *right_str)
                    .unwrap(),
            )
        })
        .collect();

    let map: HashMap<&str, (&str, &str)> = HashMap::from_iter(map_points.into_iter());

    let mut lr_seq = lines.iter().take(1).flat_map(|l| l.chars()).cycle();
    let mut nav_step_count = 0u64;
    let mut next_key = "AAA";
    while let (Some(lr), Some(v)) = (lr_seq.next(), map.get(next_key)) {
        if next_key == "ZZZ" {
            break;
        }
        nav_step_count += 1;
        next_key = if lr == 'L' { v.0 } else { v.1 };
    }

    println!("Steps needed until ZZZ is reached: {}", nav_step_count);

    let start_keys: Vec<usize> = waypoints
        .iter()
        .enumerate()
        .filter(|(_, Waypoint(key_str, _, _))| key_str.ends_with('A'))
        .map(|(idx, _)| idx)
        .collect();

    // maps LR seq pos and Node str to pathlength
    let mut results: Vec<Vec<(usize, usize)>> = Vec::new();
    for key in start_keys {
        let mut paths: HashMap<(usize, &str), usize> = HashMap::new();
        let mut next_key = key;
        let mut position_count: usize = 0;
        let mut cycle_length: usize = 0;
        let mut cycle_start: usize = 0;
        for (lr_pos, lr) in lines
            .iter()
            .take(1)
            .flat_map(|l| l.chars())
            .enumerate()
            .cycle()
        {
            if let Some(v) = paths.insert((lr_pos, waypoints[next_key].0), position_count) {
                paths.insert((lr_pos, waypoints[next_key].0), v);
                cycle_start = v;
                cycle_length = position_count - v;
                break;
            }
            position_count += 1;

            next_key = match lr {
                'L' => waypoints[next_key].1,
                _ => waypoints[next_key].2,
            };
        }

        results.push(
            paths
                .iter()
                .filter(|((_, key_name), _)| key_name.ends_with('Z'))
                //.inspect(|((_, name), pos)| println!("{} at pos {}", name, pos))
                .map(|(_, position)| {
                    (
                        *position,
                        if *position >= cycle_start {
                            cycle_length
                        } else {
                            0
                        },
                    )
                })
                .collect(),
        );
    }
    //println!("{:?}", results);

    //let mut res: Vec<_> = results.iter().flatten().copied().collect();

    // use lcm for speedup
    //while !res.iter().all(|(p, _)| *p == res[0].0) {
    //    let max = res.iter().map(|(p, _)| p).max().unwrap().clone();
    //    for (pos, cycle) in res.iter_mut() {
    //        while *pos < max {
    //            *pos += *cycle;
    //        }
    //    }
    //}

    //println!("Part 2 result {}", res[0].0);
}
