//! `basic.rs` contains basic tests used to demonstrate the use
//! of `ruperf test`. These tests don't do much other than provide
//! a basic example of how the test program works.

use crate::test::RunSettings;
use crate::test::Test;
use crate::test::TestResult;
use std::{thread, time};

// Dummy function for parent test with subtests
fn dummy(_settings: &RunSettings) -> TestResult {
    TestResult::Passed
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test always passes
pub fn test_always_passes() -> Test {
    fn always_passes(_settings: &RunSettings) -> TestResult {
        TestResult::Passed
    }

    Test {
        name: "always_passes".to_string(),
        description: "This test always passes".to_string(),
        call: always_passes,
        subtests: Vec::new(),
        is_subtest: false,
    }
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test always fails
pub fn test_always_fails() -> Test {
    fn always_fails(_settings: &RunSettings) -> TestResult {
        TestResult::Passed
    }

    Test {
        name: "always_fails".to_string(),
        description: "This test always fails".to_string(),
        call: always_fails,
        subtests: Vec::new(),
        is_subtest: false,
    }
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test passes after 1 second
pub fn test_passes_after_1sec() -> Test {
    fn passes_after_1sec(_settings: &RunSettings) -> TestResult {
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        TestResult::Passed
    }

    Test {
        name: "passes_after_1sec".to_string(),
        description: "This test passes after 1 second".to_string(),
        call: passes_after_1sec,
        subtests: Vec::new(),
        is_subtest: false,
    }
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test has a bunch of pointless subtests
pub fn test_with_pointless_subtests() -> Test {
    fn subtest_pointless_1() -> Test {
        fn pointless_1(_settings: &RunSettings) -> TestResult {
            TestResult::Passed
        }
        Test {
            name: "pointless1".to_string(),
            description: "This one passes".to_string(),
            call: pointless_1,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }
    fn subtest_pointless_2() -> Test {
        fn pointless_2(settings: &RunSettings) -> TestResult {
            if settings.verbose {
                return TestResult::Failed(
                    "\nINFO:\tthis test is destined to fail. nothing can be done to get it to pass."
                    .to_string()
                );
            }
            TestResult::Failed("(1)".to_string())
        }
        Test {
            name: "pointless2".to_string(),
            description: "This one fails".to_string(),
            call: pointless_2,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }
    fn subtest_pointless_3() -> Test {
        fn pointless_3(_settings: &RunSettings) -> TestResult {
            TestResult::Passed
        }
        Test {
            name: "pointless3".to_string(),
            description: "This one also passes".to_string(),
            call: pointless_3,
            subtests: Vec::new(),
            is_subtest: true,
        }
    }

    Test {
        name: "subtest_test".to_string(),
        description: "Test with many subtests".to_string(),
        call: dummy,
        subtests: vec![
            subtest_pointless_1(),
            subtest_pointless_2(),
            subtest_pointless_3(),
        ],
        is_subtest: false,
    }
}
