use std::{collections::HashSet, fs};

type Coord = (isize, isize);
const NEIGHBOR_CELLS: [Coord; 4] = [
    (-1, 0), // up
    (0, 1),  // down
    (1, 0),  // left
    (0, -1), // right
];

struct Garden {
    plants: Vec<Vec<char>>,
    size: isize, // height and width are the same
}

/// Return an (area, perimeter) tuple
fn flood_fill(garden: &Garden, coord: Coord, visited: &mut HashSet<Coord>) -> Coord {
    if visited.contains(&coord) {
        return (0, 0);
    }

    let current_plant = &garden.plants[coord.0 as usize][coord.1 as usize];
    let size = garden.size;
    let mut neighbors = vec![];
    visited.insert(coord);

    // For our 4-neighbor scenario, fence perimeter for the current block is 4-(valid neighbors)
    //    B B
    //    - -
    //   |A A|B   <- 1st: 1 neighbor, 3 perimeter. 2nd: 2 neighbors, 2 perimeter
    //    -|A|B
    //      -
    //
    let mut perimeter = NEIGHBOR_CELLS.iter().fold(0, |mut acc, shift| {
        let neighbor_coord = (coord.0 + shift.0, coord.1 + shift.1);
        if neighbor_coord.0 >= 0
            && neighbor_coord.0 < size
            && neighbor_coord.1 >= 0
            && neighbor_coord.1 < size
        {
            // Neighbor coordinates are valid
            let neighbor = &garden.plants[neighbor_coord.0 as usize][neighbor_coord.1 as usize];
            if neighbor == current_plant {
                neighbors.push(neighbor_coord);
                // The currently visited neighbor is of the same type, perimeter doesn't increase
            } else {
                // The currently visited neighbor is of another type, perimeter += 1
                acc += 1;
            }
        } else {
            // The currently visited plant is at the edge -> add an edge fence -> perimeter += 1
            acc += 1;
        }
        acc
    });

    let mut area = 1; // Each cell has their own area

    for n in neighbors {
        let neighbor_visit = flood_fill(garden, n, visited);
        area += neighbor_visit.0;
        perimeter += neighbor_visit.1;
    }

    (area, perimeter)
}

/// Return an (area, sides) tuple
fn flood_fill_corner(garden: &Garden, coord: Coord, visited: &mut HashSet<Coord>) -> Coord {
    if visited.contains(&coord) {
        return (0, 0);
    }
    if garden.plants[coord.0 as usize][coord.1 as usize] == '.' {
        return (0, 0);
    }

    let current_plant = garden.plants[coord.0 as usize][coord.1 as usize];
    let size = garden.size;
    visited.insert(coord);

    // Here we need to check for corners
    // No neighbors == 4 corners
    // 1 neighbor == 2 corners
    // 2 neighbors == check if 0, 1 or 2
    // 3 neighbors == ouch
    // 4 neighbors == anything between 0-4

    let neighbors: Vec<Coord> = NEIGHBOR_CELLS
        .iter()
        .map(|shift| (coord.0 + shift.0, coord.1 + shift.1))
        .filter(|c| {
            (c.0 >= 0 && c.0 < size && c.1 >= 0 && c.1 < size)
                && (garden.plants[c.0 as usize][c.1 as usize] == current_plant)
        })
        .collect();

    // Ugh.
    // There are only 8 ways a corner can appear, depending on the neighbors of a plant
    // Since the data is padded with '.'s, no boundary checks are needed

    let top_left = garden.plants[coord.0 as usize - 1][coord.1 as usize - 1];
    let top = garden.plants[coord.0 as usize - 1][coord.1 as usize];
    let top_right = garden.plants[coord.0 as usize - 1][coord.1 as usize + 1];
    let left = garden.plants[coord.0 as usize][coord.1 as usize - 1];
    let right = garden.plants[coord.0 as usize][coord.1 as usize + 1];
    let bottom_left = garden.plants[coord.0 as usize + 1][coord.1 as usize - 1];
    let bottom = garden.plants[coord.0 as usize + 1][coord.1 as usize];
    let bottom_right = garden.plants[coord.0 as usize + 1][coord.1 as usize + 1];

    let mut corners = 0;

    if top != current_plant && left != current_plant {
        corners += 1
    }
    if bottom != current_plant && left != current_plant {
        corners += 1
    }
    if top != current_plant && right != current_plant {
        corners += 1
    }
    if bottom != current_plant && right != current_plant {
        corners += 1
    }
    if top == current_plant && left == current_plant && top_left != current_plant {
        corners += 1
    }
    if bottom == current_plant && left == current_plant && bottom_left != current_plant {
        corners += 1
    }
    if top == current_plant && right == current_plant && top_right != current_plant {
        corners += 1
    }
    if bottom == current_plant && right == current_plant && bottom_right != current_plant {
        corners += 1
    }

    let mut area = 1; // Each cell has their own area

    for n in neighbors {
        let neighbor_visit = flood_fill_corner(garden, n, visited);
        area += neighbor_visit.0;
        corners += neighbor_visit.1;
    }

    (area, corners)
}

pub fn solve_1(input: &str) -> isize {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut fence_price = 0;
    let plants: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut visited = HashSet::new();
    let garden = Garden {
        size: plants.len() as isize,
        plants: plants.clone(),
    };

    for (row, plant_row) in plants.iter().enumerate() {
        for (col, _) in plant_row.iter().enumerate() {
            let coord = (row as isize, col as isize);
            if !visited.contains(&coord) {
                let price = flood_fill(&garden, coord, &mut visited);
                fence_price += price.0 * price.1;
            }
        }
    }

    fence_price
}
pub fn solve_2(input: &str) -> isize {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut fence_price = 0;
    let mut plants: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    for row in &mut plants {
        row.push('.');
        row.insert(0, '.');
    }
    let new_size = plants[0].len();
    plants.push(vec!['.'; new_size]);
    plants.insert(0, vec!['.'; new_size]);
    let mut visited = HashSet::new();
    let garden = Garden {
        size: plants.len() as isize,
        plants: plants.clone(),
    };

    for (row, plant_row) in plants.iter().enumerate() {
        for (col, _) in plant_row.iter().enumerate() {
            let coord = (row as isize, col as isize);
            if !visited.contains(&coord) && plants[row][col] != '.' {
                let price = flood_fill_corner(&garden, coord, &mut visited);
                fence_price += price.0 * price.1;
            }
        }
    }
    fence_price
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day12/test1.txt");
        assert_eq!(result, 140);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day12/test2.txt");
        assert_eq!(result, 772);
    }

    #[test]
    fn part1_3() {
        let result = solve_1("src/day12/test3.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day12/test1.txt");
        assert_eq!(result, 80);
    }

    #[test]
    fn part2_2() {
        let result = solve_2("src/day12/test2.txt");
        assert_eq!(result, 436);
    }

    #[test]
    fn part2_3() {
        let result = solve_2("src/day12/test3.txt");
        assert_eq!(result, 1206);
    }

    #[test]
    fn part2_4() {
        let result = solve_2("src/day12/test4.txt");
        assert_eq!(result, 236);
    }

    #[test]
    fn part2_5() {
        let result = solve_2("src/day12/test5.txt");
        assert_eq!(result, 368);
    }

    #[test]
    fn part2_6() {
        let result = solve_2("src/day12/test6.txt");
        assert_eq!(result, 4 * 16);
    }
}
