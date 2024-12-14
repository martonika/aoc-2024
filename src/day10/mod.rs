use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy)]
struct Height {
    height: u32,
    coord: (usize, usize),
}

#[derive(Debug, Clone)]
struct Graph {
    node: Height,
    neighbors: RefCell<HashMap<(usize, usize), Height>>,
}

impl Graph {
    fn new(node: Height) -> Self {
        Self {
            node,
            neighbors: RefCell::new(HashMap::new()),
        }
    }
}

fn build_graphs(
    map: &mut Vec<Vec<Height>>,
    graphs: &mut Vec<Graph>,
    row: usize,
    col: usize,
    trail_ends: &mut HashSet<(usize, usize)>,
) -> (u32, u32) {
    let mut score_pt2 = 0;
    let current_node = map[row][col];
    graphs.push(Graph {
        node: current_node,
        neighbors: RefCell::new(HashMap::new()),
    });
    let curr_graph = graphs.last().unwrap();

    if row > 0 {
        let neighbor = map[row - 1][col];
        if neighbor.height == current_node.height + 1 {
            curr_graph
                .neighbors
                .borrow_mut()
                .insert((row - 1, col), neighbor);
            if neighbor.height == 9 {
                trail_ends.insert((row - 1, col));
                score_pt2 += 1;
            }
        }
    }
    if row < map.len() - 1 {
        let neighbor = map[row + 1][col];
        if neighbor.height == current_node.height + 1 {
            curr_graph
                .neighbors
                .borrow_mut()
                .insert((row + 1, col), neighbor);
            if neighbor.height == 9 {
                trail_ends.insert((row + 1, col));
                score_pt2 += 1;
            }
        }
    }
    if col > 0 {
        let neighbor = map[row][col - 1];
        if neighbor.height == current_node.height + 1 {
            curr_graph
                .neighbors
                .borrow_mut()
                .insert((row, col - 1), neighbor);
            if neighbor.height == 9 {
                trail_ends.insert((row, col - 1));
                score_pt2 += 1;
            }
        }
    }
    if col < map[row].len() - 1 {
        let neighbor = map[row][col + 1];
        if neighbor.height == current_node.height + 1 {
            curr_graph
                .neighbors
                .borrow_mut()
                .insert((row, col + 1), neighbor);
            if neighbor.height == 9 {
                trail_ends.insert((row, col + 1));
                score_pt2 += 1;
            }
        }
    }

    for ((row, col), _) in curr_graph.neighbors.clone().borrow().iter() {
        let (_, tmp_score_pt2) = build_graphs(map, graphs, *row, *col, trail_ends);
        score_pt2 += tmp_score_pt2;
    }
    (trail_ends.len() as u32, score_pt2)
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut id = 0;

    let mut points: HashMap<usize, Height> = HashMap::new();
    let mut map: Vec<Vec<Height>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, b)| {
                    id += 1;
                    let h = Height {
                        coord: (row, col),
                        height: b.to_digit(10).unwrap(),
                    };
                    points.insert(id, h);
                    h
                })
                .collect()
        })
        .collect();

    let mut graphs: Vec<Graph> = Vec::new();
    let mut scores = 0;

    for (row, m) in map.clone().iter().enumerate() {
        for (col, point) in m.iter().enumerate() {
            if point.height == 0 {
                let mut trail_ends: HashSet<(usize, usize)> = HashSet::new();
                let (tmp_score, _) = build_graphs(&mut map, &mut graphs, row, col, &mut trail_ends);
                scores += tmp_score;
            }
        }
    }
    scores
}
pub fn solve_2(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut id = 0;

    let mut points: HashMap<usize, Height> = HashMap::new();
    let mut map: Vec<Vec<Height>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, b)| {
                    id += 1;
                    let h = Height {
                        coord: (row, col),
                        height: b.to_digit(10).unwrap(),
                    };
                    points.insert(id, h);
                    h
                })
                .collect()
        })
        .collect();

    let mut graphs: Vec<Graph> = Vec::new();
    let mut scores = 0;

    for (row, m) in map.clone().iter().enumerate() {
        for (col, point) in m.iter().enumerate() {
            if point.height == 0 {
                let mut trail_ends: HashSet<(usize, usize)> = HashSet::new();
                let (_, tmp_score) = build_graphs(&mut map, &mut graphs, row, col, &mut trail_ends);
                scores += tmp_score;
            }
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day10/test1.txt");
        assert_eq!(result, 1);
    }

    #[test]
    fn part1_larger() {
        let result = solve_1("src/day10/test2.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day10/test2.txt");
        assert_eq!(result, 81);
    }
}
