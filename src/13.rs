use std::io;
use std::io::Read;
use std::str::FromStr;

use num_rational::Rational64 as r64;
use winnow::{
    Parser,
    PResult,
    ascii::{ digit1, line_ending },
    combinator::{ delimited, preceded, separated, separated_pair },
    token::one_of,
};

#[derive(Debug)]
struct ClawGame {
    pub a: (r64, r64),
    pub b: (r64, r64),
    pub prize: (r64, r64),
}

fn parse_xy(input: &mut &str) -> PResult<(r64, r64)> {
    separated_pair(
        preceded(('X', one_of(['+', '='])), digit1.try_map(r64::from_str)),
        ", ",
        preceded(('Y', one_of(['+', '='])), digit1.try_map(r64::from_str))
    ).parse_next(input)
}

fn parse_game(input: &mut &str) -> PResult<ClawGame> {
    let parser = (delimited("Button A: ", parse_xy, line_ending),
                  delimited("Button B: ", parse_xy, line_ending),
                  delimited("Prize: ", parse_xy, line_ending));

    parser.map(|(a, b, prize)| ClawGame { a, b, prize }).parse_next(input)
}

// sa * a0 + sb * b0 = p0
// sa * a1 + sb * b1 = p1

// sb = (p1 - p0 * a1 / a0) / (b1 - b0 * a1 / a0)
// sa = (p0 - sb * b0) / a0

fn min_tokens(game: &ClawGame) -> Option<i64> {
    let div = game.b.1 - (game.b.0 * game.a.1 / game.a.0);
    if div == r64::ZERO { return None }

    let steps_b = (game.prize.1 - (game.prize.0 * game.a.1 / game.a.0)) / div;
    let steps_a = (game.prize.0 - (steps_b * game.b.0)) / game.a.0;

    if !(steps_a.is_integer() && steps_b.is_integer()) { return None }
    if steps_a < r64::ZERO || steps_b < r64::ZERO { return None }

    Some(steps_a.to_integer() * 3 + steps_b.to_integer())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let input: Vec<_> = separated(1.., parse_game, line_ending).parse(&input)
        .expect("parse error");

    let result: i64 = input.iter().filter_map(min_tokens).sum();

    println!("part 1: {result}");

    let bonus: r64 = 10000000000000.into();
    let input: Vec<_> = input.into_iter()
        .map(|game| ClawGame {
            prize: (game.prize.0 + bonus, game.prize.1 + bonus),
            ..game
        })
        .collect();

    let result: i64 = input.iter().filter_map(min_tokens).sum();

    println!("part 2: {result}");
}
