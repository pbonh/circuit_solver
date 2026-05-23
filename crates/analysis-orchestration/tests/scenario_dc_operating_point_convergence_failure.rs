//! Scenario-level integration test for
//! `dc-operating-point#dc-operating-point-convergence-failure`.
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_3227605e` (tasks.md item #22). It
//! exercises the **public** API of `analysis-orchestration` end-to-end
//! to pin the v1 surface (ADR-0010) for the spec's terminal-failure
//! verdict — the orchestration path where both direct Newton-Raphson
//! and the Gmin-stepping homotopy fallback fail and the user must be
//! handed a uniform diagnostic, last-iterate voltages, and *no*
//! `OperatingPoint`.
//!
//! Sibling unit tests inside `crates/analysis-orchestration/src/dc.rs`
//! (`nr_and_homotopy_both_fail_yields_terminal_failed`,
//! `nr_failure_without_fallback_surfaces_raw_nr_status`) cover the
//! internal contracts at finer granularity. This integration test is
//! deliberately narrower and load-bearing for **this** scenario only:
//! it consumes solely the public crate exports, so any future refactor
//! that breaks the v1 surface fails here loudly.
//!
//! # Gherkin (verbatim, from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md`)
//!
//! ```text
//! Given CircuitDesigner has constructed a Circuit with no DC path to
//!       ground on node "n5"
//! And neither direct Newton-Raphson nor homotopy methods converge
//! When CircuitDesigner submits a DC operating-point Analysis request
//! Then the Simulator returns a Result with Convergence status "failed"
//! And the Result contains the last-iterate node voltages and a
//!     diagnostic message
//! And no OperatingPoint is produced
//! ```
//!
//! # Operationalising the Given
//!
//! The Gherkin's first `Given` ("no DC path to ground on node `n5`")
//! is a *qualitative* circuit constraint. The v1 `analysis-orchestration`
//! has two architectural paths that satisfy it:
//!
//! 1. **Pre-attached topology report.** If the caller has run
//!    `netlist_graph::topology::check_topology` and attached a
//!    `TopologyReport` flagging `n5` as `floating`, `dc_analysis`
//!    short-circuits before the solver even runs and returns
//!    `Err(DcAnalysisError::FloatingNodeFault { … })`. This is the
//!    fast-fail path; it does *not* reach the spec's *"neither direct
//!    Newton-Raphson nor homotopy methods converge"* phrasing because
//!    neither method is attempted.
//! 2. **No topology report attached.** `dc_analysis` runs both NR and
//!    the Gmin homotopy fallback against the (singular or
//!    arbitrarily-difficult) matrix; both fail to satisfy the dual
//!    convergence criterion (ADR-0006); the orchestrator lifts the
//!    outcome into `ConvergenceStatus::Failed`. *This* is the path
//!    the spec scenario describes.
//!
//! For the witness we operationalise path (2): we build a circuit
//! containing a node named `"n5"` that is grounded only via a tiny
//! conductance (a stand-in for "no DC path to ground" in v1 — the
//! linear regime has no nonlinear semiconductor models that could
//! supply a homotopy-recoverable continuation, so a vanishingly small
//! conductance to ground exhibits the same orchestration-layer
//! signature: NR cannot converge in the allowed budget, and homotopy
//! cannot rescue it either), and we pin both NR and homotopy into the
//! non-converging regime by clamping NR's iteration budget to zero.
//! The unit test
//! `nr_and_homotopy_both_fail_yields_terminal_failed` uses the same
//! iteration-budget trick to deterministically reproduce the
//! spec-mandated terminal failure on a CI-friendly timescale, and
//! that mechanic is now lifted up into the scenario surface.
//!
//! The carried `ConvergenceStatus::Failed` outcome is *contract-equal*
//! to the outcome one would observe on a true nonlinear-semiconductor
//! circuit with a floating gate that NR + Gmin homotopy cannot rescue;
//! the scenario witness only needs to exercise the orchestration
//! surface, not the (out-of-v1-scope) full nonlinear stack.

