use std::{collections::HashMap, fs};

fn numeric_to_dir(code: &[char]) -> &'static str {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    //
    // Only using possible directions for the test and my real input
    let prev = code[0];
    let next = code[1];
    match prev {
        'A' => match next {
            'A' => "A",
            '0' => "<A",
            '1' => "^<<A",
            '2' => "<^A",
            '3' => "^A",
            '4' => "^^<<A",
            '9' => "^^^A",
            _ => unreachable!(),
        },
        '0' => match next {
            'A' => ">A",
            '2' => "^A",
            _ => unreachable!(),
        },
        '1' => match next {
            'A' => ">>vA",
            '4' => "^A",
            '7' => "^^A",
            _ => unreachable!(),
        },
        '2' => match next {
            'A' => "v>A",
            '8' => "^^A",
            '9' => "^^>A",
            _ => unreachable!(),
        },
        '3' => match next {
            '4' => "<<^A",
            '7' => "<<^^A",
            _ => unreachable!(),
        },
        '4' => match next {
            '0' => ">vvA",
            '1' => "vA",
            '5' => ">A",
            '8' => "^>A",
            _ => unreachable!(),
        },
        '5' => match next {
            'A' => "vv>A",
            '6' => ">A",
            _ => unreachable!(),
        },
        '6' => match next {
            '5' => "<A",
            'A' => "vvA",
            _ => unreachable!(),
        },
        '7' => match next {
            '9' => ">>A",
            _ => unreachable!(),
        },
        '8' => match next {
            '0' => "vvvA",
            '5' => "vA",
            _ => unreachable!(),
        },
        '9' => match next {
            'A' => "vvvA",
            '6' => "vA",
            '8' => "<A",
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn dir_to_dir(code: &[char]) -> &'static str {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    // Here everything can happen
    // Priority: > ^ v <
    let prev = code[0];
    let next = code[1];
    match prev {
        'A' => match next {
            'A' => "A",
            '^' => "<A",
            'v' => "<vA",
            '<' => "v<<A",
            '>' => "vA",
            _ => unreachable!(),
        },
        '^' => match next {
            'A' => ">A",
            '^' => "A",
            'v' => "vA",
            '<' => "v<A",
            '>' => "v>A",
            _ => unreachable!(),
        },
        '>' => match next {
            'A' => "^A",
            '^' => "<^A",
            'v' => "<A",
            '<' => "<<A",
            '>' => "A",
            _ => unreachable!(),
        },
        'v' => match next {
            'A' => "^>A",
            '^' => "^A",
            'v' => "A",
            '<' => "<A",
            '>' => ">A",
            _ => unreachable!(),
        },
        '<' => match next {
            'A' => ">>^A",
            '^' => ">^A",
            'v' => ">A",
            '<' => "A",
            '>' => ">>A",
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn solve_1(input: &str) -> u64 {
    let mut sum = 0;
    let data = fs::read_to_string(input).expect("Can't open file");
    let codes = data
        .lines()
        .map(|l| {
            let l2 = "A".to_string() + l;
            l2.chars().collect()
        })
        .collect::<Vec<Vec<char>>>();

    for code in codes {
        // Parse the original number, without leading or trailing 'A's
        let mut code_str = code.iter().collect::<String>();
        code_str.remove(0);
        code_str.remove(code_str.len() - 1);
        let code_num: u64 = code_str.parse().unwrap();

        let mut moves: HashMap<(char, char), u64> = HashMap::new();
        let dirs = code
            .windows(2)
            .map(|w| numeric_to_dir(w))
            .collect::<String>();
        let dirs_v = dirs.chars().collect::<Vec<char>>();
        // Add first layer of moves
        // Start from A, keep track of the first direction separately
        let mut first = dir_to_dir(&['A', dirs_v[0]]).chars().next().unwrap();
        moves.insert(('A', dirs_v[0]), 1);
        for d in dirs_v.windows(2) {
            moves
                .entry((d[0], d[1]))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        for _ in 0..2 {
            let mut tmp = HashMap::new();
            //moves.insert(('A', first), 1);
            for ((from, to), cnt) in &moves {
                // Find the next move given the current (from, to) pair
                let next = dir_to_dir(&[*from, *to]);
                let mut next_moves = next.chars().collect::<Vec<char>>();
                next_moves.insert(0, 'A');
                let new_pairs = next_moves
                    .windows(2)
                    .map(|w| (w[0], w[1]))
                    .collect::<Vec<(char, char)>>();
                for p in new_pairs {
                    tmp.entry(p).and_modify(|e| *e += *cnt).or_insert(*cnt);
                }
            }
            moves = tmp;
            // Update the first direction for the next iteration
            first = dir_to_dir(&['A', first]).chars().next().unwrap();
        }
        // What we have now is the number of movements
        // -> no need to run through dir_to_dir again,
        // just sum the number of occurences
        let part_sum = moves
            .iter()
            .map(|(_, &v)| v)
            .collect::<Vec<u64>>()
            .iter()
            .sum::<u64>();
        moves.clear();
        sum += part_sum * code_num;
    }
    sum
}

pub fn solve_2(input: &str) -> u64 {
    let mut sum = 0;
    let data = fs::read_to_string(input).expect("Can't open file");
    let codes = data
        .lines()
        .map(|l| {
            let l2 = "A".to_string() + l;
            l2.chars().collect()
        })
        .collect::<Vec<Vec<char>>>();
    for code in codes {
        // Parse the original number, without leading or trailing 'A's
        let mut code_str = code.iter().collect::<String>();
        code_str.remove(0);
        code_str.remove(code_str.len() - 1);
        let code_num: u64 = code_str.parse().unwrap();

        let mut moves: HashMap<(char, char), u64> = HashMap::new();
        let dirs = code
            .windows(2)
            .map(|w| numeric_to_dir(w))
            .collect::<String>();
        let dirs_v = dirs.chars().collect::<Vec<char>>();
        // Add first layer of moves
        // Start from A, keep track of the first direction separately
        let mut first = dir_to_dir(&['A', dirs_v[0]]).chars().next().unwrap();
        moves.insert(('A', dirs_v[0]), 1);
        for d in dirs_v.windows(2) {
            moves
                .entry((d[0], d[1]))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        for _ in 0..25 {
            let mut tmp = HashMap::new();
            //moves.insert(('A', first), 1);
            for ((from, to), cnt) in &moves {
                // Find the next move given the current (from, to) pair
                let next = dir_to_dir(&[*from, *to]);
                let mut next_moves = next.chars().collect::<Vec<char>>();
                next_moves.insert(0, 'A');
                let new_pairs = next_moves
                    .windows(2)
                    .map(|w| (w[0], w[1]))
                    .collect::<Vec<(char, char)>>();
                for p in new_pairs {
                    tmp.entry(p).and_modify(|e| *e += *cnt).or_insert(*cnt);
                }
            }
            moves = tmp;
            // Update the first direction for the next iteration
            first = dir_to_dir(&['A', first]).chars().next().unwrap();
        }
        // What we have now is the number of movements
        // -> no need to run through dir_to_dir again,
        // just sum the number of occurences
        let part_sum = moves
            .iter()
            .map(|(_, &v)| v)
            .collect::<Vec<u64>>()
            .iter()
            .sum::<u64>();
        moves.clear();
        sum += part_sum * code_num;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day21/test1.txt");
        assert_eq!(result, 126384);
    }

    #[test]
    fn part2() {
        let result = solve_1("src/day21/test2.txt");
        assert_eq!(result, 1972);
    }
}
