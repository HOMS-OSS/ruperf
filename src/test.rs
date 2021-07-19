//! Test driver
use crate::utils::ParseError;
use std::str::FromStr;
extern crate structopt;
use structopt::StructOpt;
mod basic;
mod pfm;
mod testutils;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn() -> bool,
    pub subtests: Vec<Test>,
    pub is_subtest: bool,
}

/// TestResult
pub enum TestResult {
    Passed,
    Failed,
    Skipped,
}

/// TestEvents
#[derive(Debug)]
pub enum TestEvent {
    RunAll,
    RunSome,
    RunWithSkips,
    List,
    Invalid(String),
}

/// Match on each supported event to parse from command line
impl FromStr for TestEvent {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(TestEvent::RunAll),
            "list" => Ok(TestEvent::List),
            "-s" | "--skip" => Ok(TestEvent::RunWithSkips),
            "-o" | "--only" => Ok(TestEvent::RunSome),
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
/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    let tests = testutils::make_tests();
    let mut events = Vec::new();
    let mut to_skip: Vec<String> = Vec::new();
    if options.command.is_empty() {
        events.push(TestEvent::RunAll);
    }
    for (index, command) in options.command.iter().enumerate() {
        let possible_event = TestEvent::from_str(command);
        let event = match possible_event {
            Ok(e) => e,
            Err(_) => TestEvent::Invalid(command.to_string()),
        };
        match event {
            TestEvent::RunWithSkips => {
                for test_to_skip in &options.command[index + 1..] {
                    let parsed = test_to_skip.trim().parse();
                    match parsed {
                        Ok(number) => to_skip.push(number),
                        Err(_) => {}
                    }
                }
                events.push(TestEvent::RunWithSkips);
                break;
            }
            TestEvent::RunSome => {
                to_skip = (0..tests.len()).map(|x| x.to_string()).collect();
                for test_to_run in &options.command[index + 1..] {
                    let parsed: Result<String, _> = test_to_run.trim().parse();
                    match parsed {
                        Ok(number) => {
                            // remove number from to_skip
                            to_skip.retain(|x| *x != number);
                        }
                        Err(_) => {}
                    }
                }
                events.push(TestEvent::RunSome);
                break;
            }
            _ => events.push(event),
        }
    }
    for event in &events {
        match event {
            TestEvent::RunAll | TestEvent::RunWithSkips | TestEvent::RunSome => {
                testutils::run_all_tests(&tests, &to_skip)
            }
            TestEvent::List => testutils::list_all_tests(&tests),
            TestEvent::Invalid(s) => {
                println! {"Unknown command {}", s};
                /*TODO print usage..?*/
            }
        }
    }
}
