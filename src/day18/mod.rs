use aoc2024::util::coord::Coord;
#[allow(unused_imports)]
use std::{fs, time};

// These are inclusive
#[cfg(test)]
const SIZE: usize = 6;
#[cfg(not(test))]
const SIZE: usize = 70;

fn astar(start: Coord, end: Coord, map: &[[bool; SIZE + 1]; SIZE + 1]) -> Option<Vec<Coord>> {
    let mut open_set = vec![start];
    let mut came_from = vec![vec![Coord::new(0, 0); SIZE + 1]; SIZE + 1];
    let mut g_score = vec![vec![usize::MAX; SIZE + 1]; SIZE + 1];
    let mut f_score = vec![vec![usize::MAX; SIZE + 1]; SIZE + 1];
    g_score[start.x as usize][start.y as usize] = 0;
    f_score[start.x as usize][start.y as usize] = start.distance(&end);

    while !open_set.is_empty() {
        let binding = open_set.clone();
        let current = binding
            .iter()
            .min_by_key(|&coord| f_score[coord.x as usize][coord.y as usize])
            .unwrap();
        if *current == end {
            let mut path = vec![*current];
            let mut current = *current;
            while current != start {
                current = came_from[current.x as usize][current.y as usize];
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }

        open_set.retain(|&coord| coord != *current);
        for neighbor in current.neighbors() {
            if neighbor.x < 0
                || neighbor.x > SIZE as isize
                || neighbor.y < 0
                || neighbor.y > SIZE as isize
            {
                continue;
            }
            if !map[neighbor.x as usize][neighbor.y as usize] {
                continue;
            }
            let tentative_g_score = g_score[current.x as usize][current.y as usize] + 1;
            if tentative_g_score < g_score[neighbor.x as usize][neighbor.y as usize] {
                came_from[neighbor.x as usize][neighbor.y as usize] = *current;
                g_score[neighbor.x as usize][neighbor.y as usize] = tentative_g_score;
                f_score[neighbor.x as usize][neighbor.y as usize] =
                    tentative_g_score + neighbor.distance(&end);
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    None
}

pub fn solve_1(input: &str) -> usize {
    #[cfg(test)]
    const FALLEN_BYTES: usize = 12;
    #[cfg(not(test))]
    const FALLEN_BYTES: usize = 1024;
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut map = [[true; SIZE + 1]; SIZE + 1];
    for line in data.lines().take(FALLEN_BYTES) {
        let coords: Vec<&str> = line.split(',').collect();
        let (x, y) = (
            coords[0].parse::<usize>().unwrap(),
            coords[1].parse::<usize>().unwrap(),
        );
        // The given coordinates are y,x in our case
        map[y][x] = false;
    }

    let start = Coord::new(0, 0);
    let end = Coord::new(SIZE as isize, SIZE as isize);
    let path = astar(start, end, &map).unwrap();

    path.len() - 1
}

pub fn solve_2(input: &str) -> (usize, usize) {
    #[cfg(test)]
    const FALLEN_BYTES: usize = 12;
    #[cfg(not(test))]
    const FALLEN_BYTES: usize = 1024;
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut map = [[true; SIZE + 1]; SIZE + 1];
    for line in data.lines().take(FALLEN_BYTES) {
        let coords: Vec<&str> = line.split(',').collect();
        let (x, y) = (
            coords[0].parse::<usize>().unwrap(),
            coords[1].parse::<usize>().unwrap(),
        );
        // The given coordinates are y,x in our case
        map[y][x] = false;
    }

    let start = Coord::new(0, 0);
    let end = Coord::new(SIZE as isize, SIZE as isize);
    let mut lines = data.lines().skip(FALLEN_BYTES);
    let mut next_line: Vec<&str>;
    let (mut x, mut y) = (0, 0);
    let mut next_coord;

    while let Some(coords) = astar(start, end, &map) {
        next_line = lines.next().unwrap().split(',').collect();
        (x, y) = (
            next_line[0].parse::<usize>().unwrap(),
            next_line[1].parse::<usize>().unwrap(),
        );
        map[y][x] = false;
        next_coord = Coord::new(y as isize, x as isize);

        while !coords.contains(&next_coord) {
            // Skip re-searching all paths if the fallen coordinate is not on our route
            next_line = lines.next().unwrap().split(',').collect();
            (x, y) = (
                next_line[0].parse::<usize>().unwrap(),
                next_line[1].parse::<usize>().unwrap(),
            );
            map[y][x] = false;
            next_coord = Coord::new(y as isize, x as isize);

            // Visualization
            //std::thread::sleep(time::Duration::from_millis(2000));
            //print!("{esc}c", esc = 27 as char); // clear screen
            //for (yy, row) in map.iter().enumerate() {
            //    for (xx, tile) in row.iter().enumerate() {
            //        if yy == y && xx == x {
            //            print!("\u{1b}[0;33m");
            //            print!("!");
            //        } else if coords.contains(&Coord::new(yy as isize, xx as isize)) {
            //            print!("\u{1b}[0;32m");
            //            print!("X");
            //        } else if *tile {
            //            print!("\u{1b}[0;37m");
            //            print!(".");
            //        } else {
            //            print!("\u{1b}[0;31m");
            //            print!("#");
            //        }
            //    }
            //    println!();
            //}
        }

        // Visualization
        //std::thread::sleep(time::Duration::from_millis(2000));
        //print!("{esc}c", esc = 27 as char); // clear screen
        //for (yy, row) in map.iter().enumerate() {
        //    for (xx, tile) in row.iter().enumerate() {
        //        if yy == y && xx == x {
        //            print!("\u{1b}[0;33m");
        //            print!("!");
        //        } else if coords.contains(&Coord::new(yy as isize, xx as isize)) {
        //            print!("\u{1b}[0;32m");
        //            print!("X");
        //        } else if *tile {
        //            print!("\u{1b}[0;37m");
        //            print!(".");
        //        } else {
        //            print!("\u{1b}[0;31m");
        //            print!("#");
        //        }
        //    }
        //    println!();
        //}
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day18/test1.txt");
        assert_eq!(result, 22);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day18/test2.txt");
        assert_eq!(result, (6, 1));
    }
}
