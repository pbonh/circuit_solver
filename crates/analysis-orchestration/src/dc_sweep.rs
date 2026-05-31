//! DC sweep control loop.
//!
//! This module covers `tasks.md` item #21 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the per-analysis driver
//! that wraps the single-point [`dc_analysis`] call in an outer loop
//! over a *source parameter range*, producing one
//! [`OperatingPoint`] per sweep value and bundling them in a result
//! that is *addressable by sweep index*.
//!
//! # Spec scope (item #21)
//!
//! `tasks.md` item #21 — *"Implement DC Sweep: iterate over source
//! parameter range, produce one `OperatingPoint` per sweep point,
//! result addressable by index"* — witnesses the
//! [`dc-operating-point#dc-sweep-over-a-voltage-source`][spec]
//! scenario:
//!
//! > Given CircuitDesigner has constructed a Circuit with a swept
//! > voltage source "V1"
//! > And the sweep range is 0 V to 5 V in 11 steps
//! > When CircuitDesigner submits a DC Sweep Analysis request
//! > Then the Simulator returns a Result containing 11 OperatingPoints
//! > And each OperatingPoint matches the corresponding Golden
//! > Reference within the tolerance envelope
//! > And the Result is addressable by sweep index
//!
//! In words: the sweep is the *outer loop* whose body is the
//! single-point [`dc_analysis`] driver (tasks.md #20). The body
//! consumes a graph whose swept-source value has been substituted
//! for the current sweep point via
//! [`CircuitGraph::with_voltage_source_value`].
//!
//! # Why a per-point graph clone
//!
//! Per ADR-0001 (*Immutable `CircuitGraph` handle*) the user-supplied
//! graph is opaque and immutable; we cannot mutate the swept
//! source's `voltage_volts` field in place. Instead, for each sweep
//! point we materialize a *fresh* graph by calling
//! [`CircuitGraph::with_voltage_source_value`] on the user's graph.
//! The substitute graph is independent (no shared storage) and feeds
//! into [`dc_analysis`] like any caller-supplied graph would.
//!
//! The [`FlattenedStructure`] is **reused** across all sweep points
//! because its content is topology-only — incidence vectors,
//! branch/node counts, ground node, optional
//! [`circuit_solver_types::TopologyReport`] — none of which depend
//! on a source's *value*. Re-flattening the substitute graph would
//! produce a [`FlattenedStructure`] that is structurally identical
//! to the input one. The single per-graph substitution + reuse-once
//! pattern keeps the sweep cost dominated by the
//! `dim`-on-`dim` linear solve at each point, not by graph
//! manipulation.
//!
//! # Sibling primitives
//!
//! - [`super::dc_analysis`] — the per-point analysis driver
//!   (tasks.md #20).
//! - [`super::LogSweep`] — the *frequency* flavor of `Sweep`
//!   (tasks.md #28). The DC sweep here is the *voltage* flavor:
//!   structurally analogous but typed against the parameter being
//!   swept.
//! - [`source_stepping`](numeric_solver::source_stepping) — a
//!   superficially similar α-scaling continuation used as a
//!   convergence aid for nonlinear DC (tasks.md #19). It scales
//!   *all* independent sources by a continuation parameter from
//!   `α = 0` to `α = 1`; the DC sweep here targets *one named*
//!   voltage source and walks the user-specified value range. The
//!   two primitives compose orthogonally: a future variant of
//!   [`dc_sweep`] could fall back to source-stepping when a
//!   particular sweep point fails to converge, but this baseline
//!   implementation surfaces each point's [`ConvergenceStatus`]
//!   verbatim and the caller decides whether to retry.
//!
//! # Honored ADRs
//!
//! - **ADR-0001 — Immutable `CircuitGraph` Handle.** Sweep
//!   substitution happens via
//!   [`CircuitGraph::with_voltage_source_value`] which returns a
//!   *new* graph rather than mutating the caller's.
//! - **ADR-0006 — Dual Convergence Criterion for Newton-Raphson.**
//!   Each sweep point's NR run honors the standard dual criterion;
//!   we delegate to the inner [`dc_analysis`] which delegates to
//!   the [`numeric_solver::NewtonRaphsonDriver`]. The sweep itself
//!   does not redefine convergence.
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** Every
//!   surface exported here is part of the v1 *unstable* public Rust
//!   API.
//!
//! # What this module does *not* do
//!
//! - **No nonlinear-aware homotopy fallback.** When a point's NR
//!   call returns a non-`Converged` status, the result is recorded
//!   as-is in the corresponding [`DcSweepPoint`] and the sweep
//!   *continues* to the next point. Whether to retry that point
//!   with Gmin- (tasks.md #18) or source-stepping (tasks.md #19)
//!   is the orchestrator's decision, not the sweep's. The sweep
//!   guarantees all-points-evaluated, not all-points-converged.
//! - **No current-source / parameter sweeps.** v1 pins the swept
//!   parameter to a *voltage source* per the witnessing scenario.
//!   Other sweep flavors are deliberate out-of-scope for the v1
//!   surface.
//! - **No interaction with `AnalysisRequest`.** The Python frontend
//!   converts user-supplied DC-sweep `AnalysisRequest` values into
//!   a [`DcSweepRequest`] before calling [`dc_sweep`]; the
//!   conversion is the frontend's responsibility.
//!
//! [spec]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md
//! [`dc_analysis`]: super::dc_analysis
//! [`FlattenedStructure`]: circuit_solver_types::flattened::FlattenedStructure
//! [`OperatingPoint`]: super::OperatingPoint
//! [`CircuitGraph::with_voltage_source_value`]: netlist_graph::CircuitGraph::with_voltage_source_value
//! [`ConvergenceStatus`]: circuit_solver_types::ConvergenceStatus

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitGraph, VoltageSourceOverrideError};
use numeric_solver::NewtonRaphsonConfig;

