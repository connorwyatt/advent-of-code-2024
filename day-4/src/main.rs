mod grid;

use std::str::FromStr;

use grid::{Cursor, Direction, Grid};

const TARGET_STRING: &str = "XMAS";

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_x_mas_occurrences(INPUT));
}

#[allow(dead_code)]
fn count_xmas_occurrences(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    let mut occurrences = 0;

    for row in 0..grid.rows {
        for column in 0..grid.columns {
            occurrences +=
                check_surrounding_characters_for_xmas_occurrences(&mut grid, row, column);
        }
    }

    occurrences
}

fn count_x_mas_occurrences(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();

    let mut occurrences = 0;

    for row in 1..grid.rows - 1 {
        for column in 1..grid.columns - 1 {
            if is_center_of_x_mas_occurrence(&mut grid, row, column) {
                occurrences += 1;
            }
        }
    }

    occurrences
}

fn check_surrounding_characters_for_xmas_occurrences(
    grid: &mut Grid,
    row: usize,
    column: usize,
) -> usize {
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

fn is_center_of_x_mas_occurrence(grid: &mut Grid, row: usize, column: usize) -> bool {
    grid.set_cursor(Cursor { x: column, y: row }).unwrap();

    if grid.get() != 'A' {
        return false;
    };

    grid.move_cursor(Direction::Up).unwrap();
    grid.move_cursor(Direction::Left).unwrap();
    let top_left = grid.get();
    grid.move_cursor(Direction::Right).unwrap();
    grid.move_cursor(Direction::Right).unwrap();
    let top_right = grid.get();
    grid.move_cursor(Direction::Down).unwrap();
    grid.move_cursor(Direction::Down).unwrap();
    let bottom_right = grid.get();
    grid.move_cursor(Direction::Left).unwrap();
    grid.move_cursor(Direction::Left).unwrap();
    let bottom_left = grid.get();

    let top_left_bottom_right_set = [top_left, bottom_right];
    let top_right_bottom_left_set = [top_right, bottom_left];

    if top_left_bottom_right_set.contains(&'M')
        && top_left_bottom_right_set.contains(&'S')
        && top_right_bottom_left_set.contains(&'M')
        && top_right_bottom_left_set.contains(&'S')
    {
        return true;
    };

    false
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn count_xmas_occurrences_works() {
        assert_eq!(count_xmas_occurrences(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn count_x_mas_occurrences_works() {
        assert_eq!(count_x_mas_occurrences(EXAMPLE_INPUT), 9);
    }
}
