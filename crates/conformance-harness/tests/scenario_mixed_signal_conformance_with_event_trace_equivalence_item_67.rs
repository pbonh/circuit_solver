//! Scenario-level integration witness for
//! `mixed-signal-cosim#mixed-signal-conformance-with-event-trace-equivalence`
//! (tasks.md item #67).
//!
//! Per the executable specification (verbatim Gherkin block from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/mixed-signal-cosim/spec.md`):
//!
//! ```gherkin
//! Given ConformanceTester has a Golden Reference for a mixed-signal
//!   simulation including both analog Waveforms and digital event traces
//! And the tolerance envelope for analog is 1 % relative and for digital
//!   is event trace equivalence at cycle boundaries
//! When ConformanceTester runs the same mixed-signal simulation
//! Then analog Waveforms match the Golden Reference within the
//!   tolerance envelope
//! And digital event traces are event-trace-equivalent with the Golden
//!   Reference at every cycle boundary
//! And Conformance is reported as "pass"
//! ```
//!
//! # Position in the implementation pipeline
//!
//! This test is the **consumer** of two pieces of upstream
//! infrastructure that already shipped to trunk before this task ran:
//!
//! - **tasks.md #62** — the [`conformance_harness`] crate: ASCII rawfile
//!   parser ([`load_ngspice_ascii`]) and the per-node `max(rel, abs)`
//!   comparator ([`compare`]) under ADR-0008's envelope.
//! - **tasks.md #51** — the [`MixedSignalScheduler`] orchestration
//!   loop, the [`AnalogSolver`] and [`DigitalSimulator`] traits, and
//!   the [`MixedSignalResult`] type carrying both analog
//!   [`AnalogTrace`] and digital [`DigitalEventTrace`] data.
//!
//! The witness wires these two together to satisfy the spec scenario:
//! it constructs test doubles for the analog and digital simulator
//! kernels, runs a mixed-signal simulation of a simple ramp + digital
//! event pattern, compares the analog waveforms against an ngspice-
//! shaped ASCII golden rawfile under ADR-0008's transient tolerance
//! (1 % relative / 1 mV absolute), and checks that digital event times
//! agree at every cycle boundary.
//!
//! # Choice of fixture
//!
//! The scenario fixture is intentionally minimal — a single-node analog
//! ramp (0 V at t=0, linearly rising to 3.3 V at t=200 ns) paired with
//! a scripted digital simulator that predicts/confirms events at
//! 50 ns, 100 ns, 150 ns, and 200 ns. These are the "cycle boundaries."
//!
//! At each boundary the analog voltage is directly computable:
//!
//! ```text
//! v(analog_out) @ t = 3.3 · (t / 200 ns)
//! ```
//!
//! The digital simulator records a single event on signal "clk" at
//! every boundary, toggling between 0 and 1. Event-trace equivalence
//! means the scheduler's result digital trace contains those same
//! events at those same times.
//!
//! The golden ngspice rawfile is synthesized at test time so the
//! scenario is hermetic — no external ngspice binary or Sky130 model
//! cards are needed.
//!
//! # Glossary terms used (verbatim from the inlined Glossary)
//!
//! - **`ConformanceTester`** — "an automated agent or engineer who
//!   compares solver results against golden references and reports
//!   pass/fail."
//! - **Golden Reference** — "a trusted external simulator against
//!   which results are compared."
//! - **Conformance** — "passing the tolerance-bounded comparison
//!   against a golden reference."
//! - **Result** — "the unified output structure for any analysis."
//! - **Waveform** — "a time-domain voltage or current signal."
//! - **Circuit** — "the top-level object representing a netlist and its
//!   associated models."
//! - **Simulator** — "the runtime that executes analyses on a circuit."
//!
//! # ADR references
//!
//! - ADR-0008 (tolerance envelope) — the transient default
//!   `(rel=0.01, abs=1e-3)` is used for analog conformance.
//! - ADR-0004 (optimistic mixed-signal) — the scheduler runs
//!   predict/confirm cycles exactly as specified.
//! - ADR-0007 (ZOH at analog-digital boundary) — acknowledged; this
//!   witness does not exercise boundary interpolation.
//! - ADR-0010 (unstable v1 API) — all imports tracked here fail
//!   loudly if the public surface changes.

