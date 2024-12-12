use aoc_2024::{
    containers::{maybe_remove_first, Vec2D, Vec2DIndex},
    read_input,
};

fn main() {
    let input = read_input("day12");
    let map = parse_input(&input);

    println!("Part 1: {}", total_fencing_price(&map));
}

fn total_fencing_price(map: &Vec2D<char>) -> u64 {
    let mut sum = 0;
    let mut unchecked_positions = Vec::from_iter(map.indizes());

    while let Some((start, _)) = unchecked_positions.split_first() {
        let region = flood_fill_region(map, start);
        sum += region.0;
        unchecked_positions.retain(|p| !region.1.contains(p));
    }

    sum
}

fn flood_fill_region(map: &Vec2D<char>, start: &Vec2DIndex) -> (u64, Vec<Vec2DIndex>) {
    let mut perimiter = 0;
    let mut positions = Vec::new();
    let mut worklist = Vec::new();
    let startval = map.get_index(start).unwrap();

    worklist.push(*start);

    while !worklist.is_empty() {
        let el = maybe_remove_first(&mut worklist).unwrap();
        if positions.contains(&el) {
            continue;
        }
        positions.push(el);

        for (dl, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some(newpos) = el.checked_add_signed(dl, dc) {
                if let Some(newval) = map.get_index(&newpos) {
                    if newval == startval {
                        worklist.push(newpos);
                    } else {
                        perimiter += 1;
                    }
                } else {
                    perimiter += 1;
                }
            } else {
                perimiter += 1;
            }
        }
    }

    (perimiter * positions.len() as u64, positions)
}

fn parse_input(input: &str) -> Vec2D<char> {
    Vec2D::from_lines(input.lines().map(|l| l.chars()))
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, total_fencing_price};

    fn run_part_1(input: &str, exp: u64) {
        let map = parse_input(input);
        assert_eq!(total_fencing_price(&map), exp);
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
}
