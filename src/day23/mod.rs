use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

// Taken from rosettacode
fn bron_kerbosch_v2<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    g: &HashMap<&str, HashSet<&'a str>>,
    cliques: &mut HashSet<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<&str> = r.iter().cloned().collect();
            clique.sort();
            cliques.insert(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<&str> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v);

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<&str>>();

            // New X is X ∩ N(v)
            let mut new_x = x
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<&str>>();

            // Recursive call
            bron_kerbosch_v2(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn bron_kerbosch_v2_vec<'a>(
    r: &HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    g: &HashMap<&str, HashSet<&'a str>>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<&str> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<&str> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v);

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<&str>>();

            // New X is X ∩ N(v)
            let mut new_x = x
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<&str>>();

            // Recursive call
            bron_kerbosch_v2_vec(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

pub fn solve_1(input: &str) -> u32 {
    let data = fs::read_to_string(input).expect("Can't open file");

    let graph = data.lines().fold(
        HashMap::new(),
        |mut graph: HashMap<&str, HashSet<&str>>, line| {
            let (node1, node2) = line.split_once('-').unwrap();
            graph.entry(node1).or_default().insert(node2);
            graph.entry(node2).or_default().insert(node1);
            graph
        },
    );

    // Initialize R, P, X
    let r: HashSet<&str> = HashSet::new();
    let mut p: HashSet<&str> = graph.keys().cloned().collect();
    let mut x: HashSet<&str> = HashSet::new();

    // Collect cliques
    let mut cliques: HashSet<Vec<&str>> = HashSet::new();
    bron_kerbosch_v2(&r, &mut p, &mut x, &graph, &mut cliques);

    // Filter the cliques for vertices starting with 't'
    cliques.retain(|c| c.iter().any(|v| v.starts_with('t')));
    // Create 3-cliques from 3< ones
    let new_3_cliques = cliques.clone().into_iter().filter(|cl| cl.len() > 3).fold(
        Vec::new(),
        |mut three_clique: Vec<Vec<&str>>, clique| {
            three_clique.extend(
                clique
                    .into_iter()
                    .combinations(3)
                    .filter(|c| c.iter().any(|v| v.starts_with('t'))),
            );

            three_clique
        },
    );

    // Extend original and retain only three-cliques
    cliques.extend(new_3_cliques);
    cliques.retain(|c| c.len() == 3);

    cliques.len() as u32
}

pub fn solve_2(input: &str) -> String {
    let data = fs::read_to_string(input).expect("Can't open file");

    let graph = data.lines().fold(
        HashMap::new(),
        |mut graph: HashMap<&str, HashSet<&str>>, line| {
            let (node1, node2) = line.split_once('-').unwrap();
            graph.entry(node1).or_default().insert(node2);
            graph.entry(node2).or_default().insert(node1);
            graph
        },
    );

    // Initialize R, P, X
    let r: HashSet<&str> = HashSet::new();
    let mut p: HashSet<&str> = graph.keys().cloned().collect();
    let mut x: HashSet<&str> = HashSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<&str>> = vec![];
    bron_kerbosch_v2_vec(&r, &mut p, &mut x, &graph, &mut cliques);

    // Sort by individual clique length, will be ascending
    cliques.sort_by_key(|c| c.len());
    // Largest clique is the last one
    let mut largest = cliques[cliques.len() - 1].clone();
    // Sort by computer names for the password
    largest.sort();

    largest.iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day23/test1.txt");
        assert_eq!(result, 7);
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day23/test2.txt");
        assert_eq!(result, "co,de,ka,ta");
    }
}
