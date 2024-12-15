use std::io;
use ndarray::Array2;

type Pos = (usize, usize);
type Delta = (isize, isize);

fn get_delta(ch: char) -> Delta {
    match ch {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => panic!("unknown instruction")
    }
}

fn advance(pos: Pos, delta: Delta) -> Pos {
    ((pos.0 as isize + delta.0) as usize,
     (pos.1 as isize + delta.1) as usize)
}

fn can_push(map: &Array2<char>, pos: Pos, delta: Delta) -> bool {
    match map[pos] {
        '.' => true,
        'O' | '@' => can_push(map, advance(pos, delta), delta),
        '[' | ']' if delta.0 == 0 => can_push(map, advance(pos, delta), delta),
        '[' => {
            let pos2 = (pos.0, pos.1 + 1);
            can_push(map, advance(pos, delta), delta) &&
                can_push(map, advance(pos2, delta), delta)
        },
        ']' => {
            let pos2 = (pos.0, pos.1 - 1);
            can_push(map, advance(pos, delta), delta) &&
                can_push(map, advance(pos2, delta), delta)
        }
        _ => false
    }
}

fn push(map: &mut Array2<char>, pusher: char, pos: Pos, delta: Delta) {
    let pushed = map[pos];
    match pushed {
        '.' => (),
        'O' | '@' => push(map, pushed, advance(pos, delta), delta),
        '[' => {
            push(map, pushed, advance(pos, delta), delta);
            if delta.0 != 0 && pusher != '.' {
                push(map, '.', (pos.0, pos.1 + 1), delta)
            }
        },
        ']' => {
            push(map, pushed, advance(pos, delta), delta);
            if delta.0 != 0 && pusher != '.' {
                push(map, '.', (pos.0, pos.1 - 1), delta);
            }
        }
        _ => panic!("immovable object")
    }

    map[pos] = pusher;
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let separator = input.iter()
        .position(Vec::is_empty)
        .expect("no separator");

    let rows = separator;
    let cols = input[0].len();

    let instructions: Vec<char> = input[(separator + 1)..].iter().flatten().cloned().collect();
    let flat_input: Vec<char> = input[..separator].iter().flatten().cloned().collect();

    let mut map = Array2::from_shape_vec((rows, cols), flat_input.clone())
        .expect("input size mismatch");

    let (start, _) = map.indexed_iter()
        .find(|(_, ch)| **ch == '@')
        .expect("no start");

    let mut pos: Pos = start;

    for &instruction in instructions.iter() {
        let delta = get_delta(instruction);

        if can_push(&map, pos, delta) {
            push(&mut map, '.', pos, delta);
            pos = advance(pos, delta);
        }
    }

    let result: usize = map.indexed_iter()
        .filter(|(_, ch)| **ch == 'O')
        .map(|((r, c), _)| c + r * 100)
        .sum();

    println!("part 1: {result}");

    let flat_input: Vec<char> = flat_input.into_iter().flat_map(|ch|{
        match ch {
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            '#' => ['#', '#'],
            'O' => ['[', ']'],
            _ => panic!("unknown symbol")
        }
    }).collect();

    let mut map = Array2::from_shape_vec((rows, cols * 2), flat_input)
        .expect("input size mismatch");

    let (start, _) = map.indexed_iter()
        .find(|(_, ch)| **ch == '@')
        .expect("no start");

    let mut pos: Pos = start;

    for &instruction in instructions.iter() {
        let delta = get_delta(instruction);

        if can_push(&map, pos, delta) {
            push(&mut map, '.', pos, delta);
            pos = advance(pos, delta);
        }
    }

    let result: usize = map.indexed_iter()
        .filter(|(_, ch)| **ch == '[')
        .map(|((r, c), _)| c + r * 100)
        .sum();

    println!("part 2: {result}");
}
