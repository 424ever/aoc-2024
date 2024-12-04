use aoc_2024::containers::Vec2D;
use aoc_2024::read_input;

fn main() {
    let input = read_input("day4");
    let input = Vec2D::from_lines(input.lines().map(|l| l.chars()));
    println!("Part 1: {}", count_xmas(&input));
    println!("Part 2: {}", count_cross_mas(&input));
}

fn count_xmas(input: &Vec2D<char>) -> u32 {
    let mut count = 0;
    for l in 0..input.lines() {
        for c in 0..input.cols() {
            count += count_xmas_starting_at(input, l, c);
        }
    }
    count
}

fn count_cross_mas(input: &Vec2D<char>) -> u32 {
    let mut count = 0;
    for l in 1..input.lines() - 1 {
        for c in 1..input.cols() - 1 {
            if input.get(l, c) == Some(&'A') && is_cross_mas_middle(input, l, c) {
                count += 1;
            }
        }
    }
    count
}

fn count_xmas_starting_at(input: &Vec2D<char>, l: usize, c: usize) -> u32 {
    let offsets = vec![
        (0, 1),   /* forwards */
        (0, -1),  /* backwards */
        (1, 0),   /* down */
        (-1, 0),  /* up */
        (1, 1),   /* down-right */
        (1, -1),  /* down-left */
        (-1, 1),  /* up-right */
        (-1, -1), /* up-left */
    ];
    offsets
        .iter()
        .filter(|(lo, co)| word_with_offsets(input, "XMAS", l, c, *lo, *co).is_some())
        .count() as u32
}

fn is_cross_mas_middle(input: &Vec2D<char>, line: usize, col: usize) -> bool {
    let offs: Vec<(isize, isize, isize, isize)> = vec![
        /* L1, C1, L2, C2 */
        (-1, -1, 1, -1), /* right */
        (-1, -1, -1, 1), /* down */
        (-1, 1, 1, 1),   /* left */
        (1, 1, 1, -1),   /* up */
    ];

    for &(l1, c1, l2, c2) in offs.iter() {
        if word_with_offsets(
            input,
            "MAS",
            line.checked_add_signed(l1).unwrap(),
            col.checked_add_signed(c1).unwrap(),
            -l1,
            -c1,
        )
        .is_some()
            && word_with_offsets(
                input,
                "MAS",
                line.checked_add_signed(l2).unwrap(),
                col.checked_add_signed(c2).unwrap(),
                -l2,
                -c2,
            )
            .is_some()
        {
            return true;
        }
    }

    false
}

fn word_with_offsets(
    input: &Vec2D<char>,
    word: &str,
    line: usize,
    col: usize,
    lo: isize,
    co: isize,
) -> Option<()> {
    for (idx, ch) in word.chars().enumerate() {
        let line = line.checked_add_signed(lo * idx as isize)?;
        let col = col.checked_add_signed(co * idx as isize)?;
        let got = input.get(line, col)?;

        if got == &ch {
            continue;
        } else {
            return None;
        }
    }
    return Some(());
}

#[cfg(test)]
mod tests {
    use aoc_2024::containers::Vec2D;

    use crate::*;

    const INPUT: &str = concat!(
        "MMMSXXMASM\n",
        "MSAMXMSMSA\n",
        "AMXSXMAAMM\n",
        "MSAMASMSMX\n",
        "XMASAMXAMM\n",
        "XXAMMXXAMA\n",
        "SMSMSASXSS\n",
        "SAXAMASAAA\n",
        "MAMMMXMMMM\n",
        "MXMXAXMASX\n",
    );

    #[test]
    fn test_part_1() {
        let input = Vec2D::from_lines(INPUT.lines().map(|l| l.chars()));
        assert_eq!(count_xmas(&input), 18);
    }

    #[test]
    fn test_part_2() {
        let input = Vec2D::from_lines(INPUT.lines().map(|l| l.chars()));
        assert_eq!(count_cross_mas(&input), 9);
    }
}
