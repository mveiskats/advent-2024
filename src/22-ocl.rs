use std::io;
use itertools::{ iterate, Itertools };

extern crate ocl;
use ocl::{ Buffer, ProQue };

fn best_price(prices: Vec<Vec<i8>>) -> ocl::Result<i32> {
    let src = r#"
        __kernel void best_price(__constant char* prices, __constant char* deltas, __global int* results, uint rows, uint cols) {
            uint a = (uint) get_global_id(0);

            char4 target;

            for(uchar i = 0; i < 4; i++) {
                target[i] = (char) ((a % 19) - 9);
                a /= 19;
            }

            int price = 0;
            for(uint row = 0; row < rows; row++) {
                for(int col = 0; col < cols - 4; col++) {
                    uint offset = row * cols + col;
                    __constant char* window = deltas + offset;

                    if (window[0] == target[0] &&
                        window[1] == target[1] &&
                        window[2] == target[2] &&
                        window[3] == target[3])
                    {
                        price += prices[offset + 3];
                        break;
                    }
                }
            }

            results[get_global_id(0)] = price;
        }
    "#;

    // -9 to 9 contains 19 values
    let dims = 19 * 19 * 19 * 19;

    let pro_que = ProQue::builder()
        .src(src)
        .dims(dims)
        .build()?;

    let flat_prices: Vec<i8> = prices.iter().flat_map(|seq| seq.iter().copied().skip(1)).collect();
    let flat_deltas: Vec<i8> = prices.iter().flat_map(|seq| seq.iter().tuple_windows().map(|(a, b)| b - a)).collect();

    let price_buffer: Buffer<i8> = pro_que.buffer_builder().len(flat_prices.len()).build()?;
    let delta_buffer: Buffer<i8> = pro_que.buffer_builder().len(flat_deltas.len()).build()?;
    let result_buffer: Buffer<i32> = pro_que.create_buffer()?;

    price_buffer.write(&flat_prices).enq()?;
    delta_buffer.write(&flat_deltas).enq()?;

    let kernel = pro_que.kernel_builder("best_price")
        .arg(&price_buffer)
        .arg(&delta_buffer)
        .arg(&result_buffer)
        .arg(prices.len() as u32)
        .arg(2000u32)
        .build()?;

    unsafe { kernel.enq()?; }

    let mut result = vec![0i32; result_buffer.len()];
    result_buffer.read(&mut result).enq()?;

    Ok(result.into_iter().max().expect("no prices"))
}

fn next_secret(secret: usize) -> usize {
    let secret = (secret ^ (secret << 6)) % 16777216;
    let secret = (secret ^ (secret >> 5)) % 16777216;
    (secret ^ (secret << 11)) % 16777216
}

fn main() {
    let input: Vec<usize> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.parse().expect("not a number"))
        .collect();

    let secret_sequences: Vec<Vec<usize>> = input.into_iter()
        .map(|secret| iterate(secret, |&a| next_secret(a)).take(2001).collect())
        .collect();

    let result: usize = secret_sequences.iter().map(|s| s[2000]).sum();

    println!("part 1: {result}");

    let prices: Vec<Vec<i8>> = secret_sequences.into_iter()
        .map(|seq| seq.into_iter().map(|i| (i % 10) as i8).collect())
        .collect();

    let result = best_price(prices);
    match result {
        Err(e) => eprintln!("{}", e),
        Ok(val) => println!("part 2: {val}")
    }
}
