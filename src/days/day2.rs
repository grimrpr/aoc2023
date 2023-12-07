// mod day2

use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum ColorCube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

struct Game {
    id: u32,
}

impl TryFrom<&str> for ColorCube {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
        match value.trim().split_once(char::is_whitespace) {
            Some((count, "red")) => Ok(ColorCube::Red(count.parse()?)),
            Some((count, "green")) => Ok(ColorCube::Green(count.parse()?)),
            Some((count, "blue")) => Ok(ColorCube::Blue(count.parse()?)),
            _ => Err("Invalid Cube Type.".into()),
        }
    }
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
        match value.trim().split_once(char::is_whitespace) {
            Some(("Game", id)) => Ok(Game { id: id.parse()? }),
            _ => Err("Invalid Cube Type.".into()),
        }
    }
}

fn sum_valid_games<R>(reader: R) -> u32
where
    R: BufRead,
{
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            if let Some((game_str, draws_str)) = line.split_once(':') {
                let game = Game::try_from(game_str).unwrap();
                for draw in draws_str.split("; ") {
                    let (result_red, result_green, result_blue) = draw
                        .split(", ")
                        .map(ColorCube::try_from)
                        .filter_map(Result::ok)
                        .fold((0, 0, 0), |(red, green, blue), c| match c {
                            ColorCube::Red(v) => (v, green, blue),
                            ColorCube::Green(v) => (red, v, blue),
                            ColorCube::Blue(v) => (red, green, v),
                        });

                    if result_red > 12 || result_green > 13 || result_blue > 14 {
                        return 0;
                    }
                }
                return game.id;
            }
            return 0;
        })
        .sum()
}

fn sum_of_minimum_set_powers<R>(reader: R) -> u32
where
    R: BufRead,
{
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            if let Some((_, draws_str)) = line.split_once(':') {
                let (red_max, green_max, blue_max) = draws_str
                    .split("; ")
                    .flat_map(|draw| draw.split(", "))
                    .map(ColorCube::try_from)
                    .filter_map(Result::ok)
                    .fold((0, 0, 0), |(red, green, blue), cubes| match cubes {
                        ColorCube::Red(v) => (max(red, v), green, blue),
                        ColorCube::Green(v) => (red, max(green, v), blue),
                        ColorCube::Blue(v) => (red, green, max(blue, v)),
                    });

                return red_max * green_max * blue_max;
            }
            0
        })
        .sum()
}

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day2").unwrap());
    println!("Sum of valid game ids: {}", sum_valid_games(reader));
    println!(
        "Part 2 Sum of min set powers: {}",
        sum_of_minimum_set_powers(BufReader::new(File::open("data/input_day2").unwrap()))
    );
}