use crate::dc::{dc_analysis, DcAnalysisError, DcAnalysisRequest, DcAnalysisResult};

// -----------------------------------------------------------------------------
// Request / Result envelopes
// -----------------------------------------------------------------------------

/// DC sweep analysis input bundle.
///
/// Identifies the swept voltage source (by user-facing element name)
/// and the slice of values to apply at each sweep point. Mirrors the
/// shape of [`DcAnalysisRequest`] for the non-swept fields
/// (`graph`, `structure`, `newton_raphson`, `ground`).
///
/// The Gherkin scenario phrasing
///
/// > Given CircuitDesigner has constructed a Circuit with a swept
/// > voltage source "V1"
/// > And the sweep range is 0 V to 5 V in 11 steps
/// > When CircuitDesigner submits a DC Sweep Analysis request
///
/// maps directly to a single value of this type:
///
/// - `graph` and `structure` — the constructed Circuit;
/// - `source_name = "V1"` — the swept voltage source's name;
/// - `values = &[0.0, 0.5, 1.0, …, 5.0]` — the 11 sweep values.
///
/// Per ADR-0010, this struct's *layout* is unstable for v1; the
/// *semantics* of each field are pinned.
#[derive(Debug, Clone, Copy)]
pub struct DcSweepRequest<'a> {
    /// The immutable source circuit graph (pre-sweep). For each
    /// sweep point the control loop derives a substitute graph via
    /// [`CircuitGraph::with_voltage_source_value`] and threads it
    /// into [`dc_analysis`].
    pub graph: &'a CircuitGraph,
    /// Pass-1 flattened incidence over `graph`. Reused across all
    /// sweep points (topology does not depend on source values).
    pub structure: &'a FlattenedStructure,
    /// User-facing element name of the swept voltage source (e.g.
    /// `"V1"`). Resolved per sweep point against the substitute
    /// graph; an unknown name or wrong-kind element surfaces as
    /// [`DcSweepError::SourceOverrideFailed`] on the *first* sweep
    /// point.
    pub source_name: &'a str,
    /// Sweep values, in volts, applied in order to `source_name`.
    /// Each value drives one [`dc_analysis`] call. May be empty
    /// (the resulting sweep has zero points, which is a degenerate
    /// but legal case).
    pub values: &'a [f64],
    /// Newton-Raphson tuning applied at every sweep point. `None`
    /// defaults to
    /// [`NewtonRaphsonConfig::DC_DEFAULTS`](numeric_solver::NewtonRaphsonConfig::DC_DEFAULTS).
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node. Forwarded verbatim to
    /// [`DcAnalysisRequest::ground`] at every sweep point.
    pub ground: Option<NodeId>,
}

