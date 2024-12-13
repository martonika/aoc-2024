use std::{fmt::Debug, fs};

#[derive(Clone, Copy)]
enum Block {
    File { id: usize },
    Free,
}

impl Block {
    fn get_id(&self) -> usize {
        match self {
            Block::File { id } => *id,
            Block::Free => usize::MAX,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::File { id } => write!(f, "{}", id),
            Block::Free => write!(f, "."),
        }
    }
}

fn move_block(blocks: &mut [Block]) {
    let mut left = 0;
    let mut right = blocks.len() - 1;
    loop {
        while let Block::File { id: _ } = blocks[left] {
            left += 1;
        }
        while let Block::Free = blocks[right] {
            right -= 1;
        }
        // Break if the two pointers meet
        if left >= right {
            break;
        }
        // Found a file block that can be swapped with a free block
        blocks.swap(right, left);
    }
}

fn move_file(blocks: &mut [Block]) {
    let mut right = blocks.len() - 1;
    loop {
        // Need to scan from the start every time for free space
        let mut left = 0;
        while let Block::Free = blocks[right] {
            right -= 1;
        }
        // Found (the end of) a file block, check its length
        // We can move the right pointer, because if it doesn't fit anywhere
        // the file stays
        let mut file_len = 0;
        let curr_id = blocks[right].get_id();
        while let Block::File { id } = blocks[right] {
            if curr_id != id || right == 0 {
                break;
            }
            file_len += 1;
            right -= 1;
        }

        while left < right {
            while let Block::File { id: _ } = blocks[left] {
                left += 1;
            }
            // Found a chunk of free blocks, check length
            // Move left pointer to be able to split_at_mut later
            // It's reset in every loop anyways
            let mut free_len = 0;
            while let Block::Free = blocks[left] {
                if left == blocks.len() - 1 {
                    break;
                }
                left += 1;
                free_len += 1;
            }
            if free_len >= file_len && left <= right + 1 { // One "overlap" is allowed - it's when the data starts right after the found free space
                let (l, r) = blocks.split_at_mut(left);
                let start_left = left - free_len;
                let start_right = right - (left - 1);

                l[start_left..start_left + file_len]
                    .swap_with_slice(&mut r[start_right..start_right + file_len]);
                break;
            }
        }
        // Break if we swept through the whole file
        if right == 0 {
            break;
        }
    }
}

fn checksum(blocks: &[Block]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block::File { id } => (i * *id) as u64,
            Block::Free => 0,
        })
        .sum()
}

pub fn solve_1(input: &str) -> u64 {
    // Real input is 19999 chars, i.e. it is possible to store everything in a vector
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut blocks: Vec<Block> = data
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                // Data
                vec![Block::File { id: i / 2 }; num as usize]
            } else {
                // Free
                vec![Block::Free; num as usize]
            }
        })
        .collect();

    move_block(&mut blocks);

    checksum(&blocks)
}
pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut blocks: Vec<Block> = data
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                // Data
                vec![Block::File { id: i / 2 }; num as usize]
            } else {
                // Free
                vec![Block::Free; num as usize]
            }
        })
        .collect();

    move_file(&mut blocks);

    checksum(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day9/test1.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day9/test2.txt");
        assert_eq!(result, 169);
    }
}
