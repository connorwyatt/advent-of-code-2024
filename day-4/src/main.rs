mod grid;

use std::str::FromStr;

use grid::{Cursor, Direction, Grid};

const TARGET_STRING: &str = "XMAS";

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_xmas_occurrences(INPUT));
}

fn count_xmas_occurrences(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    let mut occurrences = 0;

    for row in 0..grid.rows {
        for column in 0..grid.columns {
            occurrences += count_occurrences(&mut grid, row, column);
        }
    }

    occurrences
}

fn count_occurrences(grid: &mut Grid, row: usize, column: usize) -> usize {
    let searches = [
        vec![Direction::Up],
        vec![Direction::Up, Direction::Right],
        vec![Direction::Right],
        vec![Direction::Down, Direction::Right],
        vec![Direction::Down],
        vec![Direction::Down, Direction::Left],
        vec![Direction::Left],
        vec![Direction::Up, Direction::Left],
    ];

    grid.set_cursor(Cursor { x: column, y: row }).unwrap();

    if grid.get() != 'X' {
        return 0;
    };

    let count = searches
        .iter()
        .filter(|search_directions| {
            let mut word = String::new();

            grid.set_cursor(Cursor { x: column, y: row }).unwrap();

            word.push(grid.get());

            for _ in 1..TARGET_STRING.len() {
                let result = search_directions
                    .iter()
                    .map(|search_direction| grid.move_cursor(*search_direction))
                    .collect::<Result<Vec<()>, _>>();

                if result.is_err() {
                    break;
                }

                word.push(grid.get());
            }

            word == TARGET_STRING
        })
        .count();

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn count_xmas_occurrences_works() {
        assert_eq!(count_xmas_occurrences(EXAMPLE_INPUT), 18);
    }
}
