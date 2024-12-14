use std::{collections::HashSet, str::FromStr};

use lab_map::{LabMap, StepForwardError};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", distinct_guard_visit_positions(INPUT));
}

fn distinct_guard_visit_positions(input: &str) -> usize {
    let mut lab_map = LabMap::from_str(input).unwrap();

    let mut visited_positions = HashSet::new();

    loop {
        let current_guard_position = lab_map.current_guard_position();
        visited_positions.insert(current_guard_position.clone());

        while lab_map.is_next_step_obstructed() {
            lab_map.turn_right();
        }

        match lab_map.step_forward() {
            Ok(_) => {}
            Err(StepForwardError::LeftMappedArea) => break,
            Err(StepForwardError::Obstruction) => panic!("Obstructions should not be hit"),
        }
    }

    visited_positions.len()
}

mod lab_map {
    use std::str::FromStr;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub(crate) struct Position {
        x: usize,
        y: usize,
    }

    #[derive(Debug)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    impl Direction {
        fn turned_right(&self) -> Self {
            match self {
                Direction::Up => Self::Right,
                Direction::Right => Self::Down,
                Direction::Down => Self::Left,
                Direction::Left => Self::Up,
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct LabMap {
        width: usize,
        height: usize,
        guard_position: Position,
        guard_direction: Direction,
        obstruction_positions: Vec<Position>,
    }

    impl FromStr for LabMap {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines = s.lines();
            let mut width: Option<usize> = None;
            let mut height = 0;

            let mut guard_position: Option<Position> = None;
            let mut obstruction_positions: Vec<Position> = Vec::new();

            for (y, line) in lines.enumerate() {
                height += 1;
                if let Some(width) = width {
                    if line.len() != width {
                        panic!("Lines have different lengths.");
                    }
                } else {
                    width = Some(line.len());
                }

                for (x, character) in line.chars().enumerate() {
                    match character {
                        '^' => {
                            if guard_position.is_some() {
                                panic!("Guard position already found.");
                            }
                            guard_position = Some(Position { x, y });
                        }
                        '#' => {
                            obstruction_positions.push(Position { x, y });
                        }
                        _ => {}
                    };
                }
            }

            Ok(Self {
                width: width.expect("Width should be set."),
                height,
                guard_position: guard_position.expect("Guard position not found."),
                guard_direction: Direction::Up,
                obstruction_positions,
            })
        }
    }

    pub(crate) enum StepForwardError {
        Obstruction,
        LeftMappedArea,
    }

    impl LabMap {
        pub(crate) fn current_guard_position(&self) -> &Position {
            &self.guard_position
        }

        pub(crate) fn is_next_step_obstructed(&self) -> bool {
            match self.next_step_position() {
                Ok(_) => false,
                Err(StepForwardError::LeftMappedArea) => false,
                Err(StepForwardError::Obstruction) => true,
            }
        }

        pub(crate) fn turn_right(&mut self) {
            self.guard_direction = self.guard_direction.turned_right()
        }

        pub(crate) fn step_forward(&mut self) -> Result<(), StepForwardError> {
            self.guard_position = self.next_step_position()?;
            Ok(())
        }

        fn next_step_position(&self) -> Result<Position, StepForwardError> {
            match self.guard_direction {
                Direction::Up => {
                    if let Some(new_y) = self.guard_position.y.checked_sub(1) {
                        let position = Position {
                            y: new_y,
                            ..self.guard_position
                        };
                        if self.is_position_obstructed(&position) {
                            Err(StepForwardError::Obstruction)
                        } else {
                            Ok(position)
                        }
                    } else {
                        Err(StepForwardError::LeftMappedArea)
                    }
                }
                Direction::Right => {
                    let position = Position {
                        x: self.guard_position.x + 1,
                        ..self.guard_position
                    };
                    if position.x >= self.width {
                        Err(StepForwardError::LeftMappedArea)
                    } else if self.is_position_obstructed(&position) {
                        Err(StepForwardError::Obstruction)
                    } else {
                        Ok(position)
                    }
                }
                Direction::Down => {
                    let position = Position {
                        y: self.guard_position.y + 1,
                        ..self.guard_position
                    };
                    if position.y >= self.height {
                        Err(StepForwardError::LeftMappedArea)
                    } else if self.is_position_obstructed(&position) {
                        Err(StepForwardError::Obstruction)
                    } else {
                        Ok(position)
                    }
                }
                Direction::Left => {
                    if let Some(new_x) = self.guard_position.x.checked_sub(1) {
                        let position = Position {
                            x: new_x,
                            ..self.guard_position
                        };
                        if self.is_position_obstructed(&position) {
                            Err(StepForwardError::Obstruction)
                        } else {
                            Ok(position)
                        }
                    } else {
                        Err(StepForwardError::LeftMappedArea)
                    }
                }
            }
        }

        fn is_position_obstructed(&self, position: &Position) -> bool {
            self.obstruction_positions.contains(position)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn distinct_guard_visit_positions_works() {
        assert_eq!(distinct_guard_visit_positions(EXAMPLE_INPUT), 41);
    }
}
