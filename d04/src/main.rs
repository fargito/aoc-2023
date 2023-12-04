use std::collections::HashSet;

use helpers::{lazy_static, read_lines, Regex};

lazy_static! {
    static ref CARD_NUMBER_REGEX: Regex = Regex::new("Card(\\s+)(?<id>\\d+): ").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new("(\\d+)").unwrap();
}

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./d04/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                result += compute_line(&line);
            }
        }
    }

    println!("result: {result}");
}

fn compute_line(line: &str) -> u64 {
    // store them in a hashmap in order to check efficiently
    let mut winning_numbers: HashSet<u64> = HashSet::new();
    let mut matches: HashSet<u64> = HashSet::new();

    // remove the game id beginning
    let cleaned_line = CARD_NUMBER_REGEX.replace_all(line, "");

    let mut it = cleaned_line.split("|");

    let (winning_numbers_str, my_numbers_str) = (it.next().unwrap(), it.next().unwrap());

    for m in NUMBER_REGEX.find_iter(winning_numbers_str) {
        let winning_number = m.as_str().parse().unwrap();
        winning_numbers.insert(winning_number);
    }

    for m in NUMBER_REGEX.find_iter(my_numbers_str) {
        let my_number = m.as_str().parse().unwrap();

        if winning_numbers.contains(&my_number) {
            matches.insert(my_number);
        }
    }

    if matches.len() == 0 {
        0
    } else {
        2_u64.pow((matches.len() - 1).try_into().unwrap())
    }
}
