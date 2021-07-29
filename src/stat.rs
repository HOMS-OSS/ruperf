//! Stat driver
use crate::event::open::*;
use crate::utils::ParseError;
use std::os::unix::process::CommandExt;
use std::process;
use std::str::{self, FromStr};
use std::sync::mpsc::sync_channel;
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

/// Match on each supported event to parse from command line
impl ToString for StatEvent {
    fn to_string(&self) -> String {
        match self {
            StatEvent::Cycles => "cycles".to_string(),
            StatEvent::Instructions => "instructions".to_string(),
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
    struct EventCounter {
        event: Event,
        start: isize,
        stop: isize,
    }

    let mut event_list: Vec<EventCounter> = Vec::new();

    // Sets up a synchronous bounded channel,
    // with a buffer size of 1.
    // The sender thread will be blocked until
    // the receiver thread has received the data.
    let (tx, rx) = sync_channel::<u32>(1);
    let mut child = unsafe {
        process::Command::new(&options.command[0])
            .args(&options.command[1..])
            .pre_exec(move || Ok(tx.send(0).unwrap()))
            .spawn()
            .unwrap()
    };

    for event in &options.event {
        let e = Event::new(*event, Some(&child.id()));
        let start = e.start_counter().unwrap();
        event_list.push(EventCounter {
            event: e,
            start: start,
            stop: 0,
        });
    }

    // Receive data from `tx`
    // Yes, it returns a result.
    // No I am not handling it right now.
    rx.recv();
    // create another process from command
    child.wait().expect("Failed to execute command");

    for e in event_list.iter_mut() {
        e.stop = e.event.stop_counter().unwrap();
    }

    //output command's output
    println!(
        "Performance counter stats for '{}'\n",
        options.command.get(0).unwrap()
    );
    for event in event_list {
        println!(
            " Number of {}: {}\n",
            event.event.event.to_string(),
            event.stop - event.start
        );
    }
}
