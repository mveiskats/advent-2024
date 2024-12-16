use std::io;
use std::collections::VecDeque;

use ndarray::{ Array2, Array3 };

type Pos = (usize, usize);
type Delta = (isize, isize);

fn advance(pos: Pos, delta: Delta) -> Pos {
    ((pos.0 as isize + delta.0) as usize,
     (pos.1 as isize + delta.1) as usize)
}

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

    let start = (rows - 2, 1);
    let end = (1, cols - 2);

    let mut queue: VecDeque<(usize, Pos, Delta)> = VecDeque::from([(0, start, (0, 1))]);

    // Store separate horizontal and vertical scores
    let mut scores: Array3::<usize> = Array3::from_elem((rows, cols, 2), usize::MAX);
    let mut min_score = usize::MAX;

    while let Some((score, pos, delta)) = queue.pop_front() {
        if score > min_score { continue }
        if map[pos] == '#' { continue }

        let score_pos = (pos.0, pos.1, delta.0.unsigned_abs());
        if score >= scores[score_pos] { continue }

        if map[pos] == 'E' { min_score = score }
        scores[score_pos] = score;

        queue.push_back((score + 1, advance(pos, delta), delta));

        let score_pos = (score_pos.0, score_pos.1, 1 - score_pos.2);
        let score = score + 1000;

        if score < scores[score_pos] {
            scores[score_pos] = score;

            let right = (delta.1, -delta.0);
            queue.push_back((score + 1, advance(pos, right), right));

            let left = (-delta.1, delta.0);
            queue.push_back((score + 1, advance(pos, left), left));
        }
    }

    println!("part 1: {min_score}");

    let mut best: Array2<bool> = Array2::from_elem((rows, cols), false);

    let mut queue: VecDeque<(usize, Pos, Delta)> = VecDeque::new();
    if scores[(end.0, end.1, 0)] == min_score { queue.push_back((min_score, end, (0, -1))) }
    if scores[(end.0, end.1, 1)] == min_score { queue.push_back((min_score, end, (1, 0))) }

    while let Some((score, pos, delta)) = queue.pop_front() {
        let score_pos = (pos.0, pos.1, delta.0.unsigned_abs());

        if score != scores[score_pos] { continue }
        best[pos] = true;

        if score >= 1 {
            queue.push_back((score - 1, advance(pos, delta), delta));
        }

        if score >= 1000 {
            let right = (delta.1, -delta.0);
            queue.push_back((score - 1000, pos, right));

            let left = (-delta.1, delta.0);
            queue.push_back((score - 1000, pos, left));
        }
    }

    let result: usize = best.iter().filter(|b| **b).count();
    println!("part 2: {result}");
}
