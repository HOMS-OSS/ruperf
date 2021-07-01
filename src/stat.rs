//! Stat driver
use crate::utils::ParseError;
use std::str::FromStr;
extern crate structopt;
use structopt::StructOpt;
///enum of supported events

#[derive(Debug)]
pub enum StatEvent {
    Cycles,
}

///match on each supported event to parse from command line
impl FromStr for StatEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cycles" => Ok(StatEvent::Cycles),
            _ => Err(ParseError::InvalidEvent),
        }
    }
}

///Configuration settings for running stat
#[derive(Debug, StructOpt)]
pub struct StatOptions {
    #[structopt(short, long, help = "Event to collect")]
    pub event: StatEvent,
    //allows multiple arguments to be passed, collects everything after the event name
    #[structopt(required = false, multiple = true, help = "Command to run")]
    pub command: Vec<String>,
}

pub fn run_stat(options: &StatOptions) {
    println!("{:?}:\n {:?}", options.command, options.event);
}
