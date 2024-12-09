use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_2024::{iters::IteratorExtensions, read_input};
use nom::{
    character::{complete, streaming},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Update = Vec<u32>;

struct PageOrderingRules {
    rules: HashMap<u32, HashSet<u32>>,
}

fn main() {
    let input = read_input("day5");
    let (rules, updates) = parse_input(&input).unwrap().1;
    println!("Part 1: {}", add_up_correctly_odered(&updates, &rules));
    println!("Part 2: {}", add_up_corrected(&updates, &rules));
}

fn add_up_correctly_odered(updates: &[Update], rules: &PageOrderingRules) -> u32 {
    updates
        .iter()
        .filter(|u| ordered_by_rules(u, rules))
        .map(|u| u.iter().middle_element().unwrap())
        .sum()
}

fn add_up_corrected(updates: &[Update], rules: &PageOrderingRules) -> u32 {
    updates
        .iter()
        .filter(|u| !ordered_by_rules(u, rules))
        .map(|u| correct_with_rules(u, rules))
        .map(|u| *u.iter().middle_element().unwrap())
        .sum()
}

fn ordered_by_rules(update: &Update, rules: &PageOrderingRules) -> bool {
    update
        .iter()
        .enumerate()
        .is_sorted_by(|a, b| compare_with_rules(a, b, rules) == Ordering::Less)
}

fn correct_with_rules(update: &Update, rules: &PageOrderingRules) -> Update {
    let mut update = update.iter().enumerate().collect::<Vec<_>>();

    update.sort_by(|a, b| compare_with_rules(a, b, rules));

    update.iter().map(|t| *t.1).collect()
}

fn compare_with_rules(a: &(usize, &u32), b: &(usize, &u32), rules: &PageOrderingRules) -> Ordering {
    let (a_idx, a) = *a;
    let (b_idx, b) = *b;

    if let Some(after_a) = rules.rules.get(a) {
        if after_a.contains(b) {
            return Ordering::Less;
        }
    }

    if let Some(after_b) = rules.rules.get(b) {
        if after_b.contains(a) {
            return Ordering::Greater;
        }
    }

    a_idx.cmp(&b_idx)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    fold_many1(
        terminated(
            separated_list1(streaming::char(','), complete::u32),
            complete::newline,
        ),
        Vec::new,
        |mut acc, pages| {
            acc.push(pages);
            acc
        },
    )(input)
}

fn parse_page_ordering_rules(input: &str) -> IResult<&str, PageOrderingRules> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, streaming::char('|'), complete::u32),
            complete::newline,
        ),
        PageOrderingRules::new,
        |mut acc, (first, second)| {
            acc.add_rule(first, second);
            acc
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (PageOrderingRules, Vec<Update>)> {
    separated_pair(parse_page_ordering_rules, complete::newline, parse_updates)(input)
}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, first: u32, second: u32) {
        self.rules
            .entry(first)
            .and_modify(|m| {
                m.insert(second);
            })
            .or_insert(HashSet::from_iter([second]));
    }
}

#[cfg(test)]
mod tests {
    use crate::{add_up_corrected, add_up_correctly_odered, correct_with_rules, parse_input};

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
            assert_eq!(correct_with_rules(&t.0, &rules), t.1);
        }
    }
}
