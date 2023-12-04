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
    let game_id: u64 = GAME_ID_REGEX
        .captures(line)
        .unwrap()
        .name("id")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    // remove the game id beginning
    let cleaned_line = GAME_ID_REGEX.replace_all(line, "");

    for subset in cleaned_line.split("; ") {
        for m in GAME_DRAW_REGEX.captures_iter(subset) {
            let count: u64 = m.name("count").unwrap().as_str().parse().unwrap();
            let color = m.name("color").unwrap().as_str();

            let is_draw_ok = match color {
                "red" => count <= 12,
                "green" => count <= 13,
                "blue" => count <= 14,
                _ => panic!(),
            };

            if !is_draw_ok {
                return 0;
            }
        }
    }

    game_id
}
