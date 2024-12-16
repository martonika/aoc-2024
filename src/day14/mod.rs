use std::fs;

use regex::Regex;

type Coord = (usize, usize);
#[cfg(test)]
const MAP: (usize, usize) = (7, 11);
#[cfg(test)]
const MID: (usize, usize) = (3, 5);
#[cfg(not(test))]
const MAP: (usize, usize) = (101, 103);
#[cfg(not(test))]
const MID: (usize, usize) = (50, 51);

const MAX_STEPS: usize = MAP.0 * MAP.1; // 101 and 103 are both primes, their least common multiple is their product
#[derive(Debug)]
struct Robot {
    pos: Coord,
    vel: (isize, isize),
}

impl Robot {
    fn move_robot(&mut self, seconds: usize, map: (usize, usize)) {
        let pos_x = self.pos.0 as isize + self.vel.0 * seconds as isize;
        let pos_y = self.pos.1 as isize + self.vel.1 * seconds as isize;

        self.pos.0 = pos_x.rem_euclid(map.0 as isize) as usize;
        self.pos.1 = pos_y.rem_euclid(map.1 as isize) as usize;
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    for caps in re.captures_iter(data.as_str()) {
        robots.push(Robot {
            pos: (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            ),
            vel: (
                caps[3].parse::<isize>().unwrap(),
                caps[4].parse::<isize>().unwrap(),
            ),
        });
    }

    for r in &mut robots {
        r.move_robot(100, MAP);
    }

    let mut quadrants = [0; 4];
    for r in robots {
        if r.pos.0 == MID.0 || r.pos.1 == MID.1 {
            continue;
        }
        // turns out bool can be converted to usize
        let top_bottom = (r.pos.0 < MID.0) as usize;
        let left_right = (r.pos.1 < MID.1) as usize;
        quadrants[top_bottom * 2 + left_right] += 1;
    }
    quadrants.iter().product()
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    for caps in re.captures_iter(data.as_str()) {
        robots.push(Robot {
            pos: (
                caps[1].parse::<usize>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
            ),
            vel: (
                caps[3].parse::<isize>().unwrap(),
                caps[4].parse::<isize>().unwrap(),
            ),
        });
    }

    // Idea: If there's a valid picture, a lot of pixels are next to each other
    // i.e. search for the minimum entropy
    // Do this by compressing the field as text and check the compressed length
    let mut minimum_entropy = (0, usize::MAX); // (step, entropy)

    // of course, off-by-one error
    for i in 1..=MAX_STEPS {
        let mut field = vec![vec!['.'; MAP.0]; MAP.1];
        for r in &mut robots {
            r.move_robot(1, MAP);
            field[r.pos.1][r.pos.0] = 'X'; // I am indexing the other way around
        }

        let field_str: Vec<u8> = field
            .iter()
            .flat_map(|rows| rows.iter().map(|c| *c as u8).collect::<Vec<_>>())
            .collect();
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&field_str[..], 6);
        if compressed.len() < minimum_entropy.1 {
            minimum_entropy.0 = i as u32;
            minimum_entropy.1 = compressed.len();
        }
    }
    minimum_entropy.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day14/test1.txt");
        assert_eq!(result, 12);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day14/test2.txt");
        assert_eq!(result, 0);
    }
}
