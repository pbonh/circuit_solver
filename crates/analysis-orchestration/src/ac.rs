//! AC small-signal analysis control loop.
//!
//! This module covers `tasks.md` item #25 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the per-frequency driver
//! that composes the AC sub-view extractor
//! ([`numeric_solver::AcSubViewBuilder`], tasks.md #24) with the
//! complex-valued sparse-LU backend
//! ([`numeric_solver::FaerComplexSolver`], tasks.md #23) into a single
//! end-to-end AC analysis that produces [`TransferFunction`] results.
//!
//! # Design references
//!
//! - **Scenario `ac-small-signal#ac-analysis-with-pre-computed-operating-point`.**
//!   The spec requires the simulator to *linearize the circuit at the
//!   `OperatingPoint` and solve a complex-valued MNA system at each
//!   frequency in the Sweep*, then return *magnitude and phase for
//!   every output/input pair at every frequency*. This control loop
//!   takes the precomputed operating-point [`MnaSystem`] as input,
//!   delegates the per-frequency `(G + jωC)` augmentation to the AC
//!   sub-view extractor, and computes magnitude (dB) and phase
//!   (degrees) for each requested output node.
//!
//! - **Scenario `ac-small-signal#ac-analysis-on-purely-linear-circuit`.**
//!   When the circuit contains only linear elements (R, L, C,
//!   independent sources), the operating-point assembly *is* the
//!   linearization — no semiconductor stamps, no Newton-Raphson. The
//!   control loop is identical: build sub-view, solve, read.
//!
//! - **ADR-0002 — Sparse Direct LU Dispatch.** Per ADR-0002 the
//!   complex-valued LU dispatch goes through the `faer` backend
//!   ([`FaerComplexSolver`]). This control loop holds a single
//!   solver instance and reuses it across the entire sweep.
//!
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** This module is the analysis-side consumer of the
//!   per-analysis sub-view pattern: the underlying operating-point
//!   [`MnaSystem`] is built once (Pass 2), and the loop reuses it
//!   across all frequency points by handing the same borrow to a
//!   fresh [`AcSubViewBuilder`] per ω.
//!
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** All
//!   surfaces exposed here are unstable per ADR-0010.
//!
//! # What this module does *not* do
//!
//! - **No auto-DC.** When no [`MnaSystem`] is available, dispatching a
//!   DC operating point first is the responsibility of `tasks.md` item
//!   #26 (`ac-small-signal#ac-analysis-without-prior-operating-point`).
//!   This module assumes its caller has already produced a converged
//!   [`MnaSystem`].
//! - **No DC failure short-circuit.** Item #27
//!   (`ac-small-signal#ac-analysis-on-circuit-with-failed-operating-point`).
//! - **No logarithmic sweep generator.** The caller supplies a
//!   pre-built frequency vector; the log-sweep generator lands in
//!   item #28 (`ac-small-signal#ac-frequency-sweep-over-multiple-decades`).
//! - **No re-linearization across frequencies.** AC small-signal at
//!   one operating point uses the *same* linearization the DC solver
//!   converged on. This is the textbook small-signal convention and
//!   matches what the AC sub-view extractor assumes.
//!
//! # Input contract
//!
//! The caller supplies, in [`AcAnalysisRequest`]:
//!
//! - the operating-point [`MnaSystem`] (real-valued, full-rank after
//!   ground suppression in the sub-view extractor),
//! - the [`FlattenedStructure`] used to assemble it (so the sub-view
//!   extractor can walk reactive elements),
//! - the source [`CircuitGraph`] (so the sub-view extractor can read
//!   `capacitance_farads` and `inductance_henries` parameters),
//! - the frequency vector (Hz) to sweep,
//! - the list of node IDs whose voltages should be reported as
//!   [`TransferFunction`]s.
//!
//! The AC stimulus is whatever the operating-point assembler already
//! stamped into the RHS — per the AC sub-view module's documentation,
//! "the AC small-signal stimulus is carried by the same DC-value
//! parameters in v1 (there is no separate AC-magnitude annotation on
//! `ElementKind::VoltageSource` / `ElementKind::CurrentSource` yet)".
//! The conventional pattern, exercised by the unit tests below, is to
//! place a `1 V` independent voltage source at the input so that the
//! solved node voltage is directly the transfer function
//! `H(jω) = V_out / V_in = V_out / 1 = V_out`.
//!
//! [`MnaSystem`]: numeric_solver::MnaSystem
//! [`AcSubViewBuilder`]: numeric_solver::AcSubViewBuilder
//! [`FaerComplexSolver`]: numeric_solver::FaerComplexSolver

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::NodeId;
use netlist_graph::CircuitGraph;
use numeric_solver::{
    AcSubViewBuilder, AcSubViewError, FaerComplexSolver, LinearSolver, LinearSolverError, MnaSystem,
};

