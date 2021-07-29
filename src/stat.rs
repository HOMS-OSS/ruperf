//! Stat driver
use crate::event::open::*;
use crate::utils::ParseError;
use std::str::{self, FromStr};
extern crate structopt;
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
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

fn create_launch_thread(
    command: Vec<String>,
    pid_pipe: Stdio,
    //    pid_sender: SyncSender<u32>,
    // ack_receiver: Receiver<u32>
) -> thread::JoinHandle<io::Result<Child>> {
    thread::spawn(move || create_work_load(command.to_owned(), pid_pipe))
}
fn create_work_load(
    command: Vec<String>,
    pid_pipe: Stdio,
    // pid_sender: SyncSender<u32>,
    // ack_receiver: Receiver<u32>,
) -> io::Result<Child> {
    unsafe {
        Command::new(&command[0])
            .args(&command[1..])
            .stdout(pid_pipe)
            //pre_exec is blocking!
            .pre_exec(move || work_load())
            .spawn()
    }
}
fn work_load(// command: Vec<String>,
    //pid_sender: SyncSender<u32>,
    // ack_receiver: Receiver<u32>,
) -> io::Result<()> {
    io::stdout()
        .write_all(&std::process::id().to_ne_bytes())
        .expect("Failed to write to pipe");
    //original attempt to send mpsc and then execvp. Of course execvp just takes over the process, would need to pair it with fork. And mpsc doesn't seem to communicate between processes

    // pid_sender
    //    .send(std::process::id())
    //   .expect("interthread communication failure");
    // if ack_receiver
    //     .recv()
    //     .expect("Failed interthread communication")
    //     == 0
    {
        // let c_s = std::ffi::CString::new(command[0].as_str()).unwrap();
        // let f_ptr = c_s.as_bytes().as_ptr() as *const i8;
        // let a_s: Vec<_> = command[1..]
        //     .iter()
        //     .map(|arg| std::ffi::CString::new(arg.as_str()).unwrap())
        //     .collect();
        // let mut a_ptr: Vec<_> = a_s.iter().map(|arg| arg.as_ptr()).collect();
        // a_ptr.push(std::ptr::null());
        // let p: *const *const std::os::raw::c_char = a_ptr.as_ptr();
        // unsafe { libc::execvp(f_ptr, p) };

        //sometimes I just need to waste some time
        // let mut sum = 0;
        // for i in 0..1000000 {
        //     if i % 2 == 0 {
        //         sum += i;
        //     } else {
        //         sum -= i;
        //     }
        // }
        // println!("Sum {}\n", sum);
    }
    // std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
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
    //mpsc route didn't work, they don't seem to communicate across processes
    let (pid_sender, pid_receiver) = sync_channel::<u32>(0);
    let (ack_sender, ack_receiver) = sync_channel::<u32>(0);

    //this pipe attempt aso not working, no support for reading from child that I can find
    let pid_pipe = Stdio::piped();
    println!("My id: {}\n", std::process::id());
    let mut child = create_launch_thread(options.command.clone(), pid_pipe);

    let mut pid_arr: [u8; std::mem::size_of::<u32>()];
    let pid_child = pid_pipe.read_exact(&pid_arr); // pid_receiver
                                                   // .recv()
                                                   // .expect("Failed interthread communication");
    println!("My id: {}, Spawned Id {}\n", std::process::id(), pid_child);

    for event in &options.event {
        event_list.push(EventCounter {
            event: Event::new(*event, Some(pid_child)),
            start: 0,
            stop: 0,
        });
    }
    println!("Events Initialized\n");
    // ack_sender
    //     .send(0)
    //     .expect("Failed interthread communication");
    for e in event_list.iter_mut() {
        e.start = e.event.start_counter().unwrap();
    }
    println!("Counters Started, waiting for child completion\n");

    //wait for thread and process
    child
        .join()
        .expect("Failed to wait for thread")
        .expect("Failed to wait for process")
        .wait()
        .expect("Faild to wait for process\n");

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
