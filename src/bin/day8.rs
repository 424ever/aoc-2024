use std::collections::{HashMap, HashSet};

use aoc_2024::{
    coord::{BoundedCoord2D, Bounds2D, Coord2D},
    read_input,
};
use itertools::iproduct;

type Frequency = char;
type Antennas = HashSet<BoundedCoord2D>;

fn main() {
    let input = read_input("day8");
    let antennas = parse_input(&input);
    println!("Part 1: {}", antinodes(&antennas).len());
    println!("Part 2: {}", antinodes_with_resonance(&antennas).len());
}

fn antinodes(antennas: &HashMap<Frequency, Antennas>) -> HashSet<Coord2D> {
    antennas
        .iter()
        .map(|(_, ant)| antinodes_of(ant))
        .flatten()
        .collect()
}

fn antinodes_with_resonance(antennas: &HashMap<Frequency, Antennas>) -> HashSet<Coord2D> {
    antennas
        .iter()
        .map(|(_, ant)| antinodes_with_resonance_of(ant))
        .flatten()
        .collect()
}

fn antinodes_of(antennas: &Antennas) -> HashSet<Coord2D> {
    //     0  1  2  3  4  5  6  7  8  9
    //   +-----------------------------
    // 0 | .  .  .  .  .  .  .  .  .  .
    // 1 | .  .  .  R  .  .  .  .  .  .
    // 2 | .  .  .  .  .  .  .  .  .  .
    // 3 | .  .  .  .  b  .  .  .  .  .
    // 4 | .  .  .  .  .  .  .  .  .  .
    // 5 | .  .  .  .  .  a  .  .  .  .
    // 6 | .  .  .  .  .  .  .  .  .  .
    // 7 | .  .  .  .  .  .  .  .  .  .
    // 8 | .  .  .  .  .  .  .  .  .  .
    // 9 | .  .  .  .  .  .  .  .  .  .
    //
    // Have:
    //   5; 5 [a]
    //   4; 3 [b]
    //
    // Want:
    //   3; 1 [R]
    //
    // [b] - [a] [d]: vector from [b] to [a]
    //   -2; -4
    //
    // [d] * 2: vector pointing in the direction from [b] to [a], but twice
    //          as long
    iproduct!(antennas.iter(), antennas.iter())
        .filter(|(a, b)| a != b)
        .filter_map(|(&a, &b)| a + ((b - a) * 2))
        .map(|b| b.unbounded())
        .collect()
}

fn antinodes_with_resonance_of(antennas: &Antennas) -> HashSet<Coord2D> {
    iproduct!(antennas.iter(), antennas.iter())
        .filter(|(a, b)| a != b)
        .map(|(&a, &b)| (1..).map_while(move |n| a + ((b - a) * n)))
        .flatten()
        .map(|b| b.unbounded())
        .collect()
}

fn parse_input(input: &str) -> HashMap<Frequency, Antennas> {
    let mut antennas = HashMap::new();

    let height = input.lines().count().try_into().unwrap();
    let width = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();

    let bounds = Bounds2D::new(width, height);

    for (linno, line) in input.lines().enumerate() {
        for (colno, char) in line.chars().enumerate() {
            if char != '.' {
                let coord = BoundedCoord2D::new(
                    Coord2D::new(colno.try_into().unwrap(), linno.try_into().unwrap()),
                    bounds,
                );
                antennas
                    .entry(char)
                    .and_modify(|e: &mut HashSet<_>| {
                        e.insert(coord);
                    })
                    .or_insert(HashSet::from([coord]));
            }
        }
    }

    antennas
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use aoc_2024::coord::{BoundedCoord2D, Bounds2D, Coord2D};

    use crate::{antinodes, antinodes_with_resonance, parse_input};

    const INPUT: &str = concat!(
        "............\n",
        "........0...\n",
        ".....0......\n",
        ".......0....\n",
        "....0.......\n",
        "......A.....\n",
        "............\n",
        "............\n",
        "........A...\n",
        ".........A..\n",
        "............\n",
        "............\n",
    );

    #[test]
    fn test_parse() {
        let ant = parse_input(INPUT);
        assert_eq!(
            ant,
            HashMap::from([
                ('0', HashSet::from([mk(8, 1), mk(5, 2), mk(7, 3), mk(4, 4)])),
                ('A', HashSet::from([mk(6, 5), mk(8, 8), mk(9, 9)]))
            ])
        );

        fn mk(x: u32, y: u32) -> BoundedCoord2D {
            BoundedCoord2D::new(Coord2D::new(x, y), Bounds2D::new(12, 12))
        }
    }

    #[test]
    fn test_part_1() {
        let antennas = parse_input(INPUT);
        dbg!(antinodes(&antennas));
        assert_eq!(antinodes(&antennas).len(), 14);
    }

    #[test]
    fn test_part_2() {
        let antennas = parse_input(INPUT);
        dbg!(antinodes(&antennas));
        assert_eq!(antinodes_with_resonance(&antennas).len(), 34);
    }
}
