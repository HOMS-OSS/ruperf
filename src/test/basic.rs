use crate::test::Test;
use std::{thread, time};

// TODO: Remove this (it's just for testing the tests)
// TEST: This test always passes
pub fn test_always_passes() -> Test {
    fn always_passes() -> bool {
        true
    }

    Test {
        name: "always_passes".to_string(),
        description: "This test always passes".to_string(),
        call: always_passes,
    }
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test always fails
pub fn test_always_fails() -> Test {
    fn always_fails() -> bool {
        false
    }

    Test {
        name: "always_fails".to_string(),
        description: "This test always fails".to_string(),
        call: always_fails,
    }
}

// TODO: Remove this (it's just for testing the tests)
// TEST: This test passes after 1 second
pub fn test_passes_after_1sec() -> Test {
    fn passes_after_1sec() -> bool {
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        true
    }

    Test {
        name: "passes_after_1sec".to_string(),
        description: "This test passes after 1 second".to_string(),
        call: passes_after_1sec,
    }
}
