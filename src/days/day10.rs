use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    PipeVertical,
    PipeHorizontal,
    PipeBendEN,
    PipeBendWN,
    PipeBendWS,
    PipeBendES,
    Ground,
    Start,
    Illegal,
}

impl Tile {
    fn allowed_movement(&self) -> Option<[Direction; 2]> {
        match self {
            Tile::PipeVertical => Some([Direction::North, Direction::South]),
            Tile::PipeHorizontal => Some([Direction::East, Direction::West]),
            Tile::PipeBendEN => Some([Direction::East, Direction::North]),
            Tile::PipeBendWN => Some([Direction::West, Direction::North]),
            Tile::PipeBendWS => Some([Direction::West, Direction::South]),
            Tile::PipeBendES => Some([Direction::East, Direction::South]),
            _ => None,
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Tile {
        match value {
            '|' => Tile::PipeVertical,
            '-' => Tile::PipeHorizontal,
            'L' => Tile::PipeBendEN,
            'J' => Tile::PipeBendWN,
            '7' => Tile::PipeBendWS,
            'F' => Tile::PipeBendES,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => Tile::Illegal,
        }
    }
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    fn new(input: &[String]) -> Self {
        Map {
            tiles: input
                .iter()
                .flat_map(|str| str.chars())
                .map(|c| Tile::from(c))
                .collect(),
            width: input[0].len(),
        }
    }

    fn find_start_position(&self) -> Option<usize> {
        self.tiles.iter().position(|t| *t == Tile::Start)
    }

    fn get_neighbor_tile(&self, center: usize, neighbor_dir: Direction) -> Tile {
        match neighbor_dir {
            Direction::North => {
                if center < self.width {
                    Tile::Illegal
                } else {
                    self.tiles[center - self.width]
                }
            }
            Direction::South => {
                if center >= (self.tiles.len() - self.width) {
                    Tile::Illegal
                } else {
                    self.tiles[center + self.width]
                }
            }
            Direction::East => {
                if (center >= (self.tiles.len() - 1)) || (((center + 1) % self.width) == 0) {
                    Tile::Illegal
                } else {
                    self.tiles[center + 1]
                }
            }
            Direction::West => {
                if (center < 1) || (center % self.width) == 0 {
                    Tile::Illegal
                } else {
                    self.tiles[center - 1]
                }
            }
        }
    }

    fn get_neighbor_position(&self, center: usize, neighbor_dir: Direction) -> usize {
        match neighbor_dir {
            Direction::North => center - self.width,
            Direction::South => center + self.width,
            Direction::East => center + 1,
            Direction::West => center - 1,
        }
    }
}

struct Track {
    coming_from: Option<Direction>,
    position: usize,
    steps: HashMap<usize, Direction>,
}

impl Track {
    fn new(start: usize) -> Self {
        Track {
            coming_from: None,
            position: start,
            steps: HashMap::new(),
        }
    }

    fn next_step(&mut self, map: &Map) -> usize {
        let directions = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        let next_dir = match self.coming_from {
            Some(_) => match map.tiles[self.position].allowed_movement().unwrap() {
                v => v
                    .iter()
                    .find(|d| **d != self.coming_from.unwrap())
                    .unwrap()
                    .clone(),
            },
            _ => directions
                .iter()
                .find(|dir| {
                    let neighbor_tile = map.get_neighbor_tile(self.position, **dir);
                    match neighbor_tile.allowed_movement() {
                        Some(dirs) if dirs.contains(&dir.opposite()) => true,
                        _ => false,
                    }
                })
                .unwrap()
                .clone(),
        };
        self.position = map.get_neighbor_position(self.position, next_dir);
        self.steps.insert(self.position, next_dir);
        self.coming_from = Some(next_dir.opposite());

        self.position
    }
}

fn space_outside_of_track(map: &Map, track: &Track) -> usize {
    map.tiles
        .iter()
        .enumerate()
        .fold((0usize, true), |(count, outside), (idx, curr_tile)| {
            if track.steps.contains_key(&idx) {
                if (*curr_tile == Tile::Start)
                    && (map.get_neighbor_tile(idx, Direction::North) != Tile::Illegal)
                    && track
                        .steps
                        .contains_key(&map.get_neighbor_position(idx, Direction::North))
                {
                    return (count + 1, !outside);
                }
                if (*curr_tile == Tile::Start)
                    || curr_tile
                        .allowed_movement()
                        .unwrap()
                        .contains(&Direction::North)
                {
                    return (count + 1, !outside);
                }
                return (count + 1, outside);
            }

            if outside {
                return (count + 1, outside);
            }
            (count, outside)
        })
        .0
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day10").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let map = Map::new(lines.as_slice());
    let mut track = Track::new(map.find_start_position().unwrap());

    loop {
        let new_pos = track.next_step(&map);
        if map.tiles[new_pos] == Tile::Start {
            break;
        }
    }

    println!("Steps until halfway {}", track.steps.len() / 2);
    println!(
        "Fields inside loop {}",
        map.tiles.len() - space_outside_of_track(&map, &track)
    );
}
