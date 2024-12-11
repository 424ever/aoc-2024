use aoc_2024::{
    read_input,
    util::{count_digits, split_digits},
};
use nom::{character::complete, multi::separated_list1, IResult};

fn main() {
    let input = read_input("day11");
    let init = parse_input(&input).unwrap().1;

    println!("Part 1: {}", count_after_blinks(&init, 25));
}

fn count_after_blinks(initial_arrangement: &[u64], count: usize) -> usize {
    let mut newstones = Vec::from(initial_arrangement);

    for _ in 0..count {
        let old = newstones.clone();
        newstones.clear();

        for e in old {
            if e == 0 {
                newstones.push(1);
                continue;
            }

            let num_digits = count_digits::<10>(e);
            if num_digits % 2 == 0 {
                let (n1, n2) = split_digits::<10>(e, num_digits / 2);
                newstones.push(n1);
                newstones.push(n2);
                continue;
            }

            newstones.push(e * 2024);
        }
    }

    newstones.len()
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
