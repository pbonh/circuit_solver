//! Scenario witness for
//! `dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy`.
//! This integration test pins the spec scenario directly. The
//! Gherkin steps appear inline as comments so a future reader can
//! line up each `Given`/`When`/`Then` with the corresponding
//! assertion. Per ADR-0010 the public Rust API surface is unstable
//! at v1; this test pins the *behavior* of `GminSteppingDriver`,
//! not its type signatures.
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit from a netlist
//!   containing floating nodes
//! And direct Newton-Raphson on the Circuit fails to converge
//! When CircuitDesigner submits a DC operating-point Analysis request
//! Then the Simulator applies Gmin-stepping homotopy
//! And the Simulator returns a Result containing an OperatingPoint
//! And the Convergence status is "converged-via-homotopy"
//! And the homotopy step count is reported in the Result
//! ```
//!
//! The full analysis-control-loop wiring (tasks.md #20 / #22) lifts
//! the `HomotopyStatus::ConvergedViaHomotopy` we produce here into
//! the user-facing `"converged-via-homotopy"` label. This test
//! verifies the *machinery* — the loop produces the right typed
//! outcome on the right scenario — independent of that lifting.

// Tests intentionally use `other => panic!(...)` arms to pin "is X,
// anything-else fails"; suppress the pedantic complaint that the
// non-converged branch only matches a single variant today.
#![allow(clippy::match_wildcard_for_single_variants)]

use circuit_solver_types::ConvergenceStatus;
use numeric_solver::linear_solver::{RussellRealSolver, SparseLinearSystem, SparseTriplet};
use numeric_solver::newton_raphson::{NewtonRaphsonDriver, NonlinearSystem, SystemError};
use numeric_solver::{GminSteppingConfig, GminSteppingDriver, HomotopyStatus, NewtonRaphsonConfig};

/// A minimal ground-suppressed MNA system representing a floating-
/// node circuit. The unsuppressed matrix would be `[0, 0; 0, 0]`
/// (no DC path from either node to anywhere); ground suppression
/// at index 0 pins row 0 to the basis row `e_0` but leaves row 1
/// (the floating non-ground node) as `0·v1 = 0`, which is singular.
///
/// This is precisely the *floating-nodes* netlist class the spec
/// scenario calls out: there is no isolated solution at `gmin = 0`.
/// Gmin-stepping adds `gmin` to the row-1 diagonal so the system
/// becomes `[1, 0; 0, gmin] · x = [0, 0]` with the unique
/// solution `x = [0, 0]` for any `gmin > 0`.
struct FloatingNodeCircuit {
    linearize_calls: usize,
    residue_calls: usize,
}

impl FloatingNodeCircuit {
    fn new() -> Self {
        Self {
            linearize_calls: 0,
            residue_calls: 0,
        }
    }
}

impl NonlinearSystem for FloatingNodeCircuit {
    fn dim(&self) -> u32 {
        2
    }

    fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
        self.linearize_calls += 1;
        SparseLinearSystem::new(
            2,
            2,
            0,
            vec![SparseTriplet {
                row: 0,
                col: 0,
                value: 1.0,
            }],
            vec![0.0, 0.0],
        )
        .map_err(|e| SystemError::new(format!("{e}")))
    }

    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
        self.residue_calls += 1;
        // F(x) = [v_gnd - 0; 0 (floating row residue is identically zero)].
        Ok(vec![iterate[0], 0.0])
    }
}

