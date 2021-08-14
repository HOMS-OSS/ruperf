pub mod perf {
    use serde::{Deserialize, Serialize};
    /// Perf Commands to be used
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PerfEvent {
        Stat,
        Record,
        Report,
        Annotate,
        Top,
        Bench,
        Test,
    }

    /// Holds an enumerated array of PerfEvents
    impl PerfEvent {
        pub const ALL: [PerfEvent; 7] = [
            PerfEvent::Annotate,
            PerfEvent::Bench,
            PerfEvent::Record,
            PerfEvent::Report,
            PerfEvent::Stat,
            PerfEvent::Test,
            PerfEvent::Top,
        ];

        pub fn as_str(&self) -> &'static str {
            match self {
                PerfEvent::Stat => "stat",
                PerfEvent::Test => "test",
                PerfEvent::Report => "report",
                PerfEvent::Record => "record",
                PerfEvent::Annotate => "annotate",
                PerfEvent::Bench => "bench",
                PerfEvent::Top => "top",
            }
        }
    }

    /// Default PerfEvent
    impl Default for PerfEvent {
        fn default() -> PerfEvent {
            PerfEvent::Stat
        }
    }

    /// Provide PerfEvents as String data types
    impl std::fmt::Display for PerfEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    PerfEvent::Annotate => "Annotate",
                    PerfEvent::Bench => "Bench",
                    PerfEvent::Record => "Record",
                    PerfEvent::Report => "Report",
                    PerfEvent::Stat => "Stat",
                    PerfEvent::Test => "Test",
                    PerfEvent::Top => "Top",
                }
            )
        }
    }
}
