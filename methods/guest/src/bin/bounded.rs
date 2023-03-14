#![no_main]
#![no_std]
#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(any(target_os = "zkvm", doc))]
use risc0_zkvm::guest::env;
#[cfg(any(target_os = "zkvm", doc))]
risc0_zkvm::guest::entry!(main);

/*
Determine if the polynomial described by the given set of points is contained within a unit circle centered at (0,0)

Determine if a square described by given points is contained within unit circle

Determine if a given point is contained within the unit circle
*/

pub fn main() {
    // let a: f32 = env::read();
    // let b: f32 = env::read();

    // // Verify factors are non-trivial
    // if a == 1 || b == 1 {
    //     panic!("Trivial factors")
    // }

    // // Compute the product checking for integer overflow
    // let _product = is_bounded(&[a, b]);//.expect("Integer overflow");
}

pub fn is_bounded(arr: &[(f32, f32)]) -> bool {
    for point in arr.iter() {
        let radius_squared: f32 = point.0 * point.0 + point.1 * point.1;
        if radius_squared >= 1. {
            return false;
        }
    }
    
    return true;
}
