//! Scenario-level integration witness for
//! `ac-small-signal#ac-analysis-on-circuit-with-failed-operating-point`.
//!
//! Per the executable specification (verbatim Gherkin block from
//! the kanban task body / spec):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit
//! And the automatic DC OperatingPoint computation fails with Convergence status "failed"
//! When CircuitDesigner submits an AC small-signal Analysis request
//! Then the Simulator returns a Result with Convergence status "failed"
//! And the Result contains the DC failure diagnostic
//! And no AC frequency-domain data is produced
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario as a single item:
//!
//! - **#27** — AC failure short-circuit (this witness's load-bearing
//!   surface). Depends on #26 (auto-DC AC composition,
//!   `ac_analysis_with_auto_dc`).
//!
//! The control-loop landing carries inline unit-test witnesses in
//! `crates/analysis-orchestration/src/auto_dc_ac.rs` exercising the
//! same code path at finer granularity (`is_dc_failed()`, diagnostic
//! shape, AC short-circuit ordering). **This file is the
//! scenario-level witness:** it materializes the *Given* clause's
//! "the automatic DC `OperatingPoint` computation fails with
//! Convergence status `failed`" precondition and checks the *three*
//! *Then* clauses one-by-one against the public
//! [`ac_analysis_with_auto_dc`] entry point.
//!
//! # How the Given precondition is materialized
//!
//! The Given clause requires the DC sub-analysis to terminate with
//! Convergence status `failed` — covering the `Stalled`,
//! `MaxIterationsExceeded`, and `Diverged` variants of
//! [`circuit_solver_types::ConvergenceStatus`] (collectively
//! "Convergence status `failed`" in spec language; the
//! ubiquitous-language term is the same as the non-`Converged`
//! outcome of the Newton-Raphson dual criterion per ADR-0006).
//!
//! We construct the failure deterministically without depending on
//! nonlinear-device pathologies (whose homotopy fall-backs live in
//! tasks.md #18 / #19 / #22 — separate slices, out of scope for the
//! AC failure short-circuit semantics tested here). The mechanism:
//!
//!   - Use the same first-order RC low-pass fixture as the sibling
//!     `scenario_ac_without_prior_operating_point` witness — a
//!     circuit whose DC operating point converges trivially under
//!     SPICE-default tolerances.
//!   - Override the NR config with `max_iterations = 1` and
//!     tolerances `(0.0, 0.0)`. Exact-zero tolerances reject every
//!     finite norm (including exact zero — `n < 0.0` is false for
//!     all finite `n`), so the budget is exhausted after one step
//!     and the driver returns
//!     `ConvergenceStatus::MaxIterationsExceeded` — one of the
//!     three variants the spec subsumes under "Convergence status
//!     `failed`".
//!
//! This isolates the failure to the NR control parameters,
//! producing a deterministic Given that does not couple the witness
//! to any device-model code path.
//!
//! # Choice of fixture
//!
//! Same RC low-pass (V1 → R → output → C → gnd, R = 1 kΩ,
//! C = 1 µF) as the sibling witness — chosen to make the contrast
//! sharp: with SPICE-default NR config the same fixture converges
//! and produces an AC sweep (the *Then* clauses of the
//! `ac-analysis-without-prior-operating-point` scenario); with the
//! forced-failure NR config the same fixture short-circuits and
//! produces no AC data (the *Then* clauses of this scenario).
//!
//! The sweep itself is small (3 points) — its content is irrelevant
//! to this witness because *Then-3* requires that no AC data be
//! produced. We pass a non-empty sweep so any regression that
//! caused the function to silently return AC data would be visible.
//!
//! [`ac_analysis_with_auto_dc`]: analysis_orchestration::ac_analysis_with_auto_dc

use analysis_orchestration::{ac_analysis_with_auto_dc, AcWithAutoDcRequest};
use circuit_solver_types::{ConvergenceTolerances, NodeId};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{flatten, NewtonRaphsonConfig};

// =============================================================================
// Fixture: first-order RC low-pass (same as the sibling witness)
// =============================================================================

const R_OHMS: f64 = 1_000.0;
const C_FARADS: f64 = 1.0e-6;
const VSRC_VOLTS: f64 = 1.0;

/// Build the RC low-pass: V1 across `n_in` → 0, R from `n_in` → `n_out`,
/// C from `n_out` → 0. Returns the source circuit graph and the
/// flattened structure. **No `MnaSystem` is built here** — the
/// "Circuit constructed but no `OperatingPoint` computed" precondition
/// is enforced by *not* calling `assemble` in the test setup; only
/// [`ac_analysis_with_auto_dc`] is allowed to materialize one.
///
/// Node layout (matches the sibling witness): `0 = gnd`, `1 = n_in`,
/// `2 = n_out`.
fn build_rc_lowpass() -> (
    netlist_graph::CircuitGraph,
    circuit_solver_types::FlattenedStructure,
) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VSRC_VOLTS,
        },
        ["n_in", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: R_OHMS,
        },
        ["n_in", "n_out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: C_FARADS,
        },
        ["n_out", "0"],
        None,
    )
    .expect("add C1");
    let graph = b.build().expect("graph build ok");
    let flat = flatten(&graph).expect("flatten ok");
    (graph, flat)
}

/// The NR configuration that deterministically forces a non-`Converged`
/// outcome on *any* well-formed circuit: budget of 1 iteration with
/// exact-zero tolerances. See module-level doc for rationale.
fn forced_dc_failure_config() -> NewtonRaphsonConfig {
    NewtonRaphsonConfig {
        max_iterations: 1,
        tolerances: ConvergenceTolerances::new(0.0, 0.0),
    }
}

