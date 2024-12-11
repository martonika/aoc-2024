use std::fs;

fn calc_possible_combinations(
    numbers: &[u64],
    result: u64,
    combinations: &mut Vec<u64>,
    pt2: bool,
) {
    // 'numbers' holds the remaining possible numbers
    // 'result' holds the current result by the actual operation chain
    // 'combinations' holds the possible end values of the operations
    if numbers.is_empty() {
        // If there are no remaining numbers, we got to the end of the operation chain
        combinations.push(result);
        return;
    }

    let next_batch = &numbers[1..];
    let next_sum = numbers[0];

    // Multiplication
    calc_possible_combinations(next_batch, next_sum * result, combinations, pt2);
    // Addition
    calc_possible_combinations(next_batch, next_sum + result, combinations, pt2);

    if pt2 {
        // Concatenation
        // Voodoo magic
        let conc = result * 10u64.pow(next_sum.ilog10() + 1) + next_sum;
        calc_possible_combinations(next_batch, conc, combinations, pt2);
    }
}

fn calculate(test_val: &u64, nums: &Vec<u64>, pt2: bool) -> u64 {
    let mut combinations = Vec::new();
    calc_possible_combinations(&nums[1..], nums[0], &mut combinations, pt2);

    if combinations.contains(test_val) {
        return *test_val;
    }
    0
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    data.lines()
        .map(|line| {
            let l: Vec<&str> = line.split(':').collect();
            let test_value = l[0].parse::<u64>().unwrap();
            let nums_str: Vec<&str> = l[1].trim().split_whitespace().collect();
            let nums: Vec<u64> = nums_str.iter().map(|s| s.parse::<u64>().unwrap()).collect();

            calculate(&test_value, &nums, false)
        })
        .sum()
}
pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    data.lines()
        .map(|line| {
            let l: Vec<&str> = line.split(':').collect();
            let test_value = l[0].parse::<u64>().unwrap();
            let nums_str: Vec<&str> = l[1].trim().split_whitespace().collect();
            let nums: Vec<u64> = nums_str.iter().map(|s| s.parse::<u64>().unwrap()).collect();

            calculate(&test_value, &nums, true)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day7/test1.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day7/test2.txt");
        assert_eq!(result, 11387);
    }
}
