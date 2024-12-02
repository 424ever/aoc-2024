use aoc_2024::read_input;
use itertools::Itertools;

struct Report {
    levels: Vec<i32>,
}

fn main() {
    let input = read_input(2);
    let reports: Vec<_> = input.lines().map(Report::from_input).collect();

    println!("Part 1: {}", reports.iter().filter(|r| r.safe()).count());
    println!(
        "Part 1: {}",
        reports.iter().filter(|r| r.actually_safe()).count()
    );
}

impl Report {
    pub fn from_input(input: &str) -> Self {
        let levels = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { levels }
    }

    pub fn safe(&self) -> bool {
        Self::safe_diffs(&Self::to_diffs(&self.levels))
    }

    pub fn actually_safe(&self) -> bool {
        (0..self.levels.len())
            .filter(|&i| {
                let mut c = self.levels.clone();
                c.remove(i);
                Self::safe_diffs(&Self::to_diffs(&c))
            })
            .count()
            > 0
    }

    fn to_diffs(levels: &Vec<i32>) -> Vec<i32> {
        levels
            .into_iter()
            .tuple_windows()
            .map(|w: (&i32, &i32)| w.0 - w.1)
            .collect()
    }

    fn safe_diffs(diffs: &Vec<i32>) -> bool {
        (diffs.iter().all(|&n| n > 0) || diffs.iter().all(|&n| n < 0))
            && diffs.iter().all(|n| n.abs() >= 1 && n.abs() <= 3)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = concat!(
        "7 6 4 2 1\n",
        "1 2 7 8 9\n",
        "9 7 6 2 1\n",
        "1 3 2 4 5\n",
        "8 6 4 4 1\n",
        "1 3 6 7 9\n",
    );

    #[test]
    fn test_part_1() {
        assert_eq!(
            2,
            INPUT
                .lines()
                .map(Report::from_input)
                .filter(|r| r.safe())
                .count()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            4,
            INPUT
                .lines()
                .map(Report::from_input)
                .filter(|r| r.actually_safe())
                .count()
        );
    }
}