/// AC analysis input bundle.
///
/// All fields are required; the control loop borrows them for the
/// duration of [`ac_analysis`].
///
/// `ground` defaults to [`NodeId::GROUND`] when `None`. The default
/// matches the convention used by both the operating-point assembler
/// and the AC sub-view extractor; callers should override only when
/// running against a synthetic test fixture with a non-zero ground.
#[derive(Debug, Clone, Copy)]
pub struct AcAnalysisRequest<'a> {
    /// The DC operating-point MNA system, with conductance and
    /// linearized-device stamps already in place.
    pub system: &'a MnaSystem,
    /// The flattened incidence used to assemble `system`.
    pub structure: &'a FlattenedStructure,
    /// The source circuit graph (for reactive-element parameter
    /// lookups).
    pub graph: &'a CircuitGraph,
    /// Frequencies (Hz) at which to evaluate the transfer function.
    /// Must be non-empty and all finite.
    pub frequencies_hz: &'a [f64],
    /// Output node IDs whose voltages should be reported. Each yields
    /// one [`TransferFunction`] in the [`AcAnalysisResult`].
    pub outputs: &'a [NodeId],
    /// Override the ground node (defaults to [`NodeId::GROUND`]).
    pub ground: Option<NodeId>,
}

/// The complex ratio of output to input in an AC small-signal analysis
/// at one circuit output, sampled at each frequency point.
///
/// Per the inlined glossary, a `TransferFunction` is "the complex
/// ratio of output to input in AC analysis." Magnitude is reported in
/// decibels (`20·log10|H|`) and phase in degrees (`arg(H)·180/π`) per
/// the spec's acceptance criterion (*"The Result contains
/// `TransferFunction` data (magnitude in dB and phase in degrees) for
/// every requested output/input pair at every frequency point."*).
///
/// The three vectors are parallel: `frequencies_hz[i]`,
/// `magnitude_db[i]`, and `phase_degrees[i]` describe the same
/// frequency point. Invariant: all three vectors have the same length.
#[derive(Debug, Clone, PartialEq)]
pub struct TransferFunction {
    /// The output node whose voltage this transfer function describes.
    pub output: NodeId,
    /// Frequency axis (Hz), monotonic in the order the caller
    /// requested. Copied from [`AcAnalysisRequest::frequencies_hz`].
    pub frequencies_hz: Vec<f64>,
    /// Magnitude `20·log10|H(jω)|` at each frequency, in decibels.
    pub magnitude_db: Vec<f64>,
    /// Phase `arg(H(jω))·180/π` at each frequency, in degrees, in the
    /// principal-value range `(-180, 180]` returned by
    /// [`num_complex::Complex::arg`].
    pub phase_degrees: Vec<f64>,
}

impl TransferFunction {
    /// Number of frequency points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.frequencies_hz.len()
    }

    /// True iff this transfer function has no samples (a degenerate
    /// construction; [`ac_analysis`] never produces this shape
    /// because it rejects empty sweeps up front).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frequencies_hz.is_empty()
    }
}

