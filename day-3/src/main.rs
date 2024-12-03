use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MULTIPLY_REGEX: Regex =
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regex should compile");
}

fn main() {
    let input = fs::read_to_string("./day-3/src/input.txt").expect("input.txt could not be found");

    println!("Result: {:?}", sum_of_multiplication_results(input));
}

fn sum_of_multiplication_results(input: String) -> usize {
    MULTIPLY_REGEX
        .captures_iter(&input)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<usize>().unwrap()
                * m.get(2).unwrap().as_str().parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn sum_of_multiplication_results_works() {
        assert_eq!(sum_of_multiplication_results(INPUT.to_string()), 161);
    }
}
