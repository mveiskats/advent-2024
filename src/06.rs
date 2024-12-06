use std::io;
use ndarray::Array2;

fn rotate((x, y): (isize, isize)) -> (isize, isize) {
    (-y, x)
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut start: (usize, usize) = (0, 0);
    let mut dir: (isize, isize) = (0, -1);

    let mut walls: Array2<bool> = Array2::from_elem((cols, rows), false);

    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == '^' {
                start = (col, row);
            } else if input[row][col] == '#' {
                walls[(col, row)] = true
            }
        }
    }

    let mut visited: Array2<bool> = Array2::from_elem((cols, rows), false);

    let mut pos = start;
    visited[pos] = true;

    loop {
        let next = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);

        if next.0 < 0 || next.0 >= cols as isize { break }
        if next.1 < 0 || next.1 >= rows as isize { break }

        let next = (next.0 as usize, next.1 as usize);

        if walls[next] {
            dir = rotate(dir);
        } else {
            pos = next;
            visited[pos] = true;
        }
    }

    let visited: usize = visited.into_iter().filter(|&vis| vis).count();

    println!("part 1: {visited}");

    let mut loops = 0;

    // Try all the things
    for x in 0..cols {
        for y in 0..rows {
            if walls[(x, y)] { continue }
            if (x, y) == start { continue }

            let mut walls = walls.clone();
            walls[(x, y)] = true;

            let mut stop_dir: Array2<(isize, isize)> = Array2::from_elem((cols, rows), (0, 0));

            let mut pos = start;
            let mut dir: (isize, isize) = (0, -1);

            loop {
                let next = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);

                if next.0 < 0 || next.0 >= cols as isize { break }
                if next.1 < 0 || next.1 >= rows as isize { break }

                let next = (next.0 as usize, next.1 as usize);

                if walls[next] {
                    if stop_dir[pos] == dir {
                        loops += 1;
                        break;
                    } else if stop_dir[pos] == (0, 0) {
                        stop_dir[pos] = dir;
                    }

                    dir = rotate(dir);
                } else {
                    pos = next;
                }
            }
        }
    }

    println!("part 2: {loops}");
}