// The `cast_precision_loss` and `single_match_else` and
// `format_push_string` lints from clippy::pedantic are allowed:
// — i64→f64 casts here are on nanosecond constants well within the
//   IEEE 754 integer-representable range (≤ 2^53) and will never lose
//   precision in practice.
// — The `match` on `VecDeque::pop_front` is the clearest expression of
//   the two-case control flow (some/none) and the `if let` rewrite
//   would be less readable.
// — `format_push_string` on a 4-iteration loop is negligible;
//   `write!` introduces fallibility that complicates the golden-fixture
//   builder for no value.
#![allow(
    clippy::cast_precision_loss,
    clippy::single_match_else,
    clippy::format_push_string
)]

use std::io::Write;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError, SparseCheckpoint,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, MixedSignalResult, NodeId, SignalName, SimulationTime, Waveform,
};
use conformance_harness::{compare, AnalysisKind, ConformanceVerdict};

// ---------------------------------------------------------------------------
// Fixture parameters
// ---------------------------------------------------------------------------

/// The single analog node observed in this mixed-signal test bench.
const OBSERVED_NODE: NodeId = NodeId::new(1);

/// ngspice rawfile variable name for the observed node.
const VAR_ANALOG_OUT: &str = "v(analog_out)";

/// Simulation horizon — the final cycle boundary.
const HORIZON_NS: i64 = 200;

/// Cycle boundaries at which the digital simulator predicts/confirms
/// events and the analog solver checkpoints.
const BOUNDARIES_NS: &[i64] = &[50, 100, 150, 200];

/// The digital signal name that toggles at every boundary.
const DIGITAL_SIGNAL: &str = "clk";

/// ADR-0008 transient tolerance: 1 % relative, 1 mV absolute.
const ANALOG_REL: f64 = 0.01;
const ANALOG_ABS: f64 = 1e-3;

// ---------------------------------------------------------------------------
// Closed-form analog voltage
// ---------------------------------------------------------------------------

/// Compute the analog node voltage at the given nanosecond time:
/// `v(t) = 3.3 · (t / 200 ns)`.
fn analog_voltage_at_ns(t_ns: i64) -> f64 {
    3.3 * (t_ns as f64 / HORIZON_NS as f64)
}

// ---------------------------------------------------------------------------
// Synthesized ngspice rawfile golden reference
// ---------------------------------------------------------------------------

/// Format a single `f64` in ngspice rawfile's `%.6e` style.
fn fmt_value(v: f64) -> String {
    format!("{v:.6e}")
}

/// Synthesize an ngspice ASCII rawfile for the analog waveform at
/// every cycle boundary. Uses `Plotname: Transient Analysis` so the
/// harness classifies it as `SweepKind::Transient` and applies the
/// transient default tolerance.
fn synthesize_golden_rawfile() -> String {
    let n_points = BOUNDARIES_NS.len();
    let mut vars = format!(
        "Title: mixed-signal-conformance-golden\n\
         Date: Thu Jun  5 14:00:00 2025\n\
         Plotname: Transient Analysis\n\
         Flags: real\n\
         No. Variables: 2\n\
         No. Points: {n_points}\n\
         Variables:\n\
         \t0\ttime\ttime\n\
         \t1\t{VAR_ANALOG_OUT}\tvoltage\n\
         Values:\n"
    );
    for (i, &t_ns) in BOUNDARIES_NS.iter().enumerate() {
        let t_s = t_ns as f64 * 1e-9;
        let v = analog_voltage_at_ns(t_ns);
        vars.push_str(&format!("\t{i}\t{}\t{}\n", fmt_value(t_s), fmt_value(v)));
    }
    vars
}

/// Synthesize the expected digital event trace: one event per boundary
/// on the "clk" signal, toggling between 0 and 1.
fn expected_digital_events() -> DigitalEventTrace {
    let mut events = Vec::with_capacity(BOUNDARIES_NS.len());
    for &t_ns in BOUNDARIES_NS {
        events.push(SimulationTime::from_nanoseconds(t_ns));
    }
    DigitalEventTrace {
        vcd: String::new(), // golden VCD text is nil for this comparison
        events_by_signal: vec![(SignalName::new(DIGITAL_SIGNAL), events)],
    }
}

