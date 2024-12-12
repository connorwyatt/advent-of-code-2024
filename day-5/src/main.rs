use std::{fs, str::FromStr};

const INPUT:&str = include_str!("aoc-input/input.txt");

fn main() {
    println!("Result: {:?}", sum_of_middle_page_numbers_from_correctly_ordered_updates(INPUT));
}

type PageOrderingRules = Vec<PageOrderingRule>;

#[derive(Debug, PartialEq)]
struct PageOrderingRule(usize, usize);

impl FromStr for PageOrderingRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('|').unwrap();
        Ok(Self(a.parse().unwrap(), b.parse().unwrap()))
    }
}

#[derive(Debug, PartialEq)]
struct UpdatePagesToProduce(Vec<usize>);

impl FromStr for UpdatePagesToProduce {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UpdatePagesToProduce(
            s.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        ))
    }
}

type PagesToProduce = Vec<UpdatePagesToProduce>;

fn sum_of_middle_page_numbers_from_correctly_ordered_updates(input: &str) -> usize {
    let (_, _) = parse_input(input);

    0
}

fn parse_input(input: &str) -> (PageOrderingRules, PagesToProduce) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut result = lines.splitn(2, |l| l.is_empty());
    let a = result.next().unwrap();
    let b = result.next().unwrap();

    let page_ordering_rules = a
        .iter()
        .map(|l| PageOrderingRule::from_str(l).unwrap())
        .collect::<Vec<_>>();
    let pages_to_produce = b
        .iter()
        .map(|l| UpdatePagesToProduce::from_str(l).unwrap())
        .collect::<Vec<_>>();

    (page_ordering_rules, pages_to_produce)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn parse_input_works() {
        let (page_ordering_rules, pages_to_produce) = parse_input(EXAMPLE_INPUT);

        assert_eq!(
            page_ordering_rules,
            vec![
                PageOrderingRule(47, 53),
                PageOrderingRule(97, 13),
                PageOrderingRule(97, 61),
                PageOrderingRule(97, 47),
                PageOrderingRule(75, 29),
                PageOrderingRule(61, 13),
                PageOrderingRule(75, 53),
                PageOrderingRule(29, 13),
                PageOrderingRule(97, 29),
                PageOrderingRule(53, 29),
                PageOrderingRule(61, 53),
                PageOrderingRule(97, 53),
                PageOrderingRule(61, 29),
                PageOrderingRule(47, 13),
                PageOrderingRule(75, 47),
                PageOrderingRule(97, 75),
                PageOrderingRule(47, 61),
                PageOrderingRule(75, 61),
                PageOrderingRule(47, 29),
                PageOrderingRule(75, 13),
                PageOrderingRule(53, 13),
            ]
        );

        assert_eq!(
            pages_to_produce,
            vec![
                UpdatePagesToProduce(vec![75, 47, 61, 53, 29]),
                UpdatePagesToProduce(vec![97, 61, 53, 29, 13]),
                UpdatePagesToProduce(vec![75, 29, 13]),
                UpdatePagesToProduce(vec![75, 97, 47, 61, 53]),
                UpdatePagesToProduce(vec![61, 13, 29]),
                UpdatePagesToProduce(vec![97, 13, 75, 29, 47]),
            ]
        );
    }
}