impl<'a> DcSweepRequest<'a> {
    /// Build a request with the SPICE-default Newton-Raphson tuning
    /// and the structure's own ground node.
    #[must_use]
    pub fn new(
        graph: &'a CircuitGraph,
        structure: &'a FlattenedStructure,
        source_name: &'a str,
        values: &'a [f64],
    ) -> Self {
        Self {
            graph,
            structure,
            source_name,
            values,
            newton_raphson: None,
            ground: None,
        }
    }

    /// Builder-style override for Newton-Raphson configuration.
    #[must_use]
    pub fn with_newton_raphson(mut self, config: NewtonRaphsonConfig) -> Self {
        self.newton_raphson = Some(config);
        self
    }

    /// Builder-style override for ground node id.
    #[must_use]
    pub fn with_ground(mut self, ground: NodeId) -> Self {
        self.ground = Some(ground);
        self
    }
}

/// One entry in a [`DcSweepResult`]: the swept-source value applied
/// and the [`DcAnalysisResult`] returned at that point.
///
/// The result is owned (not borrowed) so a `DcSweepResult` can be
/// stored and inspected independently of the request that produced
/// it.
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepPoint {
    /// The value (volts) that was substituted into the swept
    /// voltage source for this point.
    pub source_value: f64,
    /// The single-point DC analysis outcome.
    pub analysis: DcAnalysisResult,
}

/// The bundled result of a DC sweep.
///
/// Per the spec's *"The Result is addressable by sweep index"*
/// acceptance criterion, [`DcSweepResult::point`] looks up by sweep
/// index `0..len`. The `points` field is `pub` so callers can also
/// iterate directly; both surfaces are part of the v1 contract.
#[derive(Debug, Clone, PartialEq)]
pub struct DcSweepResult {
    /// Name of the swept voltage source. Echoed back from the
    /// request so consumers reading the result alone (no request
    /// in hand) can render diagnostics.
    pub source_name: String,
    /// Per-point outcomes, in the same order as the input
    /// [`DcSweepRequest::values`] slice.
    pub points: Vec<DcSweepPoint>,
}

impl DcSweepResult {
    /// Number of sweep points evaluated.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// True iff zero sweep points were evaluated (the caller passed
    /// an empty `values` slice).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Look up one sweep point by index, or `None` if out of range.
    ///
    /// This is the load-bearing accessor for the spec's
    /// *"addressable by sweep index"* criterion.
    #[must_use]
    pub fn point(&self, index: usize) -> Option<&DcSweepPoint> {
        self.points.get(index)
    }

    /// True iff every sweep point converged. Cheap pass over the
    /// per-point convergence flags; useful when the caller wants a
    /// one-line "did the whole sweep succeed" signal without
    /// iterating manually.
    #[must_use]
    pub fn all_converged(&self) -> bool {
        self.points.iter().all(|p| p.analysis.is_converged())
    }
}

// -----------------------------------------------------------------------------
// Error surface
// -----------------------------------------------------------------------------

