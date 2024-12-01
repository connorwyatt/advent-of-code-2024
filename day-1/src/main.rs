use std::{fs, iter::zip};

fn main() {
    let input = fs::read_to_string("./day-1/src/input.txt").expect("input.txt could not be found");

    println!("Result: {:?}", similarity_score(input));
}

#[allow(dead_code)]
fn total_distance_between_lists(input: String) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        input.lines().map(|line| parse_input_line(line)).unzip();

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

fn similarity_score(input: String) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = input.lines().map(|line| parse_input_line(line)).unzip();

    left.iter()
        .map(|left_number| {
            right
                .iter()
                .filter(|&right_number| right_number == left_number)
                .count()
                * left_number
        })
        .sum()
}

fn parse_input_line(input_line: &str) -> (usize, usize) {
    let (first, second) = input_line.split_once("   ").unwrap();

    (
        first.parse::<usize>().unwrap(),
        second.parse::<usize>().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn total_distance_between_lists_works() {
        assert_eq!(total_distance_between_lists(INPUT.to_string()), 11)
    }

    #[test]
    fn similarity_score_works() {
        assert_eq!(similarity_score(INPUT.to_string()), 31)
    }
}
