use std::mem;

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        compacted_filesystem_checksum(INPUT)
    );
}

fn compacted_filesystem_checksum(input: &str) -> usize {
    let mut blocks = parse_input(input);

    compact_blocks(&mut blocks[..]);

    blocks.iter().enumerate().fold(0usize, |acc, (i, block)| {
        let Some(block_id) = block else {
            return acc;
        };
        acc + (i * block_id)
    })
}

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let mut result = Vec::new();

    let mut id = 0;

    for (i, character) in input.trim().chars().enumerate() {
        let is_file = i % 2 == 0;


        let len = character.to_digit(10).unwrap();

        for _ in 0..len {
            if is_file {
                result.push(Some(id));
            } else {
                result.push(None);
            }
        }

        if is_file {
            id += 1;
        }
    }

    result
}

fn compact_blocks(blocks: &mut [Option<usize>]) {
    let mut block_iter_mut = blocks.iter_mut();

    loop {
        let Some(block) = block_iter_mut.next() else {
            break;
        };

        if block.is_some() {
            continue;
        };

        loop {
            let Some(end_block) = block_iter_mut.next_back() else {
                break;
            };

            if end_block.is_none() {
                continue;
            }

            mem::swap(block, end_block);
            break;
        }
    }
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
    fn parse_input_works() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
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
    fn compact_blocks_works() {
        let mut blocks = vec![
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

        compact_blocks(&mut blocks[..]);

        assert_eq!(
            blocks,
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
}
