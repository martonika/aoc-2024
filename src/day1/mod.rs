use std::{collections::HashMap, fs};

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut nums_1 = vec![];
    let mut nums_2 = vec![];

    for line in data.lines() {
        let mut iter = line.split_whitespace();
        nums_1.push(iter.next().unwrap().parse::<u32>().unwrap());
        nums_2.push(iter.next().unwrap().parse::<u32>().unwrap());
    }
    nums_1.sort();
    nums_2.sort();

    let nums: Vec<_> = nums_1
        .iter()
        .zip(nums_2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .collect();

    nums.iter().sum()
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut nums_1 = vec![];
    let mut similarity: HashMap<u32, u32> = HashMap::new();

    for line in data.lines() {
        let mut iter = line.split_whitespace();
        nums_1.push(iter.next().unwrap().parse::<u32>().unwrap());
        let sim = iter.next().unwrap().parse::<u32>().unwrap();
        *similarity.entry(sim).or_insert(0) += 1;
    }

    let nums: Vec<u32> = nums_1
        .iter()
        .map(|v| v * similarity.get(v).unwrap_or(&0))
        .collect();

    nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day1/test1.txt");
        assert_eq!(result, 11);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day1/test2.txt");
        assert_eq!(result, 31);
    }
}
