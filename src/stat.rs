//! Stat driver
use crate::event::open::*;
use crate::utils::ParseError;
use std::str::{self, FromStr};
extern crate structopt;
use std::process::Command;
use structopt::StructOpt;

/// Supported events
#[derive(Debug, Copy, Clone)]
pub enum StatEvent {
    Cycles,
    Instructions,
}

/// Match on each supported event to parse from command line
impl FromStr for StatEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cycles" => Ok(StatEvent::Cycles),
            "instructions" => Ok(StatEvent::Instructions),
            _ => Err(ParseError::InvalidEvent),
        }
    }
}

/// Configuration settings for running stat
#[derive(Debug, StructOpt)]
pub struct StatOptions {
    #[structopt(short, long, help = "Event to collect", number_of_values = 1)]
    pub event: Vec<StatEvent>,

    // Allows multiple arguments to be passed, collects everything remaining on
    // the command line
    #[structopt(required = false, help = "Command to run")]
    pub command: Vec<String>,
}

/// Run perf stat on the given command and event combinations. Currently starts and stops a cycles timer in serial for each event specified.
pub fn run_stat(options: &StatOptions) {
    //demonstrating from cli. In future rather than starting and stopping counter in series for each event, events will have the ability to be added in groups that will coordinate their timing.

    for event in &options.event {
        let e = Event::new(*event);
        let cnt: isize = e.start_counter().unwrap();

        //create another process from command
        let output = Command::new(options.command.get(0).unwrap())
            .output()
            .expect("failed to execute process");

        let final_cnt = e.stop_counter().unwrap();
        let total_cnt = final_cnt - cnt;

        // Create buffer variable
        let buf = &output.stdout;

        // Convert &vec[u8] into string
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        //output command's output
        println!(
            "{}\nPerformance counter stats for '{}'\n",
            s.to_string(),
            options.command.get(0).unwrap()
        );
        println!(" Number of cycles: {}\n", total_cnt);
    }
}
