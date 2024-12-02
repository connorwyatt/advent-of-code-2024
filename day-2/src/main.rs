const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_safe_reports(INPUT));
}

fn count_safe_reports(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|numbers| {
            let mut windowed_numbers = numbers.windows(2);

            let [first, second] = windowed_numbers.next().unwrap() else {
                panic!("windows should be 2 items long");
            };

            match first.abs_diff(*second) {
                1..=3 => {}
                _ => return false,
            }

            let is_increasing = second > first;

            while let Some([first, second]) = windowed_numbers.next() {
                if first == second {
                    return false;
                }

                if (is_increasing && second <= first) || (!is_increasing && second >= first) {
                    return false;
                }

                match first.abs_diff(*second) {
                    1..=3 => {}
                    _ => return false,
                }
            }

            true
        })
        .count()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn count_safe_reports_works() {
        assert_eq!(count_safe_reports(EXAMPLE_INPUT), 2)
    }
}
