use std::collections::HashSet;

use helpers::{lazy_static, read_lines, Regex};

lazy_static! {
    static ref CARD_NUMBER_REGEX: Regex = Regex::new("Card(\\s+)(?<id>\\d+): ").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new("(\\d+)").unwrap();
}

fn main() {
    let mut lines: Vec<String> = vec![];

    if let Ok(lines_reader) = read_lines("./d04/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines_reader {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    }

    let mut card_copies = vec![1_usize; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let res = compute_line(&line);
        let index_copies = card_copies[index];

        for i in index + 1..index + res + 1 {
            card_copies[i] += index_copies;
        }
    }

    println!("result: {}", card_copies.iter().sum::<usize>());
}

fn compute_line(line: &str) -> usize {
    // store them in a hashmap in order to check efficiently
    let mut winning_numbers: HashSet<usize> = HashSet::new();
    let mut matches: HashSet<usize> = HashSet::new();

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

    matches.len()
}
