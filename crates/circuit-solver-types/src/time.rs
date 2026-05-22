//! Picosecond-resolution simulation time on the shared scheduler timeline.
//!
//! The Mixed-Signal Scheduler (ADR-0004) coordinates a continuous-time
//! analog solver with an event-driven digital simulator via a *single
//! monotonic time axis*. Both kernels report and accept times in this
//! unit so that "run-until 50 ns" and "next event at 50 ns" are
//! exact-comparison-equal at the boundary, with no floating-point
//! drift.
//!
//! Picosecond resolution gives us:
//!
//! - Headroom for sub-nanosecond digital event spacing (well beyond
//!   the spec's 50 ns / 80 ns / 100 ns example boundaries),
//! - Exact equality on integer comparisons, so a "predicted next-event
//!   time at exactly T" check cannot be confounded by floating-point
//!   representation error, and
//! - Roughly ±9 million seconds of range in `i64`, far more than any
//!   single transient analysis runs.

use core::fmt;
use core::ops::{Add, Sub};

/// Monotonic simulation time, measured in picoseconds from t=0.
///
/// The zero value represents the start of the analysis time interval.
/// Operations are saturating to keep arithmetic deterministic in the
/// presence of pathological inputs (which the scheduler treats as a
/// contract violation rather than a panic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SimulationTime {
    picoseconds: i64,
}

impl SimulationTime {
    /// The zero point of the analysis timeline.
    pub const ZERO: Self = Self { picoseconds: 0 };

    /// Construct from a raw picosecond count. Negative values are valid
    /// (e.g., for relative offsets), but the scheduler rejects them
    /// when used as absolute event times.
    #[must_use]
    pub const fn from_picoseconds(picoseconds: i64) -> Self {
        Self { picoseconds }
    }

    /// Construct from nanoseconds. Common in scenario language
    /// ("predicts a next event at time 50 ns").
    #[must_use]
    pub const fn from_nanoseconds(nanoseconds: i64) -> Self {
        Self {
            picoseconds: nanoseconds.saturating_mul(1_000),
        }
    }

    /// Construct from microseconds.
    #[must_use]
    pub const fn from_microseconds(microseconds: i64) -> Self {
        Self {
            picoseconds: microseconds.saturating_mul(1_000_000),
        }
    }

    /// Raw picosecond count.
    #[must_use]
    pub const fn as_picoseconds(self) -> i64 {
        self.picoseconds
    }

    /// Lossy conversion to nanoseconds (truncating).
    #[must_use]
    pub const fn as_nanoseconds(self) -> i64 {
        self.picoseconds / 1_000
    }

    /// Lossy conversion to seconds as `f64`. Useful for waveform export
    /// to formats that demand a real-valued time axis. The conversion
    /// loses precision for picosecond counts beyond ±2^52, but no
    /// realistic simulation horizon comes close.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_seconds_f64(self) -> f64 {
        (self.picoseconds as f64) * 1e-12
    }
}

impl Add for SimulationTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            picoseconds: self.picoseconds.saturating_add(rhs.picoseconds),
        }
    }
}

impl Sub for SimulationTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            picoseconds: self.picoseconds.saturating_sub(rhs.picoseconds),
        }
    }
}

impl fmt::Display for SimulationTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ps = self.picoseconds;
        if ps == 0 {
            return write!(f, "0 s");
        }
        let abs = ps.unsigned_abs();
        if abs >= 1_000_000_000_000 {
            write!(f, "{} s", self.as_seconds_f64())
        } else if abs >= 1_000_000_000 {
            write!(f, "{} µs", ps / 1_000_000)
        } else if abs >= 1_000 {
            write!(f, "{} ns", ps / 1_000)
        } else {
            write!(f, "{ps} ps")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_nanoseconds_matches_picoseconds() {
        assert_eq!(
            SimulationTime::from_nanoseconds(50).as_picoseconds(),
            50_000
        );
    }

    #[test]
    fn equality_is_exact_at_boundary() {
        // The scenario hinges on this: predicted=50ns equals confirmed=50ns
        // with no floating-point ambiguity.
        let predicted = SimulationTime::from_nanoseconds(50);
        let confirmed = SimulationTime::from_picoseconds(50_000);
        assert_eq!(predicted, confirmed);
    }

    #[test]
    fn ordering_is_monotonic() {
        let t1 = SimulationTime::from_nanoseconds(10);
        let t2 = SimulationTime::from_nanoseconds(20);
        assert!(t1 < t2);
        assert!(t1 + SimulationTime::from_nanoseconds(10) == t2);
    }

    #[test]
    fn display_picks_a_unit() {
        assert_eq!(format!("{}", SimulationTime::from_nanoseconds(50)), "50 ns");
        assert_eq!(format!("{}", SimulationTime::ZERO), "0 s");
    }
}
