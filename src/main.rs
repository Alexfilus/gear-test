pub mod parallel;
mod parallel_tests;

use crate::parallel::parallel_computation;

const THRESHOLD: usize = 1000;

fn main() {
    let data: Vec<u32> = (1..=12345).collect();
    let result: Vec<u32> = parallel_computation(data, |x| x * x, THRESHOLD);
    println!("{:?}", result);
}

