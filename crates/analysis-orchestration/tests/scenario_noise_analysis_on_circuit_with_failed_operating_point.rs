//! Scenario-level integration witness for
//! `noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point`.
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body / spec):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit
//! And the automatic DC OperatingPoint computation fails with Convergence status "failed"
//! When CircuitDesigner submits a noise spectral-density Analysis request
//! Then the Simulator returns a Result with Convergence status "failed"
//! And the Result contains the DC failure diagnostic
//! And no noise spectral-density data is produced
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario as a single item:
//!
//! - **#41** — Noise failure short-circuit (this witness's load-bearing
//!   surface). Depends on **#40** (auto-DC entry point for noise
//!   analysis, `noise_analysis_with_auto_dc`).
//!
//! The auto-DC noise control-loop landed in #40 already carries
//! inline unit-test witnesses in
//! `crates/analysis-orchestration/src/noise.rs` exercising the same
//! short-circuit code path at finer granularity
//! (`auto_dc_failure_short_circuits_without_running_noise_loop`,
//! `auto_dc_hard_dc_error_surfaces_as_dc_failed_variant`). **This
//! file is the scenario-level witness:** it materializes the *Given*
//! clause's "the automatic DC `OperatingPoint` computation fails with
//! Convergence status `failed`" precondition and checks the *three*
//! *Then* clauses one-by-one against the public
//! [`noise_analysis_with_auto_dc`] entry point.
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
//! noise failure short-circuit semantics tested here). The mechanism:
//!
//!   - Use the same single-resistor noise fixture as the sibling
//!     `scenario_noise_analysis_without_prior_operating_point`
//!     witness — a circuit whose DC operating point converges
//!     trivially under SPICE-default tolerances.
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
//! to any device-model code path. It mirrors the construction used
//! by the sibling AC failure short-circuit witness
//! (`scenario_ac_analysis_on_circuit_with_failed_operating_point`),
//! reinforcing that the AC and noise auto-DC entry points share one
//! contract surface.
//!
//! # Choice of fixture
//!
//! Same single-resistor noise topology as the sibling
//! `scenario_noise_analysis_without_prior_operating_point` witness:
//! V1 (1 V) → R1 (1 kΩ) → R2 (1 PΩ) → ground. Chosen to make the
//! contrast sharp: with SPICE-default NR config the same fixture
//! converges and produces a noise sweep (the *Then* clauses of the
//! `noise-analysis-without-prior-operating-point` scenario); with
//! the forced-failure NR config the same fixture short-circuits and
//! produces no noise data (the *Then* clauses of this scenario).
//!
//! The sweep itself is small (3 points) — its content is irrelevant
//! to this witness because *Then-3* requires that no noise data be
//! produced. We pass a non-empty sweep so any regression that
//! caused the function to silently return noise data would be
//! visible.
//!
//! [`noise_analysis_with_auto_dc`]: analysis_orchestration::noise_analysis_with_auto_dc

use analysis_orchestration::{
    noise_analysis_with_auto_dc, NoiseAnalysisWithAutoDcRequest, NoiseAnalysisWithAutoDcResult,
};
use circuit_solver_types::{ConvergenceTolerances, NodeId};
use device_modeling::noise::ROOM_TEMPERATURE_K;
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, NewtonRaphsonConfig};

// =============================================================================
// Fixture: single-resistor noise topology (same as the sibling #40 witness)
// =============================================================================

const R1_OHMS: f64 = 1_000.0;
const R2_OHMS: f64 = 1.0e15; // effective open at noise port
const VSRC_VOLTS: f64 = 1.0;

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

/// Build the single-resistor noise topology used by the sibling #40
/// witness:
///
/// ```text
///     ┌──── V1 (1 V) ──── n_in
///     │                      │
///    GND                     R1 = 1 kΩ
///     │                      │
///     │                    n_out
///     │                      │
///     │                     R2 = 1 PΩ   (effective open at noise port)
///     │                      │
///     └────────────────── GND
/// ```
///
/// **No `MnaSystem` is built here** — the "Circuit constructed but
/// no `OperatingPoint` computed" precondition is enforced by *not*
/// calling `assemble` in the test setup; only
/// [`noise_analysis_with_auto_dc`] is allowed to materialize one.
///
/// Returns the graph plus the resolved `n_out` `NodeId` (the witness
/// port for the noise request).
fn build_single_resistor_witness() -> (CircuitGraph, NodeId) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_in", "0", VSRC_VOLTS);
    add_resistor(&mut b, "R1", "n_in", "n_out", R1_OHMS);
    add_resistor(&mut b, "R2", "n_out", "0", R2_OHMS);
    let graph = b.build().expect("graph build ok");

    let n_out = graph
        .nodes()
        .iter()
        .find(|n| n.name() == "n_out")
        .expect("n_out present")
        .id();

    (graph, n_out)
}

