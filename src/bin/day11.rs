use std::collections::HashMap;

use aoc_2024::{
    read_input,
    util::{count_digits, split_digits},
};
use nom::{character::complete, multi::separated_list1, IResult};

fn main() {
    let input = read_input("day11");
    let init = parse_input(&input).unwrap().1;

    println!("Part 1: {}", count_after_blinks(&init, 25));
    println!("Part 1: {}", count_after_blinks(&init, 75));
}

fn count_after_blinks(initial_arrangement: &[u64], steps: usize) -> usize {
    let mut lookup = HashMap::<(u64, usize), usize>::new();
    initial_arrangement
        .iter()
        .map(|s| count_stone(*s, steps, &mut lookup))
        .sum()
}

fn count_stone(stone: u64, steps: usize, lookup: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(res) = lookup.get(&(stone, steps)) {
        *res
    } else {
        let res = count_stone_uncached(stone, steps, lookup);
        lookup.insert((stone, steps), res);
        res
    }
}

fn count_stone_uncached(
    stone: u64,
    steps: usize,
    lookup: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if steps == 0 {
        1
    } else if stone == 0 {
        count_stone(1, steps - 1, lookup)
    } else {
        let digits = count_digits::<10>(stone);
        match digits % 2 {
            0 => {
                let (n1, n2) = split_digits::<10>(stone, digits / 2);
                count_stone(n1, steps - 1, lookup) + count_stone(n2, steps - 1, lookup)
            }
            _ => count_stone(stone * 2024, steps - 1, lookup),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::space1, complete::u64)(input)
}

#[cfg(test)]
mod tests {
    use crate::{count_after_blinks, parse_input};

    #[test]
    fn test_part_1_1() {
        let input = "0 1 10 99 999";
        let init = parse_input(input).unwrap().1;
        assert_eq!(count_after_blinks(&init, 1), 7);
    }

    #[test]
    fn test_part_1_2() {
        let input = "125 17";
        let init = parse_input(input).unwrap().1;
        assert_eq!(count_after_blinks(&init, 6), 22);
    }

    #[test]
    fn test_part_1_3() {
        let input = "125 17";
        let init = parse_input(input).unwrap().1;
        assert_eq!(count_after_blinks(&init, 25), 55312);
    }
}
