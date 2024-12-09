use std::collections::HashSet;

use aoc_2024::{
    coord::{BoundedCoord2D, Bounds2D, Coord2D, Direction2D},
    read_input,
};
use nom::{character::complete, multi::fold_many1, sequence::terminated, IResult};

type Obstacles = HashSet<Coord2D>;

fn main() {
    let input = read_input("day6");
    let (obstacles, init) = parse_input(&input).unwrap().1;
    let init = init.unwrap();
    let path = travel(&init, &obstacles);

    println!("Part 1: {}", path.len());
    println!(
        "Part 2: {}",
        count_obstacles_causing_loop(&init, &obstacles)
    );
}

fn count_obstacles_causing_loop(init: &BoundedCoord2D, obstacles: &Obstacles) -> usize {
    let mut newobstacles = obstacles.clone();
    travel(init, obstacles)
        .iter()
        .filter(|c| **c != init.unbounded())
        .filter(|c| !obstacles.contains(*c))
        .filter(|c| {
            newobstacles.insert(**c);
            let res = travel_loops(init, &newobstacles);
            newobstacles.remove(*c);
            res
        })
        .count()
}

fn states_to_positions(states: &HashSet<(Coord2D, Direction2D)>) -> HashSet<Coord2D> {
    states.iter().map(|(c, _)| *c).collect()
}

fn travel(init: &BoundedCoord2D, obstacles: &Obstacles) -> HashSet<Coord2D> {
    let mut visited: HashSet<(Coord2D, Direction2D)> = HashSet::new();
    let mut cur_pos = *init;
    let mut cur_dir = Direction2D::Up;

    visited.insert((cur_pos.unbounded(), cur_dir));
    while let Some(new_pos) = cur_pos.go_in(&cur_dir) {
        if obstacles.contains(&new_pos.unbounded()) {
            cur_dir = cur_dir.turn_right();
        } else {
            cur_pos = new_pos;
            if !visited.insert((cur_pos.unbounded(), cur_dir)) {
                panic!("travel() looped");
            }
        }
    }

    states_to_positions(&visited)
}

fn travel_loops(init: &BoundedCoord2D, obstacles: &Obstacles) -> bool {
    let mut cur_pos = *init;
    let mut cur_dir = Direction2D::Up;
    let mut turns = Vec::new();

    while let Some(new_pos) = cur_pos.go_in(&cur_dir) {
        if obstacles.contains(&new_pos.unbounded()) {
            cur_dir = cur_dir.turn_right();
            if turns.contains(&(cur_pos, cur_dir)) {
                return true;
            }
            turns.push((cur_pos, cur_dir));
        } else {
            cur_pos = new_pos;
        }
        if cur_pos == *init && cur_dir == Direction2D::Up {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> IResult<&str, (Obstacles, Option<BoundedCoord2D>)> {
    let (after, (obstacles, init, _, bounds)) = fold_many1(
        terminated(
            nom::multi::many1(complete::one_of(".#^")),
            complete::newline,
        ),
        || (HashSet::<Coord2D>::new(), None, 0, Bounds2D::new(0, 0)),
        |mut acc, l| {
            acc.3.expand_height(acc.2 + 1);
            for (idx, ch) in l.iter().enumerate() {
                {
                    acc.3.expand_width(idx as u32 + 1);
                }
                match ch {
                    '^' => {
                        assert!(acc.1.is_none(), "multiple initial positions");
                        acc.1 = Some(Coord2D::new(idx as u32, acc.2))
                    }
                    '.' => {}
                    '#' => {
                        acc.0.insert(Coord2D::new(idx as u32, acc.2));
                    }
                    c => {
                        panic!("unknown char {c}")
                    }
                }
            }
            acc.2 += 1;
            acc
        },
    )(input)?;

    Ok((after, (obstacles, init.map(|c| c.into_bounded(bounds)))))
}

#[cfg(test)]
mod tests {
    use aoc_2024::coord::Coord2D;

    use crate::{count_obstacles_causing_loop, parse_input, travel, travel_loops};

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part_1() {
        let (_, (obstacles, init)) = parse_input(INPUT).unwrap();
        assert_eq!(travel(&init.unwrap(), &obstacles).len(), 41);
    }

    #[test]
    fn test_loop() {
        let (_, (mut obstacles, init)) = parse_input(INPUT).unwrap();
        obstacles.insert(Coord2D::new(3, 6));
        assert!(travel_loops(&init.unwrap(), &obstacles));
    }

    #[test]
    fn test_part_2() {
        let (_, (obstacles, init)) = parse_input(INPUT).unwrap();
        assert_eq!(count_obstacles_causing_loop(&init.unwrap(), &obstacles), 6);
    }
}
