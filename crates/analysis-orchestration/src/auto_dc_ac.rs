//! Auto-DC AC analysis: compute a DC `OperatingPoint` first when none
//! is cached, then proceed with AC small-signal analysis at that
//! `OperatingPoint`. Returns both in a single bundled [`Result`].
//!
//! This module covers `tasks.md` item #26 of
//! `circuit-solver/2026-05-21-v1-spec`. It is a thin composition of:
//!
//! - the DC operating-point control loop ([`dc_analysis`], tasks.md #20,
//!   landed under `analysis_orchestration::dc`), which produces the
//!   converged `OperatingPoint` plus its `ConvergenceStatus`;
//! - the Pass-2 MNA assembler ([`numeric_solver::assemble()`], tasks.md
//!   #14), called *once again* after DC to materialize the linearized
//!   `MnaSystem` that the AC control loop consumes; and
//! - the AC small-signal control loop ([`ac_analysis`], tasks.md #25,
//!   landed under `analysis_orchestration::ac`), which produces the
//!   per-frequency [`TransferFunction`][crate::ac::TransferFunction]s.
//!
//! # Spec scope (item #26)
//!
//! `tasks.md` item #26 — *"Implement auto-DC computation: when no
//! `OperatingPoint` cached, run DC first, then proceed with AC; return
//! both in `Result`"* — maps directly to the
//! [`ac-small-signal#ac-analysis-without-prior-operating-point`][spec]
//! scenario:
//!
//! > Given CircuitDesigner has constructed a Circuit
//! > And no OperatingPoint has been computed for this Circuit
//! > When CircuitDesigner submits an AC small-signal Analysis request
//! > Then the Simulator first computes a DC OperatingPoint
//! > And the Simulator proceeds with AC linearization at that OperatingPoint
//! > And the Result contains both the OperatingPoint and the AC
//! >   frequency-domain data
//!
//! The Gherkin's three Then clauses translate one-to-one to the
//! function's behavior:
//!
//! 1. *"The Simulator first computes a DC `OperatingPoint`"* —
//!    internally invokes [`dc_analysis`] with the same `(graph,
//!    structure)` pair the caller supplied.
//! 2. *"The Simulator proceeds with AC linearization at that
//!    `OperatingPoint`"* — re-assembles the `MnaSystem` (which is the
//!    operating-point linearization on the v1 linear-only DC path) and
//!    passes it to [`ac_analysis`].
//! 3. *"The Result contains both the `OperatingPoint` and the AC
//!    frequency-domain data"* — returns
//!    [`AcWithAutoDcResult`] carrying both the
//!    [`OperatingPoint`] (with its `ConvergenceStatus`) and the
//!    [`AcAnalysisResult`] (with one [`TransferFunction`][crate::ac::TransferFunction] per output
//!    node).
//!
//! # Design references
//!
//! - **ADR-0006 — Dual Convergence Criterion for Newton-Raphson.**
//!   Honored transitively through [`dc_analysis`], which configures the
//!   Newton-Raphson driver per ADR-0006.
//! - **ADR-0008 — Per-Node max(Relative, Absolute) Tolerance
//!   Envelope.** Honored transitively: the conformance-bound used by
//!   golden-reference comparison sits at the test harness layer, not
//!   inside this control loop. Both [`dc_analysis`] and [`ac_analysis`]
//!   produce raw solver output suitable for ADR-0008 envelope checks.
//! - **ADR-0009 — Topology Checker for Floating-Node Detection.**
//!   Honored transitively through [`dc_analysis`], which short-circuits
//!   with [`DcAnalysisError::FloatingNodeFault`] when the Pass-1
//!   topology report flags hard-floating nodes. We propagate that
//!   surface through [`AcWithAutoDcError::DcFailed`].
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** Every
//!   surface in this module is part of the v1 *unstable* public Rust
//!   API.
//!
//! # What this module does *not* do
//!
//! - **Topology floating-node faults still short-circuit.** Honored
//!   transitively through [`dc_analysis`], which short-circuits with
//!   [`DcAnalysisError::FloatingNodeFault`] when the Pass-1 topology
//!   report flags hard-floating nodes. We propagate that surface
//!   through [`AcWithAutoDcError::DcFailed`] — a topology *fault* is
//!   a hard error, not a "Convergence status `failed`" outcome.
//! - **Non-convergence of the DC sub-analysis** (`Stalled`,
//!   `MaxIterationsExceeded`, `Diverged`) is *not* a hard error: it
//!   is the **failed-DC** path of [`AcWithAutoDcResult`], where the
//!   `ac-analysis-on-circuit-with-failed-operating-point` scenario
//!   (tasks.md #27) demands `Ok(AcWithAutoDcResult { ac: None,
//!   dc_convergence: <failed>, … })`. The AC step is short-circuited
//!   on this path; no AC frequency-domain data is produced.
//! - **No operating-point cache.** Per the Gherkin's *"no
//!   `OperatingPoint` has been computed for this Circuit"*, this
//!   function always runs DC. Callers who *do* have a cached
//!   `OperatingPoint` (the
//!   `ac-analysis-with-pre-computed-operating-point` scenario) call
//!   [`ac_analysis`] directly instead.
//! - **No nonlinear-device linearization.** The v1 DC path
//!   ([`dc_analysis`]) is linear-only (the nonlinear branch lives in
//!   tasks.md #18 / #19's homotopy work). Re-assembling the MNA with an
//!   empty linearization slice is therefore exactly the operating-point
//!   linearization on the v1 path. When the homotopy work lands and DC
//!   reports a converged nonlinear iterate, this composition will need
//!   to thread the device-linearization slice from the DC iterate into
//!   the second [`assemble()`] call. That extension is out of scope for
//!   items #26 / #27.
//!
//! # Input contract
//!
//! The caller supplies, in [`AcWithAutoDcRequest`]:
//!
//! - the immutable [`CircuitGraph`] (for element parameter lookups,
//!   used by both DC and AC),
//! - the [`FlattenedStructure`] produced by Pass 1
//!   ([`netlist_graph::flatten()`]),
//! - the frequency vector (Hz) to sweep,
//! - the list of node IDs whose voltages should be reported as
//!   [`TransferFunction`][crate::ac::TransferFunction]s,
//! - optional Newton-Raphson tuning and ground override, passed through
//!   verbatim to [`dc_analysis`] *and* [`ac_analysis`].
//!
//! The frequency-vector and outputs validation is delegated to
//! [`ac_analysis`]; the topology / structure / graph validation is
//! delegated to [`dc_analysis`].
//!
//! [spec]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/ac-small-signal/spec.md
//! [`dc_analysis`]: crate::dc::dc_analysis
//! [`ac_analysis`]: crate::ac::ac_analysis
//! [`assemble()`]: numeric_solver::assemble()
//! [`MnaSystem`]: numeric_solver::MnaSystem

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::{ConvergenceStatus, NodeId};
use netlist_graph::CircuitGraph;
use numeric_solver::{assemble, NewtonRaphsonConfig};

