//! Integration test for the **linear interpolation opt-in** at the
//! analog-digital boundary (tasks.md item #46 / ADR-0007 Option C).
//!
//! This file is the executable specification for the linear branch
//! of the scenario
//! `mixed-signal-cosim#analog-digital-boundary-signal-exchange`:
//!
//! ```gherkin
//! Given SimulationEngineer has configured boundary signals:
//!   analog output "vout" driving digital input "din"
//!   and digital output "dout" driving analog input "vin"
//! And SimulationEngineer has opted in to linear interpolation
//!   (boundary_interpolation = "linear", ADR-0007 Option C)
//! When the Scheduler reaches a synchronization point at time T that
//!   falls between the two most recent accepted analog samples
//! Then the analog solver provides the linearly interpolated value
//!   of "vout" at T to the digital simulator as "din"
//! And the digital simulator provides the linearly interpolated value
//!   of "dout" at T to the analog solver as "vin"
//! And both simulators proceed from time T with the exchanged
//!   boundary values
//! ```
//!
//! Per **ADR-0007 Option C**:
//! - The numeric solver retains the two most recent accepted solution
//!   vectors when the user opts in to linear interpolation.
//! - The `BoundarySignalExchanger` interpolates at the event time
//!   using `v(T) = v0 + (v1 - v0) * (T - t0) / (t1 - t0)`.
//! - When only one sample has been accepted yet (typical at t=0), the
//!   linear path degrades to ZOH-equivalent on that signal — the same
//!   single value is delivered.
//! - When no sample exists, the source signal is surfaced in
//!   `missing_sources`, matching the ZOH path.
//!
//! Sibling scenario test (ZOH) lives in
//! `scenario_analog_digital_boundary_signal_exchange.rs`. The two
//! files share the same Gherkin scenario but witness different
//! interpolation modes, demonstrating that the opt-in matters.

#![allow(clippy::float_cmp)] // tests below assert exact ZOH latching at 0.0 V

use std::cell::RefCell;
use std::collections::HashMap;

use analysis_orchestration::{
    AnalogSampleHistoryProvider, AnalogValueProvider, BoundaryInterpolationMode, BoundarySample,
    BoundarySignalExchanger, BoundarySignals, DigitalSampleHistoryProvider, DigitalValueProvider,
};
use circuit_solver_types::SignalName;

/// Two-sample sliding window per signal — exactly what
/// [`AnalogSampleHistoryProvider`] / [`DigitalSampleHistoryProvider`]
/// return.
type SampleWindow = (Option<BoundarySample>, Option<BoundarySample>);

/// Minimal analog "solver" double that remembers the two most recent
/// accepted `(time, value)` samples per signal — exactly the
/// retain-two-vectors discipline ADR-0007 names for the linear opt-in.
struct AnalogSolverHistoryDouble {
    /// Signal name → (older, newer); `None` slots until samples are
    /// accepted.
    table: RefCell<HashMap<String, SampleWindow>>,
}

impl AnalogSolverHistoryDouble {
    fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
        }
    }

    /// Push a newly-accepted sample; the previous "newer" rotates to
    /// "older" and the prior "older" is dropped. Mirrors the sliding
    /// two-vector window the ADR-0007 retain-policy requires.
    fn accept_sample(&self, signal: &str, time_ns: i64, value: f64) {
        let mut t = self.table.borrow_mut();
        let entry = t.entry(signal.to_string()).or_insert((None, None));
        entry.0 = entry.1.take();
        entry.1 = Some(BoundarySample::new(time_ns, value));
    }
}

impl AnalogSampleHistoryProvider for AnalogSolverHistoryDouble {
    fn analog_sample_history(&self, signal: &SignalName) -> SampleWindow {
        self.table
            .borrow()
            .get(signal.as_str())
            .copied()
            .unwrap_or((None, None))
    }
}

