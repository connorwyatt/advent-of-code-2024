use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        calculate_minimum_tokens_to_win_all_prizes(INPUT)
    );
}

const BUTTON_A_COST: usize = 3;
const BUTTON_B_COST: usize = 1;

fn calculate_minimum_tokens_to_win_all_prizes(input: &str) -> usize {
    let machines = parse_input(input);

    machines
        .iter()
        .filter_map(|machine| {
            let button_a = &machine.button_a;
            let button_b = &machine.button_b;
            let prize = &machine.prize;

            let maximum_button_a_presses = (prize.x / button_a.x).min(prize.y / button_a.y);

            let mut minimum_tokens_required = None;

            for button_a_presses in 0..=maximum_button_a_presses {
                let position_after_button_a_presses = button_a.apply_n_times(button_a_presses);
                let distance_remaining_x = prize.x - position_after_button_a_presses.x;
                let distance_remaining_y = prize.y - position_after_button_a_presses.y;

                if distance_remaining_x % button_b.x != 0 || distance_remaining_y % button_b.y != 0
                {
                    continue;
                }

                let button_b_presses_x = distance_remaining_x / button_b.x;
                let button_b_presses_y = distance_remaining_y / button_b.y;

                if button_b_presses_x != button_b_presses_y {
                    continue;
                }

                let button_b_presses = button_b_presses_x;

                let tokens_required =
                    button_a_presses * BUTTON_A_COST + button_b_presses * BUTTON_B_COST;

                match minimum_tokens_required {
                    None => minimum_tokens_required = Some(tokens_required),
                    Some(mtr) if mtr > tokens_required => {
                        minimum_tokens_required = Some(tokens_required)
                    }
                    _ => {}
                };
            }

            minimum_tokens_required
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

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

impl Button {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn apply_n_times(&self, presses: usize) -> Position {
        Position::new(self.x * presses, self.y * presses)
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
