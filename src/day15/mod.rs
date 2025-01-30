use aoc2024::util::coord::Coord;
use std::{fmt::Debug, fs};

#[derive(Eq, PartialEq, Clone, Copy)]
enum Object {
    Wall,
    Box,
    Robot,
    Floor,

    BoxLeft,
    BoxRight,
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Object::Wall => "#",     //"🧱",
            Object::Box => "O",      //"📦",
            Object::Robot => "@",    //"🤖",
            Object::Floor => ".",    //"  ",
            Object::BoxLeft => "[",  //"🟨",
            Object::BoxRight => "]", //"🟨",
        };
        write!(f, "{c}")
    }
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn to_coord(&self) -> (isize, isize) {
        match self {
            Instruction::Up => (-1, 0),
            Instruction::Down => (1, 0),
            Instruction::Left => (0, -1),
            Instruction::Right => (0, 1),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Instruction::Up => '▲',
            Instruction::Down => '▼',
            Instruction::Left => '◄',
            Instruction::Right => '►',
        };
        write!(f, "{c}")
    }
}

fn handle_move(map: &mut [Vec<Object>], obj: &mut (usize, usize), instruction: &Instruction) {
    let current_type = map[obj.0][obj.1];

    let mut possible_changes = vec![];
    let mut new_type = current_type;
    let mut coord = *obj;
    match instruction {
        Instruction::Up => {
            while new_type != Object::Wall && new_type != Object::Floor {
                possible_changes.push((map[coord.0][coord.1], coord));
                coord = (coord.0 - 1, coord.1);
                new_type = map[coord.0][coord.1];
            }
        }
        Instruction::Down => {
            while new_type != Object::Wall && new_type != Object::Floor {
                possible_changes.push((map[coord.0][coord.1], coord));
                coord = (coord.0 + 1, coord.1);
                new_type = map[coord.0][coord.1];
            }
        }
        Instruction::Left => {
            while new_type != Object::Wall && new_type != Object::Floor {
                possible_changes.push((map[coord.0][coord.1], coord));
                coord = (coord.0, coord.1 - 1);
                new_type = map[coord.0][coord.1];
            }
        }
        Instruction::Right => {
            while new_type != Object::Wall && new_type != Object::Floor {
                possible_changes.push((map[coord.0][coord.1], coord));
                coord = (coord.0, coord.1 + 1);
                new_type = map[coord.0][coord.1];
            }
        }
    }
    // Traverse the possible moves in reverse
    for object in possible_changes.iter().rev() {
        match instruction {
            Instruction::Up => {
                let next = (object.1 .0 - 1, object.1 .1); // this looks awful
                if map[next.0][next.1] == Object::Floor {
                    // Possible to move
                    map[next.0][next.1] = object.0;
                    map[object.1 .0][object.1 .1] = Object::Floor;
                    if current_type == Object::Robot {
                        *obj = next;
                    }
                }
            }
            Instruction::Down => {
                let next = (object.1 .0 + 1, object.1 .1);
                if map[next.0][next.1] == Object::Floor {
                    // Possible to move
                    map[next.0][next.1] = object.0;
                    map[object.1 .0][object.1 .1] = Object::Floor;
                    if current_type == Object::Robot {
                        *obj = next;
                    }
                }
            }
            Instruction::Left => {
                let next = (object.1 .0, object.1 .1 - 1);
                if map[next.0][next.1] == Object::Floor {
                    // Possible to move
                    map[next.0][next.1] = object.0;
                    map[object.1 .0][object.1 .1] = Object::Floor;
                    if current_type == Object::Robot {
                        *obj = next;
                    }
                }
            }
            Instruction::Right => {
                let next = (object.1 .0, object.1 .1 + 1);
                if map[next.0][next.1] == Object::Floor {
                    // Possible to move
                    map[next.0][next.1] = object.0;
                    map[object.1 .0][object.1 .1] = Object::Floor;
                    if current_type == Object::Robot {
                        *obj = next;
                    }
                }
            }
        }
    }
}