#[test]
fn dc_operating_point_with_gmin_stepping_homotopy_scenario() {
    // ─── Given: a Circuit with floating nodes ───────────────────
    let mut circuit = FloatingNodeCircuit::new();

    // ─── And: direct Newton-Raphson fails to converge ───────────
    //
    // We verify this precondition explicitly: NR on the un-shunted
    // system hits the singular row-1 (`0·v1 = 0`) on the first
    // linear solve, which the linear-solver backend reports as
    // `SingularMatrix`. Per the NewtonRaphsonDriver's docs that
    // failure mode collapses into `ConvergenceStatus::Diverged`
    // (last-iterate diagnostics preserved). The spec calls this
    // "fails to converge"; we confirm that's exactly what the
    // typed outcome reports.
    let direct = NewtonRaphsonDriver
        .solve(
            NewtonRaphsonConfig::DC_DEFAULTS,
            &mut circuit,
            &RussellRealSolver,
            vec![0.0, 0.0],
        )
        .expect("NR hard-failure surface should not trigger here");
    assert!(
        !direct.status.is_converged(),
        "direct NR must fail on a floating-node circuit, got {:?}",
        direct.status
    );
    // The specific failure should be `Diverged` (singular linear
    // system), not `Stalled` or `MaxIterationsExceeded`. We don't
    // hard-pin that — the spec only demands "fails to converge"
    // — but documenting it here helps the next reader.
    assert!(matches!(
        direct.status,
        ConvergenceStatus::Diverged(_) | ConvergenceStatus::Stalled(_)
    ));

    // Reset the call counters and re-create the system so the
    // homotopy step counts are clean (the inner system is borrowed
    // mutably by both NR and the homotopy driver; we want this
    // test to measure only the homotopy invocations).
    let mut circuit = FloatingNodeCircuit::new();

    // ─── When: a DC operating-point Analysis request is submitted ─
    // ─── Then: the Simulator applies Gmin-stepping homotopy ─────
    let outcome = GminSteppingDriver
        .solve(
            GminSteppingConfig::DC_DEFAULTS,
            &mut circuit,
            &RussellRealSolver,
            vec![0.0, 0.0],
        )
        .expect("homotopy must succeed on this floating-node scenario");

    // ─── And: the Simulator returns a Result containing an OperatingPoint ─
    // The "OperatingPoint" in our typed surface is the
    // `outcome.iterate` vector (node-voltage values). It must have
    // the system's dim.
    assert_eq!(outcome.iterate.len(), 2);

    // ─── And: the Convergence status is "converged-via-homotopy" ─
    // The typed equivalent of the spec's string label is
    // `HomotopyStatus::ConvergedViaHomotopy`. tasks.md #20 will
    // lift this into the user-facing string.
    let (steps, final_diag) = match outcome.status {
        HomotopyStatus::ConvergedViaHomotopy {
            steps,
            final_diagnostic,
        } => (steps, final_diagnostic),
        other => panic!("expected HomotopyStatus::ConvergedViaHomotopy, got {other:?}"),
    };
    assert!(outcome.status.is_converged());

    // ─── And: the homotopy step count is reported in the Result ─
    // The step count is exposed on the typed outcome (the spec
    // requires *reporting* the count; the user-facing wrapping at
    // tasks.md #20 surfaces it via the `Result` type).
    assert!(
        steps >= 1,
        "homotopy must perform at least one step, got {steps}"
    );

    // Sanity: the final NR diagnostic at gmin = final_gmin is
    // well-conditioned and dual-satisfied.
    assert!(
        final_diag.dual_satisfied(),
        "final-step NR must satisfy dual criterion: {final_diag:?}"
    );

    // Sanity: the operating point for this trivially-zero-solution
    // circuit is the zero vector.
    for &v in &outcome.iterate {
        assert!(v.abs() < 1e-9, "expected zero solution, got {v}");
    }
}

/// Pinning test for the **step count** reported on a successful
/// homotopy with the SPICE-default schedule. If the schedule
/// changes in a future revision this test fails loudly so any
/// downstream consumer that depends on the count is notified.
///
/// The SPICE default: `initial=1.0, final=1e-12, ratio=10`. The
/// geometric walk emits `1, 0.1, 0.01, ..., 1e-13` (14 values
/// because `1e-13 > 1e-12` due to float arithmetic on the
/// inclusive-upper-bound condition), then the explicit terminal
/// `1e-12` is appended → 14 steps total.
#[test]
fn spice_default_homotopy_emits_expected_step_count() {
    let mut circuit = FloatingNodeCircuit::new();
    let outcome = GminSteppingDriver
        .solve(
            GminSteppingConfig::DC_DEFAULTS,
            &mut circuit,
            &RussellRealSolver,
            vec![0.0, 0.0],
        )
        .unwrap();
    let steps = match outcome.status {
        HomotopyStatus::ConvergedViaHomotopy { steps, .. } => steps,
        other => panic!("expected converged, got {other:?}"),
    };
    // SPICE-default schedule pinned to this exact count. Bumping
    // it requires re-thinking downstream telemetry that depends
    // on the SPICE-conventional step count.
    assert_eq!(steps, 14);
}
