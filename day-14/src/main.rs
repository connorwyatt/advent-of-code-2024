const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        calculate_safety_factor(INPUT, 100, 101, 103)
    );
}

fn calculate_safety_factor(input: &str, seconds: u16, area_width: u16, area_height: u16) -> u32 {
    let x_midpoint: u16 = area_width / 2;
    let y_midpoint: u16 = area_height / 2;

    let robots = parse_input(input);

    let mut quadrant_counts: [u16; 4] = [0, 0, 0, 0];

    for robot in robots {
        let position = simulate_robot_movement(&robot, seconds, area_width, area_height);

        let quadrant = match position {
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

fn simulate_robot_movement(
    robot: &Robot,
    seconds: u16,
    area_width: u16,
    area_height: u16,
) -> Position {
    let seconds = seconds as i16;
    let area_width_i16 = area_width as i16;
    let area_height_i16 = area_height as i16;

    let position = &robot.position;

    let total_distance_x = robot.velocity.x * seconds;
    let total_distance_y = robot.velocity.y * seconds;

    let wrapped_total_distance_x = total_distance_x % area_width_i16;
    let wrapped_total_distance_y = total_distance_y % area_height_i16;

    let new_position_x = wrap(position.x as i16 + wrapped_total_distance_x, area_width_i16);
    let new_position_y = wrap(
        position.y as i16 + wrapped_total_distance_y,
        area_height_i16,
    );

    Position {
        x: new_position_x.try_into().unwrap(),
        y: new_position_y.try_into().unwrap(),
    }
}

fn wrap(value: i16, max: i16) -> i16 {
    if value < 0 {
        value + max
    } else if value >= max {
        value - max
    } else {
        value
    }
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

#[derive(Debug, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug)]
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
    fn calculate_safety_factor_works() {
        assert_eq!(calculate_safety_factor(EXAMPLE_INPUT, 100, 11, 7), 12);
    }

    #[test]
    fn simulate_robot_movement_works() {
        let robot = Robot {
            position: Position { x: 2, y: 4 },
            velocity: Velocity { x: 2, y: -3 },
        };
        assert_eq!(
            simulate_robot_movement(&robot, 1, 11, 7),
            Position { x: 4, y: 1 }
        );
        assert_eq!(
            simulate_robot_movement(&robot, 2, 11, 7),
            Position { x: 6, y: 5 }
        );
        assert_eq!(
            simulate_robot_movement(&robot, 3, 11, 7),
            Position { x: 8, y: 2 }
        );
        assert_eq!(
            simulate_robot_movement(&robot, 4, 11, 7),
            Position { x: 10, y: 6 }
        );
        assert_eq!(
            simulate_robot_movement(&robot, 5, 11, 7),
            Position { x: 1, y: 3 }
        );
    }
}
