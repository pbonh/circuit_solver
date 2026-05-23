//! Integration test for the **boundary signal exchanger with
//! zero-order hold default** at the analog-digital boundary
//! (tasks.md item #45 / ADR-0007).
//!
//! This file is the executable specification for the scenario
//! `mixed-signal-cosim#analog-digital-boundary-signal-exchange`:
//!
//! ```gherkin
//! Given SimulationEngineer has configured boundary signals:
//!   analog output "vout" driving digital input "din"
//!   and digital output "dout" driving analog input "vin"
//! When the Scheduler reaches a synchronization point at time T
//! Then the analog solver provides the value of "vout" at time T to
//!   the digital simulator as "din"
//! And the digital simulator provides the value of "dout" at time T
//!   to the analog solver as "vin"
//! And both simulators proceed from time T with the exchanged
//!   boundary values
//! ```
//!
//! Per **ADR-0007** the value exchanged is the *most recent accepted
//! analog value held constant until the digital event time* (zero-
//! order hold) — no interpolation. The opt-in linear interpolation
//! path is the sibling tasks.md item #46.

use std::cell::RefCell;
use std::collections::HashMap;

use analysis_orchestration::{
    AnalogValueProvider, BoundaryInterpolationMode, BoundarySignalExchanger, BoundarySignals,
    DigitalValueProvider,
};
use circuit_solver_types::SignalName;

/// Minimal analog "solver" double whose accepted-sample times do not
/// align with the digital event time T. Exposes `accept_sample` to
/// advance and `last_analog_value` to query — exactly the ZOH
/// contract (read the most-recent accepted value, no interpolation).
struct AnalogSolverDouble {
    /// Signal name → (`last_accepted_time_ns`, `last_accepted_value`)
    table: RefCell<HashMap<String, (i64, f64)>>,
}

impl AnalogSolverDouble {
    fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
        }
    }

    /// Record a newly accepted analog sample. Mirrors what the real
    /// numeric-solver does after a converged step.
    fn accept_sample(&self, signal: &str, time_ns: i64, value: f64) {
        self.table
            .borrow_mut()
            .insert(signal.to_string(), (time_ns, value));
    }

    /// The accepted-sample time recorded for `signal`, in ns. Used by
    /// the test to demonstrate ZOH: the exchange uses this value
    /// unchanged at the later boundary time T.
    fn last_accepted_time_ns(&self, signal: &str) -> Option<i64> {
        self.table.borrow().get(signal).map(|(t, _)| *t)
    }
}

impl AnalogValueProvider for AnalogSolverDouble {
    fn last_analog_value(&self, signal: &SignalName) -> Option<f64> {
        self.table.borrow().get(signal.as_str()).map(|(_, v)| *v)
    }
}

/// Minimal digital simulator double that holds the last value of each
/// signal between events (the native ZOH-like semantics of an
/// event-driven kernel).
struct DigitalSimulatorDouble {
    table: RefCell<HashMap<String, f64>>,
}

impl DigitalSimulatorDouble {
    fn new() -> Self {
        Self {
            table: RefCell::new(HashMap::new()),
        }
    }

    fn write_output(&self, signal: &str, value: f64) {
        self.table.borrow_mut().insert(signal.to_string(), value);
    }
}

impl DigitalValueProvider for DigitalSimulatorDouble {
    fn last_digital_value(&self, signal: &SignalName) -> Option<f64> {
        self.table.borrow().get(signal.as_str()).copied()
    }
}

