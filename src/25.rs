use std::io;

use itertools::Itertools;

fn main() {
    let input: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split(String::is_empty)
        .partition(|slice| slice[0] == "#####");

    let locks: Vec<_> = locks.into_iter().map(|lines| {
        let chars: Vec<_> = lines[1..].iter()
            .flat_map(|line| line.chars())
            .map(|ch| (ch == '#') as usize)
            .collect();

        (0..5).map(|i| (0..6).map(|j| chars[j * 5 + i]).sum::<usize>())
            .collect::<Vec<_>>()
    }).collect();

    let keys: Vec<_> = keys.into_iter().map(|lines| {
        let chars: Vec<_> = lines[..6].iter()
            .flat_map(|line| line.chars())
            .map(|ch| (ch == '#') as usize)
            .collect();

        (0..5).map(|i| (0..6).map(|j| chars[j * 5 + i]).sum::<usize>())
            .collect::<Vec<_>>()
    }).collect();

    let result: usize = locks.iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| {
            lock.iter().zip(key.iter()).all(|(&l, &k)| l + k < 6)
        })
        .count();

    println!("{result}");
}
