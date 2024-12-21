use std::io;
use std::iter;
use std::collections::HashMap;

use itertools::Itertools;

type Pos = (usize, usize);

fn nkey_pos(key: char) -> Pos {
    match key {
        '7' => (0, 0), '8' => (1, 0), '9' => (2, 0),
        '4' => (0, 1), '5' => (1, 1), '6' => (2, 1),
        '1' => (0, 2), '2' => (1, 2), '3' => (2, 2),
                       '0' => (1, 3), 'A' => (2, 3),
        _ => panic!("unknown key {key}")
    }
}

fn dkey_pos(key: char) -> Pos {
    match key {
                       '^' => (1, 0), 'A' => (2, 0),
        '<' => (0, 1), 'v' => (1, 1), '>' => (2, 1),
        _ => panic!("unknown key {key}")
    }
}

struct Memo {
    cache: HashMap<(char, char, usize), usize>
}

impl Memo {
    // Travel from start to end on numeric pad and press it
    pub fn npad_len(&mut self, start: char, end: char, dpads: usize) -> usize {
        let (sx, sy) = nkey_pos(start);
        let (ex, ey) = nkey_pos(end);

        // x+ y+ => v then >, except when crossing the void
        // x- y+ => < then v
        // x+ y- => v then >
        // x- y- => < then ^, except when crossing the void
        let x_first =
            (sx < ex && sy < ey && (sx == 0 && ey == 3)) ||
            (sx > ex && sy < ey) ||
            (sx > ex && sy > ey && !(sy == 3 && ex == 0));

        let x_button = if sx <= ex { '>' } else { '<' };
        let y_button = if sy <= ey { 'v' } else { '^' };

        let ((ch1, l1), (ch2, l2)) =
            if x_first {
                ((x_button, sx.abs_diff(ex)),
                 (y_button, sy.abs_diff(ey)))
            } else {
                ((y_button, sy.abs_diff(ey)),
                 (x_button, sx.abs_diff(ex)))
            };

        let seq = iter::once('A')
            .chain(iter::repeat_n(ch1, l1))
            .chain(iter::repeat_n(ch2, l2))
            .chain(iter::once('A'));

        seq.tuple_windows()
            .map(|(start, end)| self.dpad_len(start, end, dpads))
            .sum::<usize>()
    }

    // Travel from start to end on directional pad and press it
    fn dpad_len(&mut self, start: char, end: char, dpads: usize) -> usize {
        if dpads == 0 { return 1 }

        let key = (start, end, dpads);
        if let Some(&len) = self.cache.get(&key) { return len }

        let (sx, sy) = dkey_pos(start);
        let (ex, ey) = dkey_pos(end);

        // x+ y+ => v then >
        // x- y+ => < then v, except when crossing the void
        // x+ y- => v then >, except when crossing the void
        // x- y- => < then ^
        let x_first =
            (sx > ex && sy < ey && !(sy == 0 && ex == 0)) ||
            (sx < ex && sy > ey && (sx == 0 && ey == 0)) ||
            (sx > ex && sy > ey);

        let x_button = if sx < ex { '>' } else { '<' };
        let y_button = if sy < ey { 'v' } else { '^' };

        let ((ch1, l1), (ch2, l2)) =
            if x_first {
                ((x_button, sx.abs_diff(ex)),
                 (y_button, sy.abs_diff(ey)))
            } else {
                ((y_button, sy.abs_diff(ey)),
                 (x_button, sx.abs_diff(ex)))
            };

        let seq = iter::once('A')
            .chain(iter::repeat_n(ch1, l1))
            .chain(iter::repeat_n(ch2, l2))
            .chain(iter::once('A'));

        let dpads = dpads - 1;
        let len: usize = seq.tuple_windows()
            .map(|(start, end)| self.dpad_len(start, end, dpads))
            .sum();

        self.cache.insert(key, len);
        len
    }
}

fn main() {
    let input: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let mut memo = Memo { cache: HashMap::new() };

    let result: usize = input.iter().map(|code| {
        let num: usize = code[..(code.len() - 1)].parse().expect("not a number");
        let len: usize = iter::once('A').chain(code.chars())
            .tuple_windows()
            .map(|(start, end)| memo.npad_len(start, end, 2))
            .sum();

        len * num
    }).sum();

    println!("part 1: {:?}", result);

    let result: usize = input.iter().map(|code| {
        let num: usize = code[..(code.len() - 1)].parse().expect("not a number");
        let len: usize = iter::once('A').chain(code.chars())
            .tuple_windows()
            .map(|(start, end)| memo.npad_len(start, end, 25))
            .sum();

        len * num
    }).sum();

    println!("part 2: {:?}", result);
}
