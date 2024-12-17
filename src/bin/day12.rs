use std::collections::{HashSet, VecDeque};

use aoc_2024::{
    containers::{Vec2D, Vec2DIndex},
    coord::Direction2D,
    read_input,
};

struct Region {
    area: HashSet<Vec2DIndex>,
    border: HashSet<(Direction2D, Vec2DIndex)>,
}

struct Regions<'a, E> {
    map: &'a Vec2D<E>,
    unchecked_positions: Vec<Vec2DIndex>,
}

struct FencingPrice {
    total: u64,
    discounted: u64,
}

fn main() {
    let input = read_input("day12");
    let map = parse_input(&input);

    println!("Part 1: {}", total_fencing_price(&map).total);
    println!("Part 2: {}", total_fencing_price(&map).discounted);
}

fn total_fencing_price(map: &Vec2D<char>) -> FencingPrice {
    Regions::new(map).fold(
        FencingPrice {
            total: 0,
            discounted: 0,
        },
        |mut acc, r| {
            acc.total += r.area.len() as u64 * r.border.len() as u64;
            acc.discounted += r.area.len() as u64 * count_sides(&r);
            acc
        },
    )
}

fn count_sides(region: &Region) -> u64 {
    let mut all_considered_sides: HashSet<(Direction2D, Vec2DIndex)> = HashSet::new();

    let neighbor_offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let valid_neighbors = |(dir, pos): (Direction2D, Vec2DIndex)| {
        neighbor_offsets
            .iter()
            .filter_map(move |&(dl, dc)| pos.checked_add_signed(dl, dc))
            .map(move |i| (dir, i))
            .filter(|e| region.border.contains(e))
    };

    for init in region.border.iter() {
        let mut worklist = VecDeque::new();
        let mut reachable_edges: HashSet<(Direction2D, Vec2DIndex)> = HashSet::new();
        worklist.push_back(*init);

        while let Some(edge) = worklist.pop_front() {
            for n in valid_neighbors(edge) {
                if !reachable_edges.contains(&n) {
                    reachable_edges.insert(n);
                    worklist.push_back(n);
                }
            }
        }

        if !reachable_edges
            .iter()
            .any(|e| all_considered_sides.contains(e))
        {
            all_considered_sides.insert(*init);
        }
    }

    all_considered_sides.len() as u64
}

fn parse_input(input: &str) -> Vec2D<char> {
    Vec2D::from_lines(input.lines().map(|l| l.chars()))
}

impl<'a, E: PartialEq + Copy> Regions<'a, E> {
    fn new(map: &'a Vec2D<E>) -> Self {
        Self {
            map,
            unchecked_positions: Vec::from_iter(map.indizes()),
        }
    }

    fn flood_fill_region(&self, start: &Vec2DIndex) -> Region {
        let mut border = HashSet::new();
        let mut positions = HashSet::new();
        let mut worklist = VecDeque::new();
        let startval = *self.map.get_index(start).unwrap();

        worklist.push_back(*start);

        while let Some(el) = worklist.pop_front() {
            if positions.contains(&el) {
                continue;
            }
            positions.insert(el);

            for dir in Direction2D::all() {
                let diff = dir.to_offset();
                let newpos =
                    el.checked_add_signed(diff.dy.try_into().unwrap(), diff.dx.try_into().unwrap());
                if newpos.is_none() {
                    border.insert((dir, el));
                    continue;
                }

                let newval = self.map.get_index(&newpos.unwrap());
                if newval.is_none() {
                    border.insert((dir, el));
                    continue;
                }

                if *newval.unwrap() != startval {
                    border.insert((dir, el));
                    continue;
                }

                worklist.push_back(newpos.unwrap());
            }
        }

        Region {
            area: positions,
            border,
        }
    }
}

impl<E: PartialEq + Copy> Iterator for Regions<'_, E> {
    type Item = Region;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((start, _)) = self.unchecked_positions.split_first() {
            let region = self.flood_fill_region(start);
            self.unchecked_positions
                .retain(|p| !region.area.contains(p));
            Some(region)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, total_fencing_price};

    fn run_part_1(input: &str, exp: u64) {
        let map = parse_input(input);
        assert_eq!(total_fencing_price(&map).total, exp);
    }

    fn run_part_2(input: &str, exp: u64) {
        let map = parse_input(input);
        assert_eq!(total_fencing_price(&map).discounted, exp);
    }

    #[test]
    fn test_part_1_1() {
        run_part_1(concat!("AAAA\n", "BBCD\n", "BBCC\n", "EEEC\n"), 140);
    }

    #[test]
    fn test_part_1_2() {
        run_part_1(
            concat!("OOOOO\n", "OXOXO\n", "OOOOO\n", "OXOXO\n", "OOOOO\n"),
            772,
        );
    }

    #[test]
    fn test_part_1_3() {
        run_part_1(
            concat!(
                "RRRRIICCFF\n",
                "RRRRIICCCF\n",
                "VVRRRCCFFF\n",
                "VVRCCCJFFF\n",
                "VVVVCJJCFE\n",
                "VVIVCCJJEE\n",
                "VVIIICJJEE\n",
                "MIIIIIJJEE\n",
                "MIIISIJEEE\n",
                "MMMISSJEEE\n",
            ),
            1930,
        );
    }

    #[test]
    fn test_part_2_1() {
        run_part_2(concat!("AAAA\n", "BBCD\n", "BBCC\n", "EEEC\n"), 80);
    }

    #[test]
    fn test_part_2_2() {
        run_part_2(
            concat!("OOOOO\n", "OXOXO\n", "OOOOO\n", "OXOXO\n", "OOOOO\n"),
            436,
        );
    }

    #[test]
    fn test_part_2_3() {
        run_part_2(
            concat!("EEEEE\n", "EXXXX\n", "EEEEE\n", "EXXXX\n", "EEEEE\n",),
            236,
        );
    }

    #[test]
    fn test_part_2_4() {
        run_part_2(
            concat!("AAAAAA\n", "AAABBA\n", "AAABBA\n", "ABBAAA\n", "ABBAAA\n", "AAAAAA\n",),
            368,
        );
    }
}