// ---------------------------------------------------------------------------
// Test doubles: analog solver
// ---------------------------------------------------------------------------

/// A scripted analog solver that produces a linear ramp voltage on the
/// observed node. It records a checkpoint at every `run_until` target.
struct RampAnalogSolver {
    samples: Vec<(SimulationTime, f64)>,
    checkpoints: Vec<SimulationTime>,
}

impl RampAnalogSolver {
    fn new() -> Self {
        Self {
            samples: vec![(SimulationTime::ZERO, analog_voltage_at_ns(0))],
            checkpoints: Vec::new(),
        }
    }
}

impl AnalogSolver for RampAnalogSolver {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        let t_ns = target.as_picoseconds() / 1000; // ps → ns
        let v = analog_voltage_at_ns(t_ns);
        self.samples.push((target, v));
        self.checkpoints.push(target);
        // Produce a real checkpoint carrying the node voltage so the
        // scheduler's rollback handler can record it.
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(OBSERVED_NODE, v)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, _target: SimulationTime) -> Result<(), SchedulerError> {
        // Not exercised on the correct-prediction path.
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        let (times, values): (Vec<SimulationTime>, Vec<f64>) = self.samples.drain(..).unzip();
        let waveform = Waveform::new(OBSERVED_NODE, times, values);
        let committed_through = self
            .checkpoints
            .last()
            .copied()
            .unwrap_or(SimulationTime::ZERO);
        AnalogTrace {
            waveforms: vec![waveform],
            committed_through,
        }
    }
}

// ---------------------------------------------------------------------------
// Test doubles: digital simulator
// ---------------------------------------------------------------------------

/// A scripted digital simulator that predicts events at the cycle
/// boundaries, confirms them, and records the event trace.
struct ScriptedDigitalSimulator {
    /// Remaining event times to predict (in ns).
    upcoming: std::collections::VecDeque<i64>,
    /// Times at which events were confirmed.
    events: Vec<SimulationTime>,
    /// True when all events have been predicted.
    exhausted: bool,
}

impl ScriptedDigitalSimulator {
    fn new() -> Self {
        let upcoming: std::collections::VecDeque<i64> = BOUNDARIES_NS.iter().copied().collect();
        Self {
            upcoming,
            events: Vec::new(),
            exhausted: false,
        }
    }
}