/// The NR configuration that deterministically forces a non-`Converged`
/// outcome on *any* well-formed circuit: budget of 1 iteration with
/// exact-zero tolerances. See module-level doc for rationale.
///
/// Identical construction to the AC failure short-circuit witness'
/// `forced_dc_failure_config()` — the two scenarios share one
/// auto-DC failure semantics, so they share one Given-materialization
/// recipe.
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
#[allow(clippy::too_many_lines)] // verbatim Given/When/Then walkthrough is intentionally linear
fn noise_analysis_on_circuit_with_failed_operating_point_scenario() {
    // ---- Given ----------------------------------------------------------
    // CircuitDesigner has constructed a Circuit.
    let (graph, n_out_id) = build_single_resistor_witness();
    let structure = flatten(&graph).expect("flatten ok");

    // And the automatic DC OperatingPoint computation fails with
    // Convergence status "failed".
    //
    // Witnessed by construction: the forced-failure NR config
    // guarantees the embedded `dc_analysis` call will terminate
    // without converging (returning one of `Stalled`,
    // `MaxIterationsExceeded`, or `Diverged` — collectively
    // "Convergence status `failed`" in spec language). The DC
    // sub-analysis runs *inside* `noise_analysis_with_auto_dc`; the
    // test scope does not pre-compute or cache any operating point.

    // ---- When -----------------------------------------------------------
    // CircuitDesigner submits a noise spectral-density Analysis
    // request.
    let frequencies_hz = [1.0e3_f64, 10.0e3, 100.0e3];
    let request = NoiseAnalysisWithAutoDcRequest::new(
        &graph,
        &structure,
        &frequencies_hz,
        n_out_id,
        ROOM_TEMPERATURE_K,
    )
    .with_newton_raphson(forced_dc_failure_config());

    let result = noise_analysis_with_auto_dc(request).expect(
        "When the DC sub-analysis fails to converge, the Simulator must \
         return a Result (Ok arm) carrying Convergence status `failed`; \
         hard errors are reserved for assembly / topology faults and \
         noise-side validation.",
    );

    // ---- Then -----------------------------------------------------------
    // [Then-1] The Simulator returns a Result with Convergence status
    //          "failed".
    //
    // Witnessed by: the function returned `Ok(_)` (not `Err(_)`) —
    // see the `.expect(...)` above — and the carried result is the
    // `Failed` variant whose `dc_status` is one of the three
    // non-`Converged` variants (the ubiquitous-language
    // `Convergence "failed"`). We assert both halves of that
    // contract:
    //   (a) the public predicate `is_failed()` returns true;
    //   (b) the raw `ConvergenceStatus::is_failure()` returns true;
    //   (c) `is_ok()` returns false (the two predicates are
    //       exhaustive on this enum's two variants).
    assert!(
        result.is_failed(),
        "Then-1: NoiseAnalysisWithAutoDcResult::is_failed() must report \
         failure on the failed-DC path; got {result:?}"
    );
    assert!(
        !result.is_ok(),
        "Then-1: NoiseAnalysisWithAutoDcResult::is_ok() must be false on \
         the failed-DC path; got {result:?}"
    );
    let (dc_status, failure_op) = match &result {
        NoiseAnalysisWithAutoDcResult::Failed {
            dc_status,
            operating_point,
        } => (dc_status, operating_point.as_ref()),
        NoiseAnalysisWithAutoDcResult::Ok { .. } => panic!(
            "Then-1: expected the failed-DC short-circuit variant on a \
             forced-non-convergence run; got Ok(_)"
        ),
    };
    assert!(
        dc_status.is_failure(),
        "Then-1: ConvergenceStatus::is_failure() must be true; got {dc_status:?}"
    );
    assert!(
        !dc_status.is_converged(),
        "Then-1: ConvergenceStatus::is_converged() must be false on the \
         failed-DC path; got {dc_status:?}"
    );

    // [Then-2] The Result contains the DC failure diagnostic.
    //
    // Witnessed by: the embedded `ConvergenceDiagnostic` accessible
    // via `dc_status.diagnostic()` carries finite NR norms and a
    // recorded iteration count consistent with the forced budget.
    // The diagnostic is the load-bearing artifact the spec demands —
    // without it the caller could not localize *why* DC failed.
    //
    // This is the same diagnostic contract the AC sibling scenario
    // (`ac-analysis-on-circuit-with-failed-operating-point`) pins;
    // both auto-DC entry points forward the same
    // `ConvergenceDiagnostic` surface from the inner Newton-Raphson
    // driver.
    let diag = dc_status.diagnostic();
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

    // Per tasks.md #22 (DC convergence-failure envelope, reconciled
    // by t_2eec72d7 inside noise.rs): on the failed-DC path the
    // `Failed::operating_point` is `None`. The diagnostic node
    // voltages are surfaced through `DcAnalysisResult::
    // last_iterate_voltages` (not propagated through this noise
    // result yet; callers needing them must call `dc_analysis`
    // directly or wait for a future surface). This test originally
    // pinned the older contract `Some(last_iterate)`; the contract
    // flipped to `None` to honor the spec scenario
    // `dc-operating-point-convergence-failure` *"no `OperatingPoint`
    // is produced"*.
    assert!(
        failure_op.is_none(),
        "Then-2: NoiseAnalysisWithAutoDcResult::Failed::operating_point \
         must be None on the failed-DC path per the #22 contract; \
         callers that need diagnostic node voltages should consult \
         DcAnalysisResult::last_iterate_voltages on a direct DC call"
    );

    // [Then-3] No noise spectral-density data is produced.
    //
    // Witnessed structurally: the `Failed` variant has no `data`
    // field (an `Ok` variant would have carried
    // `NoiseAnalysisData { spectral_density_v2_per_hz, … }` with one
    // sample per requested frequency), and the public accessor
    // `data()` returns `None`. A converged path would have populated
    // both; the failed-DC path skips the noise sub-analysis
    // entirely.
    assert!(
        result.data().is_none(),
        "Then-3: NoiseAnalysisWithAutoDcResult::data() must return None \
         on the failed-DC path; got Some(_) — noise loop was not \
         supposed to run"
    );

    eprintln!(
        "noise-analysis-on-circuit-with-failed-operating-point scenario \
         witness: dc_status = {:?}; iterations = {}; update_norm = {}; \
         residue_norm = {}; data.is_none() = {}",
        dc_status,
        diag.iterations,
        diag.update_norm,
        diag.residue_norm,
        result.data().is_none(),
    );
}
