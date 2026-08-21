use std::ffi::OsStr;
use std::fmt;
use std::time::{Duration, Instant};

const PREPARE_TIMINGS_ENV: &str = "FORTLET_PREPARE_TIMINGS";

pub struct PrepareTimings {
    enabled: bool,
    started: Instant,
    previous: Instant,
}

#[derive(Clone, Copy)]
pub enum PreparePhase {
    Image,
    RuntimeStore,
    HarnessClosure,
    Schema1Layer,
    Schema2Provider,
    FinalVerification,
}

impl PreparePhase {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::RuntimeStore => "runtime-store",
            Self::HarnessClosure => "harness-closure",
            Self::Schema1Layer => "schema-1-layer",
            Self::Schema2Provider => "schema-2-provider",
            Self::FinalVerification => "final-verification",
        }
    }
}

impl PrepareTimings {
    pub fn from_environment() -> Self {
        Self::new(std::env::var_os(PREPARE_TIMINGS_ENV).as_deref())
    }

    fn new(value: Option<&OsStr>) -> Self {
        let now = Instant::now();
        Self {
            enabled: value == Some(OsStr::new("1")),
            started: now,
            previous: now,
        }
    }

    pub fn record(&mut self, phase: PreparePhase) {
        if let Some(event) = self.record_at(phase, Instant::now()) {
            eprintln!("{event}");
        }
    }

    fn record_at(&mut self, phase: PreparePhase, now: Instant) -> Option<PrepareTimingEvent> {
        if !self.enabled {
            return None;
        }
        let event = PrepareTimingEvent {
            phase,
            delta: now.saturating_duration_since(self.previous),
            total: now.saturating_duration_since(self.started),
        };
        self.previous = now;
        Some(event)
    }
}

struct PrepareTimingEvent {
    phase: PreparePhase,
    delta: Duration,
    total: Duration,
}

impl fmt::Display for PrepareTimingEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "fortlet: prepare-timing phase={} delta_ms={} total_ms={}",
            self.phase.as_str(),
            self.delta.as_millis(),
            self.total.as_millis()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_timings_require_an_exact_opt_in() {
        assert!(PrepareTimings::new(Some(OsStr::new("1"))).enabled);
        assert!(!PrepareTimings::new(None).enabled);
        assert!(!PrepareTimings::new(Some(OsStr::new("true"))).enabled);
        assert!(!PrepareTimings::new(Some(OsStr::new("0"))).enabled);
    }

    #[test]
    fn prepare_timing_output_contains_only_bounded_phase_and_durations() {
        let started = Instant::now();
        let mut timings = PrepareTimings {
            enabled: true,
            started,
            previous: started,
        };
        let event = timings
            .record_at(
                PreparePhase::HarnessClosure,
                started + Duration::from_millis(23),
            )
            .unwrap();

        assert_eq!(
            event.to_string(),
            "fortlet: prepare-timing phase=harness-closure delta_ms=23 total_ms=23"
        );
    }
}
