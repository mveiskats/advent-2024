use std::io;
use std::io::Read;
use std::collections::HashMap;

use winnow::{
    Parser,
    PResult,
    ascii::{ alpha1, line_ending, newline },
    combinator::{ repeat, separated, separated_pair, terminated },
};

struct MatchCache {
    pub cache: HashMap<String, usize>,
}

impl MatchCache {
    fn get_count(&mut self, pattern: &str, towels: &Vec<String>) -> usize {
        if pattern.is_empty() { return 1 }

        let pattern = pattern.to_string();

        if let Some(&count) = self.cache.get(&pattern) { return count }

        let count = towels.iter()
            .filter_map(|towel| {
                if !pattern.starts_with(towel.as_str()) { return None }
                Some(self.get_count(&pattern[towel.len()..], towels))
            })
            .sum();

        self.cache.insert(pattern, count);
        count
    }
}

fn input_parser(input: &mut &str) -> PResult<(Vec<String>, Vec<String>)>
{
    let towels = terminated(separated(1.., alpha1.map(str::to_owned), ", "), line_ending);
    let patterns = repeat(1.., terminated(alpha1.map(str::to_owned), line_ending));

    separated_pair(towels, newline, patterns)
        .parse_next(input)
}

fn main () {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let (towels, patterns) = input_parser.parse(&input).expect("parse error");

    let mut matches = MatchCache { cache: HashMap::new() };

    let counts: Vec<_> = patterns.iter()
        .map(|pattern| matches.get_count(pattern, &towels))
        .collect();

    println!("part 1: {}", counts.iter().filter(|&&a| a > 0).count());
    println!("part 2: {}", counts.iter().sum::<usize>());
}
