//! Scenario test: `transient-time-domain#transient-analysis-with-uic-initial-conditions`.
//!
//! tasks.md item #34 — *"Implement UIC (Use Initial Conditions) mode:
//! skip DC operating point, start from user-supplied node voltages"*.
//!
//! ## Gherkin (from `specs/transient-time-domain/spec.md`, verbatim)
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit
//! And CircuitDesigner specifies UIC with initial node voltages for node "n1" = 3.3 V
//! When CircuitDesigner submits a transient Analysis request with UIC flag
//! Then the Simulator skips the DC OperatingPoint computation
//! And the Simulator starts the transient solve using the user-supplied initial conditions
//! And the Waveform at node "n1" begins at 3.3 V at time 0 s
//! ```
//!
//! ## What this scenario witness exercises
//!
//! The UIC capability (`InitialState::UseInitialConditions { node_voltages }`)
//! was wired by tasks.md #33 into the transient control loop in
//! `src/transient.rs`. The parent task's residual-risk handoff
//! explicitly flagged that the **"Then 1 — Simulator skips the DC
//! `OperatingPoint` computation"** clause was not yet covered by a
//! dedicated scenario witness — the existing UIC-bearing test in
//! `scenario_transient_with_default_method.rs` exercises UIC as a
//! *substitute* for DC OP under the headline scenario, but does not
//! contrast UIC against the DC path on the **same** circuit. This
//! test closes that gap.
//!
//! ### Witness construction
//!
//! We pick a circuit whose DC operating point is *observably different*
//! from the user-supplied UIC. The minimal such circuit:
//!
//! - Node `"n1"` with a 1 nF capacitor to ground.
//! - A 1 kΩ resistor between `"n1"` and ground (DC path so the
//!   topology checker per ADR-0009 does *not* short-circuit the
//!   analysis — we want DC analysis to *succeed*, just with a
//!   different value than UIC, so that the UIC initial sample is
//!   demonstrably not "merely the DC OP that happened to coincide
//!   with the UIC request").
//! - No source.
//!
//! For this passive RC, the DC operating point is `V(n1) = 0 V`
//! (Kirchhoff's current law with no source). Under UIC at 3.3 V the
//! capacitor begins pre-charged and the circuit *discharges*
//! exponentially through R toward 0:
//!
//! ```text
//!     v_n1(t) = 3.3 · exp(-t / τ),   τ = R · C = 1 µs
//! ```
//!
//! The two **observably distinct** outcomes:
//!
//! | `initial_state`                | `wf.values[0]` |
//! | ------------------------------ | ------------ |
//! | `InitialState::DcOperatingPoint` | `0.0 V`    |
//! | `InitialState::UseInitialConditions { n1: 3.3 }` | `3.3 V` |
//!
//! Witnessing **both** within a single test pins down every Then of
//! the Gherkin scenario:
//!
//! 1. **Then 1 — "skips the DC `OperatingPoint` computation"**: the
//!    UIC-arm initial sample is 3.3 V (the user-supplied value).
//!    If `dc_analysis` had run, it would have driven `V(n1) → 0`,
//!    seeding the transient at the *DC* value rather than the UIC
//!    value. The 3.3 V starting sample is positive evidence that
//!    the DC path was not taken. The contrast against the DC-path
//!    arm (which starts at 0 V on the same circuit) cements this.
//! 2. **Then 2 — "starts the transient solve using the user-supplied
//!    initial conditions"**: every subsequent sample of the UIC
//!    waveform tracks `v_n1(t) = 3.3 · exp(-t/τ)`, asserted within
//!    the ADR-0008 transient envelope (1 % rel / 1 mV abs). That
//!    decay shape is the *transient response* of the RC initialized
//!    at the UIC voltage — there is no other initial condition the
//!    integrator could have used to produce that trajectory.
//! 3. **Then 3 — "begins at 3.3 V at time 0 s"**: asserted as a
//!    bit-exact float equality on `wf.values[0]`. The transient
//!    control loop seeds `waveform_values[i][0]` directly from
//!    `initial_node_voltages[i]` (see `src/transient.rs`), so this
//!    equality is structural rather than discretization-dependent.
//!
//! ## ADR alignment
//!
//! - **ADR-0006** — every per-timestep NR solve under both DC and UIC
//!   paths honors the dual-criterion (step + residue) envelope.
//! - **ADR-0007** — vacuous (no analog-digital boundary).
//! - **ADR-0008** — UIC discharge samples checked against the
//!   `max(1 % rel, 1 mV abs)` per-node tolerance envelope.
//! - **ADR-0009** — the witness deliberately uses a DC-connected
//!   topology (R1 to ground) so floating-node detection is *not*
//!   the discriminator between the two arms; the discriminator is
//!   the *initial-state value*, isolating exactly the UIC bypass
//!   contract.
//! - **ADR-0010** — uses only the `analysis_orchestration` public
//!   unstable API.