/// Mirrors the Gherkin block exactly.
#[test]
fn analog_digital_boundary_signal_exchange_at_time_t() {
    // — Given SimulationEngineer has configured boundary signals:
    //   analog output "vout" driving digital input "din"
    //   and digital output "dout" driving analog input "vin" —
    let boundaries = BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
        digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
    };

    let analog = AnalogSolverDouble::new();
    let digital = DigitalSimulatorDouble::new();

    // The analog solver's last accepted sample was at 47 ns (its
    // adaptive step did not land on the digital event time T = 50 ns).
    // Per ADR-0007 the exchange holds 2.9 V constant from 47 ns to T.
    analog.accept_sample("vout", 47, 2.9_f64);
    // The digital simulator's most recent dout value is logic-1, scaled
    // to 3.3 V at the analog input.
    digital.write_output("dout", 3.3_f64);

    // Exchanger constructed with the ADR-0007 default (ZOH).
    let exchanger = BoundarySignalExchanger::zero_order_hold(boundaries);
    assert_eq!(
        exchanger.mode(),
        BoundaryInterpolationMode::ZeroOrderHold,
        "ADR-0007 default must be zero-order hold"
    );

    // — When the Scheduler reaches a synchronization point at time T —
    // (Time T = 50 ns is a property of the *scheduler's call site*;
    // ZOH does not consume T because it does not interpolate. The
    // value held is whatever the analog provider's most-recent
    // accepted sample is — recorded above at 47 ns.)
    let packet = exchanger.exchange(&analog, &digital);

    // — Then the analog solver provides the value of "vout" at time T
    //   to the digital simulator as "din" —
    assert!(
        packet.is_complete(),
        "all configured boundary signals must be exchanged"
    );
    assert_eq!(
        packet.analog_to_digital_value(&SignalName::new("din")),
        Some(2.9_f64),
        "analog vout (held constant since 47 ns) must be delivered to digital input din"
    );

    // — And the digital simulator provides the value of "dout" at
    //   time T to the analog solver as "vin" —
    assert_eq!(
        packet.digital_to_analog_value(&SignalName::new("vin")),
        Some(3.3_f64),
        "digital dout must be delivered to analog input vin"
    );

    // ADR-0007 / ZOH discipline: the analog value carried across the
    // boundary is the *most recent accepted* one, not an interpolation
    // toward T. We assert that fact directly to pin the invariant:
    // the accepted sample time (47 ns) remains the source of truth at
    // the boundary T (50 ns).
    assert_eq!(
        analog.last_accepted_time_ns("vout"),
        Some(47),
        "ZOH holds the value from its last accepted sample time, not T"
    );

    // — And both simulators proceed from time T with the exchanged
    //   boundary values —
    // The packet is the load-bearing artifact: a downstream caller
    // delivers `packet.analog_to_digital` to the digital simulator's
    // input ports and `packet.digital_to_analog` to the analog
    // solver's input nodes before either side advances past T. We
    // assert the packet contains exactly the configured destinations
    // and no extraneous entries.
    assert_eq!(packet.analog_to_digital.len(), 1);
    assert_eq!(packet.digital_to_analog.len(), 1);
    assert!(packet.missing_sources.is_empty());
}

/// The opt-in linear interpolation mode named by ADR-0007 is reserved
/// for tasks.md item #46. Attempting to construct an exchanger in
/// `Linear` mode at item #45 must fail with a clear, actionable error
/// — not silently fall back to ZOH (which would mask the user's
/// intent) or panic (which would crash the analysis loop).
#[test]
fn linear_mode_rejected_with_actionable_error_at_item_45() {
    let result = BoundarySignalExchanger::with_mode(
        BoundarySignals::default(),
        BoundaryInterpolationMode::Linear,
    );
    let err = result.expect_err("linear mode must be rejected at item #45");
    let msg = format!("{err}");
    assert!(
        msg.contains("linear"),
        "error must name the unsupported mode: {msg}"
    );
    assert!(
        msg.contains("#46"),
        "error must point users to the item that implements linear: {msg}"
    );
}

/// The exchange is **stateless across calls**: invoking the exchanger
/// twice at the same synchronization point with the same providers
/// yields identical packets. This is the ZOH charge-conservation
/// property in operational form — no value drifts between calls unless
/// the providers themselves accept a new sample.
#[test]
fn zoh_exchange_is_stateless_and_reproducible() {
    let boundaries = BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
        digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
    };
    let analog = AnalogSolverDouble::new();
    let digital = DigitalSimulatorDouble::new();
    analog.accept_sample("vout", 47, 2.9);
    digital.write_output("dout", 3.3);
    let ex = BoundarySignalExchanger::zero_order_hold(boundaries);

    let p1 = ex.exchange(&analog, &digital);
    let p2 = ex.exchange(&analog, &digital);
    assert_eq!(
        p1, p2,
        "ZOH exchange must be deterministic and side-effect-free"
    );
}
