//! Sample program to run `ruperf stat` against.
//! Performs some floating point computations.

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "fp-calc", about = "A test program to run ruperf against")]
struct Opt {
    // Starting position
    #[structopt(short, long, default_value = "100")]
    start: i32,

    // Ending position
    #[structopt(short, long, default_value = "10000000")]
    end: i32,
}

fn main() {
    let opt = Opt::from_args();
    let start: i32 = opt.start;
    let end: i32 = opt.end;

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
