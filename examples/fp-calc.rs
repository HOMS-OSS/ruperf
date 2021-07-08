//! A simple test program to run ruperf against
//!
//! Do some calculations
//!
//! usage:
//!     ruperf stat ./fp-calc
fn main() {
    let start: i32 = 100;
    let end: i32 = 100000000;

    for i in start..end {
        let a: f64 = i as f64;
        let b: f64 = (i - 1) as f64;

        let x = (a * b).sqrt();

        let y1: f64;
        let y2: f64;

        // Try branching
        if i % 2 == 0 {
            y1 = (x * a).sin();
            y2 = (x * b).sin();
        } else {
            y1 = (x * a).asin();
            y2 = (x * b).asin();
        }

        let _ = y1 / y2;
    }
}
