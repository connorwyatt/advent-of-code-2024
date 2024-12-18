use std::mem;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        contiguous_compacted_filesystem_checksum(INPUT)
    );
}

#[allow(dead_code)]
fn compacted_filesystem_checksum(input: &str) -> usize {
    let (file_blocks, len) = parse_input(input);

    let mut expanded_file_blocks = expand_file_blocks(&file_blocks, len);

    compact_expanded_file_blocks(&mut expanded_file_blocks[..]);

    expanded_file_blocks
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, file_block)| {
            let Some(file_block_id) = file_block else {
                return acc;
            };
            acc + (i * file_block_id)
        })
}

fn contiguous_compacted_filesystem_checksum(input: &str) -> usize {
    let (file_blocks, len) = parse_input(input);

    let compacted_file_blocks = compact_file_blocks(&file_blocks[..]);

    let compacted_expanded_file_blocks = expand_file_blocks(&compacted_file_blocks[..], len);

    compacted_expanded_file_blocks
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, file_block)| {
            let Some(file_block_id) = file_block else {
                return acc;
            };
            acc + (i * file_block_id)
        })
}

#[derive(Clone, Debug, PartialEq)]
struct FileBlock {
    id: usize,
    offset: usize,
    len: usize,
}

fn parse_input(input: &str) -> (Vec<FileBlock>, usize) {
    let mut result = Vec::new();

    let mut next_id = 0;
    let mut next_offset = 0;

    for (i, character) in input.trim().chars().enumerate() {
        let is_file = i % 2 == 0;

        let len: usize = character.to_digit(10).unwrap().try_into().unwrap();

        if len == 0 {
            continue;
        }

        if is_file {
            result.push(FileBlock {
                id: next_id,
                offset: next_offset,
                len,
            });

            next_id += 1;
        }

        next_offset += len;
    }

    (result, next_offset)
}

fn expand_file_blocks(file_blocks: &[FileBlock], len: usize) -> Vec<Option<usize>> {
    let mut result = Vec::new();

    for file_block in file_blocks {
        while file_block.offset > result.len() {
            result.push(None);
        }

        for _ in 0..file_block.len {
            result.push(Some(file_block.id));
        }
    }

    while result.len() < len {
        result.push(None);
    }

    result
}

fn compact_expanded_file_blocks(file_blocks: &mut [Option<usize>]) {
    let mut file_block_iter_mut = file_blocks.iter_mut();

    loop {
        let Some(file_block) = file_block_iter_mut.next() else {
            break;
        };

        if file_block.is_some() {
            continue;
        };

        loop {
            let Some(end_file_block) = file_block_iter_mut.next_back() else {
                break;
            };

            if end_file_block.is_none() {
                continue;
            }

            mem::swap(file_block, end_file_block);
            break;
        }
    }
}

