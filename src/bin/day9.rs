use std::{fmt::Write, iter::repeat_n};

use aoc_2024::read_input;
use nom::{character::complete, multi::fold_many0, IResult};
use std::iter::RepeatN;

#[derive(PartialEq, Clone, Copy)]
enum DiskBlockType {
    Free,
    File(u64),
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct DiskBlock {
    length: u32,
    block_type: DiskBlockType,
}

fn main() {
    let input = read_input("day9");
    let fragged = parse_input(&input).unwrap().1;
    println!(
        "Part 1: {}",
        compressed_checksum(
            &fragged
                .iter()
                .map(|p| p.into_iter())
                .flatten()
                .collect::<Vec<_>>()
        )
    );
    println!("Part 2: {}", defragged_checksum(&fragged));
}

fn compressed_checksum(fragged: &[DiskBlockType]) -> u64 {
    let mut checksum = 0;
    let mut fragged = Vec::from(fragged);
    let mut indx = 0;
    let mut curlen = fragged.len();

    while indx < curlen {
        let cur = fragged.get(indx).unwrap();

        match cur {
            DiskBlockType::Free => {
                if let Some((last_used_index, last_used)) = fragged
                    .iter()
                    .enumerate()
                    .skip(indx)
                    .rev()
                    .find(|(_, b)| matches!(b, DiskBlockType::File(_)))
                {
                    curlen = last_used_index;
                    match last_used {
                        DiskBlockType::Free => panic!(),
                        DiskBlockType::File(id) => {
                            checksum += indx as u64 * id;
                        }
                    }
                    fragged.swap(last_used_index, indx);
                }
            }
            DiskBlockType::File(id) => {
                checksum += indx as u64 * id;
            }
        }

        indx += 1;
    }

    checksum
}

fn defragged_checksum(blocks: &[DiskBlock]) -> u64 {
    let mut checksum = 0;
    let mut fragged = Vec::from(blocks);
    let mut mul = 0;

    while let Some((block, newfragged)) = fragged.split_first() {
        let mut newfragged = Vec::from(newfragged);
        match block.block_type {
            DiskBlockType::Free => {
                if let Some((mut remove_index, last_used)) = newfragged
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| {
                        matches!(b.block_type, DiskBlockType::File(_)) && b.length <= block.length
                    })
                    .last()
                {
                    match last_used.block_type {
                        DiskBlockType::Free => panic!(),
                        DiskBlockType::File(id) => {
                            for _ in 0..last_used.length {
                                checksum += id * mul;
                                mul += 1;
                            }

                            let a = last_used.length;
                            if last_used.length < block.length {
                                newfragged.insert(
                                    0,
                                    DiskBlock {
                                        length: block.length - last_used.length,
                                        block_type: DiskBlockType::Free,
                                    },
                                );
                                remove_index += 1;
                            }
                            newfragged.remove(remove_index);
                            newfragged.insert(
                                remove_index,
                                DiskBlock {
                                    length: a,
                                    block_type: DiskBlockType::Free,
                                },
                            );
                        }
                    }
                } else {
                    mul += block.length as u64;
                }
            }
            DiskBlockType::File(id) => {
                for _ in 0..block.length {
                    checksum += id * mul;
                    mul += 1;
                }
            }
        }

        fragged = newfragged;
    }

    checksum
}

fn parse_input(input: &str) -> IResult<&str, Vec<DiskBlock>> {
    fold_many0(
        complete::satisfy(|c| c.is_ascii_digit()),
        || (Vec::new(), false, 0),
        |(mut v, free, mut id), c| {
            let length = c.to_string().parse::<u32>().unwrap();
            if free {
                v.push(DiskBlock {
                    length,
                    block_type: DiskBlockType::Free,
                });
            } else {
                v.push(DiskBlock {
                    length,
                    block_type: DiskBlockType::File(id),
                });
                id += 1;
            }
            (v, !free, id)
        },
    )(input)
    .map(|(r, (v, _, _))| (r, v))
}

impl std::fmt::Debug for DiskBlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskBlockType::Free => f.write_char('.'),
            DiskBlockType::File(id) => write!(f, "{}", id),
        }
    }
}

impl IntoIterator for DiskBlock {
    type Item = DiskBlockType;

    type IntoIter = RepeatN<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        repeat_n(self.block_type, self.length as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::{compressed_checksum, defragged_checksum, parse_input, DiskBlock, DiskBlockType};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let fragged = parse_input(INPUT).unwrap().1;
        assert_eq!(
            compressed_checksum(
                &fragged
                    .iter()
                    .map(|p| p.into_iter())
                    .flatten()
                    .collect::<Vec<_>>()
            ),
            1928
        );
    }

    #[test]
    fn test_part_2() {
        let fragged = parse_input(INPUT).unwrap().1;
        assert_eq!(defragged_checksum(&fragged), 2858);
    }

    #[test]
    fn test_parse() {
        let input = "12345";
        assert_eq!(
            parse_input(input).unwrap().1,
            [
                DiskBlock {
                    length: 1,
                    block_type: DiskBlockType::File(0)
                },
                DiskBlock {
                    length: 2,
                    block_type: DiskBlockType::Free
                },
                DiskBlock {
                    length: 3,
                    block_type: DiskBlockType::File(1)
                },
                DiskBlock {
                    length: 4,
                    block_type: DiskBlockType::Free
                },
                DiskBlock {
                    length: 5,
                    block_type: DiskBlockType::File(2)
                },
            ]
        );
    }
}
