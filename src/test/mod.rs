//! Test driver
use crate::utils::ParseError;
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
    let mut tests: Vec<Test> = Vec::new();

    // from basic.rs
    tests.push(basic::test_always_passes());
    tests.push(basic::test_always_fails());

    // from pfm.rs
    tests.push(pfm::test_check_for_libpfm4());

    return tests;
}

/// Runs all tests and outputs results to stdout
pub fn run_all_tests() {
    println!("Running Sanity Tests\n");
    let tests: Vec<Test> = make_tests();
    for (index, test) in tests.iter().enumerate() {
        let result = (test.call)();
        let result_text: String;
        if result {
            result_text = "Ok".to_string();
        } else {
            result_text = "\x1b[0;31mFAILED!\x1b[0m".to_string();
        }
        println!("{:<2}: {:<60} : {}", index, test.description, result_text);
    }
}

/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    println!("event: {:#?}", options.event);
    println!("command: {:#?}", options.command);
    run_all_tests();
}