use crate::ac::{ac_analysis, AcAnalysisError, AcAnalysisRequest, AcAnalysisResult};
use crate::dc::{dc_analysis, DcAnalysisError, DcAnalysisRequest, OperatingPoint};

// -----------------------------------------------------------------------------
// Request / Result envelopes
// -----------------------------------------------------------------------------

/// Auto-DC AC analysis input bundle.
///
/// The Gherkin's *"`CircuitDesigner` submits an AC small-signal Analysis
/// request"* maps to a single value of this type. The
/// [`CircuitGraph`] / [`FlattenedStructure`] pair is shared between DC
/// and AC; the frequency vector and output node list are the AC-side
/// parameters; the Newton-Raphson and ground overrides are
/// pass-through to the DC sub-analysis (and the ground override is
/// honored by the AC sub-analysis too).
///
/// All fields are required references; the request is `Copy` so it
/// can be passed by value cheaply.
///
/// Per ADR-0010, this struct's *layout* is unstable for v1; the
/// *semantics* of each field are pinned.
#[derive(Debug, Clone, Copy)]
pub struct AcWithAutoDcRequest<'a> {
    /// The immutable source circuit graph. Threaded through both DC
    /// (for element parameter lookups during MNA stamping) and AC (for
    /// reactive-element parameter lookups during sub-view extraction).
    pub graph: &'a CircuitGraph,
    /// The Pass-1 flattened incidence over `graph`. Same instance
    /// passed to both sub-analyses.
    pub structure: &'a FlattenedStructure,
    /// Frequencies (Hz) at which to evaluate the AC transfer
    /// functions. Must be non-empty and all finite (validation
    /// delegated to [`ac_analysis`]).
    pub frequencies_hz: &'a [f64],
    /// Output node IDs whose voltages should be reported as
    /// [`TransferFunction`][crate::ac::TransferFunction]s. Must be non-empty (validation delegated
    /// to [`ac_analysis`]).
    pub outputs: &'a [NodeId],
    /// Newton-Raphson tuning for the DC sub-analysis. `None` defaults
    /// to [`NewtonRaphsonConfig::DC_DEFAULTS`].
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node for both sub-analyses. `None` defaults
    /// to the structure's own ground node (always [`NodeId::GROUND`]
    /// in v1).
    pub ground: Option<NodeId>,
}

