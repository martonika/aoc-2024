use std::{collections::HashSet, fs};

#[repr(u8)]
#[allow(non_camel_case_types)]
enum Instructions {
    adv = 0, // Register A division (by combo operand) stored in A
    bdv = 6, // Register A division (by combo operand) stored in B
    cdv = 7, // Register A division (by combo operand) stored in C
    bxl = 1, // Bitwise XOR, operand ^ B
    bst = 2, // Combo operand mod 8, write to B
    jnz = 3, // Jump if not zero, check A, jump to operand
    bxc = 4, // Bitwise XOR, B ^ C
    out = 5, // Combo operand mod 8, write to stdout
}

impl From<u8> for Instructions {
    fn from(orig: u8) -> Self {
        match orig {
            0 => Self::adv,
            1 => Self::bxl,
            2 => Self::bst,
            3 => Self::jnz,
            4 => Self::bxc,
            5 => Self::out,
            6 => Self::bdv,
            7 => Self::cdv,
            _ => unreachable!(),
        }
    }
}

fn get_combo(operand: u8, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

fn handle_instruction(
    opcode: Instructions,
    operand: u8,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
    ic: usize,
    out: &mut Vec<u8>,
) -> usize {
    let mut new_ic = ic + 2;
    match opcode {
        Instructions::adv => {
            let num = *reg_a;
            let op = get_combo(operand, *reg_a, *reg_b, *reg_c);
            let den = 2u64.pow(op as u32);
            *reg_a = num / den;
        }
        Instructions::bdv => {
            let num = *reg_a;
            let op = get_combo(operand, *reg_a, *reg_b, *reg_c);
            let den = 2u64.pow(op as u32);
            *reg_b = num / den;
        }
        Instructions::cdv => {
            let num = *reg_a;
            let op = get_combo(operand, *reg_a, *reg_b, *reg_c);
            let den = 2u64.pow(op as u32);
            *reg_c = num / den;
        }
        Instructions::bxl => {
            *reg_b ^= operand as u64;
        }
        Instructions::bst => {
            *reg_b = get_combo(operand, *reg_a, *reg_b, *reg_c) % 8;
        }
        Instructions::jnz => {
            if *reg_a != 0 {
                new_ic = operand as usize;
            }
        }
        Instructions::bxc => {
            *reg_b ^= *reg_c;
        }
        Instructions::out => {
            out.push((get_combo(operand, *reg_a, *reg_b, *reg_c) % 8) as u8);
        }
    }

    new_ic
}

pub fn solve_1(input: &str) -> String {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut it = data.lines();
    let mut s = it.next().unwrap();
    let mut reg_a = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    s = it.next().unwrap();
    let mut reg_b = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    s = it.next().unwrap();
    let mut reg_c = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    it.next();
    s = it.next().unwrap();
    let inst_split = s.split(": ").collect::<Vec<_>>()[1];
    let inst = inst_split
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut out = vec![];
    let mut ic = 0;

    while ic < inst.len() {
        ic = handle_instruction(
            inst[ic].into(),
            inst[ic + 1],
            &mut reg_a,
            &mut reg_b,
            &mut reg_c,
            ic,
            &mut out,
        )
    }
    // Some format!() magic might work with {:?}, out, but it could just be worse performance anyway
    out.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve_2(input: &str) -> u64 {
    let data = fs::read_to_string(input).expect("Can't open file");
    let mut it = data.lines();
    let mut s = it.next().unwrap();
    let _ = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap(); // reg_a not used here
    let mut reg_a: u64;
    s = it.next().unwrap();
    let mut reg_b = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    s = it.next().unwrap();
    let mut reg_c = s.split(": ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
    it.next();
    s = it.next().unwrap();
    let program = s.split(": ").collect::<Vec<_>>()[1];
    let inst = program
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut out = vec![];
    let mut ic = 0;
    let reg_b_orig = reg_b;
    let reg_c_orig = reg_c;

    // Due to that super-not-suspicious left shift by 3 bits, we can work our way in reverse order.
    // If we multiply by 8 (right shift by 3), we have three bits' worth of possible values for the
    // desired output.
    // So first round, we want to find what possible numbers generate the last digit of our program
    // Then use these possible numbers as base to find the second number(s), etc.
    // In the end we'll find a few that are over 8^15, but less than 8^16 (that would bring the output length to 17)
    // And the smallest of these is our result

    let mut possible_as = HashSet::new();
    possible_as.insert(0u64);
    for required_out in inst.iter().rev() {
        let mut next_possible = HashSet::new();

        for a in &possible_as {
            let next = 8 * a; // shift by 3
            for candidate in next..next + 8 {
                reg_a = candidate;
                while ic < inst.len() {
                    ic = handle_instruction(
                        inst[ic].into(),
                        inst[ic + 1],
                        &mut reg_a,
                        &mut reg_b,
                        &mut reg_c,
                        ic,
                        &mut out,
                    )
                }
                if out[0] == *required_out {
                    next_possible.insert(candidate);
                }
                // Reset registers
                out.clear();
                reg_b = reg_b_orig;
                reg_c = reg_c_orig;
                ic = 0;
            }
        }
        possible_as = next_possible;
    }

    *possible_as.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let result = solve_1("src/day17/test1.txt");
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2() {
        let result = solve_2("src/day17/test2.txt");
        assert_eq!(result, 117440);
    }
}
