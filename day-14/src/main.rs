use std::io;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        render_robot_movements_until_christmas_tree(INPUT, 101, 103)
    );
}

#[allow(dead_code)]
fn calculate_safety_factor_after_seconds(
    input: &str,
    seconds: u16,
    area_width: u16,
    area_height: u16,
) -> u32 {
    let mut robots = parse_input(input);

    for robot in robots.as_mut_slice() {
        robot.simulate_movement(seconds, area_width, area_height);
    }

    calculate_safety_factor(&robots, area_width, area_height)
}

fn render_robot_movements_until_christmas_tree(
    input: &str,
    area_width: u16,
    area_height: u16,
) -> Option<u16> {
    let mut robots = parse_input(input);
    let robots_clone = robots.clone();

    let mut safety_factors = (1..=u16::MAX)
        .map(|i| {
            for robot in robots.as_mut_slice() {
                robot.simulate_movement(1, area_width, area_height);
            }

            (i, calculate_safety_factor(&robots, area_width, area_height))
        })
        .collect::<Vec<_>>();

    safety_factors.sort_by(|a, b| a.1.cmp(&b.1));

    for (i, _) in safety_factors {
        let mut robots = robots_clone.clone();
        for robot in robots.as_mut_slice() {
            robot.simulate_movement(i, area_width, area_height);
        }

        render_robots(&robots, area_width, area_height);

        println!("Is this a Christmas tree? [i = {}] (y/N)", i);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("user input expected to succeed");
        if input.to_lowercase().trim_start().starts_with('y') {
            return Some(i);
        }
    }

    None
}

fn calculate_safety_factor(robots: &[Robot], area_width: u16, area_height: u16) -> u32 {
    let x_midpoint: u16 = area_width / 2;
    let y_midpoint: u16 = area_height / 2;

    let mut quadrant_counts: [u16; 4] = [0, 0, 0, 0];

    for robot in robots {
        let quadrant = match robot.position {
            Position { x, y } if x < x_midpoint && y < y_midpoint => 0,
            Position { x, y } if x > x_midpoint && y < y_midpoint => 1,
            Position { x, y } if x < x_midpoint && y > y_midpoint => 2,
            Position { x, y } if x > x_midpoint && y > y_midpoint => 3,
            _ => continue,
        };

        *quadrant_counts.get_mut(quadrant).unwrap() += 1;
    }

    quadrant_counts.iter().map(|&x| x as u32).product()
}

fn render_robots(robots: &[Robot], area_width: u16, area_height: u16) {
    for y in 0..area_height {
        for x in 0..area_width {
            let count = robots
                .iter()
                .filter(|r| r.position.x == x && r.position.y == y)
                .count();
            print!(
                "{}",
                if count > 0 {
                    String::from("█")
                } else {
                    String::from(" ")
                }
            );
        }
        println!();
    }
}

#[derive(Clone, Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn simulate_movement(&mut self, seconds: u16, area_width: u16, area_height: u16) {
        let seconds = seconds as i32;
        let area_width = area_width as i32;
        let area_height = area_height as i32;

        let total_distance_x = self.velocity.x as i32 * seconds;
        let total_distance_y = self.velocity.y as i32 * seconds;

        let wrapped_total_distance_x = total_distance_x % area_width;
        let wrapped_total_distance_y = total_distance_y % area_height;

        let new_position_x = wrap(
            self.position.x as i32 + wrapped_total_distance_x,
            area_width,
        );
        let new_position_y = wrap(
            self.position.y as i32 + wrapped_total_distance_y,
            area_height,
        );

        self.position = Position {
            x: new_position_x.try_into().unwrap(),
            y: new_position_y.try_into().unwrap(),
        };
    }
}

fn wrap(value: i32, max: i32) -> i32 {
    if value < 0 {
        value + max
    } else if value >= max {
        value - max
    } else {
        value
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Clone, Debug)]
struct Velocity {
    x: i16,
    y: i16,
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();

            let (_, position) = parts.next().unwrap().split_once('=').unwrap();

            let (position_x, position_y) = position.split_once(',').unwrap();

            let (_, velocity) = parts.next().unwrap().split_once('=').unwrap();

            let (velocity_x, velocity_y) = velocity.split_once(',').unwrap();

            Robot {
                position: Position {
                    x: position_x.parse().unwrap(),
                    y: position_y.parse().unwrap(),
                },
                velocity: Velocity {
                    x: velocity_x.parse().unwrap(),
                    y: velocity_y.parse().unwrap(),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn calculate_safety_factor_after_seconds_works() {
        assert_eq!(
            calculate_safety_factor_after_seconds(EXAMPLE_INPUT, 100, 11, 7),
            12
        );
    }

    #[test]
    fn simulate_robot_movement_works() {
        let mut robot = Robot {
            position: Position { x: 2, y: 4 },
            velocity: Velocity { x: 2, y: -3 },
        };
        robot.simulate_movement(1, 11, 7);
        assert_eq!(robot.position, Position { x: 4, y: 1 });
        robot.simulate_movement(1, 11, 7);
        assert_eq!(robot.position, Position { x: 6, y: 5 });
        robot.simulate_movement(1, 11, 7);
        assert_eq!(robot.position, Position { x: 8, y: 2 });
        robot.simulate_movement(1, 11, 7);
        assert_eq!(robot.position, Position { x: 10, y: 6 });
        robot.simulate_movement(1, 11, 7);
        assert_eq!(robot.position, Position { x: 1, y: 3 });
    }
}
