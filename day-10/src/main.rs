use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    str::FromStr,
};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", sum_of_trailhead_ratings(INPUT));
}

#[allow(dead_code)]
fn sum_of_trailhead_scores(input: &str) -> u16 {
    let topographical_map = TopographicalMap::from_str(input).unwrap();

    topographical_map
        .trailhead_positions()
        .iter()
        .map(|trailhead_position| calculate_trailhead_score(trailhead_position, &topographical_map))
        .sum()
}

fn sum_of_trailhead_ratings(input: &str) -> u16 {
    let topographical_map = TopographicalMap::from_str(input).unwrap();

    topographical_map
        .trailhead_positions()
        .iter()
        .map(|trailhead_position| {
            calculate_trailhead_rating(trailhead_position, &topographical_map)
        })
        .sum()
}

fn calculate_trailhead_score(
    trailhead_position: &Position,
    topographical_map: &TopographicalMap,
) -> u16 {
    let mut stack = VecDeque::from([(trailhead_position.clone(), 0)]);
    let mut nine_height_positions_reachable = HashSet::new();

    while !stack.is_empty() {
        let (position, height) = stack.pop_back().unwrap();

        for direction in Direction::all() {
            let Some(translated_position) = position.translate(&direction) else {
                continue;
            };

            let Some(translated_position_height) = topographical_map.get(&translated_position)
            else {
                continue;
            };

            if translated_position_height == height + 1 {
                if translated_position_height == 9 {
                    nine_height_positions_reachable.insert(translated_position);
                    continue;
                }

                stack.push_back((translated_position, translated_position_height));
            }
        }
    }

    nine_height_positions_reachable.len().try_into().unwrap()
}

fn calculate_trailhead_rating(
    trailhead_position: &Position,
    topographical_map: &TopographicalMap,
) -> u16 {
    let mut stack = VecDeque::from([(trailhead_position.clone(), 0)]);
    let mut rating = 0u16;

    while !stack.is_empty() {
        let (position, height) = stack.pop_back().unwrap();

        for direction in Direction::all() {
            let Some(translated_position) = position.translate(&direction) else {
                continue;
            };

            let Some(translated_position_height) = topographical_map.get(&translated_position)
            else {
                continue;
            };

            if translated_position_height == height + 1 {
                if translated_position_height == 9 {
                    rating += 1;
                    continue;
                }

                stack.push_back((translated_position, translated_position_height));
            }
        }
    }

    rating
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(u8, u8);

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Self(x, y)
    }

    fn translate(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let new_y = self.1.checked_sub(1)?;
                Some(Position::new(self.0, new_y))
            }
            Direction::Right => {
                let new_x = self.0.checked_add(1)?;
                Some(Position::new(new_x, self.1))
            }
            Direction::Down => {
                let new_y = self.1.checked_add(1)?;
                Some(Position::new(self.0, new_y))
            }
            Direction::Left => {
                let new_x = self.0.checked_sub(1)?;
                Some(Position::new(new_x, self.1))
            }
        }
    }
}

#[derive(Debug)]
struct TopographicalMap {
    heights: Vec<Vec<u8>>,
    trailhead_positions: HashSet<Position>,
}

impl FromStr for TopographicalMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trailhead_positions = HashSet::new();

        let heights = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let y: u8 = y.try_into().unwrap();

                line.chars()
                    .enumerate()
                    .map(|(x, character)| {
                        let x: u8 = x.try_into().unwrap();

                        let height: u8 = character.to_digit(10).unwrap().try_into().unwrap();

                        if height == 0 {
                            trailhead_positions.insert(Position::new(x, y));
                        }

                        height
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            heights,
            trailhead_positions,
        })
    }
}

impl TopographicalMap {
    fn trailhead_positions(&self) -> &HashSet<Position> {
        &self.trailhead_positions
    }

    fn get(&self, position: &Position) -> Option<u8> {
        let x: usize = position.0.into();
        let y: usize = position.1.into();

        Some(*self.heights.get(y)?.get(x)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn sum_of_trailhead_scores_works() {
        assert_eq!(sum_of_trailhead_scores(EXAMPLE_INPUT), 36);
    }

    #[test]
    fn sum_of_trailhead_ratings_works() {
        assert_eq!(sum_of_trailhead_ratings(EXAMPLE_INPUT), 81);
    }

    #[test]
    fn calculate_trailhead_score_works() {
        let topographical_map = TopographicalMap::from_str(EXAMPLE_INPUT).unwrap();

        assert_eq!(
            calculate_trailhead_score(&Position::new(2, 0), &topographical_map),
            5
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(4, 0), &topographical_map),
            6
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(4, 2), &topographical_map),
            5
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(6, 4), &topographical_map),
            3
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(2, 5), &topographical_map),
            1
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(5, 5), &topographical_map),
            3
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(0, 6), &topographical_map),
            5
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(6, 6), &topographical_map),
            3
        );
        assert_eq!(
            calculate_trailhead_score(&Position::new(1, 7), &topographical_map),
            5
        );
    }

    #[test]
    fn calculate_trailhead_rating_works() {
        let topographical_map = TopographicalMap::from_str(EXAMPLE_INPUT).unwrap();

        assert_eq!(
            calculate_trailhead_rating(&Position::new(2, 0), &topographical_map),
            20
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(4, 0), &topographical_map),
            24
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(4, 2), &topographical_map),
            10
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(6, 4), &topographical_map),
            4
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(2, 5), &topographical_map),
            1
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(5, 5), &topographical_map),
            4
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(0, 6), &topographical_map),
            5
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(6, 6), &topographical_map),
            8
        );
        assert_eq!(
            calculate_trailhead_rating(&Position::new(1, 7), &topographical_map),
            5
        );
    }
}
