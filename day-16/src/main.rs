use std::{collections::HashSet, f32::consts::PI, rc::Rc};

use glam::{IVec2, UVec2};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", calculate_lowest_score(INPUT));
}

fn calculate_lowest_score(input: &str) -> u32 {
    let maze = parse_input(input);

    let potential_directions = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

    let mut lowest_score = None;

    let mut paths_to_check = Vec::from([Rc::new(PathStep {
        position: maze.start,
        previous: None,
        score: 0,
        priority: calculate_priority(&maze.start, &maze.end, &IVec2::X, false),
    })]);

    while let Some(path_to_check) = paths_to_check.pop() {
        if let Some(ls) = lowest_score {
            if ls < path_to_check.score {
                continue;
            }
        }

        if maze.is_end(&path_to_check.position) {
            if path_to_check.score < lowest_score.unwrap_or(u32::MAX) {
                lowest_score = Some(path_to_check.score);
            }
            continue;
        }

        if maze.is_wall(&path_to_check.position) {
            continue;
        }

        {
            let mut current = path_to_check.clone();
            let mut already_visited = false;
            while let Some(previous) = &current.previous {
                if previous.position == path_to_check.position {
                    already_visited = true;
                    break;
                }

                current = previous.clone();
            }

            if already_visited {
                continue;
            }
        }

        for direction in potential_directions {
            let position = path_to_check.position.saturating_add_signed(direction);

            let Some(previous) = &path_to_check.previous else {
                let mut score = 1;

                if direction != IVec2::X {
                    score += 1000;
                }

                add_path_to_check(
                    &mut paths_to_check,
                    Rc::new(PathStep {
                        position,
                        previous: Some(path_to_check.clone()),
                        score,
                        priority: calculate_priority(&position, &maze.end, &direction, false),
                    }),
                );
                continue;
            };

            let previous_direction =
                path_to_check.position.as_ivec2() - previous.position.as_ivec2();

            let mut score = path_to_check.score + 1;

            if direction != previous_direction {
                score += 1000;
            }

            add_path_to_check(
                &mut paths_to_check,
                Rc::new(PathStep {
                    position,
                    previous: Some(path_to_check.clone()),
                    score,
                    priority: calculate_priority(
                        &position,
                        &maze.end,
                        &direction,
                        direction == previous_direction,
                    ),
                }),
            );
        }
    }

    lowest_score.unwrap()
}

fn calculate_priority(
    position: &UVec2,
    end: &UVec2,
    direction: &IVec2,
    is_same_direction: bool,
) -> i32 {
    let distance = (end.as_vec2() - position.as_vec2()).length_squared();
    let desired_direction_vec = end.as_vec2() - position.as_vec2();
    let angle_diff = direction.as_vec2().angle_to(desired_direction_vec);
    let angle_factor = 0.2 + 0.8 * (PI - angle_diff.abs()) / PI;
    (-distance * angle_factor * if is_same_direction { 1.0 } else { 0.5 } * 100.0).round() as i32
}

fn add_path_to_check(paths_to_check: &mut Vec<Rc<PathStep>>, path_to_check: Rc<PathStep>) {
    paths_to_check.push(path_to_check.clone());
    paths_to_check.sort_unstable_by_key(|p| p.priority);
}

#[derive(Debug)]
struct PathStep {
    position: UVec2,
    previous: Option<Rc<PathStep>>,
    score: u32,
    priority: i32,
}

impl PathStep {
    fn points(&self) -> Vec<UVec2> {
        let mut vec = vec![self.position];

        {
            let mut current = self.previous.clone();
            while let Some(c) = current {
                vec.push(c.position);
                current = c.previous.clone();
            }
        }

        vec.reverse();

        vec
    }
}

struct Maze {
    start: UVec2,
    end: UVec2,
    walls: HashSet<UVec2>,
}

impl Maze {
    fn is_start(&self, position: &UVec2) -> bool {
        position == &self.start
    }

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

#[allow(dead_code)]
fn debug_path(maze: &Maze, path: Rc<PathStep>) {
    let area_size = maze
        .walls
        .iter()
        .fold(UVec2::new(0, 0), |acc, &wall| acc.max(wall))
        + UVec2::ONE;

    let path_points = path.points();

    for y in 0..area_size.y {
        for x in 0..area_size.x {
            let position = UVec2 { x, y };

            if maze.is_wall(&position) {
                print!("#");
                continue;
            }
            let is_path_point = path_points.contains(&position);
            if maze.is_end(&position) {
                if is_path_point {
                    print!("!");
                } else {
                    print!("E");
                }
                continue;
            }
            if maze.is_start(&position) {
                print!("S");
                continue;
            }
            if is_path_point {
                print!("█");
                continue;
            }
            print!(" ");
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use crate::calculate_lowest_score;

    const EXAMPLE_INPUT_1: &str = include_str!("aoc-input/example-input-1.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("aoc-input/example-input-2.txt");

    #[test]
    fn calculate_lowest_score_works() {
        assert_eq!(calculate_lowest_score(EXAMPLE_INPUT_1), 7036);
        assert_eq!(calculate_lowest_score(EXAMPLE_INPUT_2), 11048);
    }
}
