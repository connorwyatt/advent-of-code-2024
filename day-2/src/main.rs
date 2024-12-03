const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", count_safe_reports_with_tolerance(INPUT));
}

#[allow(dead_code)]
fn count_safe_reports(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|report| is_valid_report(report.as_slice()))
        .count()
}

fn count_safe_reports_with_tolerance(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|report| {
            if is_valid_report(report.as_slice()) {
                return true;
            }

            for index in 0..report.len() {
                let modified_report = [&report[..index], &report[index + 1..]].concat();

                if is_valid_report(modified_report.as_slice()) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn is_valid_report(report: &[usize]) -> bool {
    let mut windowed_report = report.windows(2);

    let [first, second] = windowed_report.next().unwrap() else {
        panic!("windows should be 2 items long");
    };

    match first.abs_diff(*second) {
        1..=3 => {}
        _ => return false,
    }

    let is_increasing = second > first;

    while let Some([first, second]) = windowed_report.next() {
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
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn count_safe_reports_works() {
        assert_eq!(count_safe_reports(EXAMPLE_INPUT), 2)
    }

    #[test]
    fn count_safe_reports_with_tolerance_works() {
        assert_eq!(count_safe_reports_with_tolerance(EXAMPLE_INPUT), 4)
    }
}