/// Minimal digital simulator double with the same retain-two-samples
/// discipline. Digital outputs are event-driven; the encoding into
/// `f64` is the adapter's responsibility — here we use 0.0/1.0 as a
/// logic-level example.
struct DigitalSimulatorHistoryDouble {
    table: RefCell<HashMap<String, SampleWindow>>,
}

impl DigitalSimulatorHistoryDouble {
    fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
        }
    }

    fn accept_sample(&self, signal: &str, time_ns: i64, value: f64) {
        let mut t = self.table.borrow_mut();
        let entry = t.entry(signal.to_string()).or_insert((None, None));
        entry.0 = entry.1.take();
        entry.1 = Some(BoundarySample::new(time_ns, value));
    }
}

impl DigitalSampleHistoryProvider for DigitalSimulatorHistoryDouble {
    fn digital_sample_history(&self, signal: &SignalName) -> SampleWindow {
        self.table
            .borrow()
            .get(signal.as_str())
            .copied()
            .unwrap_or((None, None))
    }
}

/// ZOH-side analog provider used by the disagreement witness — holds
/// a single latched analog value.
struct LatchedAnalogValueProvider {
    analog: f64,
}

impl AnalogValueProvider for LatchedAnalogValueProvider {
    fn last_analog_value(&self, _signal: &SignalName) -> Option<f64> {
        Some(self.analog)
    }
}

/// ZOH-side digital provider used by the disagreement witness — the
/// witness only cares about the analog→digital direction so this one
/// always returns `None`.
struct EmptyDigitalValueProvider;

impl DigitalValueProvider for EmptyDigitalValueProvider {
    fn last_digital_value(&self, _signal: &SignalName) -> Option<f64> {
        None
    }
}

fn vout_din_dout_vin() -> BoundarySignals {
    BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
        digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
    }
}

/// Scenario witness: `SimulationEngineer` opts in to linear; T falls
/// between the two accepted analog samples; the exchanger emits the
/// linearly-interpolated value at T (not the most-recent or the
/// closest-in-time value).
#[test]
fn scenario_linear_interpolation_at_t_between_samples() {
    // Given: SimulationEngineer has configured boundary signals
    let boundaries = vout_din_dout_vin();

    // And: SimulationEngineer has opted in to linear interpolation.
    let exchanger = BoundarySignalExchanger::linear(boundaries);
    assert_eq!(exchanger.mode(), BoundaryInterpolationMode::Linear);

    // And: the analog solver has accepted two samples for vout —
    //   (t=0 ns, 0.0 V) and (t=10 ns, 3.3 V). Picture a rising edge.
    let analog = AnalogSolverHistoryDouble::new();
    analog.accept_sample("vout", 0, 0.0);
    analog.accept_sample("vout", 10, 3.3);

    // And: the digital simulator has accepted two samples for dout —
    //   (t=0 ns, 0.0) and (t=10 ns, 1.0). Quantised but we still
    //   apply linear per ADR-0007 since SimulationEngineer opted in.
    let digital = DigitalSimulatorHistoryDouble::new();
    digital.accept_sample("dout", 0, 0.0);
    digital.accept_sample("dout", 10, 1.0);

    // When: the Scheduler reaches a synchronization point at T=5 ns
    //   (midpoint, no accepted sample exactly at T).
    let packet = exchanger.exchange_linear(5, &analog, &digital).unwrap();

    // Then: the analog solver provides the value of vout at T to the
    //   digital simulator as din, *linearly interpolated* — 1.65 V
    //   (= 0 + 3.3 * 5/10).
    assert!(packet.is_complete(), "every configured pair must resolve");
    let din = packet
        .analog_to_digital_value(&SignalName::new("din"))
        .expect("din must be delivered");
    assert!(
        (din - 1.65).abs() < 1e-12,
        "expected linear midpoint 1.65 V, got {din}"
    );

    // And: the digital simulator provides the value of dout at T to
    //   the analog solver as vin, *linearly interpolated* — 0.5
    //   (= 0 + 1.0 * 5/10).
    let vin = packet
        .digital_to_analog_value(&SignalName::new("vin"))
        .expect("vin must be delivered");
    assert!(
        (vin - 0.5).abs() < 1e-12,
        "expected linear midpoint 0.5, got {vin}"
    );

    // And: both simulators proceed from time T with the exchanged
    //   boundary values — the scenario does not assert what they do
    //   with them; the exchanger's contract ends here.
}

