//! Test driver
use crate::utils::ParseError;
use std::io::stdout;
use std::io::Write;
use std::str::FromStr;
extern crate structopt;
use structopt::StructOpt;
pub mod basic;
pub mod pfm;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn() -> bool,
}

/// TestEvents
#[derive(Debug)]
pub enum TestEvent {
    RunAll,
    List,
}

/// Match on each supported event to parse from command line
impl FromStr for TestEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(TestEvent::RunAll),
            "list" => Ok(TestEvent::List),
            _ => Err(ParseError::InvalidEvent),
        }
    }
}

/// Configuration settings for running test
#[derive(Debug, StructOpt)]
pub struct TestOptions {
    #[structopt(short, long, help = "Event to collect", number_of_values = 1)]
    pub event: Vec<TestEvent>,

    // Allows multiple arguments to be passed, collects everything remaining on
    // the command line
    #[structopt(required = false, help = "Command to run")]
    pub command: Vec<String>,
}

/// Gathers all tests and returns a Vec with them all
pub fn make_tests() -> Vec<Test> {
    let tests: Vec<Test> = vec![
        basic::test_always_passes(),
        basic::test_always_fails(),
        basic::test_passes_after_1sec(),
        pfm::test_check_for_libpfm4(),
    ];

    tests
}

/// Runs all tests and outputs results to stdout
pub fn run_all_tests(tests: &[Test]) {
    for (index, test) in tests.iter().enumerate() {
        print!("{:>2}: {:<60} : ", index, test.description);
        stdout().flush().unwrap();
        let result = (test.call)();
        let result_text: String;
        if result {
            result_text = "Ok".to_string();
        } else {
            result_text = "\x1b[0;31mFAILED!\x1b[0m".to_string();
        }
        println!("{}", result_text);
    }
}

/// Lists all tests and outputs results to stdout
pub fn list_all_tests(tests: &[Test]) {
    for (index, test) in tests.iter().enumerate() {
        println!("{:>2}: {:<60}", index, test.description);
    }
}

/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    let tests = make_tests();
    let mut events = Vec::new();
    if options.command.is_empty() {
        events.push(TestEvent::RunAll);
    }
    for command in &options.command {
        events.push(TestEvent::from_str(command).unwrap());
    }
    for event in &events {
        match event {
            TestEvent::RunAll => run_all_tests(&tests),
            TestEvent::List => list_all_tests(&tests),
        }
    }
}
