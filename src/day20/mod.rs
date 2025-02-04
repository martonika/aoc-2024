use std::{collections::BTreeMap, fs, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

fn dfs(maze: &[Vec<TileType>], start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut stack = vec![start];
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    visited[start.0][start.1] = true;
    let mut path = vec![(start.0, start.1)];
    while let Some(current) = stack.pop() {
        if current == end {
            // Visualization
            //for row in maze {
            //    for tile in row {
            //        if path.contains(&(tile.coord.0, tile.coord.1)) {
            //            print!("\u{1b}[0;33m");
            //        } else {
            //            print!("\u{1b}[0;31m");
            //        }
            //        print!(
            //            "{:0>2} ",
            //            path.iter()
            //                .position(|x| x == &(tile.coord.0, tile.coord.1))
            //                .unwrap_or(99)
            //        );
            //    }
            //    println!();
            //}
            //print!("\u{1b}[0;37m");
            return path;
        }
        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let x = current.0 as i32 + dir.0;
            let y = current.1 as i32 + dir.1;
            let new_coord = (x, y);
            if new_coord.0 < 0
                || new_coord.0 >= maze.len() as i32
                || new_coord.1 < 0
                || new_coord.1 >= maze[0].len() as i32
                || visited[new_coord.0 as usize][new_coord.1 as usize]
                || maze[new_coord.0 as usize][new_coord.1 as usize] == TileType::Wall
            {
                continue;
            }
            visited[new_coord.0 as usize][new_coord.1 as usize] = true;
            path.push((new_coord.0 as usize, new_coord.1 as usize));
            stack.push((new_coord.0 as usize, new_coord.1 as usize));
        }
    }
    unreachable!();
}

fn search_shortcuts(path: &[(usize, usize)], cheat_len: usize) -> BTreeMap<u64, u64> {
    let mut shortcuts = BTreeMap::new();
    for (start_time, tile) in path.iter().enumerate() {
        for (end_time, t) in path.iter().enumerate().skip(start_time + cheat_len - 1) {
            let manhattan_dist = tile.0.abs_diff(t.0) + tile.1.abs_diff(t.1);
            if manhattan_dist > cheat_len {
                continue;
            }
            let mut gained_time = 0;
            if end_time - start_time > manhattan_dist {
                gained_time = end_time - start_time - manhattan_dist;
            }
            if gained_time > 0 {
                shortcuts
                    .entry(gained_time as u64)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    shortcuts
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut start = (0, 0);
    let mut end = (0, 0);
    let maze = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => TileType::Wall,
                    '.' => TileType::Floor,
                    'S' => {
                        start = (row, col);
                        TileType::Floor
                    }
                    'E' => {
                        end = (row, col);
                        TileType::Floor
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<TileType>>()
        })
        .collect::<Vec<Vec<TileType>>>();

    // Build original route
    let path = dfs(&maze, start, end);

    let shortcuts = search_shortcuts(&path, 2);

    shortcuts
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| *v)
        .sum()
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut start = (0, 0);
    let mut end = (0, 0);
    let maze = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => TileType::Wall,
                    '.' => TileType::Floor,
                    'S' => {
                        start = (row, col);
                        TileType::Floor
                    }
                    'E' => {
                        end = (row, col);
                        TileType::Floor
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<TileType>>()
        })
        .collect::<Vec<Vec<TileType>>>();

    // Build original route
    let path = dfs(&maze, start, end);

    let shortcuts = search_shortcuts(&path, 20);

    shortcuts
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| *v)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day20/test1.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day20/test1.txt");
        assert_eq!(result, 0);
    }
}