/// At t=0 before the analog solver has produced a second accepted
/// sample, the linear path degrades to ZOH-equivalent on that signal.
/// The scenario "both simulators proceed" still holds — the
/// exchanged value is simply the lone available sample's value.
#[test]
fn scenario_linear_with_single_sample_degrades_to_zoh() {
    let exchanger = BoundarySignalExchanger::linear(vout_din_dout_vin());

    let analog = AnalogSolverHistoryDouble::new();
    analog.accept_sample("vout", 0, 1.65); // only one accepted sample yet

    let digital = DigitalSimulatorHistoryDouble::new();
    digital.accept_sample("dout", 0, 0.0); // ditto for the digital side

    let packet = exchanger.exchange_linear(7, &analog, &digital).unwrap();
    assert!(packet.is_complete());
    assert_eq!(
        packet.analog_to_digital_value(&SignalName::new("din")),
        Some(1.65),
        "single-sample fallback must return that sample's value"
    );
    assert_eq!(
        packet.digital_to_analog_value(&SignalName::new("vin")),
        Some(0.0)
    );
}

/// When the analog solver has accepted no boundary samples for the
/// configured source, the linear path surfaces the source name in
/// `missing_sources` rather than inventing a default — matching the
/// ZOH path's "do not invent" invariant.
#[test]
fn scenario_linear_with_no_samples_surfaces_missing() {
    let exchanger = BoundarySignalExchanger::linear(vout_din_dout_vin());
    let analog = AnalogSolverHistoryDouble::new();
    let digital = DigitalSimulatorHistoryDouble::new();
    let packet = exchanger.exchange_linear(0, &analog, &digital).unwrap();
    assert!(!packet.is_complete());
    assert_eq!(
        packet.missing_sources,
        vec![SignalName::new("vout"), SignalName::new("dout")]
    );
}

/// The opt-in matters: with the same fast edge, ZOH and Linear emit
/// different values at the same T. The scenario does not assert
/// which is "correct" — both satisfy "both simulators proceed with
/// the exchanged values" — but the `SimulationEngineer`'s opt-in must
/// be observable.
#[test]
fn scenario_linear_disagrees_with_zoh_on_fast_edge() {
    let boundaries = BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
        digital_to_analog: vec![],
    };

    // Linear path with two samples bracketing the edge.
    let lin = BoundarySignalExchanger::linear(boundaries.clone());
    let lin_analog = AnalogSolverHistoryDouble::new();
    lin_analog.accept_sample("vout", 0, 0.0);
    lin_analog.accept_sample("vout", 10, 3.3);
    let lin_digital = DigitalSimulatorHistoryDouble::new();
    let lin_pkt = lin.exchange_linear(5, &lin_analog, &lin_digital).unwrap();
    let lin_v = lin_pkt
        .analog_to_digital_value(&SignalName::new("din"))
        .unwrap();

    // ZOH path with the *pre-edge* value latched (analog has not yet
    // accepted the 3.3 V step at T=5 ns).
    let zoh = BoundarySignalExchanger::zero_order_hold(boundaries);
    let zoh_pkt = zoh.exchange(
        &LatchedAnalogValueProvider { analog: 0.0 },
        &EmptyDigitalValueProvider,
    );
    let zoh_v = zoh_pkt
        .analog_to_digital_value(&SignalName::new("din"))
        .unwrap();

    assert_eq!(zoh_v, 0.0, "ZOH latches pre-edge value");
    assert!((lin_v - 1.65).abs() < 1e-12, "Linear midpoint is 1.65 V");
    assert!(
        (zoh_v - lin_v).abs() > 1e-3,
        "ZOH and Linear must disagree on fast edges (got {zoh_v} vs {lin_v})"
    );
}
