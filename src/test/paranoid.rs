//!
//!
//!

use crate::test::RunSettings;
use crate::test::Test;
use crate::test::TestResult;
use std::fs;
use std::path::Path;

// TEST: Check for presence of libpfm4
pub fn test_check_paranoid_flag() -> Test {
    // This uses the linux command "ldconfig" and returns
    // based on whether or not the output contains "libpfm."
    fn check_paranoid_flag(settings: &RunSettings) -> TestResult {
        let paranoid_flag = "/proc/sys/kernel/perf_event_paranoid".to_string();
        let paranoid_flag_path = Path::new(&paranoid_flag);
        if !paranoid_flag_path.exists() {
            if settings.verbose {
                return TestResult::Failed(format!("\nINFO: Couldn't find {}", paranoid_flag));
            }
            return TestResult::Failed("(1)".to_string());
        }
        let contents = fs::read_to_string(paranoid_flag_path).unwrap();
        let value = contents.trim_end().parse::<i32>().unwrap();
        match value {
            x if x <= 0 => TestResult::Passed,
            x => {
                if settings.verbose {
                    return TestResult::Failed(format!(
                        "\nINFO:\tExpected 0 but instead flag was {}",
                        x
                    ));
                }
                TestResult::Failed("(1)".to_string())
            }
        }
    }

    Test {
        name: "paranoid_flag_check".to_string(),
        description: "Checks that perf_event_paranoid flag is <= 0".to_string(),
        call: check_paranoid_flag,
        subtests: Vec::new(),
        is_subtest: false,
    }
}
