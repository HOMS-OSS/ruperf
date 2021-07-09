use crate::test::Test;

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
