/// Perf Commands to be used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// Default PerfEvent
impl Default for PerfEvent {
    fn default() -> PerfEvent {
        PerfEvent::Test
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
