use std::{cmp::Ordering, str::FromStr};

const INPUT:&str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        sum_of_middle_page_numbers_from_incorrectly_ordered_updates(INPUT)
    );
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

#[allow(dead_code)]
fn sum_of_middle_page_numbers_from_correctly_ordered_updates(input: &str) -> usize {
    let (page_ordering_rules, pages_to_produce) = parse_input(input);

    pages_to_produce
        .iter()
        .filter(|update_pages_to_produce| {
            is_update_pages_to_produce_valid(update_pages_to_produce, &page_ordering_rules)
        })
        .map(|update_pages_to_produce| {
            let pages = &update_pages_to_produce.0;
            pages[pages.len() / 2]
        })
        .sum()
}

fn sum_of_middle_page_numbers_from_incorrectly_ordered_updates(input: &str) -> usize {
    let (page_ordering_rules, pages_to_produce) = parse_input(input);

    pages_to_produce
        .iter()
        .filter(|update_pages_to_produce| {
            !is_update_pages_to_produce_valid(update_pages_to_produce, &page_ordering_rules)
        })
        .map(|update_pages_to_produce| {
            reorder_update_pages_to_produce(update_pages_to_produce, &page_ordering_rules)
        })
        .map(|update_pages_to_produce| {
            let pages = &update_pages_to_produce.0;
            pages[pages.len() / 2]
        })
        .sum()
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

fn is_update_pages_to_produce_valid(
    update_pages_to_produce: &UpdatePagesToProduce,
    page_ordering_rules: &PageOrderingRules,
) -> bool {
    let pages = update_pages_to_produce.0.clone();
    for page in pages.as_slice() {
        for later_page in pages.iter().skip_while(|&x| x != page) {
            for rule in page_ordering_rules {
                if &rule.0 == page && &rule.1 == later_page {
                    break;
                } else if &rule.0 == later_page && &rule.1 == page {
                    return false;
                }
            }
        }
    }
    true
}

fn reorder_update_pages_to_produce(
    update_pages_to_produce: &UpdatePagesToProduce,
    page_ordering_rules: &PageOrderingRules,
) -> UpdatePagesToProduce {
    let rules = page_ordering_rules.as_slice();

    let mut result = update_pages_to_produce.0.clone().clone();

    result.sort_unstable_by(|a, b| {
        match rules.iter().find(|rule| (&rule.0 == a && &rule.1 == b) || (&rule.0 == b && &rule.1 == a)) {
            Some(rule) => {
                if &rule.0 == a {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal,
        }
    });
    UpdatePagesToProduce(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn sum_of_middle_page_numbers_from_correctly_ordered_updates_works() {
        assert_eq!(
            sum_of_middle_page_numbers_from_correctly_ordered_updates(EXAMPLE_INPUT),
            143
        );
    }

    #[test]
    fn sum_of_middle_page_numbers_from_incorrectly_ordered_updates_works() {
        assert_eq!(
            sum_of_middle_page_numbers_from_incorrectly_ordered_updates(EXAMPLE_INPUT),
            123
        );
    }

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

    #[test]
    fn reorder_update_pages_to_produce_works() {
        let (page_ordering_rules, _) = parse_input(EXAMPLE_INPUT);

        assert_eq!(
            reorder_update_pages_to_produce(
                &UpdatePagesToProduce(vec![75, 97, 47, 61, 53]),
                &page_ordering_rules
            ),
            UpdatePagesToProduce(vec![97, 75, 47, 61, 53])
        );

        assert_eq!(
            reorder_update_pages_to_produce(
                &UpdatePagesToProduce(vec![61, 13, 29]),
                &page_ordering_rules
            ),
            UpdatePagesToProduce(vec![61, 29, 13])
        );

        assert_eq!(
            reorder_update_pages_to_produce(
                &UpdatePagesToProduce(vec![97, 13, 75, 29, 47]),
                &page_ordering_rules
            ),
            UpdatePagesToProduce(vec![97, 75, 47, 29, 13])
        );
    }
}
