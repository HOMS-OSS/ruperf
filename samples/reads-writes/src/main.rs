//! A simple test program to run ruperf against
//!
//! Do some calculations
//!
//! usage:
//!     ruperf stat ./reads-writes
//!
//! Note this program must be built with +nightly specified:
//!      cargo +nightly build -p reads-writes

#![feature(asm)]
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "fp-calc", about = "A test program to run ruperf against")]
struct Opt {
    // Starting position
    #[structopt(short, long, default_value = "10000000")]
    reads: isize,
    #[structopt(short, long, default_value = "2000000")]
    writes: isize,
}

#[inline(never)]
fn read_write(reads: isize, writes: isize) {
    unsafe {
        asm!(
            "sub rsp, 8",
            "xor rbx, rbx",
            "2:", // Loop over writes
            "mov [rsp], rbx",
            "sub rsi, 1",
            "jg 2b",
            "2:", // Loop over reads
            "mov rbx, [rsp]",
            "sub rdi, 1",
            "jg 2b",
            "add rsp, 8",
            in("rdi") reads,
            in("rsi") writes,
        );
    }
}

fn clamp(value: isize, min: isize) -> isize {
    if value >= min {
        value
    } else {
        min
    }
}
fn main() {
    let opt = Opt::from_args();
    let reads: isize = clamp(opt.reads, 1);
    let writes: isize = clamp(opt.writes, 1);

    println!("Performing {} reads and {} writes\n", reads, writes);
    read_write(reads, writes);
}
