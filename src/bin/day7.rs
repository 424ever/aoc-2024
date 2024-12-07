use std::{collections::HashSet, iter::repeat_n};

use aoc_2024::read_input;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{self, complete},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug)]
struct Calibration {
    target: u128,
    values: Vec<u128>,
}

fn main() {
    let input = read_input("day7");
    let cals = parse_input(&input).unwrap().1;
    println!("Part 1: {}", total_calibration_result(&cals, false));
    println!("Part 2: {}", total_calibration_result(&cals, true));
}

fn total_calibration_result(cals: &Vec<Calibration>, concat: bool) -> u128 {
    let mut available_operators = HashSet::new();
    available_operators.insert(Operator::Add);
    available_operators.insert(Operator::Multiply);
    if concat {
        available_operators.insert(Operator::Concat);
    }

    cals.iter()
        .filter(|c| c.can_be_made_true(&available_operators))
        .map(|c| c.target)
        .sum()
}

fn parse_calibration(input: &str) -> IResult<&str, Calibration> {
    separated_pair(
        character::complete::u128,
        tag(": "),
        separated_list1(complete::space1, complete::u128),
    )(input)
    .map(|(s, (target, values))| (s, Calibration { target, values }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Calibration>> {
    many1(terminated(parse_calibration, complete::newline))(input)
}

impl Operator {
    fn apply(&self, a: u128, b: u128) -> u128 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

impl Calibration {
    fn can_be_made_true(&self, available_operators: &HashSet<Operator>) -> bool {
        self.possible_operators(available_operators)
            .any(|o| self.is_true_with(&o))
    }

    fn is_true_with(&self, ops: &Vec<&Operator>) -> bool {
        assert!(ops.len() == self.values.len() - 1);

        let mut val = self.values[0];

        for (idx, op) in ops.iter().enumerate() {
            val = op.apply(val, self.values[idx + 1]);

            if val > self.target {
                return false;
            }
        }

        val == self.target
    }

    fn possible_operators<'a>(
        &'a self,
        available_operators: &'a HashSet<Operator>,
    ) -> impl Iterator<Item = Vec<&'a Operator>> {
        repeat_n(available_operators, self.values.len() - 1).multi_cartesian_product()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse_input, total_calibration_result, Calibration, Operator};

    const INPUT: &str = concat!(
        "190: 10 19\n",
        "3267: 81 40 27\n",
        "83: 17 5\n",
        "156: 15 6\n",
        "7290: 6 8 6 15\n",
        "161011: 16 10 13\n",
        "192: 17 8 14\n",
        "21037: 9 7 18 13\n",
        "292: 11 6 16 20\n",
    );

    #[test]
    fn test_part_1() {
        let cals = parse_input(INPUT).unwrap().1;
        assert_eq!(total_calibration_result(&cals, false), 3749);
    }

    #[test]
    fn test_part_2() {
        let cals = parse_input(INPUT).unwrap().1;
        assert_eq!(total_calibration_result(&cals, true), 11387);
    }

    #[test]
    fn test_possible_operators() {
        let cal = Calibration {
            target: 3267,
            values: vec![81, 40, 27],
        };
        use Operator::*;
        assert_eq!(
            cal.possible_operators(&HashSet::from([Add, Multiply]))
                .collect::<HashSet<_>>(),
            HashSet::from([
                vec![&Add, &Add],
                vec![&Add, &Multiply],
                vec![&Multiply, &Add],
                vec![&Multiply, &Multiply]
            ])
        );
    }

    #[test]
    fn test_is_true_with() {
        let cal = Calibration {
            target: 190,
            values: vec![10, 19],
        };
        assert_eq!(cal.is_true_with(&vec![&Operator::Add]), false);
        assert_eq!(cal.is_true_with(&vec![&Operator::Multiply]), true);
    }
}