/// Errors raised by [`dc_sweep`] before or during the sweep in a
/// way that prevented the sweep from running to its natural
/// termination.
///
/// Non-convergence outcomes at individual sweep points are **not**
/// errors here; they are reported on the `Ok` path inside the
/// corresponding [`DcSweepPoint::analysis`]'s
/// [`DcAnalysisResult::convergence`]. The sweep aborts only when a
/// *structural* failure occurs (bad source name, wrong element
/// kind, NaN value, or a hard inner-analysis error).
///
/// This split matches the [`dc_analysis`] convention and ensures
/// that a single transient non-convergence does not lose all the
/// already-computed sweep points.
#[derive(Debug, Clone, PartialEq)]
pub enum DcSweepError {
    /// The swept voltage source could not be resolved or its value
    /// could not be substituted. Surfaced verbatim from
    /// [`CircuitGraph::with_voltage_source_value`]; carries the
    /// underlying [`VoltageSourceOverrideError`] together with the
    /// zero-based sweep index that triggered it.
    SourceOverrideFailed {
        /// Sweep index that triggered the failure (zero-based).
        sweep_index: usize,
        /// Underlying override error from `netlist-graph`.
        source: VoltageSourceOverrideError,
    },
    /// The per-point [`dc_analysis`] call returned a hard error
    /// (assembly failure, sub-view build failure, topology
    /// floating-node fault, Newton-Raphson hard failure). Carries
    /// the underlying [`DcAnalysisError`] together with the
    /// zero-based sweep index that triggered it.
    PointAnalysisFailed {
        /// Sweep index that triggered the failure (zero-based).
        sweep_index: usize,
        /// Underlying analysis error from [`dc_analysis`].
        source: DcAnalysisError,
    },
}

impl core::fmt::Display for DcSweepError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SourceOverrideFailed {
                sweep_index,
                source,
            } => write!(
                f,
                "dc-sweep: source override failed at sweep index {sweep_index}: {source}"
            ),
            Self::PointAnalysisFailed {
                sweep_index,
                source,
            } => write!(
                f,
                "dc-sweep: per-point analysis failed at sweep index {sweep_index}: {source}"
            ),
        }
    }
}

impl std::error::Error for DcSweepError {}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------

/// Run a DC sweep over a voltage source.
///
/// For each value `v` in `req.values`, materialize a substitute
/// graph via
/// [`CircuitGraph::with_voltage_source_value(req.source_name, v)`](CircuitGraph::with_voltage_source_value),
/// call [`dc_analysis`] on the substitute graph paired with
/// `req.structure`, and record the resulting [`DcAnalysisResult`]
/// in a [`DcSweepPoint`].
///
/// The function returns once every sweep point has been visited.
/// Per-point non-convergence does **not** abort the sweep; only
/// structural failures (bad source name, wrong element kind,
/// non-finite value, inner-analysis hard error) do.
///
/// # Errors
///
/// - [`DcSweepError::SourceOverrideFailed`] — the swept source name
///   could not be resolved or the new value was rejected.
/// - [`DcSweepError::PointAnalysisFailed`] — the inner
///   [`dc_analysis`] call returned a hard
///   [`DcAnalysisError`].
///
/// # Panics
///
/// Does not panic in normal operation.
pub fn dc_sweep(req: DcSweepRequest<'_>) -> Result<DcSweepResult, DcSweepError> {
    let mut points = Vec::with_capacity(req.values.len());
    for (sweep_index, value) in req.values.iter().copied().enumerate() {
        let swept_graph = req
            .graph
            .with_voltage_source_value(req.source_name, value)
            .map_err(|source| DcSweepError::SourceOverrideFailed {
                sweep_index,
                source,
            })?;

        let mut inner = DcAnalysisRequest::new(&swept_graph, req.structure);
        if let Some(cfg) = req.newton_raphson {
            inner = inner.with_newton_raphson(cfg);
        }
        if let Some(g) = req.ground {
            inner = inner.with_ground(g);
        }

        let analysis = dc_analysis(inner).map_err(|source| DcSweepError::PointAnalysisFailed {
            sweep_index,
            source,
        })?;

        points.push(DcSweepPoint {
            source_value: value,
            analysis,
        });
    }

    Ok(DcSweepResult {
        source_name: req.source_name.to_string(),
        points,
    })
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::similar_names, clippy::float_cmp)]
mod tests {
    use super::*;
    use circuit_solver_types::ConvergenceTolerances;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use netlist_graph::flatten;

