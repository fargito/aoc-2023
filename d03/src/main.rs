use helpers::{lazy_static, read_lines, Regex};

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new("(?<number>\\d+)").unwrap();
    static ref SPECIAL_CHAR_REGEX: Regex = Regex::new("[^\\d\\.]").unwrap();
}

fn main() {
    let mut result = 0;
    let mut lines: Vec<String> = vec![];

    if let Ok(lines_iter) = read_lines("./d03/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines_iter {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    }

    let lines_length = lines[0].len();

    let mut lines_to_search: Vec<&str>;

    // naive non-optimized solution, because we lookup the same line several times
    for (i, line) in lines.iter().enumerate() {
        lines_to_search = if i == 0 {
            vec![&lines[0], &lines[1]]
        } else if i == lines_length - 1 {
            vec![&lines[i - 1], &lines[i]]
        } else {
            vec![&lines[i - 1], &lines[i], &lines[i + 1]]
        };

        for number_match in NUMBER_REGEX.find_iter(&line) {
            let number_start = std::cmp::max(number_match.start(), 1) - 1;
            let number_end = std::cmp::min(number_match.end() + 1, lines_length - 1);
            let number: u64 = number_match.as_str().parse().unwrap();

            // check if the number is ok by looking all the possible lines
            for line in lines_to_search.iter() {
                if let Some(special_char_match) = SPECIAL_CHAR_REGEX.find_at(&line, number_start) {
                    if special_char_match.start() >= number_start
                        && special_char_match.end() <= number_end
                    {
                        result += number;
                        break;
                    }
                }
            }
        }
    }

    println!("{result}");
}
