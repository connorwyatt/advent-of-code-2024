use std::iter::zip;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", total_distance_between_lists(INPUT));
}

fn total_distance_between_lists(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.lines().map(parse_input_line).unzip();

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(first, second)| first.abs_diff(second))
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
        assert_eq!(total_distance_between_lists(INPUT), 11)
    }
}
