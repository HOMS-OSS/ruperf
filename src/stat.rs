//! Stat driver
use crate::event::open::*;
use crate::utils::ParseError;
use std::str::FromStr;
extern crate structopt;
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
    println!("{:?}:\n {:?}", options.command, options.event);
    //demonstrating from cli. In future rather than starting and stopping counter in series for each event, events will have the ability to be added in groups that will coordinate their timing.
    for event in &options.event {
        let e = Event::new(*event);
        let cnt: isize = e.start_counter().unwrap();
        let mut sum = 0;
        let additions = 1000000;
        for i in 0..additions {
            if i % 2 == 0 {
                sum += 1;
            } else {
                sum -= 1;
            }
        }
        let final_cnt = e.stop_counter().unwrap();
        let total_cnt = final_cnt - cnt;
        println!("Counter value for {:?}: {} at start\n", event, cnt);
        println!(
            "Counter value for {:?}: {} at stop after counting to {}\nNumber of cycles: {}\n",
            event, final_cnt, sum, total_cnt
        );
        println!(
            "Performed {:?} additions per cycle",
            additions as f64 / total_cnt as f64
        );
    }
}
