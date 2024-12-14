use std::io;
use ndarray::Array2;

fn main() {
    let mut input: Vec<Vec<_>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.chars().collect())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    // pad with . around all sides to simplify bounds checks
    for row in input.iter_mut() {
        row.insert(0, '.');
        row.push('.');
    }
    input.insert(0, vec!['.'; cols + 2]);
    input.push(vec!['.'; cols + 2]);


    let flat_input = input.into_iter().flatten().collect();
    let map = Array2::from_shape_vec((rows + 2, cols + 2), flat_input)
        .expect("input size mismatch");

    let mut regions: Array2<Option<usize>> = Array2::from_elem(map.dim(), None);

    let mut region_sizes: Vec<usize> = vec![0];
    let mut region_perimeters: Vec<usize> = vec![0];

    // Manually mark edges
    for row in 0..(rows + 1) {
        regions[(row, 0)] = Some(0);
        regions[(row, cols + 1)] = Some(0);
    }

    for col in 0..(cols + 2) {
        regions[(0, col)] = Some(0);
        regions[(rows + 1, col)] = Some(0);
    }

    while let Some((start, None)) = regions.indexed_iter().find(|(_, val)| val.is_none()) {
        let ch = map[start];
        let idx = region_sizes.len();
        let mut search: Vec<(usize, usize)> = vec![start];

        let mut size = 0;
        let mut perimeter = 0;

        while let Some(pos) = search.pop() {
            if regions[pos].is_some() { continue }

            regions[pos] = Some(idx);
            size += 1;

            let (row, col) = pos;
            if map[(row - 1, col)] == ch { search.push((row - 1, col)) }
            if map[(row + 1, col)] == ch { search.push((row + 1, col)) }
            if map[(row, col - 1)] == ch { search.push((row, col - 1)) }
            if map[(row, col + 1)] == ch { search.push((row, col + 1)) }

            if map[(row - 1, col)] != ch { perimeter += 1 }
            if map[(row + 1, col)] != ch { perimeter += 1 }
            if map[(row, col - 1)] != ch { perimeter += 1 }
            if map[(row, col + 1)] != ch { perimeter += 1 }
        }

        region_sizes.push(size);
        region_perimeters.push(perimeter);
    }

    let result: usize = region_sizes[1..].iter().zip(region_perimeters[1..].iter())
        .map(|(size, perimeter)| size * perimeter)
        .sum();

    println!("part 1: {result}");

    let regions = regions.map(|val| val.expect("unvisited plot"));
    let mut region_sides: Vec<usize> = vec![0; region_sizes.len()];

    for window in regions.windows((2, 2)) {
        let is_fence = window[(0, 1)] != window[(1, 1)];
        let was_fence = window[(0, 0)] != window[(1, 0)];

        // Top sides
        let new_region = window[(0, 0)] != window[(0, 1)];

        if is_fence && (!was_fence || new_region) {
            region_sides[window[(0, 1)]] += 1;
        }

        // Bottom sides
        let new_region = window[(1, 0)] != window[(1, 1)];

        if is_fence && (!was_fence || new_region) {
            region_sides[window[(1, 1)]] += 1;
        }

        let is_fence = window[(1, 0)] != window[(1, 1)];
        let was_fence = window[(0, 0)] != window[(0, 1)];

        // Left sides
        let new_region = window[(0, 0)] != window[(1, 0)];

        if is_fence && (!was_fence || new_region) {
            region_sides[window[(1, 0)]] += 1;
        }

        // Right sides
        let new_region = window[(0, 1)] != window[(1, 1)];

        if is_fence && (!was_fence || new_region) {
            region_sides[window[(1, 1)]] += 1;
        }
    }

    let result: usize = region_sizes[1..].iter().zip(region_sides[1..].iter())
        .map(|(size, sides)| size * sides)
        .sum();

    println!("part 2: {result}");
}
