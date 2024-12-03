use aoc_2024::iters::Differences;
use aoc_2024::read_input;

trait SafeDiffs: Iterator {
    fn safe_diffs(self) -> bool;
}

#[derive(Debug)]
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
        self.levels.clone().into_iter().differences().safe_diffs()
    }

    pub fn actually_safe(&self) -> bool {
        (0..self.levels.len())
            .filter(|&i| {
                let mut c = self.levels.clone();
                c.remove(i);
                c.into_iter().differences().safe_diffs()
            })
            .count()
            > 0
    }
}

impl<T> SafeDiffs for T
where
    T: Iterator<Item = i32>,
{
    fn safe_diffs(self) -> bool {
        let t: Vec<_> = self.collect();
        (t.iter().all(|&n| n > 0) || t.iter().all(|&n| n < 0))
            && t.iter().all(|n| n.abs() >= 1 && n.abs() <= 3)
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
                .inspect(|r| println!("safe: {:?}", r))
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