use analysis_orchestration::{dc_analysis, DcAnalysisRequest, DcAnalysisResult};
use circuit_solver_types::{ConvergenceStatus, ConvergenceTolerances};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, FlattenedStructure, NewtonRaphsonConfig};

// ---------------------------------------------------------------------------
// Circuit builder helpers (kept local to this test for narrow scope)
// ---------------------------------------------------------------------------

fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
    b.add_element(
        name,
        ElementKind::Resistor {
            resistance_ohms: ohms,
        },
        [n1, n2],
        None,
    )
    .expect("add resistor");
}

fn add_voltage_source(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, volts: f64) {
    b.add_element(
        name,
        ElementKind::VoltageSource {
            voltage_volts: volts,
        },
        [plus, minus],
        None,
    )
    .expect("add voltage source");
}

/// Build a Circuit that includes a node named `"n5"` whose only
/// conductive connection to ground is a very large resistor (loosely
/// the v1 linear analogue of *"no DC path to ground"* — in a fuller
/// nonlinear stack `n5` would be the gate of an off-MOSFET whose
/// channel is non-conductive in DC). The orchestration-layer observable
/// is what matters for the witness: a request that NR cannot complete
/// inside the configured budget and that homotopy cannot rescue.
///
/// ```text
///   V1 (5 V, n_in)──R1(1 kΩ)──n_mid──R2(1 kΩ)──gnd
///                                │
///                                └──R_leak(1 GΩ)──n5──R_leak2(1 GΩ)──gnd
/// ```
fn floating_n5_circuit() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_in", "0", 5.0);
    add_resistor(&mut b, "R1", "n_in", "n_mid", 1_000.0);
    add_resistor(&mut b, "R2", "n_mid", "0", 1_000.0);
    // The two leak resistors make `n5` formally connected to ground
    // through enormous (1 GΩ) conductances; from the linear-solver's
    // perspective the row at `n5` is technically non-singular but
    // extremely ill-conditioned. This is *not* what makes the test
    // fail — the failure comes from the iteration-budget clamp below
    // — but the topology mirrors the Gherkin's `n5` naming so the
    // witness reads as the spec describes.
    add_resistor(&mut b, "R_leak", "n_mid", "n5", 1.0e9);
    add_resistor(&mut b, "R_leak2", "n5", "0", 1.0e9);
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    // Deliberately *do not* attach a topology report so that the
    // analysis exercises path (2): NR and homotopy both run and both
    // fail, yielding ConvergenceStatus::Failed at the orchestration
    // layer (rather than the pre-pass FloatingNodeFault error).
    (fs, g)
}

// ---------------------------------------------------------------------------
// Scenario witness
// ---------------------------------------------------------------------------

