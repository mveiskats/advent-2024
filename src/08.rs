use std::io;
use std::collections::HashMap;

use gcd::Gcd;
use glam::IVec2;
use itertools::Itertools;
use ndarray::Array2;

fn out_of_bounds(p: IVec2, (cols, rows): (usize, usize)) -> bool {
    p.x < 0 || p.x >= cols as i32 || p.y < 0 || p.y >= rows as i32
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut antennae: HashMap<char, Vec<IVec2>> = HashMap::new();

    for (y, row) in input.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '.' { continue }

            let p = IVec2 { x: x as i32, y: y as i32 };
            antennae.entry(*ch).or_default().push(p);
        }
    }

    let mut v1_antinodes: Array2<bool> = Array2::from_elem((cols,rows), false);
    let mut v2_antinodes: Array2<bool> = Array2::from_elem((cols,rows), false);

    for positions in antennae.values() {
        for (p1, p2) in positions.iter().tuple_combinations() {
            let slope = p2 - p1;
            let gcd = slope.x.unsigned_abs().gcd(slope.y.unsigned_abs());

            // Generally there could be antinodes between pair of anteannae,
            // but input doesn't have any cases like this
            if gcd > 1 { panic!("slope gcd > 1") }

            // Scan forward until out of bounds
            let mut i: i32 = 0;
            loop {
                let p = p1 + i * slope;

                if out_of_bounds(p, (cols, rows)) { break }

                if i == 2 { v1_antinodes[(p.x as usize, p.y as usize)] = true; }
                v2_antinodes[(p.x as usize, p.y as usize)] = true;

                i += 1;
            }

            // Scan backward until out of bounds
            i = -1;
            loop {
                let p = p1 + i * slope;

                if out_of_bounds(p, (cols, rows)) { break }

                if i == -1 { v1_antinodes[(p.x as usize, p.y as usize)] = true; }
                v2_antinodes[(p.x as usize, p.y as usize)] = true;

                i -= 1;
            }
        }
    }

    let result = v1_antinodes.into_iter().filter(|&b| b).count();
    println!("part 1: {result}");

    let result = v2_antinodes.into_iter().filter(|&b| b).count();
    println!("part 2: {result}");
}
