//! Run ruperf against some file calls
//!
//! This is a small program to run ruperf against. Write and read a file. This 
//! should call `syscall_write` and `syscall_read`. 
//! 
//! The test environment should not need to be a create and should only include
//! std::* libs.
//!
//! Run `make clean` to remove the file.
//!
//! Usage:
//!     ruperf stat ./file-io
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static WRITE_STRING: &str = "The quick brown fox jumped over the lazy dog\n";

fn main() {
    let writes = 10000;
    let path = Path::new("read-write-test.txt");

    // Create a file and write to it.
    if path.exists() {
        println!("Test read/write path exists. If this is a mistake, run `make clean`");
        return    
    }

    let mut file = match File::create(&path) {
        Err(e) => panic!("Could not create file {} because {}", path.display(), e),
        Ok(file) => file,
    };

    for _ in 0..writes {
        match file.write_all(WRITE_STRING.as_bytes()) {
            Err(e) => panic!("Failed to write to file because {}", e),
            Ok(_) => (),
        }
    }

    // Close file
    drop(file);

    // Open same file and read from it.
    let mut file = match File::open(&path) {
        Err(e) => panic!("Could not open the file because {}", e),
        Ok(file) => file,
    };

    let mut contents: Vec::<u8> = Vec::new();
    let size: usize;
    match file.read_to_end(&mut contents) {
        Err(e) => panic!("Could not read the entire file because {}", e),
        Ok(sz) => size = sz,
    }

    assert!(size == contents.len());
}
