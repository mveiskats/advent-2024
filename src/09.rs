use std::io;

#[derive(Debug, Clone, Copy)]
struct Block {
    pub len: usize,
    pub id: Option<usize>,
}

fn main() {
    let input: Vec<usize> = io::stdin().lines().next()
        .expect("empty input")
        .expect("read error")
        .chars()
        .map(|ch| ch.to_digit(10).expect("not a digit") as usize)
        .collect();

    let mut disk: Vec<Option<usize>> = input.iter()
        .enumerate()
        .flat_map(|(i, len)| {
            let elem = if i % 2 == 1 { None } else { Some(i / 2) };
            vec![elem; *len].into_iter()
        })
        .collect();

    let mut start = 0;
    let mut end = disk.len() - 1;

    while start < end {
        if disk[end].is_none() { end -= 1; continue }
        if disk[start].is_some() { start += 1; continue }
        disk[start] = disk[end];
        disk[end] = None;
    }

    let result: usize = disk.iter()
        .flatten()
        .enumerate()
        .map(|(i, id)| i * id)
        .sum();

    println!("part 1: {result}");

    let mut disk: Vec<Block> = input.iter()
        .enumerate()
        .map(|(i, len)| {
            let id = if i % 2 == 1 { None } else { Some(i / 2) };
            Block { len: *len, id }
        })
        .collect();

    let mut current = disk.len() - 1;

    while current > 0 {
        if disk[current].id.is_none() { current -= 1; continue }

        let current_len = disk[current].len;
        let free = disk[0..current].iter()
            .position(|block| block.id.is_none() && block.len >= current_len);

        if let Some(free) = free {
            let free_len = disk[free].len;
            let file = disk[current];

            disk[free] = Block { len: free_len - current_len, id: None };
            disk[current] = Block { len: file.len, id: None };
            disk.insert(free, file);
        } else {
            current -= 1;
        }
    }

    let mut result = 0;
    let mut offset = 0;

    for block in disk.iter() {
        let id = block.id.unwrap_or(0);
        let block_range = offset..(offset + block.len);

        result += block_range.sum::<usize>() * id;
        offset += block.len;
    }

    println!("part 2: {result}");
}
