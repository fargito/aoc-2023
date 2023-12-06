use helpers::{lazy_static, Regex};

lazy_static! {
    static ref INPUT_REGEX: Regex = Regex::new("\\d+").unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut it = input.lines();

    let times: Vec<u64> = INPUT_REGEX
        .find_iter(
            it.next().unwrap(), // first line
        )
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let distances: Vec<u64> = INPUT_REGEX
        .find_iter(
            it.next().unwrap(), // second line
        )
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let res = times
        .iter()
        .zip(distances.iter())
        .map(get_ways_to_win)
        .reduce(|acc, e| acc * e)
        .unwrap();

    println!("result = {res}");
}

fn get_ways_to_win((total_time, total_distance): (&u64, &u64)) -> u64 {
    // binary search for performance
    let mut max_nok_time = 0;
    let mut min_ok_time = total_time / 2;

    while min_ok_time - max_nok_time > 1 {
        let time_to_eval = max_nok_time + (min_ok_time - max_nok_time) / 2;

        if evaluate(time_to_eval, *total_time) > *total_distance {
            // then time is ok
            min_ok_time = time_to_eval;
        } else {
            max_nok_time = time_to_eval;
        }
    }

    total_time + 1 - (2 * (max_nok_time + 1))
}

fn evaluate(time: u64, total_time: u64) -> u64 {
    time * (total_time - time)
}

#[cfg(test)]
mod tests {
    use crate::evaluate;

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate(1, 7), 6);
        assert_eq!(evaluate(2, 7), 10);
        assert_eq!(evaluate(3, 7), 12);
    }
}
