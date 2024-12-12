use std::fs;

#[cfg(not(test))]
const SIZE: usize = 146; // 140x140 chars + 3 padding on each side
#[cfg(test)]
const SIZE: usize = 16; // 10x10 chars + 3 padding on each side

pub fn valid_words_arr(input: &[[char; SIZE]; SIZE]) -> u32 {
    let mut cnt = 0;

    for row in 3..(SIZE - 3) {
        for col in 3..(SIZE - 3) {
            match input[row][col] {
                'X' | 'S' => {
                    // horizontal:
                    let word = [
                        input[row][col],
                        input[row][col + 1],
                        input[row][col + 2],
                        input[row][col + 3],
                    ];
                    match word {
                        ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                        _ => {}
                    }
                    // vertical:
                    let word = [
                        input[row][col],
                        input[row - 1][col],
                        input[row - 2][col],
                        input[row - 3][col],
                    ];
                    match word {
                        ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                        _ => {}
                    }
                    // CW:
                    let word = [
                        input[row][col],
                        input[row - 1][col + 1],
                        input[row - 2][col + 2],
                        input[row - 3][col + 3],
                    ];
                    match word {
                        ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                        _ => {}
                    }
                    // CCW:
                    let word = [
                        input[row][col],
                        input[row - 1][col - 1],
                        input[row - 2][col - 2],
                        input[row - 3][col - 3],
                    ];
                    match word {
                        ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    cnt
}

pub fn valid_words_pt2(input: &[[char; SIZE]; SIZE]) -> u32 {
    let mut cnt = 0;

    for row in 3..(SIZE - 3) {
        for col in 3..(SIZE - 3) {
            if input[row][col] == 'A' {
                let chars = [
                    input[row - 1][col - 1],
                    input[row - 1][col + 1],
                    input[row + 1][col - 1],
                    input[row + 1][col + 1],
                ];
                match chars {
                    ['M', 'M', 'S', 'S']
                    | ['S', 'S', 'M', 'M']
                    | ['M', 'S', 'M', 'S']
                    | ['S', 'M', 'S', 'M'] => cnt += 1,
                    _ => {}
                }
            }
        }
    }

    cnt
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    // idea:
    // 1. create padding - 4 lines at the start, 4 at the end, 4 on each side
    // 2. remove newlines, create a very long char array
    // 3. find "X" or "S"
    // 4. check letters:
    //    - 90 degrees up/down: position - N*SIZE should be either "M" "A" "S" or "A" "M" "X"
    //    - 45 degrees CW:      position - N*(SIZE-1)
    //    - 45 degrees CCW:     position - N*(SIZE+1)
    // create a 2D array
    let mut arr = [['.'; SIZE]; SIZE];
    let mut j = 3;
    for line in data.lines() {
        for (i, c) in line.char_indices() {
            arr[j][i + 3] = c
        }
        j += 1;
    }

    valid_words_arr(&arr)
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    // even easier
    // check for 'A' then check surroundings
    let mut arr = [['.'; SIZE]; SIZE];
    let mut j = 3;
    for line in data.lines() {
        for (i, c) in line.char_indices() {
            arr[j][i + 3] = c
        }
        j += 1;
    }

    valid_words_pt2(&arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = solve_1("src/day4/test1.txt");
        assert_eq!(result, 18);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day4/test2.txt");
        assert_eq!(result, 9);
    }
}