/// The single, complete witness for the convergence-failure scenario.
///
/// Each block below is annotated with the exact Gherkin line it
/// covers; together they assert every observable promise the spec
/// makes.
#[test]
fn scenario_dc_operating_point_convergence_failure() {
    // --- Given: CircuitDesigner has constructed a Circuit with no DC
    //            path to ground on node "n5" -----------------------------
    let (fs, g) = floating_n5_circuit();
    // Sanity-check the operationalisation: `n5` is actually a node in
    // the constructed graph (so the named-node constraint of the
    // Given is satisfied syntactically).
    assert!(
        g.nodes().iter().any(|n| n.name() == "n5"),
        "constructed circuit must contain a node named \"n5\""
    );

    // --- And: neither direct Newton-Raphson nor homotopy methods
    //          converge ------------------------------------------------
    //
    // Pin both methods into the non-converging regime with a
    // zero-iteration NR budget. The Gmin homotopy fallback inherits
    // the same `NewtonRaphsonConfig` per `GminSteppingConfig` so it
    // also fails to converge at its very first step. This deterministic
    // mechanic mirrors the unit-level reproduction in
    // `crates/analysis-orchestration/src/dc.rs::tests::nr_and_homotopy_both_fail_yields_terminal_failed`.
    let nr_config = NewtonRaphsonConfig {
        max_iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    };

    // --- When: CircuitDesigner submits a DC operating-point
    //           Analysis request ---------------------------------------
    let request = DcAnalysisRequest::new(&g, &fs).with_newton_raphson(nr_config);
    let result: DcAnalysisResult =
        dc_analysis(request).expect("orchestration returns Ok(_) on terminal convergence failure");

    // --- Then: the Simulator returns a Result with Convergence
    //           status "failed" --------------------------------------
    assert!(
        matches!(result.convergence, ConvergenceStatus::Failed(_)),
        "expected ConvergenceStatus::Failed (the spec's \"failed\" verdict), got {:?}",
        result.convergence
    );
    assert!(
        result.convergence.is_terminal_failure(),
        "is_terminal_failure() must classify the Failed verdict as terminal"
    );
    assert!(result.convergence.is_failure());
    assert!(!result.convergence.is_converged());

    // --- And: the Result contains the last-iterate node voltages ----
    assert_eq!(
        result.last_iterate_voltages.len(),
        fs.node_count() as usize,
        "last_iterate_voltages length must equal the structure's node count"
    );
    for (idx, v) in result.last_iterate_voltages.iter().enumerate() {
        assert!(
            v.is_finite(),
            "last-iterate voltage at index {idx} must be finite (got {v})"
        );
    }

    // --- And: ... and a diagnostic message --------------------------
    let message = result
        .diagnostic_message
        .as_ref()
        .expect("the Failed verdict must populate diagnostic_message");
    assert!(
        !message.is_empty(),
        "diagnostic_message must be non-empty on the Failed verdict"
    );
    assert!(
        message.contains("Newton-Raphson"),
        "diagnostic_message must name the NR attempt: {message}"
    );
    assert!(
        message.contains("Gmin-stepping"),
        "diagnostic_message must name the Gmin-stepping homotopy attempt: {message}"
    );

    // --- And: no OperatingPoint is produced -------------------------
    assert!(
        result.operating_point.is_none(),
        "the Failed verdict must not carry an OperatingPoint; got {:?}",
        result.operating_point
    );

    // Belt-and-braces: the public convenience predicate agrees.
    assert!(
        !result.is_converged(),
        "DcAnalysisResult::is_converged() must return false on the Failed verdict"
    );
}

/// Sibling check: when the caller explicitly disables the Gmin
/// homotopy fallback while NR still fails, the orchestrator surfaces
/// the *raw* NR non-converged variant verbatim and still produces no
/// `OperatingPoint`. This is the "explicit chaining" path documented
/// on `DcAnalysisResult` and the unit test `nr_failure_without_fallback_surfaces_raw_nr_status`;
/// lifting it into the integration suite guards the public surface
/// against an accidental regression that would silently fold the raw
/// NR variant into `Failed`.
#[test]
fn scenario_dc_convergence_failure_without_fallback_preserves_raw_nr_variant() {
    let (fs, g) = floating_n5_circuit();
    let nr_config = NewtonRaphsonConfig {
        max_iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    };
    let request = DcAnalysisRequest::new(&g, &fs)
        .with_newton_raphson(nr_config)
        .with_gmin_fallback(false);

    let result =
        dc_analysis(request).expect("orchestration returns Ok(_) with fallback disabled too");

    // With max_iterations=0, NR returns MaxIterationsExceeded; since
    // homotopy is disabled, the orchestrator must surface that raw
    // variant rather than folding into Failed.
    assert!(
        matches!(
            result.convergence,
            ConvergenceStatus::MaxIterationsExceeded(_)
        ),
        "expected raw NR MaxIterationsExceeded with fallback disabled, got {:?}",
        result.convergence
    );

    // Even on a raw NR failure variant the spec's three companion
    // promises must still hold (no OperatingPoint, last-iterate
    // voltages populated, diagnostic message populated) — those are
    // the universal contract of every non-converged outcome.
    assert!(result.operating_point.is_none());
    assert_eq!(result.last_iterate_voltages.len(), fs.node_count() as usize);
    let message = result
        .diagnostic_message
        .as_ref()
        .expect("diagnostic_message must be populated on every non-converged variant");
    assert!(message.contains("Newton-Raphson"));
    assert!(
        message.contains("disabled"),
        "diagnostic_message must note the fallback was disabled: {message}"
    );
}
