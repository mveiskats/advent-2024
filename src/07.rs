use std::io;

fn is_possible1(value: usize, operands: &[usize], acc: usize) -> bool {
    if acc > value { return false }
    match operands {
        [] => { acc == value }
        [op, tail @ ..] => {
            is_possible1(value, tail, acc + op) ||
                is_possible1(value, tail, acc * op)
        }
    }
}

fn combine(a: usize, b: usize) -> usize {
    (a.to_string() + &b.to_string()).parse().expect("not a number")
}

fn is_possible2(value: usize, operands: &[usize], acc: usize) -> bool {
    if acc > value { return false }
    match operands {
        [] => { acc == value }
        [op, tail @ ..] => {
            is_possible2(value, tail, acc + op) ||
                is_possible2(value, tail, acc * op) ||
                is_possible2(value, tail, combine(acc, *op))
        }
    }
}

fn main() {
    let input: Vec<(usize, Vec<usize>)> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let (value, operands) = line
                .split_once(": ")
                .expect("missing separator");

            let value = value.parse::<usize>().expect("not a number");
            let operands: Vec<usize> = operands
                .split(" ")
                .map(str::parse::<usize>)
                .map(|op| op.expect("not a number"))
                .collect();
            (value, operands)
        })
        .collect();

    let result: usize = input.iter()
        .filter(|(value, operands)| is_possible1(*value, &operands[1..], operands[0]))
        .map(|(value, _)| value)
        .sum();

    println!("part 1: {result}");

    let result: usize = input.iter()
        .filter(|(value, operands)| is_possible2(*value, &operands[1..], operands[0]))
        .map(|(value, _)| value)
        .sum();

    println!("part 2: {result}");
}
