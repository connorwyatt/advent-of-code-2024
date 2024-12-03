use std::fs;

fn main() {
    let input = fs::read_to_string("./day-2/src/input.txt").expect("input.txt could not be found");

    println!("Result: {:?}", count_safe_reports_with_tolerance(input));
}

#[allow(dead_code)]
fn count_safe_reports(input: String) -> usize {
    input
        .lines()
        .map(|line| parse_line(line))
        .filter(|report| is_valid_report(report.as_slice()))
        .count()
}

fn count_safe_reports_with_tolerance(input: String) -> usize {
    input
        .lines()
        .map(|line| parse_line(line))
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

            return false;
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

    #[test]
    fn count_safe_reports_with_tolerance_works() {
        assert_eq!(count_safe_reports_with_tolerance(INPUT.to_string()), 4)
    }
}
