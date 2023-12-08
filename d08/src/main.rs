use std::collections::HashMap;

use helpers::{lazy_static, Regex};

lazy_static! {
    static ref DIRECTIONS_REGEX: Regex = Regex::new("[RL]+").unwrap();
    static ref NODE_REGEX: Regex = Regex::new(
        "(?<source_node>[A-Z\\d]{3}) = \\((?<left_node>[A-Z\\d]{3}), (?<right_node>[A-Z\\d]{3})\\)"
    )
    .unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let result = handle(input);
    let result_with_ghosts = handle_ghosts(input);

    println!("result = {result}");
    println!("result_with_ghosts = {result_with_ghosts}");
}

fn handle(input: &str) -> u64 {
    let map: HashMap<&str, Node> = NODE_REGEX
        .captures_iter(input)
        .map(|c| {
            let source_node = c.name("source_node").unwrap().as_str();
            let left = c.name("left_node").unwrap().as_str();
            let right = c.name("right_node").unwrap().as_str();

            let node_type = match source_node {
                "AAA" => NodeType::Start,
                "ZZZ" => NodeType::End,
                _ => NodeType::Default,
            };

            (
                source_node,
                Node {
                    left,
                    right,
                    node_type,
                },
            )
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

    let start_node = map.get("AAA").unwrap();

    get_node_counter(&map, &directions, start_node)
}

fn get_node_counter(
    map: &HashMap<&str, Node>,
    directions: &Vec<Direction>,
    start_node: &Node,
) -> u64 {
    let mut current_node = start_node;

    let mut counter = 0;

    for direction in directions.iter().cycle() {
        current_node = match direction {
            Direction::Left => map.get(current_node.left).unwrap(),
            Direction::Right => map.get(current_node.right).unwrap(),
        };
        counter += 1;

        if current_node.node_type == NodeType::End {
            break;
        }
    }

    counter
}

fn handle_ghosts(input: &str) -> u64 {
    let mut start_nodes: Vec<&str> = vec![];

    let map: HashMap<&str, Node> = NODE_REGEX
        .captures_iter(input)
        .map(|c| {
            let source_node = c.name("source_node").unwrap().as_str();
            let left = c.name("left_node").unwrap().as_str();
            let right = c.name("right_node").unwrap().as_str();

            let node_type = match source_node.chars().last().unwrap() {
                'A' => NodeType::Start,
                'Z' => NodeType::End,
                _ => NodeType::Default,
            };

            let node = Node {
                left,
                right,
                node_type,
            };

            if node.node_type == NodeType::Start {
                start_nodes.push(source_node);
            }

            (source_node, node)
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

    start_nodes
        .iter()
        .map(|start_node| {
            let start_node = map.get(start_node).unwrap();

            get_node_counter(&map, &directions, start_node)
        })
        .reduce(|acc: u64, e: u64| num::integer::lcm(acc, e))
        .unwrap()
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
    node_type: NodeType,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Start,
    End,
    Default,
}

#[cfg(test)]
mod tests {
    use crate::{handle, handle_ghosts};

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

    #[test]
    fn test_handle_ghost() {
        let res = handle_ghosts(
            "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            ",
        );

        assert_eq!(res, 6)
    }
}
