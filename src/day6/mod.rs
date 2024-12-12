use std::{collections::HashSet, fmt::Debug, fs};

// Totally unnecessary enums for everything!
#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Orientation {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

#[derive(Clone)]
pub enum FloorType {
    Safe,
    Lava, // Visited by the guard
}

#[derive(Clone)]
pub enum Tile {
    Floor(FloorType),
    Obstacle,
    Guard(Orientation),
}

#[derive(PartialEq)]
pub enum Route {
    InProgress,
    Ended,
    Loop,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor(FloorType::Safe),
            'X' => Self::Floor(FloorType::Lava),
            '#' => Self::Obstacle,
            '^' => Self::Guard(Orientation::Up),
            '>' => Self::Guard(Orientation::Right),
            '<' => Self::Guard(Orientation::Left),
            'v' => Self::Guard(Orientation::Down),
            _ => panic!("Invalid character!"),
        }
    }

    pub fn turn(&mut self) {
        if let Tile::Guard(o) = self {
            match o {
                Orientation::Up => *o = Orientation::Right,
                Orientation::Down => *o = Orientation::Left,
                Orientation::Left => *o = Orientation::Up,
                Orientation::Right => *o = Orientation::Down,
            }
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Floor(floor_type) => match floor_type {
                FloorType::Safe => write!(f, "."),
                FloorType::Lava => write!(f, "X"),
            },
            Tile::Obstacle => write!(f, "#"),
            Tile::Guard(orientation) => match orientation {
                Orientation::Up => write!(f, "^"),
                Orientation::Down => write!(f, "v"),
                Orientation::Left => write!(f, "<"),
                Orientation::Right => write!(f, ">"),
            },
        }
    }
}

pub fn step(
    map: &mut [Vec<Tile>],
    mut path: Option<&mut HashSet<(usize, usize, Orientation)>>,
) -> Route {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let c = &map[row][col];
            match c {
                Tile::Guard(orientation) => {
                    //let mut new_map = map.to_vec();
                    match orientation {
                        Orientation::Up => {
                            if row == 0 {
                                if let Some(ref mut p) = path {
                                    p.insert((row, col, Orientation::Up));
                                }
                                map[row][col] = Tile::Floor(FloorType::Lava);
                                return Route::Ended;
                            } else {
                                let front = &map[row - 1][col];
                                match front {
                                    Tile::Floor(_) => {
                                        if let Some(ref mut p) = path {
                                            if !p.insert((row, col, Orientation::Up)) {
                                                return Route::Loop;
                                            }
                                        }
                                        map[row - 1][col] = Tile::Guard(Orientation::Up);
                                        map[row][col] = Tile::Floor(FloorType::Lava);
                                    }
                                    Tile::Obstacle => map[row][col].turn(),
                                    Tile::Guard(_) => panic!("Guard in front of guard?"),
                                }
                            }
                        }
                        Orientation::Down => {
                            if row == map.len() - 1 {
                                if let Some(ref mut p) = path {
                                    p.insert((row, col, Orientation::Down));
                                }
                                map[row][col] = Tile::Floor(FloorType::Lava);
                                return Route::Ended;
                            } else {
                                let front = &map[row + 1][col];
                                match front {
                                    Tile::Floor(_) => {
                                        if let Some(ref mut p) = path {
                                            if !p.insert((row, col, Orientation::Down)) {
                                                return Route::Loop;
                                            }
                                        }
                                        map[row + 1][col] = Tile::Guard(Orientation::Down);
                                        map[row][col] = Tile::Floor(FloorType::Lava);
                                    }
                                    Tile::Obstacle => map[row][col].turn(),
                                    Tile::Guard(_) => panic!("Guard in front of guard?"),
                                }
                            }
                        }
                        Orientation::Left => {
                            if col == 0 {
                                if let Some(ref mut p) = path {
                                    p.insert((row, col, Orientation::Left));
                                }
                                map[row][col] = Tile::Floor(FloorType::Lava);
                                return Route::Ended;
                            } else {
                                let front = &map[row][col - 1];
                                match front {
                                    Tile::Floor(_) => {
                                        if let Some(ref mut p) = path {
                                            if !p.insert((row, col, Orientation::Left)) {
                                                return Route::Loop;
                                            }
                                        }
                                        map[row][col - 1] = Tile::Guard(Orientation::Left);
                                        map[row][col] = Tile::Floor(FloorType::Lava);
                                    }
                                    Tile::Obstacle => map[row][col].turn(),
                                    Tile::Guard(_) => panic!("Guard in front of guard?"),
                                }
                            }
                        }
                        Orientation::Right => {
                            if col == map[row].len() - 1 {
                                if let Some(ref mut p) = path {
                                    p.insert((row, col, Orientation::Right));
                                }
                                map[row][col] = Tile::Floor(FloorType::Lava);
                                return Route::Ended;
                            } else {
                                let front = &map[row][col + 1];
                                match front {
                                    Tile::Floor(_) => {
                                        if let Some(ref mut p) = path {
                                            if !p.insert((row, col, Orientation::Right)) {
                                                return Route::Loop;
                                            }
                                        }
                                        map[row][col + 1] = Tile::Guard(Orientation::Right);
                                        map[row][col] = Tile::Floor(FloorType::Lava);
                                    }
                                    Tile::Obstacle => map[row][col].turn(),
                                    Tile::Guard(_) => panic!("Guard in front of guard?"),
                                }
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }
    Route::InProgress
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut map: Vec<Vec<Tile>> = data
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    while step(&mut map, None) == Route::InProgress {
        //for line in &map {
        //    println!("{:?}", line);
        //}
        //println!();
    }

    let visited: Vec<&Tile> = map
        .iter()
        .flatten()
        .filter(|c| matches!(c, Tile::Floor(FloorType::Lava)))
        .collect();

    visited.len() as u32
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let original_map: Vec<Vec<Tile>> = data
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    let mut map = original_map.clone();
    // Do a pass of pt1 to get the path of the guard
    let mut path: HashSet<(usize, usize, Orientation)> = HashSet::new();
    while step(&mut map, Some(&mut path)) == Route::InProgress {
        //for line in &map {
        //    println!("{:?}", line);
        //}
        //println!();
    }
    // Remove the guard's starting point
    for (row, _) in original_map.iter().enumerate() {
        for col in 0..original_map[row].len() {
            if let Tile::Guard(_) = original_map[row][col] {
                // Remove all possible movements through the starting point
                path.remove(&(row, col, Orientation::Up));
                path.remove(&(row, col, Orientation::Down));
                path.remove(&(row, col, Orientation::Left));
                path.remove(&(row, col, Orientation::Right));
            }
        }
    }

    // Create another set to filter out possible duplicates due to same coordinates, different orientation
    let filtered_path: HashSet<(usize, usize)> = path.iter().map(|p| (p.0, p.1)).collect();

    // Now we have a set of possible coordinates for obstacles
    let mut cnt = 0;

    for (x, y) in filtered_path {
        let mut new_map = original_map.clone();
        let mut new_path: HashSet<(usize, usize, Orientation)> = HashSet::new();

        new_map[x][y] = Tile::Obstacle;
        loop {
            match step(&mut new_map, Some(&mut new_path)) {
                Route::InProgress => continue,
                Route::Ended => break,
                Route::Loop => {
                    cnt += 1;
                    break;
                }
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day6/test1.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day6/test2.txt");
        assert_eq!(result, 6);
    }
}
