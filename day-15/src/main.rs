use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        sum_of_boxes_gps_coordinates_scaled_up(INPUT)
    );
}

#[allow(dead_code)]
fn sum_of_boxes_gps_coordinates(input: &str) -> usize {
    let (mut warehouse, movements) = parse_input(input);

    for movement in movements.iter() {
        warehouse.apply_robot_movement(movement);
    }

    warehouse
        .boxes
        .iter()
        .map(|b| {
            let box_position = b.first().unwrap();
            100 * box_position.y + box_position.x
        })
        .sum()
}

fn sum_of_boxes_gps_coordinates_scaled_up(input: &str) -> usize {
    let scaled_input = scale_up_input(input);
    let (mut warehouse, movements) = parse_input(scaled_input.as_str());

    for (i, movement) in movements.iter().enumerate() {
        println!();
        println!("Movement {}: {:?}", i, movement);
        warehouse.apply_robot_movement(movement);
    }

    warehouse
        .boxes
        .iter()
        .map(|b| {
            let box_position = b.first().unwrap();
            100 * box_position.y + box_position.x
        })
        .sum()
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Warehouse {
    robot: Coordinate,
    walls: Vec<Coordinate>,
    boxes: Vec<Vec<Coordinate>>,
}

impl Warehouse {
    fn apply_robot_movement(&mut self, movement: &Direction) {
        let mut positions_to_check = VecDeque::from([self.robot.translate(movement)]);

        let mut boxes_to_move = HashSet::new();

        while let Some(position_to_check) = positions_to_check.pop_front() {
            if self.wall_at(&position_to_check).is_some() {
                return;
            }

            if let Some(b) = self.box_at(&position_to_check) {
                boxes_to_move.insert(b.clone());

                match movement {
                    Direction::Up | Direction::Down => {
                        for position in b {
                            let position_to_check = position.translate(movement);
                            positions_to_check.push_back(position_to_check);
                        }
                    }
                    Direction::Left | Direction::Right => {
                        let mut position = position_to_check.clone();
                        loop {
                            if b.contains(&position) {
                                position = position.translate(movement);
                            } else {
                                break;
                            }
                        }
                        positions_to_check.push_back(position);
                    }
                }
            }
        }

        self.robot = self.robot.translate(movement);

        for b in boxes_to_move {
            for position in self.box_mut(&b).unwrap().iter_mut() {
                *position = position.translate(movement);
            }
        }
    }

    fn wall_at(&self, coordinate: &Coordinate) -> Option<&Coordinate> {
        self.walls.iter().find(|&w| w == coordinate)
    }

    fn box_at(&self, coordinate: &Coordinate) -> Option<&Vec<Coordinate>> {
        self.boxes.iter().find(|b| b.contains(coordinate))
    }

    fn box_mut(&mut self, r#box: &Vec<Coordinate>) -> Option<&mut Vec<Coordinate>> {
        self.boxes.iter_mut().find(|b| b == &r#box)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn translate(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y.checked_sub(1).unwrap(),
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y.checked_add(1).unwrap(),
            },
            Direction::Left => Coordinate {
                x: self.x.checked_sub(1).unwrap(),
                y: self.y,
            },
            Direction::Right => Coordinate {
                x: self.x.checked_add(1).unwrap(),
                y: self.y,
            },
        }
    }
}

fn scale_up_input(input: &str) -> String {
    input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
}

fn parse_input(input: &str) -> (Warehouse, Vec<Direction>) {
    let mut lines = input.lines();

    let mut robot = None;
    let mut walls = Vec::new();
    let mut boxes = Vec::new();

    for (y, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        for (x, character) in line.chars().enumerate() {
            match character {
                '@' => {
                    if robot.is_some() {
                        panic!("multiple robots present");
                    }
                    robot = Some(Coordinate { x, y });
                }
                '#' => {
                    walls.push(Coordinate { x, y });
                }
                'O' => {
                    boxes.push(vec![Coordinate { x, y }]);
                }
                '[' => {
                    boxes.push(vec![Coordinate { x, y }, Coordinate { x: x + 1, y }]);
                }
                _ => {}
            }
        }
    }

    let robot = robot.unwrap();

    let mut robot_movements = Vec::new();

    for line in lines {
        for character in line.chars() {
            let movement = match character {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("unknown character"),
            };

            robot_movements.push(movement);
        }
    }

    (
        Warehouse {
            robot,
            walls,
            boxes,
        },
        robot_movements,
    )
}

#[allow(dead_code)]
fn debug_warehouse(warehouse: &Warehouse) {
    let width = warehouse.walls.iter().map(|w| w.x).max().unwrap() + 1;
    let height = warehouse.walls.iter().map(|w| w.y).max().unwrap() + 1;

    for y in 0..height {
        for x in 0..width {
            let position = Coordinate { x, y };
            if warehouse.wall_at(&position).is_some() {
                print!("#");
                continue;
            }
            if let Some(b) = warehouse.box_at(&position) {
                match b.as_slice() {
                    [_only] => {
                        print!("O");
                    }
                    [left, _right] => {
                        if &position == left {
                            print!("[");
                        } else {
                            print!("]");
                        }
                    }
                    _ => {
                        panic!("boxes must be 1 or 2 squares wide");
                    }
                }
                continue;
            }
            if warehouse.robot == position {
                print!("@");
                continue;
            }
            print!(".");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = include_str!("aoc-input/example-input-1.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("aoc-input/example-input-2.txt");

    #[test]
    fn sum_of_boxes_gps_coordinates_works() {
        assert_eq!(sum_of_boxes_gps_coordinates(EXAMPLE_INPUT_1), 10092);
        assert_eq!(sum_of_boxes_gps_coordinates(EXAMPLE_INPUT_2), 2028);
    }

    #[test]
    fn sum_of_boxes_gps_coordinates_scaled_up_works() {
        assert_eq!(
            sum_of_boxes_gps_coordinates_scaled_up(EXAMPLE_INPUT_1),
            9021
        )
    }
}
