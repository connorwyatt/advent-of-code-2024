use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MULTIPLY_REGEX: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex should compile");
}

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", sum_of_multiplication_results(INPUT));
}

fn sum_of_multiplication_results(input: &str) -> usize {
    MULTIPLY_REGEX
        .captures_iter(input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * m.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SUM_OF_MULTIPLICATION_RESULTS_INPUT: &str = include_str!("aoc-input/example-input-1.txt");

    #[test]
    fn sum_of_multiplication_results_works() {
        assert_eq!(
            sum_of_multiplication_results(SUM_OF_MULTIPLICATION_RESULTS_INPUT),
            161
        );
    }
}