#![allow(clippy::float_cmp)]

use std::collections::HashMap;

use analysis_orchestration::{transient_analysis, InitialState, TransientAnalysisRequest};
use circuit_solver_types::{NodeId, SimulationTime};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::flatten;

/// ADR-0008 transient envelope: `max(1 % rel, 1 mV abs)` per node.
fn envelope(reference: f64) -> f64 {
    let rel = 0.01 * reference.abs();
    let abs = 1.0e-3;
    if rel > abs {
        rel
    } else {
        abs
    }
}

fn node_id_of(g: &netlist_graph::CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .expect("node present")
        .id()
}

/// Construct the witness circuit: passive RC between `n1` and ground.
///
/// - `R1`: 1 kΩ between `n1` and `0`.
/// - `C1`: 1 nF between `n1` and `0`.
///
/// The "ground" node is conventionally named `"0"` per the netlist
/// graph layer; the spec's node naming uses `"n1"` for the
/// initial-condition node.
fn build_rc_witness() -> (
    netlist_graph::CircuitGraph,
    circuit_solver_types::flattened::FlattenedStructure,
) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: 1.0e3,
        },
        ["n1", "0"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: 1.0e-9,
        },
        ["n1", "0"],
        None,
    )
    .expect("add C1");
    let g = b.build().expect("build");
    let fs = flatten(&g).expect("flatten");
    (g, fs)
}