    // -------- helpers ------------------------------------------------------

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

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    /// Voltage divider whose top-source value is the swept
    /// parameter. R1 = R2 = 1 kΩ so `V(n_mid)` = V1 / 2 at every
    /// sweep point.
    fn divider_with_swept_source(initial_v: f64) -> (FlattenedStructure, CircuitGraph) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", initial_v);
        add_resistor(&mut b, "R1", "n_in", "n_mid", 1_000.0);
        add_resistor(&mut b, "R2", "n_mid", "0", 1_000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        (fs, g)
    }

    fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
        g.nodes()
            .iter()
            .find(|n| n.name() == name)
            .expect("node present")
            .id()
    }

    // -------- happy path ---------------------------------------------------

    /// The headline scenario: 11-step sweep 0 V→5 V on `V1` across
    /// a balanced voltage divider. Each sweep point's `V(n_mid)`
    /// must equal `V1 / 2`.
    #[test]
    fn eleven_step_sweep_zero_to_five_volts_matches_analytic_per_point() {
        let (fs, g) = divider_with_swept_source(0.0);
        // 0.0, 0.5, 1.0, ..., 5.0
        let values: Vec<f64> = (0..=10).map(|i| 0.5 * f64::from(i)).collect();
        let req = DcSweepRequest::new(&g, &fs, "V1", &values);
        let result = dc_sweep(req).expect("sweep ok");

        assert_eq!(result.len(), 11, "11 sweep points expected");
        assert!(!result.is_empty());
        assert_eq!(result.source_name, "V1");
        assert!(result.all_converged());

        let n_in = node_id(&g, "n_in");
        let n_mid = node_id(&g, "n_mid");

        for (i, expected_v) in values.iter().copied().enumerate() {
            let pt = result.point(i).expect("point in range");
            assert_eq!(pt.source_value, expected_v);
            let op = pt.analysis.operating_point.as_ref().expect("op available");
            assert!(approx(op.voltage_at(n_in).unwrap(), expected_v, 1e-9));
            assert!(approx(
                op.voltage_at(n_mid).unwrap(),
                expected_v / 2.0,
                1e-9
            ));
            assert!(approx(op.voltage_at(NodeId::GROUND).unwrap(), 0.0, 1e-9));
            // The only MNA branch unknown is V1's current.
            assert_eq!(op.branch_currents.len(), 1);
            // |i_V1| = V1 / (R1 + R2) = V1 / 2 kΩ.
            assert!(approx(
                op.branch_currents[0].current_amperes.abs(),
                expected_v / 2_000.0,
                1e-9,
            ));
        }
    }

    /// `point()` returns `None` past the end and behaves like a
    /// regular slice index.
    #[test]
    fn point_lookup_is_bounded() {
        let (fs, g) = divider_with_swept_source(0.0);
        let values = [1.0_f64, 2.0, 3.0];
        let result = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &values)).expect("ok");
        assert!(result.point(0).is_some());
        assert!(result.point(2).is_some());
        assert!(result.point(3).is_none());
        assert!(result.point(usize::MAX).is_none());
    }

    /// Empty `values` slice is a degenerate-but-legal zero-point
    /// sweep.
    #[test]
    fn empty_values_produces_empty_result() {
        let (fs, g) = divider_with_swept_source(0.0);
        let result = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &[])).expect("ok");
        assert!(result.is_empty());
        assert_eq!(result.len(), 0);
        assert!(result.all_converged(), "vacuously");
    }

    /// Single-element sweep is supported.
    #[test]
    fn single_value_sweep_matches_single_point_dc_analysis() {
        let (fs, g) = divider_with_swept_source(0.0);
        let result = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &[3.0])).expect("ok");
        assert_eq!(result.len(), 1);
        let pt = result.point(0).unwrap();
        let op = pt.analysis.operating_point.as_ref().unwrap();
        let n_mid = node_id(&g, "n_mid");
        assert!(approx(op.voltage_at(n_mid).unwrap(), 1.5, 1e-9));
    }

    /// Builder-style overrides on the request are honored at every
    /// sweep point.
    #[test]
    fn builder_overrides_threaded_into_each_point() {
        let (fs, g) = divider_with_swept_source(0.0);
        let cfg = NewtonRaphsonConfig {
            max_iterations: 10,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        let result = dc_sweep(
            DcSweepRequest::new(&g, &fs, "V1", &[1.0, 2.0])
                .with_newton_raphson(cfg)
                .with_ground(NodeId::GROUND),
        )
        .expect("ok");
        for pt in &result.points {
            // Linear circuit, dual-criterion NR converges in 2
            // iterations under the default DC config (cf.
            // dc.rs:builder_overrides_are_applied).
            assert!(pt.analysis.is_converged());
            assert_eq!(pt.analysis.convergence.diagnostic().iterations, 2);
        }
    }

    // -------- error surface ------------------------------------------------

    /// Unknown source name surfaces as `SourceOverrideFailed` on
    /// the *first* sweep index attempted.
    #[test]
    fn unknown_source_name_errors_on_first_point() {
        let (fs, g) = divider_with_swept_source(0.0);
        let err = dc_sweep(DcSweepRequest::new(&g, &fs, "VBOGUS", &[1.0, 2.0]))
            .expect_err("expected SourceOverrideFailed");
        match err {
            DcSweepError::SourceOverrideFailed {
                sweep_index,
                source: VoltageSourceOverrideError::UnknownElement { element },
            } => {
                assert_eq!(sweep_index, 0);
                assert_eq!(element, "VBOGUS");
            }
            other => panic!("expected SourceOverrideFailed/UnknownElement, got {other:?}"),
        }
    }

    /// Pointing at a non-voltage-source element (here, the resistor
    /// `R1`) errors with `WrongElementKind`.
    #[test]
    fn wrong_element_kind_errors() {
        let (fs, g) = divider_with_swept_source(0.0);
        let err = dc_sweep(DcSweepRequest::new(&g, &fs, "R1", &[1.0]))
            .expect_err("expected SourceOverrideFailed");
        match err {
            DcSweepError::SourceOverrideFailed {
                sweep_index,
                source:
                    VoltageSourceOverrideError::WrongElementKind {
                        element,
                        actual_tag,
                    },
            } => {
                assert_eq!(sweep_index, 0);
                assert_eq!(element, "R1");
                assert_eq!(actual_tag, "R");
            }
            other => panic!("expected SourceOverrideFailed/WrongElementKind, got {other:?}"),
        }
    }

    /// Non-finite sweep values are rejected at the override layer.
    #[test]
    fn non_finite_value_errors() {
        let (fs, g) = divider_with_swept_source(0.0);
        let err = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &[1.0, f64::NAN, 2.0]))
            .expect_err("expected SourceOverrideFailed");
        match err {
            DcSweepError::SourceOverrideFailed {
                sweep_index,
                source: VoltageSourceOverrideError::NonFiniteValue { element, .. },
            } => {
                // index 1 because the first point (1.0) succeeded.
                assert_eq!(sweep_index, 1);
                assert_eq!(element, "V1");
            }
            other => panic!("expected SourceOverrideFailed/NonFiniteValue, got {other:?}"),
        }
    }

    /// The sweep does not share storage between substitute graphs:
    /// the user's original graph is unchanged after the sweep
    /// returns.
    #[test]
    fn user_graph_unchanged_after_sweep() {
        let (fs, g) = divider_with_swept_source(7.5);
        let original_v1 = match g.element_by_name("V1").unwrap().kind() {
            ElementKind::VoltageSource { voltage_volts } => *voltage_volts,
            other => panic!("expected V1 to be a voltage source, got {other:?}"),
        };
        let _ = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &[0.0, 1.0, 2.0])).expect("ok");
        let after_v1 = match g.element_by_name("V1").unwrap().kind() {
            ElementKind::VoltageSource { voltage_volts } => *voltage_volts,
            other => panic!("expected V1 to be a voltage source, got {other:?}"),
        };
        assert_eq!(original_v1, after_v1);
    }
}
