use helpers::{lazy_static, Regex};
use itertools::Itertools;

lazy_static! {
    static ref UNKNOWN_SPRINGS_REGEX: Regex = Regex::new("\\?").unwrap();
    static ref SPRINGS_REGEX: Regex = Regex::new("[\\?#]+").unwrap();
}

fn main() {
    let input = include_str!("../inputs/input.txt");

    let res_part_1 = handle_part_1(input);
    let res_part_2 = handle_part_2(input);

    println!("part1 = {res_part_1}");
    println!("part2 = {res_part_2}");
}

fn handle_line(line: &str) -> usize {
    let (springs, checksums) = line.split_once(' ').unwrap();

    let checksums: String = checksums
        .split(",")
        .map(|e| format!("#{{{e}}}"))
        .join("[\\.?]+");

    // there can be an arbitrary number of valid springs at the start and end
    let checksums = format!("^[\\.?]*{checksums}[\\.?]*$");

    let checksums = Regex::new(&checksums).unwrap();

    let unknown_springs: Vec<usize> = UNKNOWN_SPRINGS_REGEX
        .find_iter(springs)
        .map(|m| m.start())
        .collect();

    let res: usize = unknown_springs
        .into_iter()
        .powerset()
        .filter(|s| {
            // replace selected indices with a broken spring
            let new_str: String = springs
                .char_indices()
                .map(|(i, c)| if s.contains(&i) { '#' } else { c })
                .collect();

            // check if it matches the checksum
            checksums.find(&new_str).is_some()
        })
        .count();

    res
}

fn handle_part_1(input: &str) -> usize {
    input.lines().map(|line| handle_line(line)).sum()
}

fn handle_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{handle_line, handle_part_1, handle_part_2};

    #[test]
    fn test_line() {
        assert_eq!(handle_line("???.### 1,1,3"), 1);
        assert_eq!(handle_line(".??..??...?##. 1,1,3"), 4);
        assert_eq!(handle_line("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(handle_line("????.#...#... 4,1,1"), 1);
        assert_eq!(handle_line("????.######..#####. 1,6,5"), 4);
        assert_eq!(handle_line("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_part_1() {
        let input = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
        "};

        assert_eq!(handle_part_1(input), 21);
    }

    #[test]
    fn test_part_2() {
        let input = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
        "};

        assert_eq!(handle_part_2(input), 525152);
    }
}