/// **Headline scenario witness.** UIC at 3.3 V on node `n1`; the
/// transient solve must start from 3.3 V (Then 3) without computing
/// a DC OP (Then 1) and produce a discharge trajectory consistent
/// with the user-supplied initial condition (Then 2).
#[test]
fn scenario_transient_analysis_with_uic_initial_conditions() {
    let (g, fs) = build_rc_witness();
    let n1 = node_id_of(&g, "n1");

    // Given CircuitDesigner specifies UIC with initial node voltages
    // for node "n1" = 3.3 V.
    let mut node_voltages: HashMap<NodeId, f64> = HashMap::new();
    node_voltages.insert(n1, 3.3);

    // When CircuitDesigner submits a transient Analysis request with
    // UIC flag.
    let req = TransientAnalysisRequest::new(
        &g,
        &fs,
        SimulationTime::ZERO,
        SimulationTime::from_nanoseconds(100), // 100 ns ≈ τ/10
        1.0e-9,
    )
    .with_initial_state(InitialState::UseInitialConditions { node_voltages });

    let result = transient_analysis(req).expect("UIC transient analysis succeeds");
    assert!(
        result.is_converged(),
        "final convergence under UIC must be Converged, got {:?}",
        result.final_convergence
    );

    let wf = result
        .transient
        .waveforms
        .iter()
        .find(|w| w.node == n1)
        .expect("n1 waveform present");

    // Then 3 — "And the Waveform at node \"n1\" begins at 3.3 V at
    // time 0 s." Bit-exact: the control loop seeds
    // waveform_values[0] directly from initial_node_voltages[n1].
    assert_eq!(
        wf.values[0], 3.3,
        "Waveform at n1 must begin at exactly 3.3 V at t=0 s, got {}",
        wf.values[0]
    );
    assert_eq!(
        wf.times[0],
        SimulationTime::ZERO,
        "first time sample must be t=0 s, got {:?}",
        wf.times[0]
    );

    // Then 2 — "And the Simulator starts the transient solve using
    // the user-supplied initial conditions." We verify this by
    // checking that every subsequent sample tracks the analytic
    // discharge `3.3 * exp(-t/τ)` within the ADR-0008 envelope.
    // No other initial condition produces this trajectory.
    let tau = 1.0e3_f64 * 1.0e-9_f64; // R·C = 1 µs
    for (t, v) in wf.times.iter().zip(wf.values.iter()) {
        let t_sec = t.as_seconds_f64();
        let v_ref = 3.3 * (-t_sec / tau).exp();
        let tol = envelope(v_ref);
        assert!(
            (v - v_ref).abs() <= tol,
            "UIC discharge mismatch at t={t_sec:.6e}: simulated {v:.6e}, \
             analytic {v_ref:.6e}, envelope ±{tol:.6e}"
        );
    }

    // The waveform must monotonically decay (no source to maintain
    // the charge), confirming the UIC seed initiated a transient
    // rather than the integrator restarting from the DC value.
    let final_v = *wf.values.last().expect("at least one sample");
    assert!(
        final_v < 3.3,
        "capacitor must discharge below 3.3 V over τ/10, got {final_v}"
    );
    assert!(
        final_v > 0.0,
        "capacitor must not over-discharge below 0 V over τ/10, got {final_v}"
    );
}

/// **Then-1 contrast witness.** On the *same* circuit, with no UIC
/// (i.e. `InitialState::DcOperatingPoint`), the DC operating-point
/// computation runs to completion and seeds the transient at
/// `V(n1) = 0 V`. This is the observable inverse of the headline
/// scenario: the DC path *was* taken, so the initial sample is the
/// DC value, not 3.3 V.
///
/// Pairing this with the headline test above pins down Then 1
/// (*"the Simulator skips the DC `OperatingPoint` computation"*) by
/// contrast: same circuit, two different `InitialState` selectors,
/// two different `wf.values[0]`. The 3.3 V starting sample under
/// UIC is positive evidence that the DC arm was bypassed.
#[test]
fn dc_operating_point_arm_seeds_at_zero_under_no_uic_same_circuit() {
    let (g, fs) = build_rc_witness();
    let n1 = node_id_of(&g, "n1");

    // No UIC — default is InitialState::DcOperatingPoint.
    let req = TransientAnalysisRequest::new(
        &g,
        &fs,
        SimulationTime::ZERO,
        SimulationTime::from_nanoseconds(100),
        1.0e-9,
    );

    let result = transient_analysis(req).expect("DC-OP transient analysis succeeds");
    assert!(
        result.is_converged(),
        "final convergence under DC-OP must be Converged, got {:?}",
        result.final_convergence
    );

    let wf = result
        .transient
        .waveforms
        .iter()
        .find(|w| w.node == n1)
        .expect("n1 waveform present");

    // Same circuit, DC-OP path: V(n1) = 0 V (no source, R to ground
    // pulls capacitor node to ground in steady state).
    assert!(
        wf.values[0].abs() <= envelope(0.0),
        "DC-OP arm must seed V(n1) at 0 V (within envelope), got {}",
        wf.values[0]
    );
    // Crucially: NOT the UIC value. If this assertion ever fails
    // alongside the headline test passing, the InitialState
    // selector is being ignored.
    assert_ne!(
        wf.values[0], 3.3,
        "DC-OP arm must not coincide with the UIC value on this circuit"
    );
}
