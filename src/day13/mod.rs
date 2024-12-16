use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Equation {
    a: (u64, u64),
    b: (u64, u64),
    target: (u64, u64),
}

fn calculate(eq: &Equation) -> Option<(u64, u64)> {
    // Linear algebra, my beloved
    // (not.)
    // Cramer's rule
    // N * a1 + M * b1 = t1
    // N * a2 + M * b2 = t2
    //         [N]
    //         [M]
    // [a1 b1]     =[t1]
    // [a2 b2]     =[t2]
    // N = (b2*t1 - b1*t2) / (a1*b2 - b1*a2)
    // M = (a1*t2 - a2*t1) / (a1*b2 - b1*a2)
    let div = eq.a.0 as f64 * eq.b.1 as f64 - eq.b.0 as f64 * eq.a.1 as f64;
    let n_f = (eq.b.1 as f64 * eq.target.0 as f64 - eq.b.0 as f64 * eq.target.1 as f64) / div;
    let m_f = (eq.a.0 as f64 * eq.target.1 as f64 - eq.a.1 as f64 * eq.target.0 as f64) / div;

    // If the results are not integers, it's not possible to reach the target
    if n_f % 1f64 != 0f64 || m_f % 1f64 != 0f64 {
        return None;
    }
    let n = n_f as u64;
    let m = m_f as u64;

    Some((n, m))
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let binding = data.replace("\r\n", "\n");
    let filtered = binding.as_str();
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut eqs = vec![];
    for caps in re.captures_iter(filtered) {
        eqs.push(Equation {
            a: (
                caps[1].parse::<u64>().unwrap(),
                caps[2].parse::<u64>().unwrap(),
            ),
            b: (
                caps[3].parse::<u64>().unwrap(),
                caps[4].parse::<u64>().unwrap(),
            ),
            target: (
                caps[5].parse::<u64>().unwrap(),
                caps[6].parse::<u64>().unwrap(),
            ),
        });
    }

    let presses: Vec<_> = eqs.iter().map(calculate).collect();

    presses
        .iter()
        .filter(|press| {
            press.is_some_and(|(n, m)|
                // Filter too long plays
                n <= 100 || m <= 100)
        })
        .map(|press| match press {
            Some(p) => 3 * p.0 + p.1,
            None => 0,
        })
        .sum()
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let binding = data.replace("\r\n", "\n");
    let filtered = binding.as_str();
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut eqs = vec![];
    for caps in re.captures_iter(filtered) {
        eqs.push(Equation {
            a: (
                caps[1].parse::<u64>().unwrap(),
                caps[2].parse::<u64>().unwrap(),
            ),
            b: (
                caps[3].parse::<u64>().unwrap(),
                caps[4].parse::<u64>().unwrap(),
            ),
            target: (
                caps[5].parse::<u64>().unwrap() + 10000000000000,
                caps[6].parse::<u64>().unwrap() + 10000000000000,
            ),
        });
    }

    let presses: Vec<_> = eqs.iter().map(calculate).collect();

    // Not filtering <= 100 presses anymore
    presses
        .iter()
        .map(|press| match press {
            Some(p) => 3 * p.0 + p.1,
            None => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day13/test1.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day13/test2.txt");
        assert_eq!(result, 0);
    }
}
