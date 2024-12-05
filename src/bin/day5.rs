use std::borrow::Cow;

use aoc_2024::{iters::IteratorExtensions, read_input};
use nom::{
    character::{complete, streaming},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
struct Update {
    pages: Vec<u32>,
}

/// A rule specifying that [`Self::first`] must come before [`Self::second`]
/// in an [`Update`].
#[derive(Debug, Clone)]
struct PageOrderingRule {
    first: u32,
    second: u32,
}

fn main() {
    let input = read_input("day5");
    let (rules, updates) = parse_input(&input).unwrap().1;
    println!("Part 1: {}", add_up_correctly_odered(&updates, &rules));
    println!("Part 2: {}", add_up_corrected(&updates, &rules));
}

fn add_up_correctly_odered(updates: &Vec<Update>, rules: &Vec<PageOrderingRule>) -> u32 {
    updates
        .iter()
        .filter(|u| rules.iter().all(|r| rule_applies(&r, u)))
        .map(|u| u.pages.iter().middle_element().unwrap())
        .sum()
}

fn add_up_corrected(updates: &Vec<Update>, rules: &Vec<PageOrderingRule>) -> u32 {
    updates
        .iter()
        .filter_map(|u| correct_with_rules(&rules, u))
        .map(|u| {
            let x = u.pages.iter().middle_element().unwrap();
            x.clone()
        })
        .sum()
}

fn rule_applies(rule: &PageOrderingRule, update: &Update) -> bool {
    let first_idx = update.pages.iter().find_index(|p| **p == rule.first);
    let second_idx = update.pages.iter().find_index(|p| **p == rule.second);

    match (first_idx, second_idx) {
        (None, None) => true,
        (None, Some(_)) => true,
        (Some(_), None) => true,
        (Some(i1), Some(i2)) => i1 < i2,
    }
}

fn correct_with_rules<'a>(rules: &Vec<PageOrderingRule>, update: &Update) -> Option<Update> {
    let mut changed = Cow::Borrowed(update);
    loop {
        let mut any_applied = false;
        for rule in rules {
            if !rule_applies(rule, changed.as_ref()) {
                any_applied = true;
                let first_idx = changed.pages.iter().find_index(|p| **p == rule.first);
                let second_idx = changed.pages.iter().find_index(|p| **p == rule.second);

                changed
                    .to_mut()
                    .pages
                    .swap(first_idx.unwrap(), second_idx.unwrap());
            }
        }
        if !any_applied {
            break;
        }
    }

    match changed {
        Cow::Borrowed(_) => None,
        Cow::Owned(u) => Some(u),
    }
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    fold_many1(
        terminated(
            separated_list1(streaming::char(','), complete::u32),
            complete::newline,
        ),
        Vec::new,
        |mut acc, pages| {
            acc.push(Update { pages });
            acc
        },
    )(input)
}

fn parse_page_ordering_rules(input: &str) -> IResult<&str, Vec<PageOrderingRule>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, streaming::char('|'), complete::u32),
            complete::newline,
        ),
        Vec::new,
        |mut acc, (first, second)| {
            acc.push(PageOrderingRule::new(first, second));
            acc
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<PageOrderingRule>, Vec<Update>)> {
    separated_pair(parse_page_ordering_rules, complete::newline, parse_updates)(input)
}

impl PageOrderingRule {
    pub fn new(first: u32, second: u32) -> Self {
        if first == second {
            panic!("first and second cannot be the same");
        } else {
            Self { first, second }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        add_up_corrected, add_up_correctly_odered, correct_with_rules, parse_input, Update,
    };

    const INPUT: &str = concat!(
        "47|53\n",
        "97|13\n",
        "97|61\n",
        "97|47\n",
        "75|29\n",
        "61|13\n",
        "75|53\n",
        "29|13\n",
        "97|29\n",
        "53|29\n",
        "61|53\n",
        "97|53\n",
        "61|29\n",
        "47|13\n",
        "75|47\n",
        "97|75\n",
        "47|61\n",
        "75|61\n",
        "47|29\n",
        "75|13\n",
        "53|13\n",
        "\n",
        "75,47,61,53,29\n",
        "97,61,53,29,13\n",
        "75,29,13\n",
        "75,97,47,61,53\n",
        "61,13,29\n",
        "97,13,75,29,47\n",
    );

    #[test]
    fn test_part_1() {
        let (rules, updates) = parse_input(INPUT).unwrap().1;
        assert_eq!(add_up_correctly_odered(&updates, &rules), 143);
    }

    #[test]
    fn test_part_2() {
        let (rules, updates) = parse_input(INPUT).unwrap().1;
        assert_eq!(add_up_corrected(&updates, &rules), 123);
    }

    #[test]
    fn test_correct() {
        let (rules, _) = parse_input(INPUT).unwrap().1;
        let tests = vec![
            (vec![75, 97, 47, 61, 53], vec![97, 75, 47, 61, 53]),
            (vec![61, 13, 29], vec![61, 29, 13]),
            (vec![97, 13, 75, 29, 47], vec![97, 75, 47, 29, 13]),
        ];

        for t in tests {
            assert_eq!(
                correct_with_rules(&rules, &Update { pages: t.0 }),
                Some(Update { pages: t.1 })
            );
        }
    }
}