impl<'a> AcWithAutoDcRequest<'a> {
    /// Build a request with SPICE-default Newton-Raphson tuning and
    /// the structure's own ground node.
    #[must_use]
    pub fn new(
        graph: &'a CircuitGraph,
        structure: &'a FlattenedStructure,
        frequencies_hz: &'a [f64],
        outputs: &'a [NodeId],
    ) -> Self {
        Self {
            graph,
            structure,
            frequencies_hz,
            outputs,
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

/// The bundled result of an auto-DC AC analysis.
///
/// On the **converged** path (`dc_convergence.is_converged()` is true),
/// both halves are present, matching the
/// `ac-analysis-without-prior-operating-point` scenario's *"The Result
/// contains both the `OperatingPoint` and the AC frequency-domain
/// data"* clause:
///
/// - [`operating_point`](Self::operating_point) — `Some(op)` with the
///   converged DC steady-state solution returned by the embedded
///   [`dc_analysis`] call;
/// - [`ac`](Self::ac) — `Some(ac)` with one
///   [`TransferFunction`][crate::ac::TransferFunction] per output node.
///
/// On the **failed-DC** path (`dc_convergence.is_failure()` is true,
/// covering the `Stalled`, `MaxIterationsExceeded`, and `Diverged`
/// variants — collectively "Convergence status `failed`" in spec
/// language), the AC sub-analysis is short-circuited and the result
/// shape matches the
/// `ac-analysis-on-circuit-with-failed-operating-point` scenario's
/// three *Then* clauses:
///
/// - [`dc_convergence`](Self::dc_convergence) carries a non-`Converged`
///   status (with its embedded
///   [`ConvergenceDiagnostic`][circuit_solver_types::ConvergenceDiagnostic]
///   — the "DC failure diagnostic" the scenario requires);
/// - [`operating_point`](Self::operating_point) is `Some(last_iterate)`
///   when [`dc_analysis`] produced a non-empty iterate, `None`
///   otherwise — last-iterate node voltages are preserved for
///   debugging but are **not** an `OperatingPoint` in the
///   ubiquitous-language sense (a converged steady-state solution);
/// - [`ac`](Self::ac) is `None` — *"no AC frequency-domain data is
///   produced"* from the scenario's third *Then* clause maps directly
///   to this field's absence.
///
/// In both paths:
///
/// - [`dc_topology_warnings`](Self::dc_topology_warnings) — possibly-floating
///   nodes flagged by the Pass-1 topology checker at ADR-0009 warning
///   level (empty when no topology report was attached or when the
///   report was clean).
///
/// Hard errors at the composition seam (DC assembly faults, topology
/// floating-node *faults*, NR driver hard failures, AC validation
/// failures after a successful DC) still surface through
/// [`AcWithAutoDcError`] — those are not convergence-status outcomes
/// in the spec sense and a caller that confuses them with a
/// "Convergence status `failed`" result would mis-categorize a real
/// bug.
#[derive(Debug, Clone, PartialEq)]
pub struct AcWithAutoDcResult {
    /// The DC steady-state solution that the AC step linearized at on
    /// the converged path. `Some(op)` when `dc_convergence` is
    /// `Converged` / `ConvergedViaHomotopy`; `None` on the failed-DC
    /// path (per tasks.md #22 the convergence-failure envelope
    /// flipped this from `Some(last_iterate)` to `None`; callers that
    /// need diagnostic node voltages should consult
    /// [`Self::dc_last_iterate_voltages`] instead).
    pub operating_point: Option<OperatingPoint>,
    /// Last-iterate node voltages from the DC sub-analysis. Populated
    /// regardless of `dc_convergence` variant so the failed-DC path
    /// still surfaces a diagnostic voltage vector even when
    /// `operating_point` is `None`. Per tasks.md #22 this is the
    /// canonical diagnostic surface for non-converged DC; on the
    /// converged path it equals `operating_point.as_ref().unwrap()
    /// .node_voltages`.
    pub dc_last_iterate_voltages: Vec<f64>,
    /// The Newton-Raphson convergence outcome of the DC sub-analysis.
    /// `Converged` on the happy path; one of `Stalled`,
    /// `MaxIterationsExceeded`, or `Diverged` (collectively
    /// "Convergence status `failed`" in spec language) on the
    /// failed-DC path. The embedded
    /// [`ConvergenceDiagnostic`][circuit_solver_types::ConvergenceDiagnostic]
    /// — iteration count, final update / residue norms, effective
    /// tolerances — is the "DC failure diagnostic" the scenario's
    /// second *Then* clause requires.
    pub dc_convergence: ConvergenceStatus,
    /// Possibly-floating nodes (ADR-0009 warning level) carried through
    /// from the DC sub-analysis. Empty when no report was attached.
    pub dc_topology_warnings: Vec<NodeId>,
    /// The AC sweep result. `Some(ac)` with one
    /// [`TransferFunction`][crate::ac::TransferFunction] per output
    /// node (in the order the caller listed them in
    /// [`AcWithAutoDcRequest::outputs`]) on the converged path;
    /// `None` on the failed-DC path where the AC step is
    /// short-circuited per the spec scenario.
    pub ac: Option<AcAnalysisResult>,
}

impl AcWithAutoDcResult {
    /// True iff the DC sub-analysis produced a converged operating
    /// point and the AC sweep ran. Equivalent to
    /// `self.dc_convergence.is_converged()`.
    ///
    /// This is the canonical predicate for "the Result has both
    /// halves" per the `ac-analysis-without-prior-operating-point`
    /// scenario. Callers branching on success vs. failure should
    /// dispatch on this rather than on `self.ac.is_some()`, even
    /// though the two are equivalent — the convergence status is the
    /// load-bearing observable per the spec.
    #[must_use]
    pub fn is_dc_converged(&self) -> bool {
        self.dc_convergence.is_converged()
    }

    /// True iff the DC sub-analysis did *not* converge and the AC
    /// step was therefore short-circuited. Equivalent to
    /// `self.dc_convergence.is_failure()` and to `self.ac.is_none()`.
    ///
    /// This is the canonical predicate for the *Then* clauses of the
    /// `ac-analysis-on-circuit-with-failed-operating-point` scenario:
    /// when this returns `true`, the caller knows the Result reports
    /// Convergence status `failed`, carries the DC failure diagnostic
    /// in `dc_convergence`, and produced no AC frequency-domain data.
    #[must_use]
    pub fn is_dc_failed(&self) -> bool {
        self.dc_convergence.is_failure()
    }

    /// Convenience: look up the
    /// [`TransferFunction`][crate::ac::TransferFunction] for a given
    /// output node, or `None` if the node was not in the original
    /// request *or* the DC sub-analysis failed and no AC data was
    /// produced.
    #[must_use]
    pub fn transfer_for(&self, output: NodeId) -> Option<&crate::ac::TransferFunction> {
        self.ac.as_ref()?.transfer_for(output)
    }
}

// -----------------------------------------------------------------------------
// Error surface
// -----------------------------------------------------------------------------

/// Errors raised by [`ac_analysis_with_auto_dc`].
///
/// The variants split cleanly along the DC / AC composition seam,
/// covering only the **hard** failure modes — non-convergence of the
/// DC sub-analysis is *not* an error here, it is the failed-DC path
/// of [`AcWithAutoDcResult`] per the
/// `ac-analysis-on-circuit-with-failed-operating-point` scenario.
///
/// - [`DcFailed`](Self::DcFailed) — the DC sub-analysis returned an
///   `Err(DcAnalysisError)`. Common causes: MNA assembly rejected the
///   inputs, the topology checker flagged hard-floating nodes, the
///   Newton-Raphson driver hit a hard failure (dim mismatch,
///   unrecoverable linear-solver error). **Non-convergence**
///   (`Stalled`, `MaxIterationsExceeded`, `Diverged`) is *not*
///   reported here — it surfaces as `Ok(AcWithAutoDcResult)` with
///   `is_dc_failed()` true.
/// - [`AcFailed`](Self::AcFailed) — the AC sub-analysis returned an
///   `Err(AcAnalysisError)` after a successfully-converged DC step.
///   Common causes: empty sweep, no outputs, non-finite frequency,
///   output node out of range, complex-LU singularity at a sweep
///   point.
///
/// The `AcFailed` variant boxes its [`OperatingPoint`] and
/// AC-error payload so the size of `Err` stays well below the clippy
/// `result_large_err` threshold; the boxed allocation only happens
/// on the failure path.
#[derive(Debug, Clone, PartialEq)]
pub enum AcWithAutoDcError {
    /// The DC sub-analysis returned a hard error before the AC step
    /// could run.
    DcFailed(DcAnalysisError),
    /// The AC sub-analysis returned an error after a converged DC
    /// step. The DC operating point is preserved (boxed) alongside
    /// so the caller has full diagnostic context.
    AcFailed {
        /// The wrapped AC error pinpoints the cause. Boxed to keep
        /// the discriminant small.
        inner: Box<AcAnalysisError>,
        /// The converged operating point that the AC step linearized
        /// at before failing. Boxed for the same reason as `inner`.
        operating_point: Box<OperatingPoint>,
        /// The DC convergence status (always `Converged`).
        dc_convergence: ConvergenceStatus,
        /// Possibly-floating warnings carried through from DC.
        dc_topology_warnings: Vec<NodeId>,
    },
}

impl core::fmt::Display for AcWithAutoDcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DcFailed(inner) => write!(
                f,
                "ac-with-auto-dc: DC sub-analysis failed before AC could run: {inner}"
            ),
            Self::AcFailed { inner, .. } => write!(
                f,
                "ac-with-auto-dc: DC converged but AC sub-analysis failed: {inner}"
            ),
        }
    }
}

