use std::io;
use std::collections::VecDeque;

use ndarray::Array2;

type Pos = (usize, usize);

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let flat_input: Vec<char> = input.into_iter().flatten().collect();

    let map = Array2::from_shape_vec((rows, cols), flat_input)
        .expect("input size mismatch");

    let (start, _) = map.indexed_iter()
        .find(|(_pos, ch)| **ch == 'S')
        .expect("no start");

    let mut distance = Array2::from_elem((rows, cols), usize::MAX);

    let mut queue: VecDeque<(usize, Pos)> = VecDeque::from([(0, start)]);

    while let Some((step, pos)) = queue.pop_front() {
        if map[pos] == '#' { continue }
        if distance[pos] <= step { continue }

        distance[pos] = step;

        let (r, c) = pos;
        let step = step + 1;

        queue.push_back((step, (r - 1, c)));
        queue.push_back((step, (r + 1, c)));
        queue.push_back((step, (r, c - 1)));
        queue.push_back((step, (r, c + 1)));
    }

    let min_save = 100;
    let max_cheat = 20;

    let mut result1 = 0;
    let mut result2 = 0;

    for r1 in 1..(rows as isize - 1) {
        for c1 in 1..(cols as isize - 1) {
            let start = (r1 as usize, c1 as usize);

            if map[start] == '#' { continue }

            let max_dr = max_cheat as isize;

            for dr in -max_dr..=max_dr {
                let max_dc = max_dr - dr.abs();

                for dc in -max_dc..=max_dc {
                    let r2 = r1 + dr;
                    let c2 = c1 + dc;

                    if r2 < 0 || r2 >= rows as isize { continue }
                    if c2 < 0 || c2 >= cols as isize { continue }

                    let end = (r2 as usize, c2 as usize);
                    if map[end] == '#' { continue }

                    let cheat_dist = dr.unsigned_abs() + dc.unsigned_abs();

                    if distance[end] >= distance[start] + min_save + cheat_dist {
                        if cheat_dist == 2 { result1 += 1 }
                        result2 += 1;
                    }
                }
            }
        }
    }

    println!("part 1: {}", result1);
    println!("part 2: {}", result2);
}
