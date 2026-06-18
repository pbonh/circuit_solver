//! Threshold-based digital transition detector for analog waveforms.
//!
//! [`ThresholdDetector`] scans a sampled waveform and records the time of
//! each rising and falling edge crossing a configurable threshold voltage.
//! This is used to verify that digital transitions propagate correctly through
//! a simulated circuit (e.g. a CMOS inverter chain).
//!
//! # Example
//!
//! ```
//! use circuit_solver_delta::threshold_detector::ThresholdDetector;
//!
//! let times  = vec![0.0, 1e-9, 2e-9, 3e-9];
//! let values = vec![0.0, 0.0, 1.8, 1.8];
//! let det = ThresholdDetector::new(0.9);
//! let edges = det.detect(&times, &values);
//! assert_eq!(edges.len(), 1);
//! assert_eq!(edges[0].kind, circuit_solver_delta::threshold_detector::EdgeKind::Rising);
//! ```

/// The direction of a detected threshold crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    /// Waveform crossed the threshold from below (low → high).
    Rising,
    /// Waveform crossed the threshold from above (high → low).
    Falling,
}

/// A detected threshold-crossing event.
#[derive(Debug, Clone)]
pub struct Edge {
    /// Interpolated time of the crossing (seconds).
    pub time: f64,
    /// Direction of the crossing.
    pub kind: EdgeKind,
}

/// Detects rising and falling edges in a sampled waveform.
#[derive(Debug, Clone)]
pub struct ThresholdDetector {
    /// The voltage threshold (volts).
    pub threshold: f64,
}

impl ThresholdDetector {
    /// Create a new detector with the given threshold voltage.
    pub fn new(threshold: f64) -> Self {
        ThresholdDetector { threshold }
    }

    /// Scan `(times, values)` and return all threshold crossings in order.
    ///
    /// The crossing time is linearly interpolated between the two samples
    /// that straddle the threshold.
    ///
    /// Consecutive crossings in the same direction are not deduplicated
    /// (the waveform is assumed to be properly monotone at each edge).
    pub fn detect(&self, times: &[f64], values: &[f64]) -> Vec<Edge> {
        assert_eq!(
            times.len(),
            values.len(),
            "times and values must have the same length"
        );
        if times.len() < 2 {
            return vec![];
        }

        let th = self.threshold;
        let mut edges = Vec::new();

        for i in 1..times.len() {
            let v0 = values[i - 1];
            let v1 = values[i];

            let was_below = v0 < th;
            let is_below = v1 < th;

            if was_below && !is_below {
                // Rising edge: v0 < threshold <= v1
                let t = interpolate_crossing(times[i - 1], times[i], v0, v1, th);
                edges.push(Edge { time: t, kind: EdgeKind::Rising });
            } else if !was_below && is_below {
                // Falling edge: v0 >= threshold > v1
                let t = interpolate_crossing(times[i - 1], times[i], v0, v1, th);
                edges.push(Edge { time: t, kind: EdgeKind::Falling });
            }
        }

        edges
    }
}

/// Linearly interpolate the time at which a waveform crosses `threshold`
/// between samples `(t0, v0)` and `(t1, v1)`.
fn interpolate_crossing(t0: f64, t1: f64, v0: f64, v1: f64, threshold: f64) -> f64 {
    let dv = v1 - v0;
    if dv.abs() < f64::EPSILON {
        return t0;
    }
    let frac = (threshold - v0) / dv;
    t0 + frac * (t1 - t0)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_single_rising_edge() {
        let times = vec![0.0, 1e-9, 2e-9, 3e-9];
        let values = vec![0.0, 0.0, 1.8, 1.8];
        let det = ThresholdDetector::new(0.9);
        let edges = det.detect(&times, &values);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, EdgeKind::Rising);
        // Crossing should be between t=1ns and t=2ns, interpolated at 1.5ns
        assert!((edges[0].time - 1.5e-9).abs() < 1e-12, "got {}", edges[0].time);
    }

    #[test]
    fn detects_single_falling_edge() {
        let times = vec![0.0, 1e-9, 2e-9, 3e-9];
        let values = vec![1.8, 1.8, 0.0, 0.0];
        let det = ThresholdDetector::new(0.9);
        let edges = det.detect(&times, &values);
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].kind, EdgeKind::Falling);
        assert!((edges[0].time - 1.5e-9).abs() < 1e-12, "got {}", edges[0].time);
    }

    #[test]
    fn detects_multiple_edges() {
        let times = vec![0.0, 2e-9, 4e-9, 6e-9];
        let values = vec![0.0, 1.8, 1.8, 0.0];
        let det = ThresholdDetector::new(0.9);
        let edges = det.detect(&times, &values);
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].kind, EdgeKind::Rising);
        assert_eq!(edges[1].kind, EdgeKind::Falling);
    }

    #[test]
    fn no_edges_below_threshold() {
        let times = vec![0.0, 5e-9, 10e-9];
        let values = vec![0.0, 0.5, 0.0];
        let det = ThresholdDetector::new(0.9);
        let edges = det.detect(&times, &values);
        assert!(edges.is_empty(), "no crossings expected");
    }

    #[test]
    fn no_edges_above_threshold() {
        let times = vec![0.0, 5e-9, 10e-9];
        let values = vec![1.8, 1.2, 1.8];
        let det = ThresholdDetector::new(0.9);
        let edges = det.detect(&times, &values);
        assert!(edges.is_empty(), "no crossings expected");
    }
}
