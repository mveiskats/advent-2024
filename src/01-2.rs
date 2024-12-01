use std::io;

fn main() {
    let (line1, line2): (Vec<_>, Vec<_>) = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            let (left, right) = line.split_once(' ').expect("no separator");

            let left = left.parse::<usize>().expect("not a number");
            let right = right.trim().parse::<usize>().expect("not a number");

            (left, right)
        }).collect();

    let result: usize = line1.into_iter().map(|left| {
        left * line2.iter().filter(|&&right| left == right).count()
    }).sum();

    println!("{result}");
}