impl std::error::Error for AcWithAutoDcError {}

impl From<DcAnalysisError> for AcWithAutoDcError {
    fn from(value: DcAnalysisError) -> Self {
        Self::DcFailed(value)
    }
}

// -----------------------------------------------------------------------------
// Entry point
// -----------------------------------------------------------------------------

/// Run an AC small-signal analysis preceded by an automatic DC
/// operating-point computation.
///
/// Steps, in order:
///
/// 1. **DC sub-analysis.** Build a [`DcAnalysisRequest`] over the
///    caller's `(graph, structure)` with the requested NR tuning /
///    ground override, dispatch [`dc_analysis`], and:
///    - on hard error, short-circuit with
///      [`AcWithAutoDcError::DcFailed`];
///    - on `Ok(DcAnalysisResult)` with a non-`Converged` status
///      (`Stalled`, `MaxIterationsExceeded`, `Diverged` —
///      collectively "Convergence status `failed`" in spec
///      language), short-circuit with `Ok(AcWithAutoDcResult)`
///      carrying `ac: None`, the failed `ConvergenceStatus`, any
///      last-iterate operating point, and topology warnings (the
///      `ac-analysis-on-circuit-with-failed-operating-point`
///      scenario's three *Then* clauses).
///
///    On the converged path, retain the [`OperatingPoint`], the
///    [`ConvergenceStatus`], and the topology warnings for the final
///    [`AcWithAutoDcResult`].
/// 2. **MNA re-assembly.** Call [`assemble()`] with the same
///    `(structure, graph, &[])` argument list [`dc_analysis`] used
///    internally. On the v1 linear-only DC path this re-assembly
///    produces the same operating-point linearization the NR loop
///    converged on. (Future nonlinear extensions will thread the
///    device-linearization slice from the DC iterate here; see the
///    module-level "What this module does *not* do" note.)
/// 3. **AC sub-analysis.** Build an [`AcAnalysisRequest`] over the
///    same `(graph, structure)`, the freshly-assembled `MnaSystem`,
///    and the caller's frequency vector / output list, dispatch
///    [`ac_analysis`], and on success bundle its result into
///    [`AcWithAutoDcResult`] with `ac: Some(_)`.
///
/// # Errors
///
/// - [`AcWithAutoDcError::DcFailed`] — [`dc_analysis`] returned an
///   `Err(DcAnalysisError)` (assembly rejected, topology floating
///   *fault*, NR hard failure). **Non-convergence** (`Stalled`,
///   `MaxIterationsExceeded`, `Diverged`) is *not* reported here —
///   it surfaces as `Ok(AcWithAutoDcResult)` with `is_dc_failed()`
///   true.
/// - [`AcWithAutoDcError::AcFailed`] — DC converged successfully but
///   the subsequent [`ac_analysis`] returned an error (empty sweep,
///   no outputs, non-finite frequency, output out of range, complex
///   LU singularity). The converged operating point is preserved in
///   the error variant.
///
/// # Panics
///
/// Does not panic in normal operation; all error conditions are
/// reported through [`AcWithAutoDcError`].
pub fn ac_analysis_with_auto_dc(
    req: AcWithAutoDcRequest<'_>,
) -> Result<AcWithAutoDcResult, AcWithAutoDcError> {
    // --- (1) DC sub-analysis ------------------------------------------------
    let dc_req = DcAnalysisRequest {
        graph: req.graph,
        structure: req.structure,
        newton_raphson: req.newton_raphson,
        ground: req.ground,
        device_models: None,
        enable_gmin_fallback: true,
    };
    let dc_result = dc_analysis(dc_req)?;

    // DC non-convergence is *not* a hard error — it is the failed-DC
    // path of the `ac-analysis-on-circuit-with-failed-operating-point`
    // scenario (tasks.md #27). We short-circuit the AC step and return
    // `Ok(AcWithAutoDcResult)` carrying:
    //
    //   - the non-`Converged` `ConvergenceStatus` (the "Convergence
    //     status `failed`" the scenario's first Then clause demands —
    //     with its embedded diagnostic, the second Then clause's "DC
    //     failure diagnostic"),
    //   - the last-iterate `OperatingPoint` (if any) under
    //     `operating_point`, preserved for debugging — *not* a
    //     converged steady-state solution,
    //   - the topology warnings from the DC pre-pass, and
    //   - `ac: None` — the third Then clause's "no AC frequency-domain
    //     data is produced".
    if !dc_result.convergence.is_converged() {
        return Ok(AcWithAutoDcResult {
            operating_point: dc_result.operating_point,
            dc_last_iterate_voltages: dc_result.last_iterate_voltages,
            dc_convergence: dc_result.convergence,
            dc_topology_warnings: dc_result.topology_warnings,
            ac: None,
        });
    }
    // Guaranteed `Some` by the `dc_analysis` invariant on the
    // converged path; we unwrap defensively so a future regression in
    // that invariant surfaces here loudly rather than as a silent
    // panic downstream.
    let operating_point = dc_result
        .operating_point
        .expect("dc_analysis returns Some(op) on the converged path");

    // --- (2) MNA re-assembly ------------------------------------------------
    // On the v1 linear-only DC path the assembled MNA *is* the
    // operating-point linearization (empty linearization slice). The
    // duplicate `assemble` call is the v1 compromise: dc_analysis
    // owns the linearize+solve loop and does not currently surface
    // its internal `MnaSystem`. When the homotopy work (tasks.md #18)
    // refactors DC to expose its linearized system, this hop can be
    // removed.
    let system = match assemble(req.structure, req.graph, &[]) {
        Ok(sys) => sys,
        Err(err) => return Err(AcWithAutoDcError::DcFailed(err.into())),
    };

    // --- (3) AC sub-analysis ------------------------------------------------
    let ac_req = AcAnalysisRequest {
        system: &system,
        structure: req.structure,
        graph: req.graph,
        frequencies_hz: req.frequencies_hz,
        outputs: req.outputs,
        ground: req.ground,
    };
    let ac = match ac_analysis(ac_req) {
        Ok(ac) => ac,
        Err(inner) => {
            return Err(AcWithAutoDcError::AcFailed {
                inner: Box::new(inner),
                operating_point: Box::new(operating_point),
                dc_convergence: dc_result.convergence,
                dc_topology_warnings: dc_result.topology_warnings,
            });
        }
    };

    Ok(AcWithAutoDcResult {
        dc_last_iterate_voltages: operating_point.node_voltages.clone(),
        operating_point: Some(operating_point),
        dc_convergence: dc_result.convergence,
        dc_topology_warnings: dc_result.topology_warnings,
        ac: Some(ac),
    })
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::similar_names, clippy::float_cmp)]
mod tests {
    use super::*;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use netlist_graph::flatten;