/// The bundled result of an AC analysis: one [`TransferFunction`] per
/// requested output node, in the order [`AcAnalysisRequest::outputs`]
/// listed them.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AcAnalysisResult {
    /// One transfer function per output node, parallel to
    /// [`AcAnalysisRequest::outputs`].
    pub transfer_functions: Vec<TransferFunction>,
}

impl AcAnalysisResult {
    /// Look up the transfer function for a given output node.
    /// Returns `None` if the node was not in the original request.
    #[must_use]
    pub fn transfer_for(&self, output: NodeId) -> Option<&TransferFunction> {
        self.transfer_functions
            .iter()
            .find(|tf| tf.output == output)
    }
}

/// Errors raised by [`ac_analysis`].
///
/// Variants for each pre-flight validation surface so the caller can
/// distinguish caller-bug failures (`EmptySweep`, `NoOutputs`,
/// `NonFiniteFrequency`, `OutputNodeOutOfRange`) from downstream
/// numerical failures (`SubViewBuildFailed`, `SolverFailed`). The
/// downstream variants carry the offending frequency so the caller can
/// diagnose which sweep point caused the failure.
#[derive(Debug, Clone, PartialEq)]
pub enum AcAnalysisError {
    /// The supplied frequency vector was empty. The spec's "frequency
    /// Sweep" implies at least one point.
    EmptySweep,
    /// The supplied outputs slice was empty. With no outputs there is
    /// nothing to compute; we surface this as a caller-bug rather than
    /// silently returning an empty result so misuse is loud.
    NoOutputs,
    /// One of the supplied frequencies was non-finite (NaN or ±∞).
    /// Stamping `jω · C` would poison the matrix.
    NonFiniteFrequency {
        /// The offending value (Hz).
        frequency_hz: f64,
    },
    /// An output `NodeId` exceeded the operating-point system's
    /// `node_count`. Indicates the caller paired a system with the
    /// wrong output list.
    OutputNodeOutOfRange {
        /// The offending node id.
        node: NodeId,
        /// The system's node count (including ground).
        node_count: u32,
    },
    /// The AC sub-view builder rejected the inputs at one frequency
    /// point. The wrapped [`AcSubViewError`] pinpoints the cause.
    SubViewBuildFailed {
        /// The frequency at which the failure occurred.
        frequency_hz: f64,
        /// The wrapped sub-view error.
        inner: AcSubViewError,
    },
    /// The complex-valued LU dispatch failed at one frequency point.
    /// Most commonly: the matrix is numerically singular at the
    /// requested ω (which can happen for resonant LC subcircuits at
    /// exactly the resonance frequency).
    SolverFailed {
        /// The frequency at which the failure occurred.
        frequency_hz: f64,
        /// The wrapped solver error.
        inner: LinearSolverError,
    },
}

impl core::fmt::Display for AcAnalysisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptySweep => write!(f, "ac-analysis: frequency sweep is empty"),
            Self::NoOutputs => write!(f, "ac-analysis: no output nodes were requested"),
            Self::NonFiniteFrequency { frequency_hz } => {
                write!(f, "ac-analysis: frequency {frequency_hz} Hz is non-finite")
            }
            Self::OutputNodeOutOfRange { node, node_count } => write!(
                f,
                "ac-analysis: output {node} is out of range for node_count={node_count}"
            ),
            Self::SubViewBuildFailed {
                frequency_hz,
                inner,
            } => write!(
                f,
                "ac-analysis: AC sub-view build failed at f={frequency_hz} Hz: {inner}"
            ),
            Self::SolverFailed {
                frequency_hz,
                inner,
            } => write!(
                f,
                "ac-analysis: complex LU solve failed at f={frequency_hz} Hz: {inner}"
            ),
        }
    }
}

