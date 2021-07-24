//! Test driver
extern crate structopt;
use structopt::StructOpt;
mod basic;
mod pfm;
mod testutils;

/// Test Struct
pub struct Test {
    pub name: String,
    pub description: String,
    pub call: fn(&RunSettings) -> TestResult,
    pub subtests: Vec<Test>,
    pub is_subtest: bool,
}

/// TestResult
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

    #[structopt(short = "l", long = "list", help = "list runnable tests")]
    pub should_list: bool,

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
        verbose: options.verbose,
    };
    if !options.to_run.is_empty() {
        to_skip = (0..tests.len()).map(|x| x.to_string()).collect();
        for s in options.to_run.split(',') {
            to_skip.retain(|x| *x != s);
        }
    }
    testutils::run_all_tests(&tests, &to_skip, &settings);
}
