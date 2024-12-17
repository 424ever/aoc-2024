use std::collections::{HashSet, VecDeque};

use aoc_2024::{
    containers::{Vec2D, Vec2DIndex},
    read_input,
};

type Height = u8;

fn main() {
    let input = read_input("day10");
    let map = parse_input(&input);
    println!("Part 1: {}", sum_trailhead_scores(&map));
    println!("Part 2: {}", sum_trailhead_ratings(&map));
}

fn parse_input(input: &str) -> Vec2D<Height> {
    Vec2D::from_lines(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_string().parse::<Height>().unwrap())),
    )
}

fn sum_trailhead_scores(map: &Vec2D<Height>) -> u64 {
    sum_trailhead_evals(
        map,
        HashSet::new,
        |mut s, (p, _)| {
            s.insert(p);
            s
        },
        |s| s.len() as u64,
    )
}

fn sum_trailhead_ratings(map: &Vec2D<Height>) -> u64 {
    sum_trailhead_evals(map, || 0, |r, _| r + 1, |r| r)
}

fn sum_trailhead_evals<I, F, R, G>(map: &Vec2D<Height>, init: I, fold: F, finalize: G) -> u64
where
    I: Fn() -> R,
    F: Fn(R, (Vec2DIndex, Height)) -> R,
    G: Fn(R) -> u64,
{
    map.enumerated_iter()
        .filter(|(_, &height)| height == 0)
        .map(|(p, h)| eval_trailhead(map, (p, *h), &init, &fold, &finalize))
        .sum()
}

fn eval_trailhead<I, F, G, R>(
    map: &Vec2D<Height>,
    trailhead: (Vec2DIndex, Height),
    init: I,
    fold: F,
    finalize: G,
) -> u64
where
    I: Fn() -> R,
    F: Fn(R, (Vec2DIndex, Height)) -> R,
    G: Fn(R) -> u64,
{
    let mut worklist = VecDeque::new();
    let mut acc = init();

    worklist.push_back(trailhead);

    while let Some((pos, cur)) = worklist.pop_front() {
        if cur == 9 {
            acc = fold(acc, (pos, cur));
            continue;
        }

        [
            (-1, 0), /* north */
            (1, 0),  /* south */
            (0, -1), /* west */
            (0, 1),  /* east */
        ]
        .iter()
        .filter_map(|(l, c)| pos.checked_add_signed(*l, *c))
        .filter_map(|i| Some((i, map.get_index(&i)?)))
        .filter(|(_, new)| **new == cur + 1)
        .for_each(|(i, new)| {
            let newtup = (i, *new);
            worklist.push_back(newtup);
        });
    }

    finalize(acc)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, sum_trailhead_ratings, sum_trailhead_scores};

    #[test]
    fn test_part_1_1() {
        let input = concat!("0123\n", "1234\n", "8765\n", "9876\n",);
        assert_eq!(sum_trailhead_scores(&parse_input(input)), 1);
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
        assert_eq!(sum_trailhead_scores(&parse_input(input)), 36);
    }

    #[test]
    fn test_part_2() {
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
        assert_eq!(sum_trailhead_ratings(&parse_input(input)), 81);
    }
}
