use std::fmt::Write;

use aoc_2024::read_input;
use nom::{character::complete, multi::fold_many0, IResult};

#[derive(PartialEq, Clone, Copy)]
enum DiskBlock {
    Free,
    File(u64),
}

fn main() {
    let input = read_input("day9");
    let fragged = parse_input(&input).unwrap().1;
    println!("Part 1: {}", compressed_checksum(&fragged));
}

fn compressed_checksum(fragged: &[DiskBlock]) -> u64 {
    let mut checksum = 0;
    let mut fragged = Vec::from(fragged);
    let mut indx = 0;
    let mut curlen = fragged.len();

    while indx < curlen {
        let cur = fragged.get(indx).unwrap();

        match cur {
            DiskBlock::Free => {
                if let Some((last_used_index, last_used)) = fragged
                    .iter()
                    .enumerate()
                    .skip(indx)
                    .rev()
                    .find(|(_, b)| matches!(b, DiskBlock::File(_)))
                {
                    curlen = last_used_index;
                    match last_used {
                        DiskBlock::Free => panic!(),
                        DiskBlock::File(id) => {
                            checksum += indx as u64 * id;
                        }
                    }
                    fragged.swap(last_used_index, indx);
                }
            }
            DiskBlock::File(id) => {
                checksum += indx as u64 * id;
            }
        }

        indx += 1;
    }

    checksum
}

fn parse_input(input: &str) -> IResult<&str, Vec<DiskBlock>> {
    fold_many0(
        complete::satisfy(|c| c.is_ascii_digit()),
        || (Vec::new(), false, 0),
        |(mut v, free, mut id), c| {
            let length = c.to_string().parse::<u8>().unwrap();
            for _ in 0..length {
                if free {
                    v.push(DiskBlock::Free);
                } else {
                    v.push(DiskBlock::File(id));
                }
            }
            if !free {
                id += 1;
            }
            (v, !free, id)
        },
    )(input)
    .map(|(r, (v, _, _))| (r, v))
}

impl std::fmt::Debug for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskBlock::Free => f.write_char('.'),
            DiskBlock::File(id) => write!(f, "{}", id),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{compressed_checksum, parse_input, DiskBlock};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let fragged = parse_input(INPUT).unwrap().1;
        assert_eq!(compressed_checksum(&fragged), 1928);
    }

    #[test]
    fn test_parse() {
        let input = "12345";
        assert_eq!(
            parse_input(input).unwrap().1,
            [
                DiskBlock::File(0),
                DiskBlock::Free,
                DiskBlock::Free,
                DiskBlock::File(1),
                DiskBlock::File(1),
                DiskBlock::File(1),
                DiskBlock::Free,
                DiskBlock::Free,
                DiskBlock::Free,
                DiskBlock::Free,
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
                DiskBlock::File(2),
            ]
        );
    }
}