// =============================================================================
// Scenario witness
// =============================================================================

#[test]
fn ac_analysis_on_circuit_with_failed_operating_point_scenario() {
    // ---- Given ----------------------------------------------------------
    // CircuitDesigner has constructed a Circuit.
    let (graph, flat) = build_rc_lowpass();

    // And the automatic DC OperatingPoint computation fails with
    // Convergence status "failed".
    //
    // Witnessed by construction: the forced-failure NR config
    // guarantees the embedded `dc_analysis` call will terminate
    // without converging (returning one of `Stalled`,
    // `MaxIterationsExceeded`, or `Diverged` — collectively
    // "Convergence status `failed`" in spec language). The DC
    // sub-analysis runs *inside* `ac_analysis_with_auto_dc`; the
    // test scope does not pre-compute or cache any operating point.

    // ---- When -----------------------------------------------------------
    // CircuitDesigner submits an AC small-signal Analysis request.
    let frequencies_hz = [1.0_f64, 100.0, 10_000.0];
    let n_out = NodeId::new(2);
    let outputs = [n_out];
    let request = AcWithAutoDcRequest::new(&graph, &flat, &frequencies_hz, &outputs)
        .with_newton_raphson(forced_dc_failure_config());

    let result = ac_analysis_with_auto_dc(request).expect(
        "When the DC sub-analysis fails to converge, the Simulator must \
         return a Result (Ok arm) carrying Convergence status `failed`; \
         hard errors are reserved for assembly / topology faults and \
         AC-side validation.",
    );

    // ---- Then -----------------------------------------------------------
    // [Then-1] The Simulator returns a Result with Convergence status
    //          "failed".
    //
    // Witnessed by: the function returned `Ok(_)` (not `Err(_)`) —
    // see the `.expect(...)` above — and the carried
    // `dc_convergence` is one of the three non-`Converged` variants
    // (the ubiquitous-language `Convergence "failed"`). We assert
    // both halves of that contract:
    //   (a) the public predicate `is_dc_failed()` returns true;
    //   (b) the raw `ConvergenceStatus::is_failure()` returns true;
    //   (c) `is_dc_converged()` returns false (no Converged variant
    //       can pass the predicate exclusivity below).
    assert!(
        result.is_dc_failed(),
        "Then-1: is_dc_failed() must report failure; got {:?}",
        result.dc_convergence
    );
    assert!(
        result.dc_convergence.is_failure(),
        "Then-1: ConvergenceStatus::is_failure() must be true; got {:?}",
        result.dc_convergence
    );
    assert!(
        !result.is_dc_converged(),
        "Then-1: is_dc_converged() must be false on the failed-DC path; \
         got dc_convergence = {:?}",
        result.dc_convergence
    );

    // [Then-2] The Result contains the DC failure diagnostic.
    //
    // Witnessed by: the embedded `ConvergenceDiagnostic` accessible
    // via `dc_convergence.diagnostic()` carries finite NR norms and
    // a recorded iteration count consistent with the forced budget.
    // The diagnostic is the load-bearing artifact the spec demands —
    // without it the caller could not localize *why* DC failed.
    let diag = result.dc_convergence.diagnostic();
    assert_eq!(
        diag.iterations, 1,
        "Then-2: diagnostic must record the forced budget exhaustion \
         (1 iteration); got {} iterations",
        diag.iterations
    );
    assert!(
        diag.update_norm.is_finite(),
        "Then-2: diagnostic update_norm must be a finite measurement \
         (so the caller can localize the failure); got {}",
        diag.update_norm
    );
    assert!(
        diag.residue_norm.is_finite(),
        "Then-2: diagnostic residue_norm must be a finite measurement \
         (so the caller can localize the failure); got {}",
        diag.residue_norm
    );
    // The diagnostic must also carry the effective tolerances so the
    // caller can correlate the norm values with the criterion that
    // was applied. With our forced-failure config both are 0.0.
    assert_eq!(
        diag.tolerances.update_tol.to_bits(),
        0.0_f64.to_bits(),
        "Then-2: diagnostic must carry the effective update_tol; \
         expected 0.0, got {}",
        diag.tolerances.update_tol
    );
    assert_eq!(
        diag.tolerances.residue_tol.to_bits(),
        0.0_f64.to_bits(),
        "Then-2: diagnostic must carry the effective residue_tol; \
         expected 0.0, got {}",
        diag.tolerances.residue_tol
    );

    // [Then-3] No AC frequency-domain data is produced.
    //
    // Witnessed structurally: `result.ac` is `None`, which is the
    // canonical signal of "AC step skipped". A converged path would
    // have set this to `Some(AcAnalysisResult { transfer_functions:
    // … })` with one entry per requested output; the failed-DC path
    // skips the AC sub-analysis entirely.
    assert!(
        result.ac.is_none(),
        "Then-3: ac must be None on the failed-DC path; got Some"
    );
    // The convenience accessor must also report nothing for any
    // output node — this is the spec contract from the user's
    // perspective: a caller asking for a TransferFunction at any
    // node receives `None`, confirming "no AC frequency-domain data".
    assert!(
        result.transfer_for(n_out).is_none(),
        "Then-3: transfer_for must return None when no AC data was produced"
    );

    eprintln!(
        "ac-analysis-on-circuit-with-failed-operating-point scenario \
         witness: dc_convergence = {:?}; iterations = {}; update_norm = \
         {}; residue_norm = {}; ac.is_none() = {}",
        result.dc_convergence,
        diag.iterations,
        diag.update_norm,
        diag.residue_norm,
        result.ac.is_none(),
    );
}
