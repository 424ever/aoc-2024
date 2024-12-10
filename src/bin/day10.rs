use std::collections::HashSet;

use aoc_2024::{
    containers::{maybe_remove_first, Vec2D, Vec2DIndex},
    read_input,
};

type Height = u8;

fn main() {
    let input = read_input("day10");
    let map = parse_input(&input);
    println!("Part 1: {}", sum_up_trailheads(&map));
}

fn parse_input(input: &str) -> Vec2D<Height> {
    Vec2D::from_lines(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_string().parse::<Height>().unwrap())),
    )
}

fn sum_up_trailheads(map: &Vec2D<Height>) -> u64 {
    let mut sum = 0;

    for (pos, &height) in map.enumerated_iter() {
        if height == 0 {
            sum += trailhead_score(map, (pos, height));
        }
    }

    sum
}

fn trailhead_score(map: &Vec2D<Height>, init: (Vec2DIndex, Height)) -> u64 {
    let mut worklist = vec![init];
    let mut ends = HashSet::new();

    while let Some((pos, cur)) = maybe_remove_first(&mut worklist) {
        if cur == 9 {
            ends.insert(pos);
            continue;
        }

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(l, c)| pos.checked_add_signed(*l, *c))
            .filter_map(|i| Some((i, map.get_index(&i)?)))
            .filter(|(_, new)| **new == cur + 1)
            .for_each(|(i, new)| {
                let newtup = (i, *new);
                worklist.push(newtup);
            });
    }

    ends.len() as u64
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, sum_up_trailheads};

    #[test]
    fn test_part_1_1() {
        let input = concat!("0123\n", "1234\n", "8765\n", "9876\n",);
        assert_eq!(sum_up_trailheads(&parse_input(input)), 1);
    }

    #[test]
    fn test_part_1_2() {
        let input = concat!(
            "89010123\n",
            "78121874\n",
            "87430965\n",
            "96549874\n",
            "45678903\n",
            "32019012\n",
            "01329801\n",
            "10456732\n",
        );
        assert_eq!(sum_up_trailheads(&parse_input(input)), 36);
    }
}
