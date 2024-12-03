use aoc_2024::read_input;
use regex::Regex;

fn main() {
    let input = read_input("day3");
    println!("Part 1: {}", execute_without_conditions(&input));
    println!("Part 2: {}", execute_with_conditions(&input));
}

fn execute_without_conditions<T: AsRef<str>>(input: T) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input.as_ref())
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap())
        .sum()
}

fn execute_with_conditions<T: AsRef<str>>(input: T) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut active = true;
    let mut result = 0;
    for c in re.captures_iter(input.as_ref()) {
        let total_match = c.get(0).unwrap();
        match total_match.as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                let n1 = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let n2 = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                if active {
                    result += n1 * n2;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        assert_eq!(161, execute_without_conditions("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))mul(,4)mul(a,b)"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            48,
            execute_with_conditions(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )
        );
    }
}
