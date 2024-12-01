use std::io;
use std::iter::zip;

fn main() {
    let (mut line1, mut line2): (Vec<_>, Vec<_>) = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let (left, right) = line.split_once(' ').expect("no separator");

            let left = left.parse::<usize>().expect("not a number");
            let right = right.trim().parse::<usize>().expect("not a number");

            (left, right)
        })
        .collect();

    line1.sort();
    line2.sort();

    let result: usize = zip(line1, line2)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    println!("{result}");
}
