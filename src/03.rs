use std::io;
use std::io::Read;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let op_rx = Regex::new(r"(?<op>do|don't|mul)\((?:(?<x>\d+),(?<y>\d+))?\)").unwrap();

    let mut result1: usize = 0;
    let mut result2: usize = 0;
    let mut enabled: bool = true;

    op_rx.captures_iter(&input).for_each(|caps| {
        match &caps["op"] {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                let x = &caps["x"].parse::<usize>().expect("not a number");
                let y = &caps["y"].parse::<usize>().expect("not a number");

                result1 += x * y;
                if enabled { result2 += x * y }
            }
            _ => panic!("couldn't match {:?}", &caps[0])
        }
    });

    println!("part 1: {result1}");
    println!("part 2: {result2}");
}
