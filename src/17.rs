use std::io;
use std::io::Read;
use std::str::FromStr;

use itertools::Itertools;
use winnow::{
    Parser,
    PResult,
    ascii::{ digit1, line_ending, newline },
    combinator::{ delimited, separated, separated_pair },
};

#[derive(Debug)]
struct Computer {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub ptr: usize,
    pub instructions: Vec<usize>,
}

impl Computer {
    fn read_combo(&mut self) -> Result<usize, ()> {
        let operand = self.read_literal()?;
        Ok(match operand {
            x if x < 4 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("undefined combo operand")
        })
    }

    fn read_literal(&mut self) -> Result<usize, ()> {
        if self.ptr < self.instructions.len() {
            let a = self.instructions[self.ptr];
            self.ptr += 1;
            Ok(a)
        } else {
            Err(())
        }
    }

    pub fn next_output(&mut self) -> Result<usize, ()> {
        loop {
            if let Some(output) = self.step()? { return Ok(output) }
        }
    }

    pub fn step(&mut self) -> Result<Option<usize>, ()> {
        let instruction = self.read_literal()?;
        match instruction {
            0 => self.a >>= self.read_combo()?, // adv
            1 => self.b ^= self.read_literal()?, // bxl
            2 => self.b = self.read_combo()? % 8, // bst
            3 => if self.a != 0 { self.ptr = self.read_literal()? }, // jnz
            4 => { self.b ^= self.c; let _ = self.read_literal()?; } // bxc
            5 => return Ok(Some(self.read_combo()? % 8)), // out
            6 => self.b = self.a >> self.read_combo()?, // bdv
            7 => self.c = self.a >> self.read_combo()?, // cdv
            _ => panic!("unknown instruction"),
        };
        Ok(None)
    }

    // This is not a general solver
    // It makes assumptions about the given instruction set:
    // 1. Each cycle has exactly one adv and one out instruction
    // 2. Each cycle output is dependent only on initial value of A
    pub fn solve(&mut self) -> Option<usize> {
        for i in 0..8 {
            let solution = self.solve_step(i, self.instructions.len() - 1);
            if solution.is_some() { return solution }
        }
        None
    }

    pub fn solve_step(&mut self, a: usize, step: usize) -> Option<usize> {
        let target: usize = self.instructions[step];

        let aa = a << 3;

        for i in 0..8 {
            let a = aa | i;

            self.a = a;
            self.ptr = 0;

            match self.next_output() {
                Ok(output) => {
                    if output == target {
                        let solution = if step == 0 {
                            Some(a)
                        } else {
                            self.solve_step(a, step - 1)
                        };
                        if solution.is_some() { return solution }
                    }
                },
                Err(()) => return None
            }
        }
        None
    }
}

fn parse_computer(input: &mut &str) -> PResult<Computer> {
    let registers = (
        delimited("Register A: ", digit1.try_map(usize::from_str), line_ending),
        delimited("Register B: ", digit1.try_map(usize::from_str), line_ending),
        delimited("Register C: ", digit1.try_map(usize::from_str), line_ending)
    );

    let instructions = delimited(
        "Program: ",
        separated(1.., digit1.try_map(usize::from_str), ","),
        newline
    );

    separated_pair(registers, newline, instructions)
        .map(|((a, b, c), instructions)| Computer { a, b, c, instructions, ptr: 0 })
        .parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let mut computer = parse_computer.parse(&input).expect("parse error");

    let mut output: Vec<usize> = vec![];

    while let Ok(a) = computer.next_output() {
        output.push(a)
    }

    println!("part 1: {}", output.iter().format(","));

    let solution = computer.solve().expect("no solution");
    println!("part 2: {solution}");
}
