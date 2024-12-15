use std::collections::HashMap;

use input::parse_input;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        possibly_true_calibration_results_with_concatenation_total(INPUT)
    );
}

#[allow(dead_code)]
fn possibly_true_calibration_results_total(input: &str) -> usize {
    let operators = vec![Operator::Addition, Operator::Multiplication];

    let mut operator_permutations_lookup: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();

    parse_input(input)
        .iter()
        .filter_map(|equation| {
            let num_operators_required = equation.parts().len() - 1;

            let operator_permutations = operator_permutations_lookup
                .entry(num_operators_required)
                .or_insert(generate_operator_permutations(
                    num_operators_required,
                    &operators,
                ));

            for permutation in operator_permutations {
                let mut permutation_iterator = permutation.iter();

                let result = equation
                    .parts()
                    .iter()
                    .cloned()
                    .reduce(|acc, p| match permutation_iterator.next().unwrap() {
                        Operator::Addition => acc + p,
                        Operator::Multiplication => acc * p,
                        _ => panic!("Concatenation is not included in these permutations"),
                    })
                    .unwrap();

                if &result == equation.expected_result() {
                    return Some(result);
                }
            }

            None
        })
        .sum()
}

fn possibly_true_calibration_results_with_concatenation_total(input: &str) -> usize {
    let operators = vec![
        Operator::Addition,
        Operator::Multiplication,
        Operator::Concatenation,
    ];

    let mut operator_permutations_lookup: HashMap<usize, Vec<Vec<Operator>>> = HashMap::new();

    parse_input(input)
        .iter()
        .filter_map(|equation| {
            let num_operators_required = equation.parts().len() - 1;

            let operator_permutations = operator_permutations_lookup
                .entry(num_operators_required)
                .or_insert(generate_operator_permutations(
                    num_operators_required,
                    &operators,
                ));

            for permutation in operator_permutations {
                let mut permutation_iterator = permutation.iter();

                let result = equation
                    .parts()
                    .iter()
                    .cloned()
                    .reduce(|acc, p| match permutation_iterator.next().unwrap() {
                        Operator::Addition => acc + p,
                        Operator::Multiplication => acc * p,
                        Operator::Concatenation => {
                            let p_log_10 = p.ilog10();
                            let acc_multiplier: usize = 10u32.pow(p_log_10 + 1).try_into().unwrap();
                            acc * acc_multiplier + p
                        }
                    })
                    .unwrap();

                if &result == equation.expected_result() {
                    return Some(result);
                }
            }

            None
        })
        .sum()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

fn generate_operator_permutations(
    num_operators_required: usize,
    operators: &[Operator],
) -> Vec<Vec<Operator>> {
    let num_operator_permutations = operators
        .len()
        .checked_pow(num_operators_required.try_into().unwrap())
        .unwrap();

    (0..num_operator_permutations)
        .map(|i| {
            (0..num_operators_required)
                .rev()
                .map(|j| {
                    operators[(i / (operators.len().pow(j.try_into().unwrap()))) % operators.len()]
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

mod input {
    pub(crate) struct Equation {
        expected_result: usize,
        parts: Vec<usize>,
    }

    impl Equation {
        pub(crate) fn expected_result(&self) -> &usize {
            &self.expected_result
        }

        pub(crate) fn parts(&self) -> &Vec<usize> {
            &self.parts
        }
    }

    pub(crate) fn parse_input(input: &str) -> Vec<Equation> {
        input.lines().map(parse_line).collect::<Vec<_>>()
    }

    fn parse_line(line: &str) -> Equation {
        let (result_str, parts_str) = line.split_once(":").unwrap();

        let expected_result = result_str.parse::<usize>().unwrap();

        let parts = parts_str
            .split_whitespace()
            .map(|p| p.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Equation {
            expected_result,
            parts,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn possibly_true_calibration_results_total_works() {
        assert_eq!(possibly_true_calibration_results_total(EXAMPLE_INPUT), 3749);
    }

    #[test]
    fn possibly_true_calibration_results_with_concatenation_total_works() {
        assert_eq!(
            possibly_true_calibration_results_with_concatenation_total(EXAMPLE_INPUT),
            11387
        );
    }

    #[test]
    fn generate_operator_permutations_works() {
        assert_eq!(
            generate_operator_permutations(1, &[Operator::Addition, Operator::Multiplication]),
            vec![vec![Operator::Addition], vec![Operator::Multiplication]]
        );

        assert_eq!(
            generate_operator_permutations(2, &[Operator::Addition, Operator::Multiplication]),
            vec![
                vec![Operator::Addition, Operator::Addition],
                vec![Operator::Addition, Operator::Multiplication],
                vec![Operator::Multiplication, Operator::Addition],
                vec![Operator::Multiplication, Operator::Multiplication]
            ]
        );

        assert_eq!(
            generate_operator_permutations(
                2,
                &[
                    Operator::Addition,
                    Operator::Multiplication,
                    Operator::Concatenation
                ]
            ),
            vec![
                vec![Operator::Addition, Operator::Addition],
                vec![Operator::Addition, Operator::Multiplication],
                vec![Operator::Addition, Operator::Concatenation],
                vec![Operator::Multiplication, Operator::Addition],
                vec![Operator::Multiplication, Operator::Multiplication],
                vec![Operator::Multiplication, Operator::Concatenation],
                vec![Operator::Concatenation, Operator::Addition],
                vec![Operator::Concatenation, Operator::Multiplication],
                vec![Operator::Concatenation, Operator::Concatenation],
            ]
        );
    }
}
