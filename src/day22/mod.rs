use std::{
    collections::{HashMap, HashSet},
    fs,
};

const PRUNE: u64 = 16777216;

fn calculate_secrets(start: &u64, count: u64) -> u64 {
    let mut next_secret = *start;
    for _ in 0..count {
        let first_pass = next_secret * 64;
        next_secret ^= first_pass;
        next_secret %= PRUNE;

        let second_pass: u64 = next_secret / 32;
        next_secret ^= second_pass;
        next_secret %= PRUNE;

        let third_pass = next_secret * 2048;
        next_secret ^= third_pass;
        next_secret %= PRUNE;
    }
    next_secret
}

fn calculate_price(start: &mut u64) -> i8 {
    let mut price = *start;

    let first_pass = price * 64;
    price ^= first_pass;
    price %= PRUNE;

    let second_pass: u64 = price / 32;
    price ^= second_pass;
    price %= PRUNE;

    let third_pass = price * 2048;
    price ^= third_pass;
    price %= PRUNE;
    *start = price;
    (price % 10) as i8 // Get last digit
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let initial_secrets = data
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    initial_secrets
        .iter()
        .map(|s| calculate_secrets(s, 2000))
        .sum()
}

pub fn solve_2(input: &str) -> i64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let initial_secrets = data
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    // Collect sequence -> price data in a map
    let mut price_map = HashMap::new();

    for mut secret in initial_secrets {
        // Keep track of found sequences at new buyers
        let mut sequence_found: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        // The first value is the initial secret's last digit
        let start = (secret % 10) as i8;
        let mut seq = (0..2000)
            .map(|_| calculate_price(&mut secret))
            .collect::<Vec<i8>>();
        seq.insert(0, start);

        for w in seq.windows(5) {
            let diffs = (w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]);
            let price = w[4];
            if sequence_found.insert(diffs) {
                // If we find a sequence again, that price will never be reached
                // because "after the hiding spot is sold, the monkey will move on to the next buyer"
                // So we only need to update a sequence if it's the first occurence at another vendor
                price_map
                    .entry(diffs)
                    .and_modify(|e| *e += price as i64)
                    .or_insert(price as i64);
            }
        }
    }
    *price_map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day22/test1.txt");
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day22/test2.txt");
        assert_eq!(result, 23);
    }
}
