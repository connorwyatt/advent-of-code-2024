const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", stone_count_after_blinks(INPUT, 25));
}

fn stone_count_after_blinks(input: &str, blinks: usize) -> usize {
    let mut stones = parse_input(input);

    for _ in 0..blinks {
        stones = apply_rules(&stones);
    }

    stones.len()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|stone_string| stone_string.parse::<usize>().unwrap())
        .collect()
}

fn apply_rules(stones: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();

    for stone in stones {
        if *stone == 0 {
            result.push(1);
            continue;
        }

        if let Some((part_1, part_2)) = split_even_digit_count_number(stone) {
            result.push(part_1);
            result.push(part_2);
            continue;
        }

        result.push(stone * 2024);
    }

    result
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
        assert_eq!(
            apply_rules(&[0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );

        assert_eq!(apply_rules(&[125, 17]), vec![253000, 1, 7]);

        assert_eq!(apply_rules(&[253000, 1, 7]), vec![253, 0, 2024, 14168]);

        assert_eq!(
            apply_rules(&[253, 0, 2024, 14168]),
            vec![512072, 1, 20, 24, 28676032]
        );

        assert_eq!(
            apply_rules(&[512072, 1, 20, 24, 28676032]),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );

        assert_eq!(
            apply_rules(&[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );

        assert_eq!(
            apply_rules(&[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn split_even_digit_count_number_works() {
        assert_eq!(split_even_digit_count_number(&253000), Some((253, 0)));
    }
}
