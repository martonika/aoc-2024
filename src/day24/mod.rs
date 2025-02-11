use std::{collections::HashMap, fs};

use itertools::Itertools;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone)]
enum GateType {
    AND,
    OR,
    XOR,
}
impl GateType {
    pub fn from(s: &str) -> Self {
        match s {
            "AND" => GateType::AND,
            "OR" => GateType::OR,
            "XOR" => GateType::XOR,
            _ => unreachable!(),
        }
    }

    pub fn out(&self, w1: u8, w2: u8) -> u8 {
        match self {
            GateType::AND => w1 & w2,
            GateType::OR => w1 | w2,
            GateType::XOR => w1 ^ w2,
        }
    }
}

fn get_wire_value<'a>(
    wire: &'a str,
    wire_map: &HashMap<&'a str, (GateType, &'a str, &'a str)>,
    value_map: &mut HashMap<&'a str, u8>,
) -> u8 {
    if let Some(val) = value_map.get(wire) {
        return *val;
    }
    let (op, w1, w2) = wire_map.get(wire).unwrap();
    value_map.insert(
        wire,
        op.out(
            get_wire_value(w1, wire_map, &mut value_map.clone()),
            get_wire_value(w2, wire_map, &mut value_map.clone()),
        ),
    );
    *value_map.get(wire).unwrap()
}

pub fn solve_1(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let (inputs, circuit) = data.split_once("\r\n\r\n").unwrap();
    // Maps the wire's name to its value
    let mut value_map: HashMap<&str, u8> = inputs
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            (wire, value.parse::<u8>().unwrap())
        })
        .collect();
    // Maps the _output_ wire to the gate and two input wires before it
    let wire_map: HashMap<&str, (GateType, &str, &str)> = circuit
        .lines()
        .map(|line| {
            let data: Vec<_> = line.split_ascii_whitespace().collect();
            (data[4], (GateType::from(data[1]), data[0], data[2]))
        })
        .collect();
    let mut z_wires: Vec<(&str, u8)> = wire_map
        .clone()
        .into_iter()
        .filter(|(w, _)| w.starts_with('z'))
        .map(|(wire, _)| (wire, get_wire_value(wire, &wire_map, &mut value_map)))
        .collect();
    z_wires.sort();
    z_wires.reverse();

    u64::from_str_radix(
        z_wires
            .iter()
            .map(|(_, val)| val.to_string())
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

pub fn solve_2(input: &str) -> String {
    let data = fs::read_to_string(input).expect("Can't open file");
    let (_, circuit) = data.split_once("\r\n\r\n").unwrap();

    let gates = circuit
        .lines()
        .map(|line| {
            let (lhs, op, rhs, _, out) = line.split_whitespace().collect_tuple().unwrap();
            (out, (lhs, rhs, op))
        })
        .collect::<HashMap<_, _>>();

    // The input is a ripple-carry adder
    // [x XOR y] XOR carry_in -> z
    // [x AND y] OR [carry_in AND (the above)] -> carry_out

    // zXX output can only be after an XOR (except z45, the last)
    let error_1 = gates
        .iter()
        .filter(|(&out, (_, _, op))| out.starts_with('z') && out != "z45" && !op.eq(&"XOR"))
        .collect::<HashMap<_, _>>();
    // non-z output, NOT x,y input -> must not be XOR
    let error_2 = gates
        .iter()
        .filter(|(&out, (lhs, rhs, op))| {
            !out.starts_with('z')
                && !lhs.starts_with('x')
                && !lhs.starts_with('y')
                && !rhs.starts_with('x')
                && !rhs.starts_with('y')
                && op.eq(&"XOR")
        })
        .collect::<HashMap<_, _>>();

    // One gate deeper:
    // x XOR y -> must be connected to XOR (value) or AND (carry bit) (except bits 00)
    let error_3 = gates
        .iter()
        .filter(|(_, (lhs, rhs, op))| {
            // x XOR y
            ((lhs.starts_with('x') && rhs.starts_with('y'))
                || (lhs.starts_with('y') && rhs.starts_with('x')))
                && op.eq(&"XOR")
        })
        .filter(|(&out, _)| {
            // Connected to an OR
            let inner = gates
                .iter()
                .filter(|(_, (lhs, rhs, op))| (lhs == &out || rhs == &out) && op.eq(&"OR"))
                .collect::<Vec<_>>();
            !inner.is_empty()
        })
        // Except 00
        .filter(|(_, (lhs, rhs, _))| !lhs.contains("00") && !rhs.contains("00"))
        .collect::<HashMap<_, _>>();

    // x AND y -> must be connected to OR (except 00)
    let error_4 = gates
        .iter()
        .filter(|(_, (lhs, rhs, op))| {
            // x AND y
            ((lhs.starts_with('x') && rhs.starts_with('y'))
                || (lhs.starts_with('y') && rhs.starts_with('x')))
                && op.eq(&"AND")
        })
        .filter(|(&out, _)| {
            // Not connected to an OR
            let inner = gates
                .iter()
                .filter(|(_, (lhs, rhs, op))| (lhs == &out || rhs == &out) && !op.eq(&"OR"))
                .collect::<Vec<_>>();
            !inner.is_empty()
        })
        .filter(|(_, (lhs, rhs, _))| !lhs.contains("00") && !rhs.contains("00"))
        .collect::<HashMap<_, _>>();

    let mut wrong_wires = vec![];
    for (e, _) in error_1 {
        wrong_wires.push(e);
    }
    for (e, _) in error_2 {
        wrong_wires.push(e);
    }
    for (e, _) in error_3 {
        wrong_wires.push(e);
    }
    for (e, _) in error_4 {
        wrong_wires.push(e);
    }
    wrong_wires.sort();
    wrong_wires.iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day24/test1.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_2() {
        let result = solve_1("src/day24/test2.txt");
        assert_eq!(result, 2024);
    }

    #[test]
    fn part2() {
        // Can't test a non-ripple-carry adder with the test data
        //let result = solve_2("src/day24/test3.txt");
        //assert_eq!(result, "");
    }
}
