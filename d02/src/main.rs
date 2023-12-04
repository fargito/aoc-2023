use helpers::read_lines;

fn main() {
    let mut result = 0;

    if let Ok(lines) = read_lines("./d02/inputs/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                println!("{line}");
            }
        }
    }

    println!("{result}");
}
