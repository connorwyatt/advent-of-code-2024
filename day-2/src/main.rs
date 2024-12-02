use std::fs;

fn main() {
    let input = fs::read_to_string("./day-2/src/input.txt").expect("input.txt could not be found");

    println!("Result: {:?}", count_safe_reports(input));
}

fn count_safe_reports(input: String) -> usize {
    input
        .lines()
        .map(|line| parse_line(line))
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

                if is_increasing && second <= first {
                    return false;
                } else if !is_increasing && second >= first {
                    return false;
                }

                match first.abs_diff(*second) {
                    1..=3 => {}
                    _ => return false,
                }
            }

            return true;
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

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn count_safe_reports_works() {
        assert_eq!(count_safe_reports(INPUT.to_string()), 2)
    }
}
