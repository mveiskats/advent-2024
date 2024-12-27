use std::io;
use std::io::Read;
use std::str::FromStr;

use itertools::Itertools;

use winnow::{
    Parser,
    PResult,
    ascii::{ alphanumeric1, digit1, line_ending, newline },
    combinator::{ preceded, repeat, separated_pair, terminated },
};

#[derive(Clone, Copy, Debug)]
enum Op { And, Or, Xor }

#[derive(Clone)]
struct Gate {
    pub in1: usize,
    pub in2: usize,
    pub op: Op,
    pub out: usize
}

#[derive(Clone)]
struct Calculator {
    names: Vec<String>,
    values: Vec<Option<usize>>,
    gates: Vec<Gate>,
    x_bits: Vec<usize>,
    y_bits: Vec<usize>,
    z_bits: Vec<usize>,
}

impl Calculator {
    pub fn new(inputs: Vec<(String, usize)>, gates: Vec<(String, Op, String, String)>) -> Self {
        let mut registers: Vec<(String, Option<usize>)> =
            inputs.into_iter().map(|(name, value)| (name, Some(value)))
            .chain(gates.iter().map(|(_, _, _, output)| (output.clone(), None)))
            .collect();

        registers.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        let (names, values): (Vec<_>, Vec<_>) = registers.into_iter().unzip();

        let gates: Vec<_> = gates.into_iter().map(|(in1, op, in2, out)|{
            let in1 = names.binary_search_by(|name| name.cmp(&in1)).expect("unknown name");
            let in2 = names.binary_search_by(|name| name.cmp(&in2)).expect("unknown name");
            let out = names.binary_search_by(|name| name.cmp(&out)).expect("unknown name");
            Gate { in1, in2, op, out }
        }).collect();

        let x_bits: Vec<_> = names.iter().enumerate()
            .filter(|(_, name)| name.starts_with("x"))
            .map(|(i, _)| i)
            .collect();

        let y_bits: Vec<_> = names.iter().enumerate()
            .filter(|(_, name)| name.starts_with("y"))
            .map(|(i, _)| i)
            .collect();

        let z_bits: Vec<_> = names.iter().enumerate()
            .filter(|(_, name)| name.starts_with("z"))
            .map(|(i, _)| i)
            .collect();

        Self { names, values, gates, x_bits, y_bits, z_bits }
    }

    fn next_gate_to_calculate(&self) -> Option<Gate> {
        self.gates.iter().find(|g| {
            self.values[g.out].is_none() &&
                self.values[g.in1].is_some() &&
                self.values[g.in2].is_some()
        }).cloned()
    }

    fn calculate(&mut self) {
        while let Some(g) = self.next_gate_to_calculate() {
            let in1 = self.values[g.in1].unwrap();
            let in2 = self.values[g.in2].unwrap();
            self.values[g.out] = match g.op {
                Op::And => Some(in1 & in2),
                Op::Or => Some(in1 | in2),
                Op::Xor => Some(in1 ^ in2)
            }
        }
    }

    pub fn calculate_z(&mut self) -> usize {
        self.calculate();
        self.z_bits.iter()
            .map(|&i| self.values[i])
            .map(|value| value.expect("uncomputed value"))
            .rev()
            .fold(0, |acc, value| (acc << 1) | value)
    }

    fn set_x(&mut self, bit: usize, value: usize) {
        self.values[self.x_bits[bit]] = Some(value);
    }

    fn set_y(&mut self, bit: usize, value: usize) {
        self.values[self.y_bits[bit]] = Some(value);
    }

    fn get_z(&self, bit: usize) -> Option<usize> {
        self.values[self.z_bits[bit]]
    }

