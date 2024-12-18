use std::io;
use std::io::Read;
use std::collections::VecDeque;
use std::str::FromStr;

use winnow::{
    Parser,
    PResult,
    ascii::{ digit1, line_ending },
    combinator::{ repeat, separated_pair, terminated },
};

use ndarray::Array2;

type Pos = (usize, usize);

fn parse_int(input: &mut &str) -> PResult<usize> {
    digit1.try_map(usize::from_str).parse_next(input)
}

fn parse_pos(input: &mut &str) -> PResult<Pos> {
    terminated(separated_pair(parse_int, ",", parse_int), line_ending)
        .parse_next(input)
}

fn main () {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let input: Vec<Pos> = repeat(1.., parse_pos).parse(&input)
        .expect("parse error");

    let size = 71;
    let mut corrupt: Array2<bool> = Array2::from_elem((size, size), false);
    let mut distance: Array2<usize> = Array2::from_elem((size, size), usize::MAX);
    let mut queue: VecDeque<(usize, Pos)> = VecDeque::new();

    let start = (0, 0);
    let end = (size - 1, size - 1);

    for (i, corrupt_pos) in input.into_iter().enumerate() {
        corrupt[corrupt_pos] = true;

        distance.fill(usize::MAX);
        queue.clear();
        queue.push_back((0, start));

        let mut failed = true;

        while let Some((step, pos)) = queue.pop_front() {
            if pos == end {
                if i == 1023 { println!("part 1: {step}") }
                failed = false;
                break
            }
            if corrupt[pos] { continue }
            if distance[pos] <= step { continue }

            distance[pos] = step;

            let (x, y) = pos;
            let step = step + 1;

            if x > 0 { queue.push_back((step, (x - 1, y))) }
            if x < size - 1 { queue.push_back((step, (x + 1, y))) }
            if y > 0 { queue.push_back((step, (x, y - 1))) }
            if y < size - 1 { queue.push_back((step, (x, y + 1))) }
        }

        if failed {
            println!("part 2: {},{}", corrupt_pos.0, corrupt_pos.1);
            break
        }
    }
}
