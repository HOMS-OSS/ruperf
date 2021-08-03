//! # Stat driver.
//! <p> Usage: <em> ruperf stat [COMMAND] [ARGS] </em>
//! Where COMMAND and ARGS are a shell command and it's arguments. </p>

extern crate structopt;
use crate::event::open::*;
use crate::utils::ParseError;
use os_pipe::pipe;
use std::io::prelude::*;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::str::{self, FromStr};
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

fn launch_command_process(
    command: Vec<String>,
    mut child_reader: os_pipe::PipeReader,
    mut child_writer: os_pipe::PipeWriter,
) -> i32 {
    match unsafe { libc::fork() as i32 } {
        0 => {
            //set up command to execute and initialize read buffer
            let mut buf = [0];
            let mut comm = Command::new(&command[0]);
            comm.args(&command[1..]);

            // Tell parent program child is set up to execute
            child_writer.write_all(&[1]).unwrap();
            drop(child_writer);

            //hear from parent that counters are set up
            let nread = child_reader.read(&mut buf).unwrap();
            assert_eq!(nread, 1);

            let e = comm.exec();
            panic!("child command failed: {}", e);
        }
        pid_child => pid_child,
    }
}

/// Run perf stat on the given command and event combinations.
/// Currently starts and stops a cycles timer in serial for each event specified.
pub fn run_stat(options: &StatOptions) {
    // In future rather than starting and stopping counter
    // in series for each event, events will have the ability
    // to be added in groups that will coordinate their timing.
    struct EventCounter {
        event: Event,
        start: isize,
        stop: isize,
    }

    let mut event_list: Vec<EventCounter> = Vec::new();

    let (reader, mut writer) = pipe().unwrap();
    let (mut parent_reader, parent_writer) = pipe().unwrap();

    let child_reader = reader.try_clone().unwrap();
    let child_writer = parent_writer.try_clone().unwrap();
    let pid_child = launch_command_process(options.command.clone(), child_reader, child_writer);

    for event in &options.event {
        event_list.push(EventCounter {
            event: Event::new(*event, Some(pid_child)),
            start: 0,
            stop: 0,
        });
    }

    // Wait for child to say it is set up to execute.
    let mut buf = [0];
    let nread = parent_reader.read(&mut buf).unwrap();
    assert_eq!(nread, 1);

    for e in event_list.iter_mut() {
        e.start = e.event.start_counter().unwrap();
    }

    // Notify child counters are set up.
    writer.write_all(&[1]).unwrap();
    drop(writer);

    // Wait for process to exit.
    let mut status: libc::c_int = 0;
    let result = unsafe { libc::waitpid(pid_child, (&mut status) as *mut libc::c_int, 0) };
    assert_eq!(result, pid_child);

    for e in event_list.iter_mut() {
        e.stop = e.event.stop_counter().unwrap();
    }

    println!(
        "Performance counter stats for '{}:'\n",
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