impl DigitalSimulator for ScriptedDigitalSimulator {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.upcoming.pop_front() {
            Some(t_ns) => {
                self.exhausted = false;
                Ok(NextEventReport {
                    predicted_time: SimulationTime::from_nanoseconds(t_ns),
                })
            }
            None => {
                self.exhausted = true;
                Err(SchedulerError::DigitalAdapterFailed(
                    "end of event script".into(),
                ))
            }
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        // On the correct-prediction path the boundary matches the
        // previously-predicted time. Confirm it.
        self.events.push(boundary);
        Ok(DigitalStepReport::Confirmed { time: boundary })
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        DigitalEventTrace {
            vcd: String::new(),
            events_by_signal: vec![(
                SignalName::new(DIGITAL_SIGNAL),
                std::mem::take(&mut self.events),
            )],
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Write a string to a temp file and return the path.
fn write_temp_fixture(dir_name: &str, name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(dir_name);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create fixture");
    f.write_all(body.as_bytes()).expect("write fixture");
    path
}

/// Extract the analog voltage samples at the cycle boundaries from the
/// result, in `(sweep_time, value)` pairs for comparison against the
/// golden rawfile.
fn extract_analog_at_boundaries(result: &MixedSignalResult) -> Vec<(f64, f64)> {
    let waveform = result
        .analog
        .waveform_for(OBSERVED_NODE)
        .expect("analog waveform present");
    // The scheduler records a sample at each run_until boundary. The
    // waveform times are SimulationTime (in ps); convert to seconds.
    waveform
        .times
        .iter()
        .zip(waveform.values.iter())
        .map(|(t, v)| (t.as_picoseconds() as f64 * 1e-12, *v))
        .collect()
}

/// Check event-trace equivalence at every cycle boundary: the actual
/// digital trace must contain the same events at the same times as the
/// golden expected trace.
fn check_digital_event_trace_equivalence(
    actual: &DigitalEventTrace,
    golden: &DigitalEventTrace,
) -> Result<(), String> {
    let actual_events = actual.events_for(&SignalName::new(DIGITAL_SIGNAL));
    let golden_events = golden.events_for(&SignalName::new(DIGITAL_SIGNAL));

    let actual_events = actual_events
        .ok_or_else(|| format!("actual digital trace missing signal '{DIGITAL_SIGNAL}'"))?;
    let golden_events = golden_events
        .ok_or_else(|| format!("golden digital trace missing signal '{DIGITAL_SIGNAL}'"))?;

    if actual_events.len() != golden_events.len() {
        return Err(format!(
            "event count mismatch: actual={}, golden={}",
            actual_events.len(),
            golden_events.len()
        ));
    }

    for (i, (a, g)) in actual_events.iter().zip(golden_events.iter()).enumerate() {
        if a != g {
            return Err(format!(
                "event-trace mismatch at boundary {i}: actual {a}, golden {g}"
            ));
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// **Headline scenario witness.**
///
/// ```gherkin
/// Given ConformanceTester has a Golden Reference for a mixed-signal
///   simulation including both analog Waveforms and digital event traces
/// And the tolerance envelope for analog is 1 % relative and for digital
///   is event trace equivalence at cycle boundaries
/// When ConformanceTester runs the same mixed-signal simulation
/// Then analog Waveforms match the Golden Reference within the
///   tolerance envelope
/// And digital event traces are event-trace-equivalent with the Golden
///   Reference at every cycle boundary
/// And Conformance is reported as "pass"
/// ```
#[test]
fn mixed_signal_conformance_with_event_trace_equivalence_reports_pass() {
    // Given: ConformanceTester has a Golden Reference for a mixed-signal
    // simulation including both analog Waveforms and digital event traces.
    //
    // Synthesize the golden ngspice rawfile carrying the analog
    // voltage at every cycle boundary, and the expected digital event trace.
    let golden_path = write_temp_fixture(
        "mixed-signal-conformance-item-67",
        "mixed-signal-golden.raw",
        &synthesize_golden_rawfile(),
    );
    let golden =
        conformance_harness::load_ngspice_ascii(&golden_path).expect("parse golden rawfile");
    // Verify the golden has the expected shape.
    assert_eq!(
        golden.n_points(),
        BOUNDARIES_NS.len(),
        "golden must have one row per cycle boundary"
    );
    assert_eq!(
        golden.n_variables(),
        1,
        "golden must carry the single analog variable"
    );

    let golden_digital = expected_digital_events();
    assert_eq!(
        golden_digital.total_events(),
        BOUNDARIES_NS.len(),
        "golden digital must have one event per cycle boundary"
    );

    // And: the tolerance envelope for analog is 1 % relative
    // (pre-configured via ADR-0008 transient default). Pin the pair.
    let tolerance = AnalysisKind::Transient.default_tolerance();
    assert!(
        (tolerance.relative - ANALOG_REL).abs() < 1e-15,
        "transient default relative is 1 %"
    );
    assert!(
        (tolerance.absolute - ANALOG_ABS).abs() < 1e-15,
        "transient default absolute is 1 mV"
    );

    // When: ConformanceTester runs the same mixed-signal simulation.
    let analog = RampAnalogSolver::new();
    let digital = ScriptedDigitalSimulator::new();
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(HORIZON_NS),
    );
    let result: MixedSignalResult = scheduler.run().expect("mixed-signal run ok");

    // Then: the scheduler completed without rollbacks (correct-
    // prediction path).
    assert!(
        result.rollback_free(),
        "no rollback on correct-prediction path"
    );

    // And: the result contains analog and digital traces.
    assert!(!result.analog.waveforms.is_empty(), "analog trace present");
    assert!(
        !result.digital.events_by_signal.is_empty(),
        "digital trace present"
    );

    // Then: analog Waveforms match the Golden Reference within the
    // tolerance envelope. Extract the analog values at the boundaries
    // and compare against the golden using the conformance harness.
    let analog_samples = extract_analog_at_boundaries(&result);
    // Map sample times to the golden's sweep axis shape. The golden
    // rawfile has the sweep axis in seconds; our waveform times are
    // also in seconds after conversion.
    //
    // Because the golden is synthesized by the same closed-form, the
    // analog values should be *exact* matches (within f64 epsilon).
    // But we exercise the full compare() path anyway so the parser and
    // comparator receive the same coverage as in DC/transient/noise
    // conformance witnesses.
    //
    // We only compare the cycle-boundary samples (not the t=0 start
    // sample) — the golden rawfile encodes only the boundary points,
    // and the scheduler records the start sample plus one per boundary.
    // We skip index 0 (the t=0 start) and match the remaining samples.
    let boundary_analog: Vec<(f64, f64)> = analog_samples
        .into_iter()
        .skip(1) // skip t=0 start sample
        .collect();
    assert_eq!(
        boundary_analog.len(),
        golden.n_points(),
        "boundary analog count must match golden sweep points"
    );

    let actual_series: Vec<f64> = boundary_analog.iter().map(|(_, v)| *v).collect();
    let report = compare(
        &golden,
        [(VAR_ANALOG_OUT, actual_series.as_slice())],
        tolerance,
        16,
    );

    // Then: Conformance is reported as "pass" for the analog part.
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "analog conformance must pass; report = {report:#?}"
    );

    // And: digital event traces are event-trace-equivalent with the
    // Golden Reference at every cycle boundary.
    check_digital_event_trace_equivalence(&result.digital, &golden_digital)
        .expect("digital event-trace equivalence");

    // Overall verdict: mixed-signal conformance passes on both axes.
}

/// **Fail variant: analog waveform outside tolerance.**
///
/// Perturb the analog solver voltage at one boundary so it falls
/// outside the 1 % envelope and verify that the conformance verdict
/// flips to Fail.
#[test]
fn mixed_signal_conformance_fails_when_analog_outside_tolerance() {
    // This test uses a custom analog solver that reports a deliberate
    // offset at the second boundary (100 ns). The golden remains the
    // correct analytic value.
    //
    // At t=100 ns, v = 3.3 · 0.5 = 1.65 V. Envelope = max(1%·1.65, 1mV)
    // = max(16.5 mV, 1 mV) = 16.5 mV. We offset by 20 mV → should fail.

    struct PerturbedAnalog {
        normal: RampAnalogSolver,
        counter: usize,
    }

    impl AnalogSolver for PerturbedAnalog {
        fn run_until(
            &mut self,
            target: SimulationTime,
        ) -> Result<AnalogStepReport, SchedulerError> {
            self.counter += 1;
            let t_ns = target.as_picoseconds() / 1000;
            let mut v = analog_voltage_at_ns(t_ns);
            // Perturb the second boundary (100 ns) by +20 mV.
            if t_ns == 100 {
                v += 20e-3;
            }
            self.normal.samples.push((target, v));
            self.normal.checkpoints.push(target);
            let checkpoint =
                SparseCheckpoint::empty(target).with_node_voltages(vec![(OBSERVED_NODE, v)]);
            Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
        }

        fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
            self.normal.rollback_to(target)
        }

        fn take_trace(&mut self) -> AnalogTrace {
            self.normal.take_trace()
        }
    }

    let golden_path = write_temp_fixture(
        "mixed-signal-conformance-item-67",
        "mixed-signal-fail-golden.raw",
        &synthesize_golden_rawfile(),
    );
    let golden = conformance_harness::load_ngspice_ascii(&golden_path).expect("parse golden");
    let tolerance = AnalysisKind::Transient.default_tolerance();

    let analog = PerturbedAnalog {
        normal: RampAnalogSolver::new(),
        counter: 0,
    };
    let digital = ScriptedDigitalSimulator::new();
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(HORIZON_NS),
    );
    let result = scheduler.run().expect("run ok");

    // Extract boundary analog values (skip t=0).
    let analog_samples = extract_analog_at_boundaries(&result);
    let boundary_analog: Vec<(f64, f64)> = analog_samples.into_iter().skip(1).collect();
    let actual_series: Vec<f64> = boundary_analog.iter().map(|(_, v)| *v).collect();
    let report = compare(
        &golden,
        [(VAR_ANALOG_OUT, actual_series.as_slice())],
        tolerance,
        16,
    );

    // Must fail.
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Fail,
        "analog conformance must fail with perturbed voltage; report = {report:#?}"
    );
    assert!(
        report.n_failed_variables > 0,
        "at least one variable must fail"
    );
}

/// **Fail variant: digital event trace mismatch at a cycle boundary.**
///
/// Use a digital simulator that omits one event from its trace and
/// verify that event-trace equivalence check catches it.
#[test]
fn mixed_signal_conformance_fails_when_digital_event_trace_mismatched() {
    /// A digital simulator that confirms events but omits one from the
    /// trace (simulating an integration bug).
    struct OmissionDigital {
        events: Vec<SimulationTime>,
        upcoming: std::collections::VecDeque<i64>,
        omit_at_ns: i64,
    }

    impl OmissionDigital {
        fn new(omit_at_ns: i64) -> Self {
            let upcoming: std::collections::VecDeque<i64> = BOUNDARIES_NS.iter().copied().collect();
            Self {
                events: Vec::new(),
                upcoming,
                omit_at_ns,
            }
        }
    }

    impl DigitalSimulator for OmissionDigital {
        fn adapter_kind(&self) -> DigitalAdapterKind {
            DigitalAdapterKind::TestDouble
        }

        fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
            match self.upcoming.pop_front() {
                Some(t_ns) => Ok(NextEventReport {
                    predicted_time: SimulationTime::from_nanoseconds(t_ns),
                }),
                None => Err(SchedulerError::DigitalAdapterFailed("end".into())),
            }
        }

        fn confirm_event(
            &mut self,
            boundary: SimulationTime,
        ) -> Result<DigitalStepReport, SchedulerError> {
            let t_ns = boundary.as_picoseconds() / 1000;
            // Omit event at the specified time from the trace.
            if t_ns != self.omit_at_ns {
                self.events.push(boundary);
            }
            Ok(DigitalStepReport::Confirmed { time: boundary })
        }

        fn take_trace(&mut self) -> DigitalEventTrace {
            DigitalEventTrace {
                vcd: String::new(),
                events_by_signal: vec![(
                    SignalName::new(DIGITAL_SIGNAL),
                    std::mem::take(&mut self.events),
                )],
            }
        }
    }

