use std::io;
use std::collections::HashMap;

struct Transformer {
    mem: HashMap<(usize, usize), usize>
}

impl Transformer {
    pub fn new() -> Self {
        Self { mem: HashMap::new() }
    }

    pub fn transform_size(&mut self, a: usize, times: usize) -> usize {
        if times == 0 { return 1 }

        let key = (a, times);
        match self.mem.get(&key) {
            Some(len) => *len,
            None => {
                let size = Self::transform_once(a).into_iter()
                    .map(|a| self.transform_size(a, times - 1))
                    .sum();

                self.mem.insert(key, size);
                size

            }
        }
    }

    fn transform_once(a: usize) -> Vec<usize> {
        if a == 0 { return vec![1] }

        // Quite a bit faster than splitting via converting to string and back
        // Not really necessary with memoization though
        let mut len = 1;
        let mut lim = 10;
        let mut half_lim = 1;

        while a >= lim {
            len += 1;
            lim *= 10;
            if len % 2 == 0 { half_lim *= 10 }
        }

        if len % 2 == 0 {
            let left = a / half_lim;
            let right = a % half_lim;

            return vec![left, right];
        }

        vec![a * 2024]
    }
}

fn main() {
    let input: Vec<_> = io::stdin().lines().next()
        .expect("empty input")
        .expect("read error")
        .split(" ")
        .map(|s| s.parse::<usize>().expect("not a number"))
        .collect();

    let mut transformer = Transformer::new();

    let result: usize = input.iter().map(|&a| transformer.transform_size(a, 25)).sum();
    println!("part 1: {}", result);

    let result: usize = input.iter().map(|&a| transformer.transform_size(a, 75)).sum();
    println!("part 2: {}", result);
}
