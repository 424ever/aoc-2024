use std::{env, fs::read_to_string};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = read_to_string(filename).unwrap();

    let list1: Vec<_> = input
        .lines()
        .map(|l| l.split_once("   ").unwrap().0.parse().unwrap())
        .collect();
    let list2: Vec<_> = input
        .lines()
        .map(|l| l.split_once("   ").unwrap().1.parse().unwrap())
        .collect();
    println!(
        "Part 1: {}",
        total_distance(list1.clone(), list2.clone()).unwrap()
    );
    println!("Part 2: {}", similarity_score(list1, list2));
}

fn total_distance(mut list1: Vec<u32>, mut list2: Vec<u32>) -> Option<u32> {
    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
        return None;
    }

    Some(
        list1
            .iter()
            .zip(list2.iter())
            .map(|t| t.0.abs_diff(*t.1))
            .sum(),
    )
}

fn similarity_score(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    list1
        .iter()
        .map(|e| e * list2.iter().filter(|e2| *e2 == e).count() as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            total_distance(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            Some(11)
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            similarity_score(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            31
        );
    }
}
