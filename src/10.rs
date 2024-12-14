use std::io;
use ndarray::Array2;

struct TrailSearch {
    pub visited: Array2<bool>,
}

impl TrailSearch {
    pub fn scores(map: &Array2<usize>, start: (usize, usize)) -> (usize, usize) {
        let mut search = Self { visited: Array2::from_elem(map.dim(), false) };
        search.visit(map, start, 0)
    }

    fn neighbours(map: &Array2<usize>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let (x_dim, y_dim) = map.dim();

        let mut result: Vec<(usize, usize)> = vec![];

        if x > 0 { result.push((x - 1, y)) }
        if y > 0 { result.push((x, y - 1)) }
        if x + 1 < x_dim { result.push((x + 1, y)) }
        if y + 1 < y_dim { result.push((x, y + 1)) }

        result
    }

    fn visit(&mut self, map: &Array2<usize>, pos: (usize, usize), height: usize) -> (usize, usize) {
        if map[pos] != height { return (0, 0) }

        if height == 9 {
            let visited = self.visited[pos];
            self.visited[pos] = true;

            return (!visited as usize, 1)
        }

        Self::neighbours(map, pos).into_iter()
            .map(|pos| self.visit(map, pos, height + 1))
            .fold((0,0), |acc, score| (acc.0 + score.0, acc.1 + score.1))
    }
}

fn main() {
    let input: Vec<Vec<_>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("not a digit") as usize)
                .collect()
        })
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let flat_input = input.into_iter().flatten().collect();
    let map = Array2::from_shape_vec((rows, cols), flat_input)
        .expect("input size mismatch");

    let starts: Vec<_> = map.indexed_iter()
        .filter(|(_, &height)| height == 0)
        .map(|((x, y), _)| (x, y))
        .collect();

    let (score1, score2) = starts.iter()
        .map(|start| TrailSearch::scores(&map, *start))
        .fold((0,0), |acc, score| (acc.0 + score.0, acc.1 + score.1));

    println!("part 1: {score1}");
    println!("part 1: {score2}");
}
