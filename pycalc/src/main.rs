use std::time::Instant;

fn main() {
    let max_size: i64 = 10000;
    let middle: i64 = max_size / 2;
    let square_mid: i64 = middle.pow(2);
    let mut calc: i64 = 0;

    let start = Instant::now();

    for x in 0..max_size {
        let x_dist = (x - middle).pow(2);
        for y in 0..max_size {
            let dist = x_dist + (y - middle).pow(2);
            if dist < square_mid {
                calc += 1;
            }
        }
    }

    println!("Pi : {}", calc as f64 / max_size.pow(2) as f64 * 4.0);
    println!("Compute time : {} seconds", start.elapsed().as_secs_f32());
}
