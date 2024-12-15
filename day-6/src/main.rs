use std::{
    collections::HashSet,
    str::FromStr,
    thread::{self, available_parallelism},
};

use crossbeam_channel::{Receiver, Sender};
use lab_map::{LabMap, Position, StepForwardError};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", add_obstruction_potential_positions(INPUT));
}

#[allow(dead_code)]
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

fn add_obstruction_potential_positions(input: &str) -> usize {
    let lab_map = LabMap::from_str(input).unwrap();

    let num_threads = available_parallelism().unwrap().get();
    let (sender, receiver): (Sender<Position>, Receiver<Position>) = crossbeam_channel::bounded(1);

    let join_handles = (0..num_threads - 1)
        .map(|_| {
            let mut lab_map = lab_map.clone();
            let receiver = receiver.clone();
            thread::spawn(move || {
                let mut potential_obstruction_positions_count: usize = 0;

                while let Ok(position) = receiver.recv() {
                    lab_map.reset();

                    if lab_map.obstruction_positions().contains(&position)
                        || lab_map.current_guard_position() == &position
                    {
                        continue;
                    };

                    lab_map.add_obstruction(&position);

                    let mut visited_positions_and_directions = HashSet::new();

                    loop {
                        while lab_map.is_next_step_obstructed() {
                            lab_map.turn_right();
                        }

                        let guard_position_and_direction = (
                            lab_map.current_guard_position().clone(),
                            lab_map.current_guard_direction().clone(),
                        );

                        if visited_positions_and_directions.contains(&guard_position_and_direction)
                        {
                            potential_obstruction_positions_count += 1;
                            break;
                        }

                        match lab_map.step_forward() {
                            Ok(_) => {}
                            Err(StepForwardError::LeftMappedArea) => break,
                            Err(StepForwardError::Obstruction) => {
                                panic!("Obstructions should not be hit")
                            }
                        }

                        visited_positions_and_directions.insert(guard_position_and_direction);
                    }
                }

                potential_obstruction_positions_count
            })
        })
        .collect::<Vec<_>>();

    for y in 0..lab_map.height() {
        for x in 0..lab_map.width() {
            let position = Position::new(x, y);

            sender.send(position).unwrap();
        }
    }

    drop(sender);

    let mut potential_obstruction_positions_count = 0;
    for handle in join_handles {
        potential_obstruction_positions_count += handle.join().unwrap();
    }

    potential_obstruction_positions_count
}

mod lab_map {
    use std::str::FromStr;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub(crate) struct Position {
        x: usize,
        y: usize,
    }

    impl Position {
        pub(crate) fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub(crate) enum Direction {
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

    #[derive(Clone, Debug)]
    pub(crate) struct LabMap {
        width: usize,
        height: usize,
        initial_guard_position: Position,
        guard_position: Position,
        initial_guard_direction: Direction,
        guard_direction: Direction,
        initial_obstruction_positions: Vec<Position>,
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

            let guard_position = guard_position.expect("Guard position not found.");

            let guard_direction = Direction::Up;

            Ok(Self {
                width: width.expect("Width should be set."),
                height,
                initial_guard_position: guard_position.clone(),
                guard_position,
                initial_guard_direction: guard_direction.clone(),
                guard_direction,
                initial_obstruction_positions: obstruction_positions.clone(),
                obstruction_positions,
            })
        }
    }

    pub(crate) enum StepForwardError {
        Obstruction,
        LeftMappedArea,
    }

    impl LabMap {
        pub(crate) fn width(&self) -> usize {
            self.width
        }

        pub(crate) fn height(&self) -> usize {
            self.height
        }

        pub(crate) fn obstruction_positions(&self) -> &Vec<Position> {
            &self.obstruction_positions
        }

        pub(crate) fn current_guard_position(&self) -> &Position {
            &self.guard_position
        }

        pub(crate) fn current_guard_direction(&self) -> &Direction {
            &self.guard_direction
        }

        pub(crate) fn is_next_step_obstructed(&self) -> bool {
            match self.next_step_position() {
                Ok(_) => false,
                Err(StepForwardError::LeftMappedArea) => false,
                Err(StepForwardError::Obstruction) => true,
            }
        }

        pub(crate) fn turn_right(&mut self) {
            self.guard_direction = self.guard_direction.turned_right();
        }

        pub(crate) fn step_forward(&mut self) -> Result<(), StepForwardError> {
            self.guard_position = self.next_step_position()?;
            Ok(())
        }

        pub(crate) fn add_obstruction(&mut self, position: &Position) {
            self.obstruction_positions.push(position.clone());
        }

        pub(crate) fn reset(&mut self) {
            self.guard_position = self.initial_guard_position.clone();
            self.guard_direction = self.initial_guard_direction.clone();
            self.obstruction_positions = self.initial_obstruction_positions.clone();
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

    #[test]
    fn add_obstruction_potential_positions_works() {
        assert_eq!(add_obstruction_potential_positions(EXAMPLE_INPUT), 6);
    }
}
