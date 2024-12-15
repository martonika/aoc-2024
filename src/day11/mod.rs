use std::{collections::HashMap, fs};

fn blink(stones: &mut Vec<u64>) {
    for (i, stone) in stones.clone().iter().enumerate() {
        let digits = (*stone as f64 + 0.1).log10().ceil() as u64;
        if *stone == 0 {
            stones[i] = 1;
        } else if digits % 2 == 0 {
            let temp = stone.to_string();
            // we know it's even
            let (t1, t2) = temp.split_at(temp.len() / 2);
            let new_1 = t1.parse::<u64>().unwrap();
            let new_2 = t2.parse::<u64>().unwrap();
            stones[i] = new_2;
            stones.push(new_1);
        } else {
            stones[i] = stone * 2024;
        }
    }
}

fn blink_2(stones: &mut HashMap<u64, u64>) {
    for stone in stones.clone().iter() {
        if *stone.1 == 0 {
            continue;
        }
        let num = *stone.0;
        let cnt = *stone.1;
        let digits = (num as f64 + 0.1).log10().ceil() as u64; // Doesn't work for 0 but that's not a problem

        if *stone.0 == 0 {
            // "move" 1 stone from 0 to 1
            stones.entry(0).and_modify(|d| *d -= cnt);
            stones.entry(1).and_modify(|d| *d += cnt).or_insert(cnt);
        } else if digits % 2 == 0 {
            let temp = stone.0.to_string();
            // we know it's even
            let (t1, t2) = temp.split_at(temp.len() / 2);
            let new_1 = t1.parse::<u64>().unwrap();
            let new_2 = t2.parse::<u64>().unwrap();
            stones.entry(new_1).and_modify(|d| *d += cnt).or_insert(cnt);
            stones.entry(new_2).and_modify(|d| *d += cnt).or_insert(cnt);
            stones
                .entry(*stone.0)
                .and_modify(|d| *d -= cnt)
                .or_insert(0);
        } else {
            stones.entry(num).and_modify(|d| *d -= cnt);
            stones
                .entry(num * 2024)
                .and_modify(|d| *d += cnt)
                .or_insert(cnt);
        }
    }
}

pub fn solve_1(input: &str, blinks: u64) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    // Contrary to the puzzle text, the order is not important at all
    let mut stones: Vec<u64> = data
        .lines()
        .flat_map(|line| {
            let nums = line.split_ascii_whitespace();
            nums.map(|num| num.parse::<u64>().unwrap())
        })
        .collect();
    for _ in 1..=blinks {
        blink(&mut stones);
    }
    stones.len() as u64
}
pub fn solve_2(input: &str, blinks: u64) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let mut stones: HashMap<u64, u64> = HashMap::new();
    for line in data.lines() {
        for num in line.split_ascii_whitespace() {
            stones
                .entry(num.parse::<u64>().unwrap())
                .and_modify(|d| *d += 1)
                .or_insert(1);
        }
    }
    for _ in 1..=blinks {
        blink_2(&mut stones);
    }
    let mut stones_filtered: HashMap<u64, u64> = HashMap::new();
    for stone in stones.iter().filter(|s| *s.1 > 0) {
        stones_filtered.insert(*stone.0, *stone.1);
    }

    stones_filtered.iter().map(|s| s.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day11/test1.txt", 1);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day11/test2.txt", 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn part1_3() {
        let result = solve_1("src/day11/test2.txt", 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day11/test1.txt", 1);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_2() {
        let result = solve_2("src/day11/test2.txt", 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn part2_3() {
        let result = solve_2("src/day11/test2.txt", 25);
        assert_eq!(result, 55312);
    }
}
