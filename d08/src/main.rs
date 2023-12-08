use std::collections::HashMap;

use helpers::{lazy_static, Regex};

lazy_static! {
    static ref DIRECTIONS_REGEX: Regex = Regex::new("[RL]+").unwrap();
    static ref NODE_REGEX: Regex = Regex::new(
        "(?<source_node>[A-Z]{3}) = \\((?<left_node>[A-Z]{3}), (?<right_node>[A-Z]{3})\\)"
    )
    .unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let result = handle(input);

    println!("result = {result}");
}

fn handle(input: &str) -> u64 {
    let map: HashMap<&str, Node> = NODE_REGEX
        .captures_iter(input)
        .map(|c| {
            let source_node = c.name("source_node").unwrap().as_str();
            let left = c.name("left_node").unwrap().as_str();
            let right = c.name("right_node").unwrap().as_str();

            (source_node, Node { left, right })
        })
        .collect();

    let directions: Vec<Direction> = DIRECTIONS_REGEX
        .find(input.lines().next().unwrap()) // only take the first line
        .unwrap()
        .as_str()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            // we are safe because of the regex
            _ => panic!(),
        })
        .collect();

    let mut current_node = "AAA";
    let mut counter = 0;

    while current_node != "ZZZ" {
        for direction in directions.iter() {
            let node = map.get(current_node).unwrap(); // we assert that the graph is complete

            match direction {
                Direction::Left => {
                    current_node = node.left;
                }
                Direction::Right => {
                    current_node = node.right;
                }
            }
            counter += 1;
        }
    }

    counter
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::handle;

    #[test]
    fn test_handle_1() {
        let res = handle(
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(res, 2)
    }

    #[test]
    fn test_handle_2() {
        let res = handle(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );

        assert_eq!(res, 6)
    }
}
