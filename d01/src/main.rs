use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

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

    for char in line.chars() {
        if let Some(digit) = char.to_digit(10) {
            last_digit = Some(digit);

            if first_digit == None {
                first_digit = Some(digit);
            }
        }
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
        assert_eq!(get_calibration_from_line("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_from_line("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_from_line("treb7uchet"), 77);
    }
}
