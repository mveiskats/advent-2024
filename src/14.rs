use std::io;
use std::io::Read;
use std::str::FromStr;

use glam::IVec2;
use ndarray::{ s, Array2 };
use winnow::{
    Parser,
    PResult,
    ascii::{ digit1, line_ending },
    combinator::{ opt, preceded, repeat, separated_pair, terminated },
};

fn parse_vec2(input: &mut &str) -> PResult<IVec2> {
    separated_pair(
        (opt('-'), digit1).take().try_map(i32::from_str),
        ",",
        (opt('-'), digit1).take().try_map(i32::from_str)
    ).map(|(x, y)| IVec2 { x, y }).parse_next(input)
}

fn parse_robot(input: &mut &str) -> PResult<(IVec2, IVec2)> {
    (preceded("p=", parse_vec2), preceded(" v=", parse_vec2)).parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let mut robots: Vec<_> = repeat(1.., terminated(parse_robot, line_ending)).parse(&input)
        .expect("parse error");

    let width: i32 = 101;
    let height: i32 = 103;

    let mid_x = (width - 1) / 2;
    let mid_y = (height - 1) / 2;

    let mut i = 0;
    loop {
        let mut map: Array2<usize> = Array2::from_elem((width as usize, height as usize), 0);

        for (p, v) in robots.iter_mut() {
            *p += *v;
            while p.x < 0 { p.x += width }
            while p.x >= width { p.x -= width }
            while p.y < 0 { p.y += height }
            while p.y >= height { p.y -= height }

            map[(p.x as usize, p.y as usize)] += 1;
        }

        i += 1;

        if i == 100 {
            let quadrants: [usize; 4] = [
                map.slice(s![..mid_x, ..mid_y]).iter().sum(),
                map.slice(s![..mid_x, (mid_y + 1)..]).iter().sum(),
                map.slice(s![(mid_x + 1).., ..mid_y]).iter().sum(),
                map.slice(s![(mid_x + 1).., (mid_y + 1)..]).iter().sum(),
            ];

            let result: usize = quadrants.into_iter().product();

            println!("part 1: {result}");
        }

        if map.windows((1, 10)).into_iter().any(|window| window.iter().all(|a| *a > 0)) {
            println!("part 2: {i}");

            // Print the map for inspection
            for y in 0..(height as usize) {
                for x in 0..(width as usize) {
                    print!("{}", if map[(x, y)] == 0 { '.' } else { '#' })
                }
                println!();
            }
            break
        }
    }
}
