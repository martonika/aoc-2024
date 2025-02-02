use std::{collections::HashMap, fs};

fn possible_designs<'a>(
    pattern: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if pattern.is_empty() {
        // If the remaining pattern is empty, we found one way to get the desired pattern
        return 1;
    }

    if let Some(&cached) = cache.get(pattern) {
        return cached;
    }

    // Check all towels, see if the current pattern starts with them
    // If it does, call the function recursively with the remaining part of the pattern
    let sum = towels
        .iter()
        .filter(|&towel| pattern.starts_with(towel))
        .map(|&towel| possible_designs(&pattern[towel.len()..], towels, cache))
        .sum();
    // Save the number of possible ways to get to an actual pattern in the cache
    cache.insert(pattern, sum);

    sum
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let (towels_str, designs_str) = data.split_once("\r\n\r\n").unwrap();
    let towels: Vec<&str> = towels_str.split(", ").collect();
    let designs: Vec<&str> = designs_str.lines().collect();
    let mut cache = HashMap::new();

    // Iterate through all wanted designs
    // Filter out anything that's not possible
    // For pt1, we simply count how many designs are possible
    designs
        .iter()
        .filter(|design| possible_designs(design, &towels, &mut cache) != 0)
        .count() as u64
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let (towels_str, designs_str) = data.split_once("\r\n\r\n").unwrap();
    let towels: Vec<&str> = towels_str.split(", ").collect();
    let designs: Vec<&str> = designs_str.lines().collect();
    let mut cache = HashMap::new();

    // Iterate through all wanted designs
    // For pt2, sum all possibilities
    designs
        .iter()
        .map(|design| possible_designs(design, &towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day19/test1.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day19/test2.txt");
        assert_eq!(result, 0);
    }
}
