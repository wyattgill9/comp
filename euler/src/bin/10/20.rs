/*
WORKING WITH SIMD, RAYON TO OPTIMIZE THIS PROBLEM aswell as using parralelization!
ACTUAL SOLUTION 648 using cpythons solution.
*/

#![feature(portable_simd)]

use rayon::prelude::*;
use std::simd::{u32x8}; 

fn factorial_simd(n: u32) -> u32 {
    let mut result = 1;
    
    let chunks: Vec<u32> = (1..=n).collect();
    
    let parallel_factorial: u32 = chunks.par_chunks(8)
        .map(|chunk| {
            let simd_vec = u32x8::from_array([
                *chunk.get(0).unwrap_or(&1),
                *chunk.get(1).unwrap_or(&1),
                *chunk.get(2).unwrap_or(&1),
                *chunk.get(3).unwrap_or(&1),
                *chunk.get(4).unwrap_or(&1),
                *chunk.get(5).unwrap_or(&1),
                *chunk.get(6).unwrap_or(&1),
                *chunk.get(7).unwrap_or(&1),
            ]);

            simd_vec[0] * simd_vec[1] * simd_vec[2] * simd_vec[3] *
            simd_vec[4] * simd_vec[5] * simd_vec[6] * simd_vec[7]
        })
        .sum();

    result * parallel_factorial
}

fn main() {
    let n = 100;
    let fact = factorial_simd(n).to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("{}", fact);
}