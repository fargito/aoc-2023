use std::cmp::max;

use helpers::read_lines;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GAME_ID_REGEX: Regex = Regex::new("Game (?<id>\\d+): ").unwrap();
    static ref GAME_DRAW_REGEX: Regex =
        Regex::new("(?<count>\\d+) (?<color>red|green|blue)").unwrap();
}

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./d02/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                result += parse_game(&line);
            }
        }
    }

    println!("{result}");
}

fn parse_game(line: &str) -> u64 {
    let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);

    // remove the game id beginning
    let cleaned_line = GAME_ID_REGEX.replace_all(line, "");

    for subset in cleaned_line.split("; ") {
        for m in GAME_DRAW_REGEX.captures_iter(subset) {
            let count: u64 = m.name("count").unwrap().as_str().parse().unwrap();
            let color = m.name("color").unwrap().as_str();

            match color {
                "red" => {
                    min_red = max(min_red, count);
                }
                "green" => {
                    min_green = max(min_green, count);
                }
                "blue" => {
                    min_blue = max(min_blue, count);
                }
                _ => panic!(),
            };
        }
    }

    min_blue * min_green * min_red
}
