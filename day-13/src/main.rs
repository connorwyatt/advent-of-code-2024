use std::usize;

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        calculate_minimum_tokens_to_win_all_prizes_with_corrected_prize_positions(INPUT)
    );
}

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

#[allow(dead_code)]
fn calculate_minimum_tokens_to_win_all_prizes(input: &str) -> usize {
    let machines = parse_input(input);

    calculate_minimum_tokens(&machines)
}

fn calculate_minimum_tokens_to_win_all_prizes_with_corrected_prize_positions(input: &str) -> usize {
    let machines = parse_input(input)
        .iter()
        .map(|machine| {
            Machine::new(
                machine.button_a.clone(),
                machine.button_b.clone(),
                Position::new(
                    machine.prize.x + 10000000000000,
                    machine.prize.y + 10000000000000,
                ),
            )
        })
        .collect::<Vec<_>>();

    calculate_minimum_tokens(&machines)
}

fn calculate_minimum_tokens(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|machine| {
            let button_a_x = machine.button_a.x as f64;
            let button_a_y = machine.button_a.y as f64;
            let button_b_x = machine.button_b.x as f64;
            let button_b_y = machine.button_b.y as f64;
            let prize_x = machine.prize.x as f64;
            let prize_y = machine.prize.y as f64;

            // Eq 1: ax * na + bx * nb = px
            // Eq 2: ay * na + by * nb = py

            // Cramer's rule:

            // Matrix:
            // ax bx
            // ay by

            let determinant = button_a_x * button_b_y - button_b_x * button_a_y;

            // A Matrix:
            // px bx
            // py by

            let determinant_a = prize_x * button_b_y - button_b_x * prize_y;

            // B Matrix:
            // ax px
            // ay py

            let determinant_b = button_a_x * prize_y - prize_x * button_a_y;

            let a = determinant_a / determinant;
            let b = determinant_b / determinant;

            if a < 0.0 || b < 0.0 || a.trunc() != a || b.trunc() != b {
                return None;
            }

            let result = a * BUTTON_A_COST as f64 + b * BUTTON_B_COST as f64;

            Some(result as usize)
        })
        .sum()
}

lazy_static! {
    static ref BUTTON_REGEX: Regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_REGEX: Regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut lines = input.lines();

    loop {
        let button_a_line = loop {
            match lines.next() {
                None => break None,
                Some(line) => {
                    if line.is_empty() {
                        continue;
                    } else {
                        break Some(line);
                    }
                }
            };
        };
        let Some(button_a_line) = button_a_line else {
            break;
        };
        let button_b_line = lines.next().expect("aoc data should be correct");
        let prize_line = lines.next().expect("aoc data should be correct");

        let button_a_captures = BUTTON_REGEX
            .captures(button_a_line)
            .expect("aoc data should be correct");
        let button_b_captures = BUTTON_REGEX
            .captures(button_b_line)
            .expect("aoc data should be correct");
        let prize_captures = PRIZE_REGEX
            .captures(prize_line)
            .expect("aoc data should be correct");

        let button_a = Button::new(
            button_a_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            button_a_captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        );
        let button_b = Button::new(
            button_b_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            button_b_captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        );
        let prize = Position::new(
            prize_captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            prize_captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        );

        machines.push(Machine::new(button_a, button_b, prize));
    }

    machines
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Position,
}

impl Machine {
    fn new(button_a: Button, button_b: Button, prize: Position) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

#[derive(Clone, Debug)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn calculate_minimum_tokens_to_win_all_prizes_works() {
        assert_eq!(
            calculate_minimum_tokens_to_win_all_prizes(EXAMPLE_INPUT),
            480
        );
    }
}
