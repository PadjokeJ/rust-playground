use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
struct Max {
    max: usize,
}

fn eratosthene(max: usize) -> Vec<i8> {
    let mut nums: Vec<i8> = vec![0; max];

    let start = Instant::now();
    for i in 2..=((max as f64).sqrt() as usize) {
        if nums[i] != -1 {
            for j in (i * i..max).step_by(i) {
                nums[j] = -1;
            }
        }
    }

    let t = start.elapsed();
    println!("program executed in {} seconds", t.as_secs_f32());

    for i in 2..max {
        if nums[i] != -1 {
            print!("{}, ", i);
        }
    }

    return nums;
}

fn main() {
    let args = Max::parse();

    eratosthene(args.max);
}
