use std::io;

use itertools::{ iterate, Itertools };

fn next_secret(secret: usize) -> usize {
    let secret = (secret ^ (secret << 6)) % 16777216;
    let secret = (secret ^ (secret >> 5)) % 16777216;
    (secret ^ (secret << 11)) % 16777216
}

fn main() {
    let input: Vec<usize> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.parse().expect("not a number"))
        .collect();

    let secret_sequences: Vec<Vec<usize>> = input.into_iter()
        .map(|secret| iterate(secret, |&a| next_secret(a)).take(2001).collect())
        .collect();

    let result: usize = secret_sequences.iter().map(|s| s[2000]).sum();

    println!("part 1: {result}");

    let price_histories: Vec<Vec<(usize, i8)>> = secret_sequences.into_iter().map(|seq| {
        seq.into_iter()
            .map(|a| a % 10)
            .tuple_windows()
            .map(|(a, b)| (b, b as i8 - a as i8))
            .collect()
    }).collect();

    let mut result = 0;

    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let best_price = price_histories.iter().map(|history| {
                        history.iter().tuple_windows().find_map(|((_, d1), (_, d2), (_, d3), (p4, d4))| {
                            if *d1 == a && *d2 == b && *d3 == c && *d4 == d { Some(*p4) } else { None }
                        }).unwrap_or(0)
                    }).sum();

                    result = result.max(best_price);
                }
            }
        }
    }

    println!("part 2: {result}");
}
