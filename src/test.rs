//! # Test driver.
//! <p> Usage: <em> ruperf test [OPTION] </em>
//! where OPTION is one of:
//! <ul>
//! <li>v, verbose</li>
//! <li>l, list</li>
//! <li>j, json</li>
//! <li>s, skip</li>
//! <li>o, only</li>
//! </ul>

mod basic;
mod events;
mod paranoid;
mod pfm;
mod testutils;

extern crate structopt;
use structopt::StructOpt;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn(&RunSettings) -> TestResult,
    pub subtests: Vec<Test>,
    pub is_subtest: bool,
}

/// TestResult
#[derive(Clone)]
pub enum TestResult {
    Passed,
    Failed(String),
    Skipped,
}

/// Configuration settings for running test
#[derive(Debug, StructOpt)]
pub struct TestOptions {
    // Verbose flag, provides additional output
    #[structopt(short = "v", long = "verbose", help = "provide additional output")]
    pub verbose: bool,

    // Should list runnable tests instead of performing them
    #[structopt(
        short = "l",
        long = "list",
        help = "list runnable tests instead of running"
    )]
    pub should_list: bool,

    // Should format output as json
    #[structopt(short = "j", long = "json", help = "format output as json")]
    pub json: bool,

    // A comma-seperated list of tests to skip
    #[structopt(
        short = "s",
        long = "skip",
        help = "a comma-seperated list of tests to skip",
        default_value = ""
    )]
    pub to_skip: String,

    // A comma-seperated list of tests to run
    #[structopt(
        short = "o",
        long = "only",
        help = "a comma-seperated list of tests to run",
        default_value = ""
    )]
    pub to_run: String,
}

pub struct RunSettings {
    pub verbose: bool,
    pub json: bool,
}

/// Handles the running of the "test" command.
pub fn run_test(options: &TestOptions) {
    let mut to_skip: Vec<String> = Vec::new();
    let tests = testutils::make_tests();
    if !options.to_skip.is_empty() {
        for s in options.to_skip.split(',') {
            to_skip.push(s.to_string());
        }
    }
    if options.should_list {
        testutils::list_all_tests(&tests);
        return;
    }
    let settings = RunSettings {
        verbose: options.verbose || options.json,
        json: options.json,
    };
    if !options.to_run.is_empty() {
        to_skip = (0..tests.len()).map(|x| x.to_string()).collect();
        for s in options.to_run.split(',') {
            to_skip.retain(|x| *x != s);
        }
    }
    testutils::run_all_tests(&tests, &to_skip, &settings);
}
