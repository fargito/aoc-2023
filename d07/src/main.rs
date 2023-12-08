use std::{cmp::Ordering, collections::HashMap};

use helpers::{lazy_static, Regex};

lazy_static! {
    static ref HAND_REGEX: Regex = Regex::new("(?<value>.+) (?<bid>(\\d+))").unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut hands: Vec<Hand> = HAND_REGEX
        .captures_iter(input)
        .map(|c| {
            let value = c.name("value").unwrap().as_str();
            let bid = c.name("bid").unwrap().as_str().parse().unwrap();

            let hand_type = get_hand_type_with_jokers(&value);

            Hand {
                value,
                bid,
                hand_type,
            }
        })
        .collect();

    hands.sort_by(|a, b| match a.hand_type.cmp(&b.hand_type) {
        std::cmp::Ordering::Equal => compare_hands_values(a.value, b.value),
        ordering => ordering,
    });

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid));

    println!("result = {result}");
}

#[derive(Debug)]
struct Hand<'a> {
    value: &'a str,
    bid: usize,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[allow(dead_code)]
fn get_hand_type(card_value: &str) -> HandType {
    let mut chars = HashMap::new();

    let mut values = card_value
        .chars()
        .fold(&mut chars, |acc, el| {
            let e = acc.entry(el).or_insert(0_u32);

            *e += 1;

            acc
        })
        .values()
        .collect::<Vec<&u32>>();

    values.sort();
    values.reverse();

    if values.len() == 1 {
        HandType::FiveOfAKind
    } else if *values[0] == 4 {
        HandType::FourOfAKind
    } else if values.len() == 2 && *values[0] == 3 {
        HandType::FullHouse
    } else if *values[0] == 3 {
        HandType::ThreeOfAKind
    } else if *values[0] == 2 && *values[1] == 2 {
        HandType::TwoPair
    } else if *values[0] == 2 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn get_hand_type_with_jokers(card_value: &str) -> HandType {
    let mut chars = HashMap::new();

    card_value.chars().fold(&mut chars, |acc, el| {
        let e = acc.entry(el).or_insert(0_u32);

        *e += 1;

        acc
    });

    // start_by removing jokers
    let mut jokers = chars.remove(&'J').unwrap_or(0);

    let mut values = chars.values_mut().collect::<Vec<&mut u32>>();

    values.sort();
    values.reverse();

    // add the jokers again to the first value
    if values.len() > 0 {
        *values[0] += jokers;
    } else {
        values.push(&mut jokers);
    }

    if values.len() == 1 {
        HandType::FiveOfAKind
    } else if *values[0] == 4 {
        HandType::FourOfAKind
    } else if values.len() == 2 && *values[0] == 3 {
        HandType::FullHouse
    } else if *values[0] == 3 {
        HandType::ThreeOfAKind
    } else if *values[0] == 2 && *values[1] == 2 {
        HandType::TwoPair
    } else if *values[0] == 2 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn compare_hands_values(hand_a: &str, hand_b: &str) -> Ordering {
    let mut ordering = Ordering::Equal;

    for (card_a, card_b) in hand_a.chars().zip(hand_b.chars()) {
        ordering = get_card_value(card_a).cmp(&get_card_value(card_b));

        if ordering != Ordering::Equal {
            break;
        }
    }

    ordering
}

fn get_card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{compare_hands_values, get_hand_type, get_hand_type_with_jokers, HandType};

    #[test]
    fn test_get_hand_type() {
        assert_eq!(get_hand_type("32T3K"), HandType::OnePair);
        assert_eq!(get_hand_type("T55J5"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type("KTJJT"), HandType::TwoPair);
        assert_eq!(get_hand_type("QQQJA"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("QQQQQ"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("QQQQT"), HandType::FourOfAKind);
    }

    #[test]
    fn test_get_hand_type_with_joker() {
        assert_eq!(get_hand_type_with_jokers("32T3K"), HandType::OnePair);
        assert_eq!(get_hand_type_with_jokers("T55J5"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_with_jokers("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type_with_jokers("KTJJT"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_with_jokers("QQQJA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_with_jokers("QQQQQ"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type_with_jokers("QQQQT"), HandType::FourOfAKind);
    }

    #[test]
    fn test_hand_type_order() {
        assert_eq!(
            HandType::FiveOfAKind.cmp(&HandType::FullHouse),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare_hand_values() {
        assert_eq!(compare_hands_values("32T3K", "32T3K"), Ordering::Equal);
        assert_eq!(compare_hands_values("32T4K", "32T3K"), Ordering::Greater);
        assert_eq!(compare_hands_values("32T4K", "3AT3K"), Ordering::Less);
    }
}
