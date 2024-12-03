use std::fs;

use regex::Regex;
extern crate regex;

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum_muls: u32 = re
        .captures_iter(&data)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
        })
        .sum();
    sum_muls
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap(); // Capture all possibilities to know the order
    let re_inner = Regex::new(r"\((\d+),(\d+)\)").unwrap(); // Inner regex just for the "mul(a,b)" matches
    let captures: Vec<&str> = re.find_iter(&data).map(|cap| cap.as_str()).collect();

    let mut enabled = true;
    let mut sum = 0;
    for cap in captures {
        match cap {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let (_, [f, s]) = re_inner.captures(cap).unwrap().extract();
                    sum += f.parse::<u32>().unwrap() * s.parse::<u32>().unwrap()
                };
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day3/test1.txt");
        assert_eq!(result, 161);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day3/test2.txt");
        assert_eq!(result, 48);
    }
}
