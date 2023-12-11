use helpers::{lazy_static, Regex};
use itertools::Itertools;

lazy_static! {
    static ref SENSOR_REGEX: Regex = Regex::new("-?\\d+").unwrap();
}

fn main() {
    println!("Hello, world!");
    let input = include_str!("../inputs/input.txt");

    let res_part_1 = handle_part_1(input);
    let res_part_2 = handle_part_2(input);

    println!("part1 = {res_part_1}");
    println!("part2 = {res_part_2}");
}

fn handle_part_1(input: &str) -> i64 {
    input.lines().map(handle_part_1_line).sum()
}

fn handle_part_1_line(line: &str) -> i64 {
    let mut values: Vec<i64> = SENSOR_REGEX
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let mut predicted = *values.last().unwrap();
    let mut is_all_zeros = false;

    while !is_all_zeros {
        is_all_zeros = true;

        values = values
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let diff = *b - *a;
                is_all_zeros &= diff == 0;
                diff
            })
            .collect();

        predicted += *values.last().unwrap();
    }

    predicted
}

fn handle_part_2(input: &str) -> i64 {
    input.lines().map(handle_part_2_line).sum()
}

fn handle_part_2_line(line: &str) -> i64 {
    let mut values: Vec<i64> = SENSOR_REGEX
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let mut is_all_zeros = false;
    let mut first_values: Vec<i64> = vec![*values.first().unwrap()];

    while !is_all_zeros {
        is_all_zeros = true;

        values = values
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                let diff = *b - *a;
                is_all_zeros &= diff == 0;
                diff
            })
            .collect();

        first_values.push(*values.first().unwrap());
    }

    first_values
        .into_iter()
        .rev()
        .reduce(|acc, e| e - acc)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{handle_part_1, handle_part_2, handle_part_2_line};

    #[test]
    fn test_part_1() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        assert_eq!(handle_part_1(input), 114);
    }

    #[test]
    fn test_part_2() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        assert_eq!(handle_part_2(input), 2);
    }

    #[test]
    fn test_part_2_lines() {
        assert_eq!(handle_part_2_line("10 13 16 21 30 45"), 5);
        // assert_eq!(handle_part_2_line("0 3 6 9 12 15"), -3);
        // assert_eq!(handle_part_2_line("1 3 6 10 15 21"), 0);
    }
}
