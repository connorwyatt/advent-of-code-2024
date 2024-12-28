use core::panic;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", sum_of_boxes_gps_coordinates(INPUT));
}

fn sum_of_boxes_gps_coordinates(input: &str) -> usize {
    let (mut warehouse, movements) = parse_input(input);

    for movement in movements.iter() {
        warehouse.apply_robot_movement(movement);
    }

    warehouse.boxes.iter().map(|b| 100 * b.y + b.x).sum()
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
    boxes: Vec<Coordinate>,
}

impl Warehouse {
    fn apply_robot_movement(&mut self, movement: &Direction) {
        let mut position = self.robot.clone();

        let mut boxes = Vec::new();

        loop {
            let Some(new_position) = position.translate(movement) else {
                break;
            };
            position = new_position;

            if self.walls.iter().any(|w| w == &position) {
                return;
            }
            if self.boxes.iter().any(|b| b == &position) {
                boxes.push(position.clone());
                continue;
            }
            break;
        }

        if boxes.is_empty() {
            self.robot = position.clone();
            return;
        }

        let first_box = self
            .boxes
            .iter_mut()
            .find(|b| b == &boxes.first().unwrap())
            .unwrap();

        self.robot = first_box.clone();
        *first_box = position.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn translate(&self, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => Some(Coordinate {
                x: self.x,
                y: self.y.checked_sub(1)?,
            }),
            Direction::Down => Some(Coordinate {
                x: self.x,
                y: self.y.checked_add(1)?,
            }),
            Direction::Left => Some(Coordinate {
                x: self.x.checked_sub(1)?,
                y: self.y,
            }),
            Direction::Right => Some(Coordinate {
                x: self.x.checked_add(1)?,
                y: self.y,
            }),
        }
    }
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
                    boxes.push(Coordinate { x, y });
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
            if warehouse.walls.iter().any(|w| w == &position) {
                print!("#");
                continue;
            }
            if warehouse.boxes.iter().any(|b| b == &position) {
                print!("O");
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
}
