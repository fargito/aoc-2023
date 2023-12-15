use itertools::Itertools;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../inputs/input.txt");

    let res_part_1 = handle(input, 1);
    let res_part_2 = handle(input, 1000000);

    println!("part1 = {res_part_1}");
    println!("part2 = {res_part_2}");
}

fn handle(input: &str, expand_size: usize) -> usize {
    let (rows_count, cols_count) = (
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
    );

    let mut res = 0;

    let mut row_is_empty = vec![true; rows_count];
    let mut col_is_empty = vec![true; cols_count];

    let mut galaxies = vec![];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.char_indices() {
            if char == '#' {
                galaxies.push((i, j));
                row_is_empty[i] = false;
                col_is_empty[j] = false;
            }
        }
    }

    for ((a_i, a_j), (b_i, b_j)) in galaxies.into_iter().tuple_combinations() {
        let mut rows = [a_i, b_i];
        let mut cols = [a_j, b_j];

        rows.sort();
        cols.sort();

        let empty_rows_count = &row_is_empty[rows[0]..rows[1]]
            .into_iter()
            .filter(|e| **e)
            .count();

        let empty_cols_count = &col_is_empty[cols[0]..cols[1]]
            .into_iter()
            .filter(|e| **e)
            .count();

        res += rows[1] - rows[0] - empty_rows_count + expand_size * empty_rows_count;
        res += cols[1] - cols[0] - empty_cols_count + expand_size * empty_cols_count;
    }

    res
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::handle;

    #[test]
    fn test_part_1() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "};

        assert_eq!(handle(input, 2), 374);
    }

    #[test]
    fn test_part_2() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "};

        let res = handle(input, 10);

        println!("{}", res - 1030);

        assert_eq!(res, 1030);
    }

    #[test]
    fn test_part_2_bis() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "};

        let res = handle(input, 100);

        assert_eq!(res, 8410);
    }
}