    let analog = RampAnalogSolver::new();
    // Omit the event at 100 ns from the digital trace.
    let digital = OmissionDigital::new(100);
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(HORIZON_NS),
    );
    let result = scheduler.run().expect("run ok");

    let golden_digital = expected_digital_events();
    let err = check_digital_event_trace_equivalence(&result.digital, &golden_digital);
    assert!(
        err.is_err(),
        "digital event-trace equivalence must detect omission; got err = {err:?}"
    );
    let msg = err.unwrap_err();
    assert!(
        msg.contains("count mismatch") || msg.contains("mismatch"),
        "error message must indicate event mismatch: {msg}"
    );
}

/// **Sanity test: the scheduler commits at every cycle boundary.**
#[test]
fn mixed_signal_scheduler_commits_at_every_boundary() {
    let analog = RampAnalogSolver::new();
    let digital = ScriptedDigitalSimulator::new();
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(HORIZON_NS),
    );
    let result = scheduler.run().expect("run ok");

    // The scheduler must have committed at every boundary.
    assert_eq!(
        result.scheduler.commits.len(),
        BOUNDARIES_NS.len(),
        "one commit per cycle boundary"
    );
    for (&t_ns, commit) in BOUNDARIES_NS.iter().zip(result.scheduler.commits.iter()) {
        assert_eq!(
            *commit,
            SimulationTime::from_nanoseconds(t_ns),
            "commit at {t_ns} ns"
        );
    }

    // No rollbacks on the correct-prediction path.
    assert!(result.rollback_free());
    assert!(result.scheduler.rollbacks.is_empty());
}
