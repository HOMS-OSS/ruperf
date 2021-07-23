//! Run ruperf against some file calls
//!
//! This is a small program to run ruperf against. Write and read a file. This 
//! should call `syscall_write` and `syscall_read`. 
//! 
//! Usage:
//!     ruperf stat ./file-io
use std::fs;
use std::io::prelude::*;
use std::process;
use std::path::PathBuf;
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "File-io", about = "A test program to run ruperf against")]
struct Opt {
    /// Number of writes to a file
    #[structopt(short, long, default_value = "100000")]
    count: usize,
    /// File name prepended with PID
    #[structopt(short, long, default_value = "ruperf-fileio-test.txt")]
    file_name: String,
}

static WRITE_STRING: &str = "The quick brown fox jumped over the lazy dog\n";

fn main() {
    let opt = Opt::from_args();
    let writes = opt.count;
    let tmp_dir = env::temp_dir();
    let mut path = PathBuf::from(tmp_dir);
    path.push(format!("{}_{}", process::id(), opt.file_name));
    path.set_extension("txt");

    if path.exists() {
        match fs::remove_file(&path) {
            Err(e) => panic!("Could not remove existing file. {} because {}", path.display(), e),
            Ok(_) => (),
        }
    }

    // Create a file and write to it.
    let mut file = match fs::File::create(&path) {
        Err(e) => panic!("Could not create file {} because {}", path.display(), e),
        Ok(file) => file,
    };

    for _ in 0..writes {
        match file.write_all(WRITE_STRING.as_bytes()) {
            Err(e) => {
                fs::remove_file(&path).unwrap();
                panic!("Failed to write to file because {}", e);
            },
            Ok(_) => (),
        }
    }

    // Close file
    drop(file);

    // Open same file and read from it.
    let mut file = match fs::File::open(&path) {
        Err(e) => {
            fs::remove_file(&path).unwrap();
            panic!("Could not open the file because {}", e);
        },
        Ok(file) => file,
    };

    let mut contents: Vec::<u8> = Vec::new();
    let size: usize;
    match file.read_to_end(&mut contents) {
        Err(e) => {
            fs::remove_file(&path).unwrap();
            panic!("Could not read the entire file because {}", e);
        },
        Ok(sz) => size = sz,
    }

    assert!(size == contents.len());

    match fs::remove_file(&path) {
        Err(e) => panic!("Could not remove file {} because {}", path.display(), e),
        Ok(_) => ()
    }
}
