use std::io;
use itertools::Itertools;

fn is_safe(levels: &[usize]) -> bool {
    levels.iter().tuple_windows().all(|(&l1, &l2)| l1.abs_diff(l2) <= 3) &&
        (levels.iter().tuple_windows().all(|(l1, l2)| l1 < l2) ||
         levels.iter().tuple_windows().all(|(l1, l2)| l1 > l2))
}

fn main() {
    let safe: usize = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            line.split(' ')
                .map(|level| level.parse::<usize>().expect("not a number"))
                .collect::<Vec<_>>()
        })
        .filter(|levels|{
            // Brute force solutions for better future
            (0..levels.len()).any(|i| {
                let mut levels = levels.to_vec();
                levels.remove(i);
                is_safe(&levels)
            })
        })
        .count();

    println!("{safe}")
}
