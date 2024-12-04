use std::{fs, str::FromStr};

use lazy_static::lazy_static;
use regex::{Match, Regex};

const MULTIPLY_REGEX_PATTERN: &str = r"mul\(\d{1,3},\d{1,3}\)";

const DO_DONT_REGEX_PATTERN: &str = r"do\(\)|don't\(\)";

lazy_static! {
    static ref MULTIPLY_REGEX: Regex =
        Regex::new(MULTIPLY_REGEX_PATTERN).expect("regex should compile");
    static ref DO_DONT_REGEX: Regex =
        Regex::new(DO_DONT_REGEX_PATTERN).expect("regex should compile");
}

fn main() {
    let input = fs::read_to_string("./day-3/src/input.txt").expect("input.txt could not be found");

    println!("Result: {:?}", sum_of_enabled_multiplication_results(input));
}

struct Multiplication(usize, usize);

impl Multiplication {
    fn result(&self) -> usize {
        self.0 * self.1
    }
}

impl FromStr for Multiplication {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_string, second_string) = s[4..(s.len() - 1)].split_once(",").unwrap();

        Ok(Self(
            first_string.parse().unwrap(),
            second_string.parse().unwrap(),
        ))
    }
}

#[derive(Clone)]
struct Instruction {
    pub do_dont: DoDont,
    pub start: usize,
}

impl Instruction {
    fn new(do_dont: DoDont, start: usize) -> Self {
        Self { do_dont, start }
    }

    fn from_match(m: Match) -> Self {
        Self {
            do_dont: DoDont::from_str(m.as_str()).unwrap(),
            start: m.start(),
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            do_dont: DoDont::Do,
            start: 0,
        }
    }
}

#[derive(Clone, PartialEq)]
enum DoDont {
    Do,
    Dont,
}

impl FromStr for DoDont {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "do()" => Ok(Self::Do),
            "don't()" => Ok(Self::Dont),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
fn sum_of_multiplication_results(input: String) -> usize {
    MULTIPLY_REGEX
        .find_iter(&input)
        .map(|m| Multiplication::from_str(m.as_str()).unwrap().result())
        .sum()
}

fn sum_of_enabled_multiplication_results(input: String) -> usize {
    let mut do_dont_iterator = DO_DONT_REGEX.find_iter(&input);

    let mut current_instruction = Instruction::default();
    let mut next_instruction = Instruction::from_match(do_dont_iterator.next().unwrap());

    MULTIPLY_REGEX
        .find_iter(&input)
        .filter_map(|multiplication_match| {
            loop {
                if multiplication_match.start() > current_instruction.start
                    && multiplication_match.start() < next_instruction.start
                {
                    break;
                }

                current_instruction = next_instruction.clone();
                next_instruction = match do_dont_iterator.next() {
                    Some(m) => Instruction::from_match(m),
                    None => Instruction::new(DoDont::Do, usize::MAX)
                };
            }

            if current_instruction.do_dont == DoDont::Dont {
                return None;
            }

            let multiplication = Multiplication::from_str(multiplication_match.as_str()).unwrap();
            Some(multiplication.result())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SUM_OF_MULTIPLICATION_RESULTS_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn sum_of_multiplication_results_works() {
        assert_eq!(
            sum_of_multiplication_results(SUM_OF_MULTIPLICATION_RESULTS_INPUT.to_string()),
            161
        );
    }

    const SUM_OF_ENABLED_MULTIPLCATION_RESULTS_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sum_of_enabled_multiplication_results_works() {
        assert_eq!(
            sum_of_enabled_multiplication_results(
                SUM_OF_ENABLED_MULTIPLCATION_RESULTS_INPUT.to_string()
            ),
            48
        );
    }
}
