use std::collections::HashMap;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", stone_count_after_blinks(INPUT, 75));
}

fn stone_count_after_blinks(input: &str, blinks: usize) -> usize {
    let mut stones = parse_input(input);

    for _ in 0..blinks {
        for (&stone, &count) in stones.clone().iter().filter(|(_, &count)| count > 0) {
            *stones.get_mut(&stone).unwrap() -= count;

            let updated_stones = apply_rules(stone);

            for updated_stone in updated_stones {
                *stones.entry(updated_stone).or_default() += count;
            }
        }
    }

    stones.values().sum()
}

fn parse_input(input: &str) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let stones = input
        .split_whitespace()
        .map(|stone_string| stone_string.parse::<usize>().unwrap());
    for stone in stones {
        *result.entry(stone).or_default() += 1;
    }
    result
}

fn apply_rules(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }

    if let Some((part_1, part_2)) = split_even_digit_count_number(&stone) {
        return vec![part_1, part_2];
    }

    vec![stone * 2024]
}

fn split_even_digit_count_number(number: &usize) -> Option<(usize, usize)> {
    let number_of_digits = number.ilog10() + 1;
    if number_of_digits % 2 != 0 {
        None
    } else {
        let split_value = 10usize.pow(number_of_digits / 2);
        let part_1 = number / (split_value);
        let part_2 = number % (split_value);
        Some((part_1, part_2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = include_str!("aoc-input/example-input-1.txt");

    const EXAMPLE_INPUT_2: &str = include_str!("aoc-input/example-input-2.txt");

    #[test]
    fn stone_count_after_blinks_works() {
        assert_eq!(stone_count_after_blinks(EXAMPLE_INPUT_1, 1), 7);
        assert_eq!(stone_count_after_blinks(EXAMPLE_INPUT_2, 6), 22);
        assert_eq!(stone_count_after_blinks(EXAMPLE_INPUT_2, 25), 55312);
    }

    #[test]
    fn apply_rules_works() {
        assert_eq!(apply_rules(0), vec![1]);
        assert_eq!(apply_rules(253010), vec![253, 10]);
        assert_eq!(apply_rules(1036288), vec![2097446912]);
    }
}
