use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    col: isize,
    row: isize,
}

impl Position {
    fn new(col: usize, row: usize) -> Self {
        Self {
            col: col as isize,
            row: row as isize,
        }
    }

    fn validate(&self, max_width: isize, max_height: isize) -> bool {
        self.row >= 0 && self.row < max_height && self.col >= 0 && self.col < max_width
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Self {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            col: self.col + other.col,
            row: self.row + other.row,
        };
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                '.' => {}
                _ => {
                    antennae
                        .entry(c)
                        .and_modify(|v| v.push(Position::new(row, col)))
                        .or_insert(vec![Position::new(row, col)]);
                }
            }
        }
    }
    let (max_width, max_height) = (data.lines().count(), data.lines().next().unwrap().len());
    let mut antinodes: HashSet<Position> = HashSet::new();
    for pos in antennae.values() {
        for pos_pair in pos.iter().combinations(2) {
            let (antenna_1, antenna_2) = (pos_pair[0], pos_pair[1]);
            let dist = *antenna_2 - *antenna_1;

            let possible_anti_1 = *antenna_2 + dist;
            let possible_anti_2 = *antenna_1 - dist;

            if possible_anti_1.validate(max_width as isize, max_height as isize) {
                antinodes.insert(possible_anti_1);
            }
            if possible_anti_2.validate(max_width as isize, max_height as isize) {
                antinodes.insert(possible_anti_2);
            }
        }
    }

    antinodes.len() as u32
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                '.' => {}
                _ => {
                    antennae
                        .entry(c)
                        .and_modify(|v| v.push(Position::new(row, col)))
                        .or_insert(vec![Position::new(row, col)]);
                }
            }
        }
    }
    let (max_width, max_height) = (data.lines().count(), data.lines().next().unwrap().len());
    let mut antinodes: HashSet<Position> = HashSet::new();
    for pos in antennae.values() {
        for pos_pair in pos.iter().combinations(2) {
            let (antenna_1, antenna_2) = (pos_pair[0], pos_pair[1]);
            let dist = *antenna_2 - *antenna_1;

            let mut possible_anti_1 = *antenna_1;
            let mut possible_anti_2 = *antenna_2;

            while possible_anti_1.validate(max_width as isize, max_height as isize) {
                antinodes.insert(possible_anti_1);
                possible_anti_1 += dist;
            }
            while possible_anti_2.validate(max_width as isize, max_height as isize) {
                antinodes.insert(possible_anti_2);
                possible_anti_2 -= dist;
            }
        }
    }

    antinodes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day8/test1.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day8/test2.txt");
        assert_eq!(result, 34);
    }
}
