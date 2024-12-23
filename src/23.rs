use std::io;
use std::io::Read;
use std::iter;

use itertools::Itertools;
use ndarray::Array2;

use winnow::{
    Parser,
    PResult,
    ascii::{ line_ending },
    combinator::{ repeat, separated_pair, terminated },
    stream::AsChar,
    token::take_while,
};

fn computer(input: &mut &str) -> PResult<String> {
    take_while(2..=2, AsChar::is_alpha).map(str::to_owned).parse_next(input)
}

fn connection(input: &mut &str) -> PResult<(String, String)> {
    separated_pair(computer, '-', computer).parse_next(input)
}

fn main () {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let connections: Vec<_> = repeat(1.., terminated(connection, line_ending))
        .parse(&input)
        .expect("parse error");

    let mut computers: Vec<String> = connections.iter()
        .flat_map(|(c1, c2)| [c1.clone(), c2.clone()])
        .collect();

    computers.sort();
    computers.dedup();

    let mut adjacent: Array2<bool> =
        Array2::from_elem((computers.len(), computers.len()), false);

    let mut neighbor_count = vec![0; computers.len()];

    for (c1, c2) in connections.iter() {
        let pos1 = computers.binary_search(c1).unwrap();
        let pos2 = computers.binary_search(c2).unwrap();
        adjacent[(pos1, pos2)] = true;
        adjacent[(pos2, pos1)] = true;

        // Assuming input has no duplicates
        neighbor_count[pos1] += 1;
        neighbor_count[pos2] += 1;
    }

    let result = (0..computers.len()).combinations(3).filter(|ids| {
        ids.iter().any(|&id| computers[id].starts_with('t')) &&
            ids.iter().tuple_combinations().all(|(&i1, &i2)| adjacent[(i1, i2)])
    }).count();

    println!("part 1: {result}");

    let mut found_size = 2;
    let mut found_set: Vec<usize> = vec![];

    for c in 0..computers.len() {
        let neighbors: Vec<_> = (0..computers.len())
            .filter(|&i| c != i && adjacent[(c, i)]).collect();

        // Find largest neighbor set that is a complete subgraph
        let max_size = neighbor_count[c] - 1;
        for size in (found_size..=max_size).rev() {
            let complete_set = neighbors.iter().combinations(size).find(|set| {
                set.iter().copied().tuple_combinations()
                    .all(|(&c1, &c2)| adjacent[(c1, c2)])
            });

            if let Some(set) = complete_set {
                found_size = size + 1;
                found_set = iter::once(c)
                    .chain(set.into_iter().copied())
                    .collect();

                break;
            }
        }
    }

    // Computers are processed in alphabetic order - no need to sort
    let result = found_set.into_iter().map(|c| computers[c].clone()).join(",");

    println!("part 2: {result}");
}
