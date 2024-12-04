use std::io;

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let mut count: usize = 0;

    let strides = [
        ( 0,  1), // right
        ( 0, -1), // left
        ( 1,  0), // down
        (-1,  0), // up
        ( 1,  1), // down right
        ( 1, -1), // down left
        (-1,  1), // down right
        (-1, -1), // down left
    ];

    let target = ['X', 'M', 'A', 'S'];

    let rows = input.len();
    let cols = input[0].len();

    for (row_stride, col_stride) in strides {
        let start_row = if row_stride == -1 { target.len() - 1 } else { 0 };
        let start_col = if col_stride == -1 { target.len() - 1 } else { 0 };
        let end_row = if row_stride == 1 { rows - target.len() } else { rows - 1};
        let end_col = if col_stride == 1 { cols - target.len() } else { cols - 1};

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let match_found = (0..target.len()).all(|i| {
                    let row = (row as isize + row_stride * i as isize) as usize;
                    let col = (col as isize + col_stride * i as isize) as usize;
                    input[row][col] == target[i]
                });

                if match_found { count += 1 }
            }
        }
    }

    println!("part 1: {count}");

    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if input[row][col] == 'A' &&
                ((input[row - 1][col - 1] == 'M' && input[row + 1][col + 1] == 'S') ||
                 (input[row - 1][col - 1] == 'S' && input[row + 1][col + 1] == 'M')) &&
                ((input[row + 1][col - 1] == 'M' && input[row - 1][col + 1] == 'S') ||
                 (input[row + 1][col - 1] == 'S' && input[row - 1][col + 1] == 'M'))
            {
                count += 1;
            }
        }
    }

    println!("part 2: {count}");
}
