use std::collections::HashSet;

use glam::{IVec2, UVec2};
use pathfinding::prelude::*;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_tiles_on_best_paths(INPUT));
}

#[allow(dead_code)]
fn calculate_lowest_score(input: &str) -> u32 {
    let maze = parse_input(input);

    let (_path, score) = dijkstra(
        &(maze.start, IVec2::X),
        |(position, direction)| {
            let mut successors = vec![
                ((*position, direction.perp()), 1000),
                ((*position, -direction.perp()), 1000),
            ];
            let next_position = (position.as_ivec2() + direction).as_uvec2();

            if !maze.is_wall(&next_position) {
                successors.push(((next_position, *direction), 1));
            }

            successors
        },
        |(position, _direction)| maze.is_end(position),
    )
    .unwrap();

    score
}

fn count_tiles_on_best_paths(input: &str) -> usize {
    let maze = parse_input(input);

    let (paths, _score) = astar_bag(
        &(maze.start, IVec2::X),
        |(position, direction)| {
            let mut successors = vec![
                ((*position, direction.perp()), 1000),
                ((*position, -direction.perp()), 1000),
            ];
            let next_position = (position.as_ivec2() + direction).as_uvec2();

            if !maze.is_wall(&next_position) {
                successors.push(((next_position, *direction), 1));
            }

            successors
        },
        |_| 0,
        |(position, _direction)| maze.is_end(position),
    )
    .unwrap();

    let mut visited_tiles = HashSet::new();

    for path in paths {
        for (tile, _score) in path {
            visited_tiles.insert(tile);
        }
    }

    visited_tiles.len()
}

struct Maze {
    start: UVec2,
    end: UVec2,
    walls: HashSet<UVec2>,
}

impl Maze {
    fn is_end(&self, position: &UVec2) -> bool {
        position == &self.end
    }

    fn is_wall(&self, position: &UVec2) -> bool {
        self.walls.contains(position)
    }
}

fn parse_input(input: &str) -> Maze {
    let mut lines = input.lines();

    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();

    for (y, line) in lines.by_ref().enumerate() {
        let y: u32 = y.try_into().expect("it to fit a u32");

        for (x, character) in line.chars().enumerate() {
            let x: u32 = x.try_into().expect("it to fit a u32");

            match character {
                'S' => {
                    if start.is_some() {
                        unreachable!("multiple starts present");
                    }
                    start = Some(UVec2 { x, y });
                }
                'E' => {
                    if end.is_some() {
                        unreachable!("multiple ends present");
                    }
                    end = Some(UVec2 { x, y });
                }
                '#' => {
                    walls.insert(UVec2 { x, y });
                }
                _ => {}
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();

    Maze { start, end, walls }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = include_str!("aoc-input/example-input-1.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("aoc-input/example-input-2.txt");

    #[test]
    fn calculate_lowest_score_works() {
        assert_eq!(calculate_lowest_score(EXAMPLE_INPUT_1), 7036);
        assert_eq!(calculate_lowest_score(EXAMPLE_INPUT_2), 11048);
    }

    #[test]
    fn count_tiles_on_best_paths_works() {
        assert_eq!(count_tiles_on_best_paths(EXAMPLE_INPUT_1), 45);
        assert_eq!(count_tiles_on_best_paths(EXAMPLE_INPUT_2), 64);
    }
}