    fn bit_ok(&mut self, bit: usize) -> bool {
        self.values.fill(Some(0));

        let non_inputs: Vec<_> = (0..self.values.len())
            .filter(|register| !self.x_bits.contains(register))
            .filter(|register| !self.y_bits.contains(register))
            .collect();

        (0..4).permutations(2).map(|xy| (xy[0], xy[1])).all(|(x, y)| {
            for &register in non_inputs.iter() {
                self.values[register] = None;
            }

            if bit > 0 {
                self.set_x(bit - 1, x % 2);
                self.set_y(bit - 1, y % 2);
            }

            self.set_x(bit, x / 2);
            self.set_y(bit, y / 2);

            self.calculate();

            let z = if bit == 0 { (x & 2) + (y & 2) } else { x + y };

            self.get_z(bit) == Some((z / 2) % 2) &&
                self.get_z(bit + 1) == Some(z / 4)
        })
    }

    fn valid(&self) -> bool {
        let mut visited_values: Vec<bool> = vec![false; self.values.len()];
        let mut visited_gates: Vec<bool> = vec![false; self.gates.len()];

        self.x_bits.iter().for_each(|&i| visited_values[i] = true);
        self.y_bits.iter().for_each(|&i| visited_values[i] = true);

        while let Some((i, g)) = self.gates.iter().enumerate().find(|(i, g)| visited_values[g.in1] && visited_values[g.in2] && !visited_gates[*i]) {
            // Loop detected
            if visited_values[g.out] { return false }

            visited_gates[i] = true;
            visited_values[g.out] = true;
        }

        // Not computable gates
        visited_gates.into_iter().all(|visited| visited)
    }

    pub fn find_fix(&mut self, bad_limit: usize, swaps: Vec<usize>) -> Option<String> {
        if swaps.len() > 8 { return None }

        let input_bits = self.x_bits.len();
        let gates = self.gates.len();
        let bit_ok: Vec<_> = (0..input_bits).map(|i| self.bit_ok(i)).collect();

        let bad_count = bit_ok.iter().filter(|&&b| !b).count();
        if bad_count > bad_limit  { return None }

        if let Some(bit) = (0..input_bits).rev().find(|&i| !bit_ok[i]) {
            for (g1, g2) in (0..gates).rev().tuple_combinations() {
                let out1 = self.gates[g1].out;
                let out2 = self.gates[g2].out;
                if swaps.contains(&out1) || swaps.contains(&out2) { continue }

                let mut swapped = self.clone();
                swapped.gates[g1].out = out2;
                swapped.gates[g2].out = out1;

                if !swapped.valid() { continue }

                if (bit..input_bits).any(|i| !swapped.bit_ok(i)) { continue }

                let mut swaps = swaps.clone();
                swaps.push(out1);
                swaps.push(out2);

                let fix = swapped.find_fix(bad_count - 1, swaps);
                if fix.is_some() { return fix }
            }
            None
        } else {
            let mut registers: Vec<_> = swaps.iter().map(|&o| self.names[o].clone()).collect();
            registers.sort();

            Some(registers.join(","))
        }
    }
}

fn str_to_op(s: &str) -> Op {
    match s {
        "AND" => Op::And,
        "OR" => Op::Or,
        "XOR" => Op::Xor,
        _ => panic!("unknown op")
    }
}

fn parse_values(input: &mut &str) -> PResult<Vec<(String, usize)>> {
    let num = digit1.try_map(usize::from_str);
    let value = separated_pair(alphanumeric1.map(str::to_owned), ": ", num);

    repeat(1.., terminated(value, line_ending)).parse_next(input)
}

fn parse_gates(input: &mut &str) -> PResult<Vec<(String, Op, String, String)>> {
    let gate = (
        alphanumeric1.map(str::to_owned),
        (preceded(" ", alphanumeric1).map(str_to_op)),
        (preceded(" ", alphanumeric1).map(str::to_owned)),
        (preceded(" -> ", alphanumeric1).map(str::to_owned))
    );

    repeat(1.., terminated(gate, line_ending)).parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let (values, gates) = separated_pair(parse_values, newline, parse_gates)
        .parse(&input)
        .expect("parse error");

    let mut calculator = Calculator::new(values, gates);
    let z = calculator.calculate_z();

    println!("part 1: {z}");

    let result = calculator.find_fix(calculator.x_bits.len(), vec![])
        .expect("fix not found");

    println!("part 2: {result}");
}
