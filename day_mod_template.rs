use std::fs;

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    0
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day1/test1.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day1/test2.txt");
        assert_eq!(result, 281);
    }
}