impl std::error::Error for AcAnalysisError {}

/// Run the AC small-signal analysis control loop.
///
/// At each frequency `f_k` in `req.frequencies_hz`:
///
/// 1. Build an [`AcSubView`][numeric_solver::AcSubView] at angular
///    frequency `ω = 2πf_k` via [`AcSubViewBuilder::from_operating_point`].
/// 2. Solve the resulting complex sparse system with
///    [`FaerComplexSolver`].
/// 3. For each output node in `req.outputs`, read the corresponding
///    complex unknown out of the solution vector and append its
///    magnitude (dB) and phase (degrees) to the matching
///    [`TransferFunction`] entry.
///
/// The returned [`AcAnalysisResult`] contains one [`TransferFunction`]
/// per output node, in the input order.
///
/// # Errors
///
/// - [`AcAnalysisError::EmptySweep`] — `req.frequencies_hz` was empty.
/// - [`AcAnalysisError::NoOutputs`] — `req.outputs` was empty.
/// - [`AcAnalysisError::NonFiniteFrequency`] — a frequency was NaN /
///   ±∞.
/// - [`AcAnalysisError::OutputNodeOutOfRange`] — an output node index
///   exceeded the system's `node_count`.
/// - [`AcAnalysisError::SubViewBuildFailed`] — the AC sub-view builder
///   rejected inputs at some sweep point. See [`AcSubViewError`] for
///   the precise root cause.
/// - [`AcAnalysisError::SolverFailed`] — the complex-LU dispatch
///   failed at some sweep point. Typically a singular matrix at an
///   undamped resonance frequency.
///
/// # Panics
///
/// Does not panic in normal operation; all error conditions are
/// reported through [`AcAnalysisError`]. (The unchecked array indexing
/// inside the inner loop is guarded by the up-front node-range check.)
pub fn ac_analysis(req: AcAnalysisRequest<'_>) -> Result<AcAnalysisResult, AcAnalysisError> {
    // --- Up-front validation -------------------------------------------------
    if req.frequencies_hz.is_empty() {
        return Err(AcAnalysisError::EmptySweep);
    }
    if req.outputs.is_empty() {
        return Err(AcAnalysisError::NoOutputs);
    }
    for &f_hz in req.frequencies_hz {
        if !f_hz.is_finite() {
            return Err(AcAnalysisError::NonFiniteFrequency { frequency_hz: f_hz });
        }
    }
    let node_count = req.system.node_count();
    for &output in req.outputs {
        if output.index() >= node_count {
            return Err(AcAnalysisError::OutputNodeOutOfRange {
                node: output,
                node_count,
            });
        }
    }

    let n_freq = req.frequencies_hz.len();
    let n_out = req.outputs.len();

    // --- Pre-allocate output structure --------------------------------------
    let mut transfer_functions: Vec<TransferFunction> = req
        .outputs
        .iter()
        .map(|&output| TransferFunction {
            output,
            frequencies_hz: req.frequencies_hz.to_vec(),
            magnitude_db: Vec::with_capacity(n_freq),
            phase_degrees: Vec::with_capacity(n_freq),
        })
        .collect();

    let solver = FaerComplexSolver;

    // --- Per-frequency loop -------------------------------------------------
    for &f_hz in req.frequencies_hz {
        let mut builder =
            AcSubViewBuilder::from_operating_point(req.system, req.structure, req.graph)
                .at_frequency(f_hz);
        if let Some(ground) = req.ground {
            builder = builder.with_ground_node(ground);
        }
        let view = builder
            .build()
            .map_err(|e| AcAnalysisError::SubViewBuildFailed {
                frequency_hz: f_hz,
                inner: e,
            })?;

        let solution = solver
            .solve(view.system())
            .map_err(|e| AcAnalysisError::SolverFailed {
                frequency_hz: f_hz,
                inner: e,
            })?;
        let unknowns = solution.unknowns();

        for (k, &output) in req.outputs.iter().enumerate() {
            // Guarded by the up-front OutputNodeOutOfRange check; the
            // AC sub-view's dim ≥ node_count, so `output.index() <
            // node_count <= unknowns.len()` always holds.
            let v = unknowns[output.index() as usize];
            let mag = v.norm();
            // 20·log10(|H|). For |H| = 0 this is -∞; we let it flow
            // through as f64::NEG_INFINITY so callers can detect a
            // dead output without us inventing a sentinel.
            let mag_db = 20.0 * mag.log10();
            let phase_deg = v.arg().to_degrees();
            transfer_functions[k].magnitude_db.push(mag_db);
            transfer_functions[k].phase_degrees.push(phase_deg);
        }
    }

    // Defense-in-depth: parallel-length invariant.
    debug_assert!(transfer_functions.iter().all(|tf| {
        tf.frequencies_hz.len() == n_freq
            && tf.magnitude_db.len() == n_freq
            && tf.phase_degrees.len() == n_freq
    }));
    debug_assert_eq!(transfer_functions.len(), n_out);

    Ok(AcAnalysisResult { transfer_functions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use numeric_solver::{assemble, flatten};

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

    fn add_inductor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, henries: f64) {
        b.add_element(
            name,
            ElementKind::Inductor {
                inductance_henries: henries,
            },
            [n1, n2],
            None,
        )
        .expect("add inductor");
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

    fn rc_lowpass(
        vsrc: f64,
        r_ohms: f64,
        c_farads: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", vsrc);
        add_resistor(&mut b, "R1", "n_in", "n_out", r_ohms);
        add_capacitor(&mut b, "C1", "n_out", "0", c_farads);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    fn rlc_bandpass(
        vsrc: f64,
        r_ohms: f64,
        l_henries: f64,
        c_farads: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        // Series RLC: V1 → R → L → C → gnd. The voltage at n_mid_lc
        // (between L and C) is V1 · (1/(jωC)) / (R + jωL + 1/(jωC)).
        // For testing we use the simpler observation that the current
        // through the loop peaks at ω0 = 1/√(LC), the resonance.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", vsrc);
        add_resistor(&mut b, "R1", "n_in", "n_a", r_ohms);
        add_inductor(&mut b, "L1", "n_a", "n_b", l_henries);
        add_capacitor(&mut b, "C1", "n_b", "0", c_farads);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol.max(tol * a.abs().max(b.abs()))
    }

    // -------- core API contracts ------------------------------------------

    #[test]
    fn ac_analysis_rejects_empty_sweep() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let err = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[],
            outputs: &[NodeId::new(2)],
            ground: None,
        })
        .unwrap_err();
        assert_eq!(err, AcAnalysisError::EmptySweep);
    }

    #[test]
    fn ac_analysis_rejects_empty_outputs() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let err = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0, 10.0],
            outputs: &[],
            ground: None,
        })
        .unwrap_err();
        assert_eq!(err, AcAnalysisError::NoOutputs);
    }

    #[test]
    fn ac_analysis_rejects_non_finite_frequency() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let err = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0, f64::NAN],
            outputs: &[NodeId::new(2)],
            ground: None,
        })
        .unwrap_err();
        match err {
            AcAnalysisError::NonFiniteFrequency { frequency_hz } => assert!(frequency_hz.is_nan()),
            other => panic!("expected NonFiniteFrequency, got {other:?}"),
        }
    }

    #[test]
    fn ac_analysis_rejects_out_of_range_output() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let bogus = NodeId::new(999);
        let err = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0],
            outputs: &[bogus],
            ground: None,
        })
        .unwrap_err();
        assert_eq!(
            err,
            AcAnalysisError::OutputNodeOutOfRange {
                node: bogus,
                node_count: sys.node_count()
            }
        );
    }

    // -------- scenario witnesses -----------------------------------------

    /// Scenario `ac-small-signal#ac-analysis-with-pre-computed-operating-point`.
    ///
    /// Given an operating point already exists for an RC low-pass.
    /// When `CircuitDesigner` submits an AC analysis with a frequency
    /// sweep that includes the cutoff frequency.
    /// Then the simulator linearizes at the operating point
    /// and the Result contains magnitude (dB) and phase (degrees) for
    /// every frequency.
    ///
    /// At the cutoff frequency ω = 1/(RC) the analytic transfer
    /// function gives |H| = 1/√2 ≈ -3.0103 dB and ∠H = -45°.
    #[test]
    fn ac_analysis_with_pre_computed_operating_point() {
        let r = 1_000.0_f64;
        let c = 1.0e-6_f64;
        let (fs, g, sys) = rc_lowpass(1.0, r, c);

        // Cutoff frequency in Hz: f_c = 1 / (2π·RC).
        let f_cutoff_hz = 1.0 / (2.0 * core::f64::consts::PI * r * c);
        // Sweep spanning three decades around the cutoff.
        let frequencies_hz: Vec<f64> = (0..=12)
            .map(|k| f_cutoff_hz * 10f64.powf((f64::from(k) - 6.0) * 0.5))
            .collect();
        // Node layout: gnd=0, n_in=1, n_out=2.
        let n_out = NodeId::new(2);

        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &frequencies_hz,
            outputs: &[n_out],
            ground: None,
        })
        .expect("ac_analysis ok");

        // Exactly one transfer function for the one output requested.
        assert_eq!(result.transfer_functions.len(), 1);
        let tf = &result.transfer_functions[0];
        assert_eq!(tf.output, n_out);
        // Parallel-length invariant.
        assert_eq!(tf.frequencies_hz.len(), frequencies_hz.len());
        assert_eq!(tf.magnitude_db.len(), frequencies_hz.len());
        assert_eq!(tf.phase_degrees.len(), frequencies_hz.len());

        // Locate the sweep point at the cutoff (index 6 by construction).
        let cutoff_idx = 6usize;
        assert!(approx(tf.frequencies_hz[cutoff_idx], f_cutoff_hz, 1e-12));
        assert!(
            approx(tf.magnitude_db[cutoff_idx], -3.0103, 1e-3),
            "magnitude at cutoff got {} dB, want -3.0103",
            tf.magnitude_db[cutoff_idx]
        );
        assert!(
            approx(tf.phase_degrees[cutoff_idx], -45.0, 1e-6),
            "phase at cutoff got {}°, want -45°",
            tf.phase_degrees[cutoff_idx]
        );

        // Sanity: the magnitude response is monotonically decreasing
        // for a low-pass (each point should not exceed the previous).
        for win in tf.magnitude_db.windows(2) {
            assert!(
                win[1] <= win[0] + 1e-9,
                "low-pass magnitude must be monotonic non-increasing: {win:?}"
            );
        }

        // Sanity: at low frequencies the response approaches 0 dB
        // (passband), at high frequencies it drops well below 0.
        assert!(
            tf.magnitude_db[0] > -0.5,
            "low-freq magnitude too low: {}",
            tf.magnitude_db[0]
        );
        let last = tf.magnitude_db.len() - 1;
        assert!(
            tf.magnitude_db[last] < -20.0,
            "high-freq magnitude not attenuated: {}",
            tf.magnitude_db[last]
        );
    }

    /// Scenario `ac-small-signal#ac-analysis-on-purely-linear-circuit`.
    ///
    /// Given a purely linear circuit (R, L, C, V source) — no
    /// semiconductors, no linearizations needed.
    /// When the AC analysis runs.
    /// Then the Result contains `TransferFunction` data and the
    /// magnitude response is flat/monotonic as expected by the topology.
    ///
    /// We use an RLC series with the output node tapped at the
    /// capacitor: H(jω) = (1/jωC) / (R + jωL + 1/jωC). At resonance
    /// ω0 = 1/√(LC) the magnitude peaks; far below resonance |H| → 1
    /// (capacitor dominates, drops all voltage); far above |H| → 0
    /// (inductor blocks).
    #[test]
    fn ac_analysis_on_purely_linear_circuit() {
        // R = 1 Ω (small, lightly damped), L = 1 mH, C = 1 µF.
        // ω0 = 1/√(LC) = 1/√(1e-3 · 1e-6) = 1/√1e-9 = 31622.78 rad/s
        // f0 = ω0/(2π) ≈ 5_032.92 Hz. Q = (1/R)·√(L/C) = √1000 ≈ 31.6.
        let r = 1.0_f64;
        let l = 1.0e-3_f64;
        let c = 1.0e-6_f64;
        let (fs, g, sys) = rlc_bandpass(1.0, r, l, c);

        let f_resonance_hz = 1.0 / (2.0 * core::f64::consts::PI * (l * c).sqrt());
        let frequencies_hz: Vec<f64> = vec![
            f_resonance_hz * 0.001,
            f_resonance_hz * 0.01,
            f_resonance_hz * 0.1,
            f_resonance_hz,
            f_resonance_hz * 10.0,
            f_resonance_hz * 100.0,
            f_resonance_hz * 1_000.0,
        ];

        // Node layout: gnd=0, n_in=1, n_a=2, n_b=3 (the cap output).
        // Read both n_in and n_b so we can verify the request supports
        // multi-output and they are returned in input order.
        let n_in = NodeId::new(1);
        let n_b = NodeId::new(3);

        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &frequencies_hz,
            outputs: &[n_in, n_b],
            ground: None,
        })
        .expect("ac_analysis ok");

        assert_eq!(result.transfer_functions.len(), 2);
        assert_eq!(result.transfer_functions[0].output, n_in);
        assert_eq!(result.transfer_functions[1].output, n_b);

        let tf_in = &result.transfer_functions[0];
        let tf_b = &result.transfer_functions[1];

        // n_in is the voltage-source plus terminal; H(n_in) = 1
        // for all frequencies, so magnitude is 0 dB and phase ≈ 0°.
        for (i, &f_hz) in frequencies_hz.iter().enumerate() {
            assert!(
                approx(tf_in.magnitude_db[i], 0.0, 1e-9),
                "n_in magnitude at f[{}]={} Hz got {} dB, want 0",
                i,
                f_hz,
                tf_in.magnitude_db[i]
            );
            assert!(
                tf_in.phase_degrees[i].abs() < 1e-6,
                "n_in phase at f[{}] got {}°, want ≈0",
                i,
                tf_in.phase_degrees[i]
            );
        }

        // n_b magnitude monotonic non-increasing once past resonance
        // (i.e. for indices >= 3 in the sweep above).
        for win in tf_b.magnitude_db[3..].windows(2) {
            assert!(
                win[1] <= win[0] + 1e-9,
                "past resonance, n_b should be non-increasing: {win:?}"
            );
        }

        // Far below resonance the cap dominates → |H_n_b| ≈ 1, so dB ≈ 0.
        assert!(
            approx(tf_b.magnitude_db[0], 0.0, 0.01),
            "low-freq n_b mag got {} dB, want ≈0",
            tf_b.magnitude_db[0]
        );

        // Far above resonance the inductor blocks → |H_n_b| ≪ 1.
        let high = tf_b.magnitude_db.len() - 1;
        assert!(
            tf_b.magnitude_db[high] < -50.0,
            "high-freq n_b mag got {} dB, want ≪ -50",
            tf_b.magnitude_db[high]
        );

        // At resonance there is a peak (relative to the low-frequency
        // passband baseline at 0 dB).
        let mag_at_resonance = tf_b.magnitude_db[3];
        assert!(
            mag_at_resonance > 20.0,
            "resonant peak below expectation: {mag_at_resonance} dB \
             (Q≈31.6 → expect ~30 dB peak)"
        );
    }

    // -------- reuse / sweep characteristics -------------------------------

    #[test]
    fn ac_analysis_returns_results_in_input_output_order() {
        // Two outputs, listed in non-default order; verify the result
        // vector preserves that order.
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let n_in = NodeId::new(1);
        let n_out = NodeId::new(2);
        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0, 1_000.0],
            outputs: &[n_out, n_in], // reversed
            ground: None,
        })
        .expect("ac_analysis ok");

        assert_eq!(result.transfer_functions[0].output, n_out);
        assert_eq!(result.transfer_functions[1].output, n_in);
    }

    #[test]
    fn ac_analysis_dc_limit_matches_dc_solution_for_low_pass() {
        // At ω = 0 the capacitor is open; the RC low-pass passes the
        // input straight through, so V_n_out = V_in = 1.0 → 0 dB,
        // 0° phase. Validates that the DC limit of the AC sweep
        // collapses to the DC solution (modulo complex promotion).
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[0.0],
            outputs: &[NodeId::new(2)],
            ground: None,
        })
        .expect("ac_analysis ok");
        let tf = &result.transfer_functions[0];
        assert!(approx(tf.magnitude_db[0], 0.0, 1e-9));
        assert!(tf.phase_degrees[0].abs() < 1e-9);
    }

    #[test]
    fn ac_analysis_transfer_for_returns_matching_tf() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let n_out = NodeId::new(2);
        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0, 1_000.0],
            outputs: &[n_out],
            ground: None,
        })
        .expect("ac_analysis ok");
        let tf = result.transfer_for(n_out).expect("lookup hits");
        assert_eq!(tf.output, n_out);
        assert_eq!(tf.len(), 2);
        assert!(!tf.is_empty());
        assert!(result.transfer_for(NodeId::new(99)).is_none());
    }

    #[test]
    fn transfer_function_parallel_lengths_invariant_holds() {
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let freqs: Vec<f64> = (0..50).map(|k| 10.0 * (1.1f64).powi(k)).collect();
        let result = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &freqs,
            outputs: &[NodeId::new(2)],
            ground: None,
        })
        .expect("ac_analysis ok");
        let tf = &result.transfer_functions[0];
        assert_eq!(tf.frequencies_hz.len(), freqs.len());
        assert_eq!(tf.magnitude_db.len(), freqs.len());
        assert_eq!(tf.phase_degrees.len(), freqs.len());
        // Verify magnitude and phase are finite everywhere on this
        // well-behaved low-pass.
        for (i, &m) in tf.magnitude_db.iter().enumerate() {
            assert!(m.is_finite(), "non-finite mag at index {i}");
        }
        for (i, &p) in tf.phase_degrees.iter().enumerate() {
            assert!(p.is_finite(), "non-finite phase at index {i}");
            // For a low-pass the principal-value phase is in [-90°, 0°].
            assert!(
                (-90.5..=0.5).contains(&p),
                "phase out of expected low-pass range at index {i}: {p}°"
            );
        }
    }

    #[test]
    fn ac_analysis_with_custom_ground_node_round_trips() {
        // Default ground = NodeId::GROUND = NodeId::new(0). Pass it
        // explicitly via `ground` and verify identical output. This
        // exercises the `with_ground_node` override path.
        let (fs, g, sys) = rc_lowpass(1.0, 1_000.0, 1.0e-6);
        let n_out = NodeId::new(2);
        let r1 = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0, 1_000.0],
            outputs: &[n_out],
            ground: None,
        })
        .expect("ok");
        let r2 = ac_analysis(AcAnalysisRequest {
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[100.0, 1_000.0],
            outputs: &[n_out],
            ground: Some(NodeId::GROUND),
        })
        .expect("ok");
        assert_eq!(r1, r2);
    }
}
