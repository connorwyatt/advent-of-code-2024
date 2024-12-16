use std::{collections::HashSet, str::FromStr};

use models::{AntennaMap, Position};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_antinodes_with_harmonics(INPUT));
}

#[allow(dead_code)]
fn count_antinodes(input: &str) -> usize {
    let antenna_map = AntennaMap::from_str(input).unwrap();

    let mut antinode_positions: HashSet<Position> = HashSet::new();

    for frequency_antenna_positions in antenna_map.antenna_positions().values() {
        for antenna_position in frequency_antenna_positions {
            for comparison_antenna_position in frequency_antenna_positions {
                if antenna_position == comparison_antenna_position {
                    continue;
                }

                if let Some(mirrored_position) = antenna_position.mirrored_by(
                    comparison_antenna_position,
                    &1,
                    antenna_map.width(),
                    antenna_map.height(),
                ) {
                    antinode_positions.insert(mirrored_position);
                }
            }
        }
    }

    antinode_positions.len()
}

fn count_antinodes_with_harmonics(input: &str) -> usize {
    let antenna_map = AntennaMap::from_str(input).unwrap();

    let mut antinode_positions: HashSet<Position> = HashSet::new();

    for frequency_antenna_positions in antenna_map.antenna_positions().values() {
        for antenna_position in frequency_antenna_positions {
            for comparison_antenna_position in frequency_antenna_positions {
                if antenna_position == comparison_antenna_position {
                    continue;
                }

                antinode_positions.insert(antenna_position.clone());

                let mut multiplier = 1;

                while let Some(mirrored_position) = antenna_position.mirrored_by(
                    comparison_antenna_position,
                    &multiplier,
                    antenna_map.width(),
                    antenna_map.height(),
                ) {
                    antinode_positions.insert(mirrored_position);

                    multiplier += 1;
                }
            }
        }
    }

    antinode_positions.len()
}

mod models {
    use std::{collections::HashMap, str::FromStr};

    #[derive(Eq, Hash, PartialEq)]
    pub(crate) struct Frequency(char);

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub(crate) struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        pub(crate) fn mirrored_by(
            &self,
            comparison_antenna_position: &Position,
            multiplier: &usize,
            width: &usize,
            height: &usize,
        ) -> Option<Position> {
            let x: i32 = self.x.try_into().unwrap();
            let y: i32 = self.y.try_into().unwrap();
            let comparison_x: i32 = comparison_antenna_position.x.try_into().unwrap();
            let comparison_y: i32 = comparison_antenna_position.y.try_into().unwrap();
            let multiplier: i32 = (*multiplier).try_into().unwrap();

            let x_diff = comparison_x - x;
            let y_diff = comparison_y - y;

            let mirrored_x: i32 = x + (x_diff * (multiplier + 1));
            let mirrored_y: i32 = y + (y_diff * (multiplier + 1));

            if mirrored_x < 0 || mirrored_y < 0 {
                None
            } else {
                let new_x: usize = mirrored_x.try_into().unwrap();
                let new_y: usize = mirrored_y.try_into().unwrap();

                if &new_x >= width || &new_y >= height {
                    None
                } else {
                    Some(Position { x: new_x, y: new_y })
                }
            }
        }
    }

    pub(crate) struct AntennaMap {
        width: usize,
        height: usize,
        antenna_positions: HashMap<Frequency, Vec<Position>>,
    }

    impl FromStr for AntennaMap {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut antenna_positions: HashMap<Frequency, Vec<Position>> = HashMap::new();

            let mut height = 0;
            let mut width: Option<usize> = None;

            for (y, line) in s.lines().enumerate() {
                height += 1;

                if let Some(c) = width {
                    if line.len() != c {
                        panic!("width is variable");
                    }
                } else {
                    width = Some(line.len());
                }

                for (x, character) in line.chars().enumerate() {
                    if character == '.' {
                        continue;
                    }

                    let frequency = Frequency(character);

                    antenna_positions
                        .entry(frequency)
                        .or_default()
                        .push(Position { x, y });
                }
            }

            Ok(AntennaMap {
                width: width.unwrap(),
                height,
                antenna_positions,
            })
        }
    }

    impl AntennaMap {
        pub(crate) fn width(&self) -> &usize {
            &self.width
        }

        pub(crate) fn height(&self) -> &usize {
            &self.height
        }

        pub(crate) fn antenna_positions(&self) -> &HashMap<Frequency, Vec<Position>> {
            &self.antenna_positions
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn count_antinodes_works() {
        assert_eq!(count_antinodes(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn count_antinodes_with_harmonics_works() {
        assert_eq!(count_antinodes_with_harmonics(EXAMPLE_INPUT), 34);
    }
}
