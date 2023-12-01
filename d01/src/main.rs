use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new("(\\d|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
}

/// <https://adventofcode.com/2023/day/1>
fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./d01/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                result += get_calibration_from_line(&line);
            }
        }
    }

    println!("{result}");
}

fn get_calibration_from_line(line: &str) -> u32 {
    let (mut first_digit, mut last_digit) = (None::<u32>, None::<u32>);

    let mut pos = 0;

    while let Some(m) = RE.find_at(line, pos) {
        let digit = match m.as_str() {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            res => res.parse().unwrap(),
        };

        last_digit = Some(digit);

        if first_digit.is_none() {
            first_digit = Some(digit);
        }

        pos += 1;
    }

    // at this point, given the input, both values are defined
    first_digit.unwrap() * 10 + last_digit.unwrap()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::get_calibration_from_line;

    #[test]
    fn test_it_works() {
        assert_eq!(get_calibration_from_line("1abc2"), 12);
        assert_eq!(get_calibration_from_line("111112"), 12);
        assert_eq!(get_calibration_from_line("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_from_line("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_from_line("treb7uchet"), 77);
        assert_eq!(get_calibration_from_line("two1nine"), 29);
        assert_eq!(get_calibration_from_line("eightwothree"), 83);
        assert_eq!(get_calibration_from_line("abcone2threexyz"), 13);
        assert_eq!(get_calibration_from_line("xtwone3four"), 24);
        assert_eq!(get_calibration_from_line("4nineeightseven2"), 42);
        assert_eq!(get_calibration_from_line("zoneight234"), 14);
        assert_eq!(get_calibration_from_line("7pqrstsixteen"), 76);
        assert_eq!(get_calibration_from_line("twone"), 21);
        assert_eq!(get_calibration_from_line("nine"), 99);
        assert_eq!(
            get_calibration_from_line("hcfxflqvkvdfmthkjdpfzzlzzh4kdmmhvspzddfivethree"),
            43
        );
    }
}
