use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
    let input = include_str!("../inputs/input.txt");

    let res_part_1 = handle_part_1(input);
    let res_part_2 = handle_part_2(input);

    println!("part1 = {res_part_1}");
    println!("part2 = {res_part_2}");
}

#[derive(PartialEq, Debug)]
enum TileType {
    Start,
    Ground,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    value: Option<u64>,
    row: usize,
    column: usize,
}

impl Tile {
    fn is_heading_north(&self) -> bool {
        match self.tile_type {
            TileType::Vertical | TileType::NorthWest | TileType::NorthEast => true,
            _ => false,
        }
    }

    fn is_heading_south(&self) -> bool {
        match self.tile_type {
            TileType::Vertical | TileType::SouthEast | TileType::SouthWest => true,
            _ => false,
        }
    }

    fn is_heading_west(&self) -> bool {
        match self.tile_type {
            TileType::Horizontal | TileType::NorthWest | TileType::SouthWest => true,
            _ => false,
        }
    }
}

fn parse_maze(input: &str) -> (Vec<Vec<Tile>>, u64) {
    let mut nodes: VecDeque<(usize, usize, u64, usize, usize)> = VecDeque::new();

    let mut max_maze_value = 0;

    let mut maze: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.trim_start()
                .char_indices()
                .map(|(column, c)| {
                    let tile_type = match c {
                        '|' => TileType::Vertical,
                        '-' => TileType::Horizontal,
                        'L' => TileType::NorthEast,
                        'J' => TileType::NorthWest,
                        '7' => TileType::SouthWest,
                        'F' => TileType::SouthEast,
                        '.' => TileType::Ground,
                        'S' => TileType::Start,
                        _ => panic!(),
                    };

                    let tile = Tile {
                        tile_type,
                        value: None,
                        row,
                        column,
                    };

                    if tile.tile_type == TileType::Start {
                        nodes.push_back((tile.row, tile.column, 0, tile.row, tile.column));
                    }

                    tile
                })
                .collect()
        })
        .collect();

    // transform the start

    let (start_row, start_column, _, _, _) = nodes.pop_front().unwrap();

    let is_north = start_row > 0
        && maze
            .get(start_row - 1)
            .map(|row| row[start_column].is_heading_south())
            .unwrap_or(false);

    let is_south = maze
        .get(start_row + 1)
        .map(|row| row[start_column].is_heading_north())
        .unwrap_or(false);

    let is_east = maze[start_row]
        .get(start_column + 1)
        .map(|v| v.is_heading_west())
        .unwrap_or(false);

    let start_type = if is_north {
        if is_south {
            TileType::Vertical
        } else if is_east {
            TileType::NorthEast
        } else {
            TileType::NorthWest
        }
    } else if is_east {
        if is_south {
            TileType::SouthEast
        } else {
            TileType::Horizontal
        }
    } else {
        TileType::SouthWest
    };

    maze[start_row][start_column].tile_type = start_type;

    nodes.push_back((start_row, start_column, 0, start_column, start_row));

    // go through the loop
    while let Some((row, column, value, prev_row, prev_column)) = nodes.pop_front() {
        let node = &maze[row][column];

        if node.value.is_some() {
            continue;
        }

        let _ = match node.tile_type {
            TileType::Start => panic!(), // we have replaced it
            TileType::Vertical => vec![(row + 1, column), (row - 1, column)],
            TileType::Horizontal => vec![(row, column + 1), (row, column - 1)],
            TileType::NorthEast => vec![(row - 1, column), (row, column + 1)],
            TileType::NorthWest => vec![(row - 1, column), (row, column - 1)],
            TileType::SouthEast => vec![(row + 1, column), (row, column + 1)],
            TileType::SouthWest => vec![(row + 1, column), (row, column - 1)],
            TileType::Ground => panic!(), // we should not arrive on ground
        }
        .into_iter()
        .filter(|(next_row, next_column)| *next_row != prev_row || *next_column != prev_column)
        .map(|(next_row, next_column)| {
            nodes.push_back((next_row, next_column, value + 1, row, column));
        })
        .collect::<Vec<()>>();

        // re-borrow as mutable
        let node = &mut maze[row][column];
        node.value = Some(value);

        max_maze_value = std::cmp::max(max_maze_value, value);
    }

    (maze, max_maze_value)
}

fn handle_part_1(input: &str) -> u64 {
    let (_, res) = parse_maze(input);

    res
}

fn handle_part_2(input: &str) -> u64 {
    let mut res = 0;

    let (maze, _) = parse_maze(input);

    for line in maze {
        let mut line_vertical_counter = 0;

        for tile in line {
            let tile_type = if tile.value.is_some() {
                tile.tile_type
            } else {
                TileType::Ground
            };

            match tile_type {
                TileType::Ground => {
                    let ab: i32 = line_vertical_counter % 4;
                    if ab.abs() == 2 {
                        res += 1
                    }
                }
                TileType::Vertical => line_vertical_counter += 2,
                TileType::NorthEast | TileType::SouthWest => line_vertical_counter -= 1,
                TileType::SouthEast | TileType::NorthWest => line_vertical_counter += 1,
                TileType::Start => panic!(), // we have removed it
                TileType::Horizontal => {}   // does not change
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{handle_part_1, handle_part_2};

    #[test]
    fn test_part_1() {
        let input = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "};

        assert_eq!(handle_part_1(input), 4);

        // Not handling edge case for start on the edge

        // let input = "..F7.
        // .FJ|.
        // SJ.L7
        // |F--J
        // LJ...";

        // assert_eq!(handle_part_1(input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "};

        assert_eq!(handle_part_2(input), 1);

        let input = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        "};

        assert_eq!(handle_part_2(input), 4);

        let input = indoc! {"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "};

        assert_eq!(handle_part_2(input), 8);

        let input = indoc! {"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        "};

        assert_eq!(handle_part_2(input), 10);
    }
}
