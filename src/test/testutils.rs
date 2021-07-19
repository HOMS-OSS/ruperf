use crate::test::basic;
use crate::test::pfm;
use crate::test::Test;
use crate::test::TestResult;
use std::io::stdout;
use std::io::Write;
/// Gathers all tests and returns a Vec with them all
pub fn make_tests() -> Vec<Test> {
    let mut tests: Vec<Test> = Vec::new();

    // from basic.rs
    tests.push(basic::test_always_passes());
    tests.push(basic::test_always_fails());
    tests.push(basic::test_passes_after_1sec());
    tests.push(basic::test_with_pointless_subtests());

    // from pfm.rs
    tests.push(pfm::test_check_for_libpfm4());

    tests
}

/// Runs all tests and outputs results to stdout
pub fn run_all_tests(tests: &Vec<Test>, to_skip: &Vec<String>) {
    let mut should_skip;
    for (index, test) in tests.iter().enumerate() {
        should_skip = to_skip.iter().any(|i| *i == index.to_string());
        run_single_test(&test, index, should_skip, "".to_string());
    }
}

/// Runs a single test (or subtest)
pub fn run_single_test(
    test: &Test,
    index: usize,
    should_skip: bool,
    parent_index_string: String,
) -> TestResult {
    print!(
        "{:>2}{}: {:<60} : ",
        parent_index_string, index, test.description
    );
    stdout().flush().unwrap();
    let result_type: TestResult;
    if should_skip {
        result_type = TestResult::Skipped;
    } else {
        if !test.subtests.is_empty() {
            print!("\n");
            let mut overall_result_type: TestResult = TestResult::Passed;
            for (i, subtest) in test.subtests.iter().enumerate() {
                // TODO: change false to a given subtest skip
                let result = run_single_test(subtest, i, false, index.to_string() + ".");
                match result {
                    TestResult::Failed => overall_result_type = TestResult::Failed,
                    _ => {}
                }
            }
            result_type = overall_result_type;
            return result_type;
        } else {
            let result = (test.call)();
            if result {
                result_type = TestResult::Passed;
            } else {
                result_type = TestResult::Failed;
            }
        }
    }
    let result_text: String = match result_type {
        TestResult::Skipped => "\x1b[0;33mSkip\x1b[0m",
        TestResult::Passed => "Ok",
        TestResult::Failed => "\x1b[0;31mFAILED!\x1b[0m",
    }
    .to_string();
    println!("{}", result_text);
    return result_type;
}

/// Lists all tests and outputs results to stdout
pub fn list_all_tests(tests: &Vec<Test>) {
    for (index, test) in tests.iter().enumerate() {
        println!("{:>2}: {:<60}", index, test.description);
    }
}
