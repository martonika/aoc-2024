use std::fs;

// todo: find a better name -.-

fn is_safe(levels: &[u32]) -> bool {
    let is_increasing = levels[0] < levels[1];
    levels[..]
        .windows(2)
        .all(|w| w[0] != w[1] && w[0].abs_diff(w[1]) <= 3 && (w[0] < w[1]) == is_increasing)
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let mut safe_reports = 0;

    for line in data.lines() {
        let levels: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if is_safe(&levels[..]) {
            safe_reports += 1;
        }
    }

    safe_reports
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    // The same as in solve_1, but a bit less readable
    data.lines()
        .filter(|line| {
            let levels = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            // The OG function
            is_safe(&levels[..])
            // Or brute force through removing an index and checking that variant
                || (0..levels.len()).any(|i| {
                    let mut variant = levels.clone();
                    variant.remove(i);
                    is_safe(&variant[..])}
                )
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day2/test1.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day2/test2.txt");
        assert_eq!(result, 4);
    }
}