    // -------- builders ----------------------------------------------------

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

    fn add_capacitor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, farads: f64) {
        b.add_element(
            name,
            ElementKind::Capacitor {
                capacitance_farads: farads,
            },
            [n1, n2],
            None,
        )
        .expect("add capacitor");
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

    /// Build a first-order RC low-pass: V1 → R → `n_out` → C → gnd.
    /// Node layout: 0 = gnd, 1 = `n_in`, 2 = `n_out`.
    fn build_rc_lowpass() -> (
        netlist_graph::CircuitGraph,
        circuit_solver_types::FlattenedStructure,
    ) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 1_000.0);
        add_capacitor(&mut b, "C1", "n_out", "0", 1.0e-6);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        (g, fs)
    }

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    // -------- core API contracts ------------------------------------------

    /// Happy path: linear RC low-pass, no prior `OperatingPoint`.
    /// DC must converge, AC must produce a `TransferFunction` per
    /// output, and the result must carry *both* halves.
    #[test]
    fn auto_dc_then_ac_happy_path_carries_both_halves() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [1.0_f64, 10.0, 100.0, 1_000.0, 10_000.0];
        let outputs = [NodeId::new(2)]; // n_out

        let result =
            ac_analysis_with_auto_dc(AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs))
                .expect("auto-DC AC must succeed on a linear RC fixture");

        // -- DC half: must carry an OperatingPoint and a Converged status.
        assert!(
            result.dc_convergence.is_converged(),
            "DC sub-analysis must report Converged; got {:?}",
            result.dc_convergence
        );
        assert!(
            result.is_dc_converged(),
            "is_dc_converged() must mirror dc_convergence.is_converged() on the happy path"
        );
        assert!(
            !result.is_dc_failed(),
            "is_dc_failed() must be false on the happy path"
        );
        // For this fixture the DC operating point is trivial: V_in = 1 V
        // (the source), V_out = 1 V (no DC current through C → no drop
        // across R at DC).
        let op = result
            .operating_point
            .as_ref()
            .expect("operating_point must be Some on the converged path");
        let v_in = op
            .voltage_at(NodeId::new(1))
            .expect("V(n_in) must be present");
        let v_out = op
            .voltage_at(NodeId::new(2))
            .expect("V(n_out) must be present");
        assert!(
            approx(v_in, 1.0, 1e-9),
            "V(n_in) should be 1.0 V (source value); got {v_in}"
        );
        assert!(
            approx(v_out, 1.0, 1e-9),
            "V(n_out) should be 1.0 V (no DC current through C); got {v_out}"
        );

        // -- AC half: must carry one TransferFunction per output node,
        //    with parallel length vectors.
        let ac = result
            .ac
            .as_ref()
            .expect("ac must be Some on the converged path");
        assert_eq!(
            ac.transfer_functions.len(),
            1,
            "expected one TransferFunction per output node"
        );
        let tf = &ac.transfer_functions[0];
        assert_eq!(tf.output, NodeId::new(2));
        assert_eq!(tf.frequencies_hz.len(), frequencies_hz.len());
        assert_eq!(tf.magnitude_db.len(), frequencies_hz.len());
        assert_eq!(tf.phase_degrees.len(), frequencies_hz.len());

        // At f = 1 Hz the RC low-pass is deep in the passband
        // (f_c ≈ 159 Hz); |H| ≈ 1, magnitude ≈ 0 dB.
        assert!(
            tf.magnitude_db[0].abs() < 0.01,
            "1 Hz magnitude should be ≈0 dB (passband); got {} dB",
            tf.magnitude_db[0]
        );
    }

    /// The `transfer_for` convenience must thread through to the
    /// embedded `AcAnalysisResult`.
    #[test]
    fn auto_dc_result_transfer_for_threads_through_to_ac() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [100.0_f64];
        let outputs = [NodeId::new(2)];

        let result =
            ac_analysis_with_auto_dc(AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs))
                .expect("happy path");

        let tf = result
            .transfer_for(NodeId::new(2))
            .expect("transfer_for must find n_out");
        assert_eq!(tf.output, NodeId::new(2));
        assert!(result.transfer_for(NodeId::new(99)).is_none());
    }

    /// Builder ergonomics: `with_ground` and `with_newton_raphson`
    /// must flow through to both sub-analyses without panic.
    #[test]
    fn auto_dc_request_builders_compose() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [100.0_f64];
        let outputs = [NodeId::new(2)];

        let req = AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs)
            .with_newton_raphson(NewtonRaphsonConfig::DC_DEFAULTS)
            .with_ground(NodeId::GROUND);

        let result = ac_analysis_with_auto_dc(req).expect("happy path with builder overrides");
        assert!(result.dc_convergence.is_converged());
        let ac = result
            .ac
            .as_ref()
            .expect("ac must be Some on the converged path");
        assert_eq!(ac.transfer_functions.len(), 1);
    }

    // -------- error surfaces ----------------------------------------------

    /// AC validation failures (empty sweep) surface after DC has
    /// already converged. The `AcFailed` variant must preserve the
    /// converged operating point so the caller has diagnostic
    /// context.
    #[test]
    fn ac_failure_after_dc_converged_preserves_operating_point() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz: [f64; 0] = []; // forces AcAnalysisError::EmptySweep
        let outputs = [NodeId::new(2)];

        let err =
            ac_analysis_with_auto_dc(AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs))
                .expect_err("empty sweep must surface as AcFailed (not silently succeed)");

        match err {
            AcWithAutoDcError::AcFailed {
                inner,
                operating_point,
                dc_convergence,
                ..
            } => {
                assert!(matches!(*inner, AcAnalysisError::EmptySweep));
                assert!(dc_convergence.is_converged());
                // Operating point still present and usable.
                let v_out = operating_point
                    .voltage_at(NodeId::new(2))
                    .expect("V(n_out) still present after AC failure");
                assert!(approx(v_out, 1.0, 1e-9));
            }
            other @ AcWithAutoDcError::DcFailed(_) => {
                panic!("expected AcFailed, got {other:?}")
            }
        }
    }

    /// Output node out of range surfaces as `AcFailed` (DC converged,
    /// AC rejected) — *not* `DcFailed`. This pins the separation of
    /// concerns between the two sub-analyses.
    #[test]
    fn output_out_of_range_routes_to_ac_failed_not_dc_failed() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [100.0_f64];
        // Node count is 3 (gnd + n_in + n_out); 99 is well past it.
        let outputs = [NodeId::new(99)];

        let err =
            ac_analysis_with_auto_dc(AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs))
                .expect_err("out-of-range output must surface as AcFailed");

        assert!(matches!(
            err,
            AcWithAutoDcError::AcFailed {
                inner: ref b,
                ..
            } if matches!(**b, AcAnalysisError::OutputNodeOutOfRange { .. })
        ));
    }

    /// Display impls render reasonable error messages for each
    /// variant. We do not pin the exact format string (it's
    /// observability surface, not API), but the variant-distinguishing
    /// prefix must be present.
    #[test]
    fn error_display_contains_variant_marker() {
        let dc_failed = AcWithAutoDcError::DcFailed(DcAnalysisError::FloatingNodeFault {
            nodes: vec![NodeId::new(1)],
        });
        let s = format!("{dc_failed}");
        assert!(s.contains("DC sub-analysis failed"), "got: {s}");
    }

    // -------- DC non-convergence → Ok partial result (tasks.md #27) -------

    /// Build an RC fixture whose DC sub-analysis is *forced* to
    /// terminate without converging.
    ///
    /// Strategy: cap `max_iterations` at 1 and set both tolerances to
    /// 0.0. With exact-zero tolerances no finite NR iterate can ever
    /// satisfy the dual criterion (`norm < 0.0` is false for every
    /// finite norm including exact zero), so a single NR step exhausts
    /// the budget and the driver returns
    /// `ConvergenceStatus::MaxIterationsExceeded` — one of the three
    /// non-`Converged` variants that map to the spec's "Convergence
    /// status `failed`".
    ///
    /// This is the **deterministic failure fixture** for the
    /// `ac-analysis-on-circuit-with-failed-operating-point` scenario:
    /// it does not depend on circuit topology pathologies (which
    /// would couple this test to nonlinear-device work in flight on
    /// tasks.md #18 / #19 / #22), only on NR config — keeping the
    /// witness self-contained against the v1 linear DC path.
    fn forced_dc_failure_config() -> NewtonRaphsonConfig {
        NewtonRaphsonConfig {
            max_iterations: 1,
            tolerances: circuit_solver_types::ConvergenceTolerances::new(0.0, 0.0),
        }
    }

    /// Tasks.md #27 / spec scenario
    /// `ac-analysis-on-circuit-with-failed-operating-point` *Then*
    /// clauses, witnessed at the control-loop level:
    ///
    /// - the function returns `Ok(_)`, not `Err(_)` — the spec says
    ///   *"the Simulator returns a Result with Convergence status
    ///   `failed`"*, which on the v1 unstable Rust API surface maps
    ///   to the `Ok` arm of [`Result`] carrying the failed status;
    /// - `dc_convergence` reports a non-`Converged` variant
    ///   (`is_failure()` true) — the "Convergence status `failed`";
    /// - `dc_convergence.diagnostic()` carries finite NR norms and
    ///   iteration count — the "DC failure diagnostic";
    /// - `ac` is `None` — *"no AC frequency-domain data is produced"*.
    #[test]
    fn dc_non_convergence_yields_ok_with_ac_none_and_failed_status() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [1.0_f64, 100.0, 10_000.0];
        let outputs = [NodeId::new(2)];

        let req = AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs)
            .with_newton_raphson(forced_dc_failure_config());

        let result = ac_analysis_with_auto_dc(req)
            .expect("DC non-convergence must yield Ok(_), not Err(_) — spec returns a Result");

        // [Then-1] Convergence status `failed` (any non-Converged
        // variant — Stalled / MaxIterationsExceeded / Diverged).
        assert!(
            result.dc_convergence.is_failure(),
            "Then-1: dc_convergence must report a non-Converged status; got {:?}",
            result.dc_convergence
        );
        assert!(
            result.is_dc_failed(),
            "Then-1: is_dc_failed() must mirror dc_convergence.is_failure()"
        );
        assert!(
            !result.is_dc_converged(),
            "Then-1: is_dc_converged() must be false on the failed-DC path"
        );

        // [Then-2] The Result contains the DC failure diagnostic.
        let diag = result.dc_convergence.diagnostic();
        assert_eq!(
            diag.iterations, 1,
            "diagnostic must record the iteration count we forced; got {}",
            diag.iterations
        );
        assert!(
            diag.update_norm.is_finite(),
            "diagnostic update_norm must be a finite measurement; got {}",
            diag.update_norm
        );
        assert!(
            diag.residue_norm.is_finite(),
            "diagnostic residue_norm must be a finite measurement; got {}",
            diag.residue_norm
        );

        // [Then-3] No AC frequency-domain data is produced.
        assert!(
            result.ac.is_none(),
            "Then-3: ac must be None on the failed-DC path; got Some"
        );
        // The convenience accessor must agree.
        assert!(
            result.transfer_for(NodeId::new(2)).is_none(),
            "transfer_for must return None when ac is None"
        );
    }

    /// The last-iterate node voltages are preserved on the
    /// failed-DC path so the caller has diagnostic context — but the
    /// `dc_convergence` status, not the presence of `operating_point`,
    /// is the canonical signal of failure. Per tasks.md #22 the
    /// failure-path contract is `operating_point = None` plus a
    /// populated `last_iterate_voltages` diagnostic surface (the test
    /// originally asserted `operating_point = Some(_)`; the #22
    /// envelope flipped that to honor the spec scenario
    /// `dc-operating-point-convergence-failure` *"no `OperatingPoint`
    /// is produced"*).
    #[test]
    fn dc_non_convergence_preserves_last_iterate_when_available() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz = [100.0_f64];
        let outputs = [NodeId::new(2)];

        let req = AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs)
            .with_newton_raphson(forced_dc_failure_config());

        let result = ac_analysis_with_auto_dc(req).expect("DC non-convergence is Ok(_)");

        // Per tasks.md #22: on a failed DC the convergence-failure
        // envelope sets `operating_point = None` and populates
        // `last_iterate_voltages` with the diagnostic node voltages
        // from the final attempted solve. The presence of
        // `last_iterate_voltages` is what gives the caller diagnostic
        // context; we assert that surface here (not the value, which
        // depends on NR's internal initial-iterate choice).
        assert!(
            result.operating_point.is_none(),
            "Then: operating_point must be None on the failed-DC path per #22 contract"
        );
        assert!(
            !result.dc_last_iterate_voltages.is_empty(),
            "Then: last_iterate_voltages must carry the per-node voltage vector for diagnostic use"
        );
    }

    /// AC validation that would have failed on the converged path
    /// (e.g. empty sweep) does *not* surface when DC has already
    /// failed: the AC step is short-circuited, so the result is
    /// `Ok(_)` with `ac: None` rather than
    /// `Err(AcWithAutoDcError::AcFailed { .. })`. This pins the
    /// short-circuit ordering: DC failure first, then any AC
    /// validation is moot.
    #[test]
    fn ac_validation_short_circuited_when_dc_fails() {
        let (g, fs) = build_rc_lowpass();
        let frequencies_hz: [f64; 0] = []; // would force AcAnalysisError::EmptySweep
        let outputs = [NodeId::new(2)];

        let req = AcWithAutoDcRequest::new(&g, &fs, &frequencies_hz, &outputs)
            .with_newton_raphson(forced_dc_failure_config());

        let result = ac_analysis_with_auto_dc(req)
            .expect("DC failure short-circuits before AC validation can run");

        assert!(result.is_dc_failed(), "DC failure path expected");
        assert!(result.ac.is_none(), "ac must be None on the failed-DC path");
    }
}
