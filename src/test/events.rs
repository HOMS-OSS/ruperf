//! events.rs contains sanity tests for each of the
//! events recordable by the event start_counter() and
//! stop_counter() methods. A simple program with a lot
//! of instructions is ran and the outputs are compared.

use crate::event::open::Event;
use crate::stat::StatEvent;
use crate::test::RunSettings;
use crate::test::Test;
use crate::test::TestResult;

// Since the event tests do very similar things, this function takes
// the event and compres the two results, failing if anything is weird.
fn event_sanity_check(event: StatEvent, settings: &RunSettings) -> TestResult {
    // A useless function that wastes cycles (collatz conjecture)
    fn useless_stuff() {
        fn next(x: u64) -> u64 {
            if x % 2 == 0 {
                x / 2
            } else {
                (x * 3) + 1
            }
        }
        for x in 0..1000 {
            let _: u64 = next(x);
        }
    }

    // This rets fail result, with verbose message if flag is on
    fn fail(x: String, settings: &RunSettings) -> TestResult {
        if settings.verbose {
            return TestResult::Failed(x);
        }
        TestResult::Failed("(1)".to_string())
    }

    let event = Event::new(event, None);
    let begin_count: isize = event.start_counter().unwrap();
    useless_stuff();
    let end_count = event.stop_counter().unwrap();
    if begin_count <= 0 || end_count <= 0 {
        return fail(
            "\nINFO:\t
                The value recieved from start / stop counter was 0."
                .to_string(),
            settings,
        );
    }
    if begin_count >= end_count {
        return fail(
            "\nINFO:\t
                    There were 0 cycles recorded between the start and stop,
                    even though a lot of computation happened"
                .to_string(),
            settings,
        );
    }
    TestResult::Passed
}

// Dummy function for parent test with subtests
fn dummy(_settings: &RunSettings) -> TestResult {
    TestResult::Passed
}

// This is the parent test for all the event subtests.
pub fn test_events() -> Test {
    // This tests StatEvent::Cycles for proper functionality
    fn test_cycles_open() -> Test {
        fn cycles_open(settings: &RunSettings) -> TestResult {
            event_sanity_check(StatEvent::Cycles, settings)
        }
        Test {
            name: "cycles_open".to_string(),
            description: "Cycles are able to be counted".to_string(),
            call: cycles_open,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }

    // This tests StatEvent::Instructions for proper functionality
    fn test_instructions_open() -> Test {
        fn instructions_open(settings: &RunSettings) -> TestResult {
            event_sanity_check(StatEvent::Instructions, settings)
        }
        Test {
            name: "instructions_open".to_string(),
            description: "Instructions are able to be counted".to_string(),
            call: instructions_open,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }

    Test {
        name: "event_sanity".to_string(),
        description: "Reading event counters sanity tests".to_string(),
        call: dummy,
        subtests: vec![test_cycles_open(), test_instructions_open()],
        is_subtest: false,
    }
}