fn handle_move_pt2(map: &mut [Vec<Object>], obj: &mut (usize, usize), instruction: &Instruction) {
    let o = (obj.0 as isize, obj.1 as isize);
    let coord = Coord::from_tuple(o);
    let change = Coord::from_tuple(Instruction::to_coord(instruction));

    let next_coord = coord + change;

    // Slightly different approach due to the need to check two coordinates

    match map[next_coord.x as usize][next_coord.y as usize] {
        Object::Floor => {
            // Just move
            map[next_coord.x as usize][next_coord.y as usize] = Object::Robot;
            map[obj.0][obj.1] = Object::Floor;
            *obj = (next_coord.x as usize, next_coord.y as usize);
        }
        b @ Object::BoxLeft | b @ Object::BoxRight => {
            let mut movable = true;
            let mut possible_changes = vec![coord, next_coord]; // Add robot plus the first half of the box

            if b == Object::BoxLeft {
                // Push the box part on the right
                possible_changes.push(
                    next_coord + Coord::from_tuple(Instruction::to_coord(&Instruction::Right)),
                );
            } else {
                // Push the box part on the left
                possible_changes.push(
                    next_coord + Coord::from_tuple(Instruction::to_coord(&Instruction::Left)),
                );
            }

            match instruction {
                Instruction::Up | Instruction::Down => {
                    let mut move_search = possible_changes.clone();

                    while move_search.len() > 1 {
                        let mut next_search = vec![];
                        // Checking for pairs
                        for m in move_search {
                            let next = m + Coord::from_tuple(Instruction::to_coord(instruction));

                            match map[next.x as usize][next.y as usize] {
                                Object::Wall => {
                                    // Can't move
                                    movable = false;
                                    next_search.clear();
                                }
                                b @ Object::BoxLeft | b @ Object::BoxRight => {
                                    if !possible_changes.contains(&next) {
                                        // Part of a box could've been already added
                                        // See:
                                        //       []
                                        //      [][]  <- here, inner box parts
                                        //       @
                                        next_search.push(next);
                                        possible_changes.push(next);
                                        if b == Object::BoxLeft {
                                            // Add right side
                                            next_search.push(
                                                next + Coord::from_tuple(Instruction::to_coord(
                                                    &Instruction::Right,
                                                )),
                                            );
                                            possible_changes.push(
                                                next + Coord::from_tuple(Instruction::to_coord(
                                                    &Instruction::Right,
                                                )),
                                            );
                                        } else {
                                            // Add left side
                                            next_search.push(
                                                next + Coord::from_tuple(Instruction::to_coord(
                                                    &Instruction::Left,
                                                )),
                                            );
                                            possible_changes.push(
                                                next + Coord::from_tuple(Instruction::to_coord(
                                                    &Instruction::Left,
                                                )),
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        move_search = next_search.clone();
                    }
                }

                Instruction::Left | Instruction::Right => {
                    // This is much easier, the same as pt1 (just the different approach)
                    // skip the first box because it's already added above
                    let mut next = next_coord
                        + Coord::from_tuple(Instruction::to_coord(instruction))
                        + Coord::from_tuple(Instruction::to_coord(instruction));

                    while [Object::BoxLeft, Object::BoxRight]
                        .contains(&map[next.x as usize][next.y as usize])
                    {
                        possible_changes.push(next);
                        next += Coord::from_tuple(Instruction::to_coord(instruction));
                    }
                    // Found the first non-box
                    if map[next.x as usize][next.y as usize] != Object::Floor {
                        movable = false;
                    }
                }
            }
            // Move the possible boxes / robot in reverse order
            if movable {
                for &c in possible_changes.iter().rev() {
                    let to = c + Coord::from_tuple(Instruction::to_coord(instruction));
                    map[to.x as usize][to.y as usize] = map[c.x as usize][c.y as usize];
                    map[c.x as usize][c.y as usize] = Object::Floor;
                }
                *obj = (next_coord.x as usize, next_coord.y as usize);
            }
        }
        _ => {}
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let mut robot = (0, 0);
    let mut map: Vec<Vec<Object>> = data
        .lines()
        .enumerate()
        .take_while(|(_, line)| !line.is_empty())
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '#' => Object::Wall,
                    'O' => Object::Box,
                    '@' => {
                        robot = (row, col);
                        Object::Robot
                    }
                    '.' => Object::Floor,
                    _ => unreachable!("Invalid object"),
                })
                .collect()
        })
        .collect();

    let instructions: Vec<Instruction> = data
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => Instruction::Left,
                '^' => Instruction::Up,
                'v' => Instruction::Down,
                '>' => Instruction::Right,
                _ => unreachable!("Invalid instruction"),
            })
        })
        .collect();

    'outer: for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            if *obj == Object::Robot {
                robot = (i, j);
                break 'outer;
            }
        }
    }

    for i in instructions {
        handle_move(&mut map, &mut robot, &i);

        // Uncomment for visualization
        //
        //print!("{esc}c", esc = 27 as char); // clear screen
        //std::thread::sleep(time::Duration::from_millis(50));
        //for row in map {
        //    for obj in row {
        //        print!("{:?}", obj);
        //    }
        //    println!();
        //}
    }

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            match obj {
                Object::Box => sum += (100 * i + j) as u32,
                _ => continue,
            }
        }
    }

    sum
}

pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut robot = (0, 0);
    let mut map: Vec<Vec<Object>> = Vec::new();
    for (row, line) in data.lines().enumerate() {
        map.push(Vec::new());
        if line.is_empty() {
            break;
        }
        for (col, c) in line.chars().enumerate() {
            let curr = match c {
                '#' => Object::Wall,
                'O' => Object::Box,
                '@' => {
                    robot = (row, col);
                    Object::Robot
                }
                '.' => Object::Floor,
                _ => unreachable!("Invalid object"),
            };
            if curr == Object::Box {
                map[row].push(Object::BoxLeft);
                map[row].push(Object::BoxRight);
            } else if curr == Object::Robot {
                map[row].push(Object::Robot);
                map[row].push(Object::Floor);
            } else {
                map[row].push(curr);
                map[row].push(curr);
            }
        }
    }

    let instructions: Vec<Instruction> = data
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => Instruction::Left,
                '^' => Instruction::Up,
                'v' => Instruction::Down,
                '>' => Instruction::Right,
                _ => unreachable!("Invalid instruction"),
            })
        })
        .collect();

    'outer: for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            if *obj == Object::Robot {
                robot = (i, j);
                break 'outer;
            }
        }
    }

    //print!("{esc}c", esc = 27 as char); // clear screen
    //for row in &map {
    //    for obj in row {
    //        print!("{:?}", obj);
    //    }
    //    println!();
    //}

    for i in instructions {
        handle_move_pt2(&mut map, &mut robot, &i);

        // Uncomment for visualization
        //
        //std::thread::sleep(time::Duration::from_millis(3000));
        //print!("{esc}c", esc = 27 as char); // clear screen
        //for row in &map {
        //    for obj in row {
        //        print!("{:?}", obj);
        //    }
        //    println!();
        //}
    }

    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, obj) in row.iter().enumerate() {
            match obj {
                Object::Box | Object::BoxLeft => sum += (100 * i + j) as u32,
                _ => continue,
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day15/test1.txt");
        assert_eq!(result, 2028);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day15/test2.txt");
        assert_eq!(result, 10092);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day15/test3.txt");
        assert_eq!(result, 618);
    }

    #[test]
    fn part2_2() {
        let result = solve_2("src/day15/test2.txt");
        assert_eq!(result, 9021);
    }
}