fn compact_file_blocks(file_blocks: &[FileBlock]) -> Vec<FileBlock> {
    let mut result = file_blocks.to_vec();
    result.sort_by_key(|b| b.offset);

    let ids = file_blocks.iter().map(|b| b.id);

    for id in ids.rev() {
        let file_block_to_move = file_blocks.iter().find(|b| b.id == id).unwrap();

        let mut offset_to_move_to: Option<usize> = None;

        let mut windowed_result = result.windows(2);
        while let Some([a, b]) = windowed_result.next() {
            if b.offset > file_block_to_move.offset {
                break;
            }

            let empty_space_offset = a.offset + a.len;
            let empty_space_between = b.offset - empty_space_offset;

            if empty_space_between < file_block_to_move.len {
                continue;
            }

            offset_to_move_to = Some(empty_space_offset);
            break;
        }

        if let Some(offset) = offset_to_move_to {
            let file_block = result.iter_mut().find(|b| b.id == id).unwrap();
            file_block.offset = offset;

            result.sort_by_key(|b| b.offset);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("aoc-input/example-input.txt");

    #[test]
    fn compacted_filesystem_checksum_works() {
        assert_eq!(compacted_filesystem_checksum(EXAMPLE_INPUT), 1928);
    }

    #[test]
    fn contiguous_compacted_filesystem_checksum_works() {
        assert_eq!(
            contiguous_compacted_filesystem_checksum(EXAMPLE_INPUT),
            2858
        );
    }

    #[test]
    fn parse_input_works() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            (
                vec![
                    FileBlock {
                        id: 0,
                        offset: 0,
                        len: 2
                    },
                    FileBlock {
                        id: 1,
                        offset: 5,
                        len: 3
                    },
                    FileBlock {
                        id: 2,
                        offset: 11,
                        len: 1
                    },
                    FileBlock {
                        id: 3,
                        offset: 15,
                        len: 3
                    },
                    FileBlock {
                        id: 4,
                        offset: 19,
                        len: 2
                    },
                    FileBlock {
                        id: 5,
                        offset: 22,
                        len: 4
                    },
                    FileBlock {
                        id: 6,
                        offset: 27,
                        len: 4
                    },
                    FileBlock {
                        id: 7,
                        offset: 32,
                        len: 3
                    },
                    FileBlock {
                        id: 8,
                        offset: 36,
                        len: 4
                    },
                    FileBlock {
                        id: 9,
                        offset: 40,
                        len: 2
                    },
                ],
                42
            )
        )
    }

    #[test]
    fn expand_file_blocks_works() {
        assert_eq!(
            expand_file_blocks(
                &vec![
                    FileBlock {
                        id: 0,
                        offset: 0,
                        len: 2
                    },
                    FileBlock {
                        id: 1,
                        offset: 5,
                        len: 3
                    },
                    FileBlock {
                        id: 2,
                        offset: 11,
                        len: 1
                    },
                    FileBlock {
                        id: 3,
                        offset: 15,
                        len: 3
                    },
                    FileBlock {
                        id: 4,
                        offset: 19,
                        len: 2
                    },
                    FileBlock {
                        id: 5,
                        offset: 22,
                        len: 4
                    },
                    FileBlock {
                        id: 6,
                        offset: 27,
                        len: 4
                    },
                    FileBlock {
                        id: 7,
                        offset: 32,
                        len: 3
                    },
                    FileBlock {
                        id: 8,
                        offset: 36,
                        len: 4
                    },
                    FileBlock {
                        id: 9,
                        offset: 40,
                        len: 2
                    },
                ],
                42
            ),
            vec![
                Some(0),
                Some(0),
                None,
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                Some(2),
                None,
                None,
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                Some(4),
                Some(4),
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                Some(9),
                Some(9),
            ]
        );
    }

    #[test]
    fn compact_expanded_file_blocks_works() {
        let mut expanded_file_blocks = vec![
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ];

        compact_expanded_file_blocks(&mut expanded_file_blocks[..]);

        assert_eq!(
            expanded_file_blocks,
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn compact_file_blocks_works() {
        let file_blocks = vec![
            FileBlock {
                id: 0,
                offset: 0,
                len: 2,
            },
            FileBlock {
                id: 1,
                offset: 5,
                len: 3,
            },
            FileBlock {
                id: 2,
                offset: 11,
                len: 1,
            },
            FileBlock {
                id: 3,
                offset: 15,
                len: 3,
            },
            FileBlock {
                id: 4,
                offset: 19,
                len: 2,
            },
            FileBlock {
                id: 5,
                offset: 22,
                len: 4,
            },
            FileBlock {
                id: 6,
                offset: 27,
                len: 4,
            },
            FileBlock {
                id: 7,
                offset: 32,
                len: 3,
            },
            FileBlock {
                id: 8,
                offset: 36,
                len: 4,
            },
            FileBlock {
                id: 9,
                offset: 40,
                len: 2,
            },
        ];

        assert_eq!(
            compact_file_blocks(&file_blocks[..]),
            vec![
                FileBlock {
                    id: 0,
                    offset: 0,
                    len: 2
                },
                FileBlock {
                    id: 9,
                    offset: 2,
                    len: 2
                },
                FileBlock {
                    id: 2,
                    offset: 4,
                    len: 1
                },
                FileBlock {
                    id: 1,
                    offset: 5,
                    len: 3
                },
                FileBlock {
                    id: 7,
                    offset: 8,
                    len: 3
                },
                FileBlock {
                    id: 4,
                    offset: 12,
                    len: 2
                },
                FileBlock {
                    id: 3,
                    offset: 15,
                    len: 3
                },
                FileBlock {
                    id: 5,
                    offset: 22,
                    len: 4
                },
                FileBlock {
                    id: 6,
                    offset: 27,
                    len: 4
                },
                FileBlock {
                    id: 8,
                    offset: 36,
                    len: 4
                },
            ]
        );
    }
}
