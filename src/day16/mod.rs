use std::cmp::{Ordering, Reverse};
// It's that time of the year
// Hey Dijkstra
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::Add;

use aoc2024::util::coord::Coord;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Object {
    Wall,
    Path,
    Goal,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_dir(diff: &Coord) -> Option<Self> {
        match diff {
            Coord { x: 0, y: 1 } => Some(Self::Right),
            Coord { x: 0, y: -1 } => Some(Self::Left),
            Coord { x: 1, y: 0 } => Some(Self::Down),
            Coord { x: -1, y: 0 } => Some(Self::Up),
            _ => None,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

const DIRS: [Coord; 4] = [
    Coord { x: 0, y: -1 },
    Coord { x: 0, y: 1 },
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
];

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    position: Coord,
    dir: Direction,
    tiles: Vec<Coord>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Coord {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

// Dijkstra's shortest path algorithm.
fn shortest_path(
    map: &[Vec<Object>],
    start: Coord,
    goal: Coord,
    direction: Direction,
) -> (usize, HashMap<usize, Vec<Vec<Coord>>>) {
    let mut heap = BinaryHeap::new();
    //let mut visited = HashSet::new();
    let mut distances: HashMap<(Coord, Direction), usize> = HashMap::new();
    let mut shortest = usize::MAX;
    let mut all_tiles = HashMap::new();

    // We're at start, with a zero cost
    heap.push(Reverse(Node {
        cost: 0,
        position: start,
        dir: direction,
        tiles: vec![start],
    }));

    while let Some(Reverse(Node {
        cost,
        position,
        dir,
        tiles,
    })) = heap.pop()
    {
        //visited.insert((position, dir));
        if position == goal {
            all_tiles
                .entry(cost)
                .and_modify(|t: &mut Vec<Vec<Coord>>| t.push(tiles.clone()))
                .or_insert(vec![tiles.clone()]);
            shortest = shortest.min(cost);
            continue;
        }

        if let Some(known_cost) = distances.get(&(position, dir)) {
            // Already found a better one
            if cost > *known_cost {
                continue;
            }
        }
        distances
            .entry((position, dir))
            .and_modify(|c| *c = cost)
            .or_insert(cost);

        let next_forward = position + dir;
        let mut next_tiles = tiles.clone();
        next_tiles.push(next_forward);
        if map[next_forward.x as usize][next_forward.y as usize] != Object::Wall {
            heap.push(Reverse(Node {
                cost: cost + 1,
                position: next_forward,
                dir,
                tiles: next_tiles,
            }));
        }

        let dir_right = dir.turn_right();
        let next_right = position + dir_right;
        next_tiles = tiles.clone();
        next_tiles.push(next_right);
        if map[next_right.x as usize][next_right.y as usize] != Object::Wall {
            heap.push(Reverse(Node {
                cost: cost + 1001,
                position: next_right,
                dir: dir_right,
                tiles: next_tiles,
            }));
        }

        let dir_left = dir.turn_left();
        let next_left = position + dir_left;
        next_tiles = tiles.clone();
        next_tiles.push(next_left);
        if map[next_left.x as usize][next_left.y as usize] != Object::Wall {
            heap.push(Reverse(Node {
                cost: cost + 1001,
                position: next_left,
                dir: dir_left,
                tiles: next_tiles,
            }));
        }
    }

    (shortest, all_tiles)
}

pub fn solve_1(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut pos = Coord::from_tuple((0, 0));
    let mut start = Coord::from_tuple((0, 0));
    let mut goal = Coord::from_tuple((0, 0));
    let map: Vec<Vec<Object>> = data
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '#' => Object::Wall,
                    '.' => Object::Path,
                    'E' => {
                        goal = Coord::from_tuple((x as isize, y as isize));
                        Object::Goal
                    }
                    'S' => {
                        pos = Coord::from_tuple((x as isize, y as isize));
                        start = Coord::from_tuple((x as isize, y as isize));
                        Object::Start
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let dir = Direction::Right;

    let (min, _) = shortest_path(&map, start, goal, dir);
    min
}

pub fn solve_2(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut pos = Coord::from_tuple((0, 0));
    let mut start = Coord::from_tuple((0, 0));
    let mut goal = Coord::from_tuple((0, 0));
    let map: Vec<Vec<Object>> = data
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, c)| match c {
                    '#' => Object::Wall,
                    '.' => Object::Path,
                    'E' => {
                        goal = Coord::from_tuple((x as isize, y as isize));
                        Object::Goal
                    }
                    'S' => {
                        pos = Coord::from_tuple((x as isize, y as isize));
                        start = Coord::from_tuple((x as isize, y as isize));
                        Object::Start
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let dir = Direction::Right;

    let (min, all_steps) = shortest_path(&map, start, goal, dir);
    let mut sum_set = HashSet::new();
    for (len, steps) in all_steps {
        if len == min {
            for s in steps {
                for c in s {
                    sum_set.insert(c);
                }
            }
        }
    }
    sum_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day16/test1.txt");
        assert_eq!(result, 7036);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day16/test2.txt");
        assert_eq!(result, 11048);
    }

    #[test]
    fn part1_3() {
        let result = solve_1("src/day16/test3.txt");
        assert_eq!(result, 21148);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day16/test1.txt");
        assert_eq!(result, 45);
    }

    #[test]
    fn part2_2() {
        let result = solve_2("src/day16/test2.txt");
        assert_eq!(result, 64);
    }
}
