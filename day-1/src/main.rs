use std::iter::zip;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", similarity_score(INPUT));
}

#[allow(dead_code)]
fn total_distance_between_lists(input: &str) -> usize {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.lines().map(parse_input_line).unzip();

    left.sort();
    right.sort();

    zip(left, right)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

fn similarity_score(input: &str) -> usize {
    let (left, right): (Vec<_>, Vec<_>) = input.lines().map(parse_input_line).unzip();

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

    const INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn total_distance_between_lists_works() {
        assert_eq!(total_distance_between_lists(INPUT), 11)
    }

    #[test]
    fn similarity_score_works() {
        assert_eq!(similarity_score(INPUT), 31)
    }
}
