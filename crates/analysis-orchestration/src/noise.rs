//! Noise spectral-density analysis control loop.
//!
//! This module covers `tasks.md` item #37 of
//! `circuit-solver-2026-05-21-v1-spec`. It is the per-frequency,
//! per-noise-source driver that composes:
//!
//! - the AC sub-view extractor ([`numeric_solver::AcSubViewBuilder`],
//!   tasks.md #24) — which linearizes the operating-point system at
//!   angular frequency `ω = 2πf`,
//! - the complex-valued sparse-LU backend
//!   ([`numeric_solver::FaerComplexSolver`], tasks.md #23, ADR-0002),
//! - the intrinsic noise source contract from
//!   [`device_modeling`] (tasks.md #36) — specifically
//!   [`device_modeling::noise::resistor_thermal_noise`] for resistor
//!   thermal noise,
//!
//! into a single end-to-end noise spectral-density analysis that
//! produces output-referred V²/Hz curves over a user-supplied
//! frequency sweep.
//!
//! # Scenarios in scope at tasks.md #37
//!
//! - `noise-spectral-density#noise-analysis-on-a-resistive-circuit`:
//!   the **witness scenario**. For a circuit containing only
//!   resistors and independent sources, each resistor `R` emits a
//!   Johnson-Nyquist white-noise current source with PSD
//!   `S_I(f) = 4·k_B·T / R` between its two terminals. The output
//!   PSD at a designated node is the squared-magnitude sum of every
//!   source's transfer function to that node, weighted by the
//!   source's PSD.
//!
//! - `noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point`:
//!   if the DC operating-point computation failed (any
//!   [`ConvergenceStatus`] other than `Converged`), the control loop
//!   returns a [`NoiseAnalysisResult::Failed`] carrying the original
//!   DC convergence diagnostic, with no spectral-density data
//!   produced. This is the short-circuit path; see the [`noise_analysis`]
//!   precondition below.
//!
//! # Scenarios *out* of scope at tasks.md #37
//!
//! - Intrinsic noise from semiconductor devices
//!   (`noise-analysis-with-flicker-and-shot-noise-contributions`,
//!   tasks.md #38 attaches per-device breakdown). The
//!   [`DeviceNoiseStamp`](device_modeling::noise::DeviceNoiseStamp)
//!   contract is already in place from tasks.md #36; this control
//!   loop deliberately handles only resistor thermal noise to keep
//!   the witness scenario tight. The architecture leaves a single
//!   extension point ([`collect_noise_sources`]) where the future
//!   device walk lands.
//! - Integrated noise over bandwidth (tasks.md #39).
//! - Auto-DC (tasks.md #40 — `noise-analysis-without-prior-operating-point`).
//! - ngspice conformance test (tasks.md #66).
//!
//! # Mathematical model
//!
//! Let `H_j(jω)` be the complex transfer function from the noise
//! current injected by source `j` (between two graph nodes) to the
//! voltage at the user-specified output node, evaluated at
//! `ω = 2πf`. Let `S_j(f)` be source `j`'s PSD at frequency `f`,
//! evaluated via [`NoiseSource::psd_at`].
//!
//! Because all intrinsic noise sources are *uncorrelated*
//! (see the discussion in [`device_modeling::noise`] and the spec's
//! "each intrinsic device noise source … contributes independently"
//! acceptance criterion), the output noise *power* spectral density
//! is the linear sum
//!
//! ```text
//! S_out(f) = Σ_j  |H_j(jω)|² · S_j(f)     [V² / Hz]
//! ```
//!
//! `H_j(jω)` is computed by solving the AC-linearized MNA system
//! with a unit current source injected at source `j`'s terminal pair
//! and reading the complex unknown at the output node. The AC
//! sub-view extractor handles the `(G + jωC)` augmentation around
//! the operating point.
//!
//! For a pure resistive circuit at one output node, the AC matrix
//! is exactly the DC conductance matrix at every frequency — there
//! are no reactive elements to stamp — so every `|H_j|` is
//! frequency-independent and the output PSD is white.
//!
//! # Design references
//!
//! - **ADR-0002 — Sparse Direct LU Dispatch.** The complex LU goes
//!   through the `faer` backend ([`FaerComplexSolver`]) just like
//!   the AC control loop.
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** The operating-point system is built once (Pass 2);
//!   the noise loop reuses the same [`FlattenedStructure`] across all
//!   `(frequency, noise-source)` pairs.
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** All
//!   surfaces here are unstable per ADR-0010.
//!
//! [`MnaSystem`]: numeric_solver::MnaSystem
//! [`AcSubViewBuilder`]: numeric_solver::AcSubViewBuilder
//! [`FaerComplexSolver`]: numeric_solver::FaerComplexSolver
//! [`NoiseSource`]: device_modeling::noise::NoiseSource

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::{ConvergenceStatus, NodeId};
use device_modeling::noise::{resistor_thermal_noise, NoiseSource};
use netlist_graph::{CircuitGraph, ElementKind};
use numeric_solver::{
    AcSubViewBuilder, AcSubViewError, FaerComplexSolver, LinearSolver, LinearSolverError,
    MnaSystem, SparseLinearSystem, SparseTriplet, C64,
};

// ---------------------------------------------------------------------
// Noise injection — one noise source mapped onto graph nodes
// ---------------------------------------------------------------------

/// One stochastic noise current source, mapped onto the graph's
/// `NodeId` space.
///
/// Distinct from [`device_modeling::noise::NoiseSource`] in two ways:
/// - terminals are graph-level `NodeId`s rather than terminal-local
///   indices `(a, b)`, so the noise control loop can stamp directly
///   into the MNA RHS;
/// - the source carries enough metadata to be diagnosed in tests
///   without going back to the originating element.
///
/// The source injects a stochastic current `i_n(t)` flowing *from*
/// [`Self::node_pos`] *into* [`Self::node_neg`]; PSD is sign-invariant
/// (squared magnitude) so the direction matters only for the
/// transfer-function solve consistency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseInjection {
    /// Positive (source) terminal of the noise current.
    pub node_pos: NodeId,
    /// Negative (sink) terminal of the noise current.
    pub node_neg: NodeId,
    /// White PSD component, in A²/Hz (constant in `f`).
    pub white_psd: f64,
    /// `1/f` PSD numerator, in A². The contribution at frequency
    /// `f` is `flicker_coeff / f`.
    pub flicker_coeff: f64,
}

impl NoiseInjection {
    /// Evaluate this source's PSD at frequency `f` (Hz).
    ///
    /// Returns `white_psd + flicker_coeff / f`. The caller must pass
    /// `f > 0`; the AC sub-view rejects non-finite ω up-front and the
    /// control loop rejects `f ≤ 0` before reaching here.
    #[must_use]
    pub fn psd_at(&self, f_hz: f64) -> f64 {
        debug_assert!(f_hz > 0.0, "noise PSD evaluated at non-positive frequency");
        self.white_psd + self.flicker_coeff / f_hz
    }

    /// `true` if this injection has neither white nor `1/f`
    /// component; the loop skips silent sources to save one
    /// complex solve per frequency.
    #[must_use]
    pub fn is_silent(&self) -> bool {
        self.white_psd == 0.0 && self.flicker_coeff == 0.0
    }
}

// ---------------------------------------------------------------------
// Input / output / error types
// ---------------------------------------------------------------------

/// Noise analysis input bundle.
///
/// All references are borrowed for the duration of [`noise_analysis`].
///
/// Mirrors the shape of
/// [`AcAnalysisRequest`](crate::ac::AcAnalysisRequest) so callers
/// driving AC and noise in the same orchestrator can share validation
/// shape, with one addition: [`Self::dc_status`] is consulted at the
/// top of the loop. When the DC convergence failed, the loop returns
/// [`NoiseAnalysisResult::Failed`] without entering the per-frequency
/// loop.
#[derive(Debug, Clone, Copy)]
pub struct NoiseAnalysisRequest<'a> {
    /// The DC convergence status. If
    /// [`ConvergenceStatus::is_failure`] returns `true`, the noise
    /// loop short-circuits with [`NoiseAnalysisResult::Failed`] —
    /// this implements the
    /// `noise-analysis-on-circuit-with-failed-operating-point`
    /// scenario.
    pub dc_status: ConvergenceStatus,
    /// The DC operating-point MNA system. Required when
    /// `dc_status.is_converged()`. May still be supplied when the
    /// DC failed (e.g. as the last iterate); the loop ignores it on
    /// the failure path.
    pub system: &'a MnaSystem,
    /// The flattened incidence used to assemble `system`.
    pub structure: &'a FlattenedStructure,
    /// The source circuit graph (resistor / capacitor / inductor
    /// parameter lookups, plus the resistor walk for thermal-noise
    /// source collection).
    pub graph: &'a CircuitGraph,
    /// Frequencies (Hz) at which to evaluate the output PSD. Must
    /// be non-empty and all `> 0` and finite (PSD with a `1/f` term
    /// diverges at `f = 0`).
    pub frequencies_hz: &'a [f64],
    /// The single output node whose voltage PSD is reported.
    pub output: NodeId,
    /// Device temperature in kelvin used by the thermal-noise
    /// formula `4·k_B·T·G`. Pass
    /// [`device_modeling::noise::ROOM_TEMPERATURE_K`] (the SPICE
    /// default) when no per-device temperature is supplied.
    pub temperature_k: f64,
    /// Override the ground node (defaults to [`NodeId::GROUND`]).
    pub ground: Option<NodeId>,
}

/// Output of [`noise_analysis`].
///
/// Two variants implement the two scenarios in scope at tasks.md #37:
///
/// - [`Self::Ok`] — DC converged. The output PSD was computed at
///   each requested frequency.
/// - [`Self::Failed`] — DC did *not* converge. The control loop did
///   not enter the per-frequency loop; the original [`ConvergenceStatus`]
///   is forwarded so the caller can render the DC diagnostic in the
///   user-visible Result.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseAnalysisResult {
    /// Successful noise analysis.
    Ok(NoiseAnalysisData),
    /// DC failure short-circuit. Carries the originating
    /// [`ConvergenceStatus`] so the caller can render the failure
    /// diagnostic.
    Failed {
        /// The DC convergence status that triggered the short-circuit.
        dc_status: ConvergenceStatus,
    },
}

impl NoiseAnalysisResult {
    /// `true` iff this result carries spectral-density data.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// `true` iff this result is the failed-OP short-circuit.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Borrow the success payload, or `None` on the failure path.
    #[must_use]
    pub fn data(&self) -> Option<&NoiseAnalysisData> {
        match self {
            Self::Ok(d) => Some(d),
            Self::Failed { .. } => None,
        }
    }
}

/// Success payload of [`NoiseAnalysisResult::Ok`].
///
/// The two parallel vectors are aligned: index `i` corresponds to
/// frequency `frequencies_hz[i]` with output PSD
/// `spectral_density_v2_per_hz[i]`.
///
/// PSD is always non-negative (sum of squared magnitudes times
/// non-negative source PSDs). The control loop emits exactly
/// `frequencies_hz.len()` samples; partial sweeps are not produced.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NoiseAnalysisData {
    /// Frequency axis (Hz), echoing
    /// [`NoiseAnalysisRequest::frequencies_hz`].
    pub frequencies_hz: Vec<f64>,
    /// Output-referred power spectral density at each frequency, in
    /// V²/Hz. Always parallel to [`Self::frequencies_hz`].
    pub spectral_density_v2_per_hz: Vec<f64>,
}

impl NoiseAnalysisData {
    /// Number of frequency points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.frequencies_hz.len()
    }

    /// `true` iff this payload carries no samples. The control loop
    /// rejects empty sweeps so the only way to construct this shape
    /// is the [`Default`] impl.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frequencies_hz.is_empty()
    }
}

/// Errors raised by [`noise_analysis`].
///
/// Variants for each pre-flight validation surface plus the
/// downstream numerical failure surfaces. Numerical failures carry
/// the offending frequency so the caller can pinpoint which sweep
/// point misbehaved.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseAnalysisError {
    /// `frequencies_hz` was empty. The spec's "frequency Sweep"
    /// implies at least one point.
    EmptySweep,
    /// `temperature_k` was non-positive or non-finite. Stamping
    /// `4·k_B·T·G` with `T ≤ 0` is unphysical.
    NonPhysicalTemperature {
        /// The offending value.
        temperature_k: f64,
    },
    /// One of the supplied frequencies was non-finite (NaN, ±∞) or
    /// not strictly positive (the noise loop divides by `f` when
    /// `flicker_coeff != 0`).
    NonPositiveFrequency {
        /// The offending value (Hz).
        frequency_hz: f64,
    },
    /// The output `NodeId` exceeded the operating-point system's
    /// `node_count`. Indicates the caller paired a system with the
    /// wrong output node.
    OutputNodeOutOfRange {
        /// The offending node id.
        node: NodeId,
        /// The system's node count (including ground).
        node_count: u32,
    },
    /// The output node was the ground node. The PSD at ground is
    /// trivially zero (ground is a forced reference), and reporting
    /// it would be misleading; we surface this as a caller bug
    /// rather than silently emitting a zero curve.
    OutputNodeIsGround {
        /// The ground node id used.
        ground: NodeId,
    },
    /// The AC sub-view builder rejected the inputs at one frequency
    /// point. The wrapped [`AcSubViewError`] pinpoints the cause.
    SubViewBuildFailed {
        /// The frequency at which the failure occurred.
        frequency_hz: f64,
        /// The wrapped sub-view error.
        inner: AcSubViewError,
    },
    /// The complex-valued LU dispatch failed at one frequency
    /// point. Typical cause: a singular AC matrix at an undamped
    /// resonance, which propagates through the noise transfer-function
    /// computation just as it does through plain AC analysis.
    SolverFailed {
        /// The frequency at which the failure occurred.
        frequency_hz: f64,
        /// The wrapped solver error.
        inner: LinearSolverError,
    },
    /// A resistor's `resistance_ohms` parameter was non-finite or
    /// non-positive. The thermal-noise formula `4·k_B·T / R` is
    /// undefined at `R ≤ 0` or non-finite `R`. The operating-point
    /// assembler already rejects these; this is a defense-in-depth
    /// check at the noise layer.
    NonPhysicalResistance {
        /// The offending value (Ω).
        resistance_ohms: f64,
        /// The graph's element index for the offending resistor.
        element_index: u32,
    },
}

impl core::fmt::Display for NoiseAnalysisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptySweep => write!(f, "noise-analysis: frequency sweep is empty"),
            Self::NonPhysicalTemperature { temperature_k } => write!(
                f,
                "noise-analysis: temperature {temperature_k} K is non-physical (must be > 0 and finite)"
            ),
            Self::NonPositiveFrequency { frequency_hz } => write!(
                f,
                "noise-analysis: frequency {frequency_hz} Hz is non-positive or non-finite"
            ),
            Self::OutputNodeOutOfRange { node, node_count } => write!(
                f,
                "noise-analysis: output {node} is out of range for node_count={node_count}"
            ),
            Self::OutputNodeIsGround { ground } => write!(
                f,
                "noise-analysis: output node coincides with ground ({ground}); the PSD at ground is trivially zero"
            ),
            Self::SubViewBuildFailed {
                frequency_hz,
                inner,
            } => write!(
                f,
                "noise-analysis: AC sub-view build failed at f={frequency_hz} Hz: {inner}"
            ),
            Self::SolverFailed {
                frequency_hz,
                inner,
            } => write!(
                f,
                "noise-analysis: complex LU solve failed at f={frequency_hz} Hz: {inner}"
            ),
            Self::NonPhysicalResistance {
                resistance_ohms,
                element_index,
            } => write!(
                f,
                "noise-analysis: resistor #{element_index} has non-physical resistance {resistance_ohms} Ω"
            ),
        }
    }
}

impl std::error::Error for NoiseAnalysisError {}

// ---------------------------------------------------------------------
// Source collection
// ---------------------------------------------------------------------

/// Walk the [`CircuitGraph`] and produce one [`NoiseInjection`] per
/// resistor.
///
/// For each [`ElementKind::Resistor`] element, the function builds
/// a thermal-noise source with PSD `4·k_B·T / R` via
/// [`resistor_thermal_noise`] (so the Johnson-Nyquist formula lives
/// in exactly one place in the codebase — tasks.md #36's
/// `device-modeling` crate). The terminal-local source `(a, b)` is
/// translated into the graph's `NodeId` space via the element's
/// recorded terminal vector.
///
/// # Errors
///
/// - [`NoiseAnalysisError::NonPhysicalResistance`] if any resistor
///   has `R ≤ 0` or non-finite `R`. (The DC assembler rejects this
///   upstream; we recheck so the noise layer's failure modes are
///   self-contained.)
///
/// # Extension point (tasks.md #38 / #66)
///
/// Future device noise contributions (diode shot+flicker, BJT
/// shot, MOSFET channel thermal + flicker) plug in here: walk the
/// graph for [`ElementKind::Semiconductor`] elements, resolve the
/// `DeviceModel` from the element's `ModelName`, call
/// [`device_modeling::DeviceModel::noise_stamp`] with the
/// operating-point state, and lift each [`NoiseSource`] onto
/// graph-level `NodeId`s by the same terminal-mapping pattern used
/// here for resistors.
pub fn collect_noise_sources(
    graph: &CircuitGraph,
    temperature_k: f64,
) -> Result<Vec<NoiseInjection>, NoiseAnalysisError> {
    let mut injections: Vec<NoiseInjection> = Vec::new();

    for (idx, elem) in graph.elements().iter().enumerate() {
        if let ElementKind::Resistor { resistance_ohms } = *elem.kind() {
            if !resistance_ohms.is_finite() || resistance_ohms <= 0.0 {
                return Err(NoiseAnalysisError::NonPhysicalResistance {
                    resistance_ohms,
                    element_index: u32::try_from(idx).unwrap_or(u32::MAX),
                });
            }
            let src: NoiseSource = resistor_thermal_noise(resistance_ohms, temperature_k);
            // Resistor terminals are exactly `[a, b]` in graph-pin
            // order; the device-modeling helper emits
            // `(a=0, b=1)` in terminal-local indices which map
            // directly onto the graph terminal slots.
            let terms = elem.terminals();
            debug_assert_eq!(
                terms.len(),
                2,
                "resistor element has terminal count != 2: invariant violated by CircuitGraph builder"
            );
            let node_pos = terms[src.a];
            let node_neg = terms[src.b];
            // A truly degenerate resistor (`R = ∞`, but caught
            // above) or a defensively-coded zero stamp would be
            // silent — skip it.
            if src.white_psd == 0.0 && src.flicker_coeff == 0.0 {
                continue;
            }
            injections.push(NoiseInjection {
                node_pos,
                node_neg,
                white_psd: src.white_psd,
                flicker_coeff: src.flicker_coeff,
            });
        }
    }

    Ok(injections)
}

// ---------------------------------------------------------------------
// Control loop
// ---------------------------------------------------------------------

/// Run the noise spectral-density analysis control loop.
///
/// # Algorithm
///
/// 1. **DC failure short-circuit.** If `req.dc_status.is_failure()`,
///    return [`NoiseAnalysisResult::Failed`] immediately with the
///    forwarded status. No frequency loop runs; no AC sub-views are
///    built.
/// 2. **Up-front validation** of the sweep, temperature, output
///    node, and resistors.
/// 3. **Source collection.** Walk the graph and produce one
///    [`NoiseInjection`] per resistor via
///    [`collect_noise_sources`].
/// 4. **Per-frequency loop.** At each frequency `f_k`: build the AC
///    sub-view at `ω = 2πf_k` via
///    [`AcSubViewBuilder::from_operating_point`]; snapshot the AC
///    matrix's sparse triplets once; for every noise source,
///    construct a fresh
///    [`SparseLinearSystem<C64>`](numeric_solver::SparseLinearSystem)
///    reusing those triplets with a custom RHS containing
///    `+1 + 0j` at `node_pos` and `−1 + 0j` at `node_neg` (the
///    standard MNA stamp for a unit current source flowing from
///    `node_pos` into `node_neg`); solve via [`FaerComplexSolver`];
///    read the complex voltage `H_j(jω_k)` at the output node from
///    the unknowns; accumulate `|H_j|² · S_j(f_k)` into the running
///    PSD sum for `f_k`.
/// 5. Emit `(frequencies_hz, spectral_density_v2_per_hz)`.
///
/// # Errors
///
/// See [`NoiseAnalysisError`] for the complete list. The function
/// never panics in normal operation.
///
/// # Output PSD invariants
///
/// - Every emitted PSD value is non-negative (sum of squared
///   magnitudes times non-negative source PSDs).
/// - For a circuit with no resistors and no other noise sources
///   (e.g. only ideal voltage sources), the PSD is exactly `0.0` at
///   every frequency: the control loop accepts this and emits a
///   zero curve rather than failing.
///
/// # Panics
///
/// Does not panic in release. Debug assertions guard parallel-length
/// invariants on the return value.
pub fn noise_analysis(
    req: NoiseAnalysisRequest<'_>,
) -> Result<NoiseAnalysisResult, NoiseAnalysisError> {
    // The per-frequency loop is small but the function still
    // crosses 100 lines because of the four up-front validation
    // surfaces, the source-collection step, and the per-source
    // per-frequency solve. Splitting would scatter shared state
    // (solver instance, output buffer, ground node) across helpers
    // without making the algorithm clearer.
    #![allow(clippy::too_many_lines)]

    // --- DC failure short-circuit ------------------------------------------
    // Implements scenario
    // noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point.
    if req.dc_status.is_failure() {
        return Ok(NoiseAnalysisResult::Failed {
            dc_status: req.dc_status,
        });
    }

    // --- Up-front validation -----------------------------------------------
    if req.frequencies_hz.is_empty() {
        return Err(NoiseAnalysisError::EmptySweep);
    }
    if !req.temperature_k.is_finite() || req.temperature_k <= 0.0 {
        return Err(NoiseAnalysisError::NonPhysicalTemperature {
            temperature_k: req.temperature_k,
        });
    }
    for &f_hz in req.frequencies_hz {
        if !f_hz.is_finite() || f_hz <= 0.0 {
            return Err(NoiseAnalysisError::NonPositiveFrequency { frequency_hz: f_hz });
        }
    }
    let node_count = req.system.node_count();
    if req.output.index() >= node_count {
        return Err(NoiseAnalysisError::OutputNodeOutOfRange {
            node: req.output,
            node_count,
        });
    }
    let ground = req.ground.unwrap_or(NodeId::GROUND);
    if req.output == ground {
        return Err(NoiseAnalysisError::OutputNodeIsGround { ground });
    }

    // --- Source collection -------------------------------------------------
    let injections = collect_noise_sources(req.graph, req.temperature_k)?;

    // --- Output buffer -----------------------------------------------------
    let n_freq = req.frequencies_hz.len();
    let mut psd_out: Vec<f64> = vec![0.0; n_freq];

    let solver = FaerComplexSolver;
    let out_idx = req.output.index() as usize;

    // --- Per-frequency loop ------------------------------------------------
    for (k, &f_hz) in req.frequencies_hz.iter().enumerate() {
        // Build the AC sub-view at this frequency. We discard the
        // RHS the sub-view assembled (which carries whatever DC
        // stimulus the operating-point assembler stamped); the
        // noise transfer-function computation needs its *own* RHS
        // per source.
        let mut builder =
            AcSubViewBuilder::from_operating_point(req.system, req.structure, req.graph)
                .at_frequency(f_hz);
        if let Some(g) = req.ground {
            builder = builder.with_ground_node(g);
        }
        let view = builder
            .build()
            .map_err(|e| NoiseAnalysisError::SubViewBuildFailed {
                frequency_hz: f_hz,
                inner: e,
            })?;

        // Snapshot the triplets once per frequency; we will reuse
        // them across every per-source solve at this frequency.
        let dim = view.system().dim();
        let node_count_view = view.system().node_count();
        let branch_count_view = view.system().branch_count();
        let triplets_snapshot: Vec<SparseTriplet<C64>> = view.system().triplets().to_vec();

        // Defensive check: every injection's terminal must be in
        // range. The AC sub-view shares the operating-point's
        // `node_count`, so this is the same bound checked up front
        // for the output node — but we validate per-source so a
        // mis-built graph fails loudly.
        for inj in &injections {
            debug_assert!(
                inj.node_pos.index() < node_count_view && inj.node_neg.index() < node_count_view,
                "noise injection terminal out of range — graph/system mismatch"
            );
        }

        for inj in &injections {
            if inj.is_silent() {
                continue;
            }

            // Per-source PSD; if zero at this frequency we can skip
            // the solve entirely (no contribution).
            let s_j = inj.psd_at(f_hz);
            if s_j == 0.0 {
                continue;
            }

            // Build the RHS for a unit current injection from
            // node_pos to node_neg. MNA convention: a current
            // source `I` flowing from node `a` to node `b` adds
            // `−I` at row `a` and `+I` at row `b`, so KCL at each
            // node reads "sum of currents leaving the node equals
            // the source contribution".
            //
            // Conventionally one chooses the sign so that
            // `+1·H_out` is what one would measure on the
            // oscilloscope. PSD is sign-invariant via the squared
            // magnitude, so the consistent choice is what matters,
            // not the sign itself. We use the standard SPICE
            // convention (positive at the *from* terminal of the
            // injection vector entering the matrix, negative at
            // the *to* terminal — equivalent to a unit current
            // flowing *out of* node_pos and *into* node_neg).
            let mut rhs: Vec<C64> = vec![C64::new(0.0, 0.0); dim as usize];
            let pos_idx = inj.node_pos.index() as usize;
            let neg_idx = inj.node_neg.index() as usize;
            if NodeId::new(inj.node_pos.index()) != ground {
                rhs[pos_idx] = C64::new(-1.0, 0.0);
            }
            if NodeId::new(inj.node_neg.index()) != ground {
                rhs[neg_idx] = C64::new(1.0, 0.0);
            }

            let system_for_source = SparseLinearSystem::new(
                dim,
                node_count_view,
                branch_count_view,
                triplets_snapshot.clone(),
                rhs,
            )
            .map_err(|e| NoiseAnalysisError::SolverFailed {
                frequency_hz: f_hz,
                inner: e,
            })?;

            let solution =
                solver
                    .solve(&system_for_source)
                    .map_err(|e| NoiseAnalysisError::SolverFailed {
                        frequency_hz: f_hz,
                        inner: e,
                    })?;
            let h = solution.unknowns()[out_idx];
            let h_sq = h.norm_sqr();

            psd_out[k] += h_sq * s_j;
        }
    }

    // Defense in depth.
    debug_assert_eq!(psd_out.len(), n_freq);
    debug_assert!(psd_out.iter().all(|p| *p >= 0.0));

    Ok(NoiseAnalysisResult::Ok(NoiseAnalysisData {
        frequencies_hz: req.frequencies_hz.to_vec(),
        spectral_density_v2_per_hz: psd_out,
    }))
}

// ---------------------------------------------------------------------
// Helpers used internally and by tests
// ---------------------------------------------------------------------

/// Build a [`ConvergenceStatus::Converged`] with synthetic
/// "trivially-converged" diagnostics — a convenience used by tests
/// (and by callers that obtained the DC operating point through a
/// non-Newton path, e.g. a purely linear circuit that solves in
/// closed form).
///
/// `_` is unused; this exists as a documentation entry point only.
#[doc(hidden)]
#[must_use]
pub fn converged_status() -> ConvergenceStatus {
    use circuit_solver_types::convergence::{ConvergenceDiagnostic, ConvergenceTolerances};
    ConvergenceStatus::Converged(ConvergenceDiagnostic {
        update_norm: 0.0,
        residue_norm: 0.0,
        iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    })
}

/// Build a [`ConvergenceStatus::Diverged`] with synthetic diagnostics
/// — convenience for tests and for callers that classify their own
/// DC failure mode through a custom diagnostic.
#[doc(hidden)]
#[must_use]
pub fn diverged_status() -> ConvergenceStatus {
    use circuit_solver_types::convergence::{ConvergenceDiagnostic, ConvergenceTolerances};
    ConvergenceStatus::Diverged(ConvergenceDiagnostic {
        update_norm: f64::INFINITY,
        residue_norm: f64::INFINITY,
        iterations: 50,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
    use netlist_graph::CircuitBuilder;
    use numeric_solver::{assemble, flatten};

    // ---------- builders -------------------------------------------------

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

    fn single_resistor_to_ground(
        r_ohms: f64,
    ) -> (FlattenedStructure, CircuitGraph, MnaSystem, NodeId) {
        // V1 (1 V) → R1 → gnd, with a wire making the resistor
        // accessible at `n_out` between V1 and gnd. We measure
        // noise at `n_out` (between R1 and gnd).
        //
        // Actually for the canonical Johnson-Nyquist witness we
        // want the resistor's output node to be a *floating* node
        // driven only by the resistor's thermal source — i.e. the
        // resistor sees an open-circuit at the AC-noise port.
        //
        // The simplest topology: a single resistor R between
        // `n_out` and ground, with NO voltage source. But the
        // operating-point assembler in v1 needs at least one
        // independent source to anchor the DC, and the topology
        // checker (ADR-0009) rejects floating nodes.
        //
        // Workaround: add a `1 V` DC source between `n_out` and a
        // *separate* dummy node, then ground the resistor's other
        // end. The DC voltage on `n_out` is then 1 V, the AC noise
        // at `n_out` from R's thermal source sees R looking at
        // ground via R itself plus the AC short of the ideal
        // voltage source — but the ideal V1 is an AC short, so the
        // noise voltage at n_out is `V_n = i_n · 0 = 0`, which
        // washes out the witness.
        //
        // Correct witness topology: `V1 (1 V) → n_in`, `R1 between
        // n_in and n_out`, `R2 (much larger) between n_out and
        // gnd`. Then the noise at n_out from R1 sees R1 in series
        // with the parallel combination of R2 and the AC short of
        // V1. As R2 → ∞, the AC impedance seen by R1's thermal
        // source from n_out is just R1 (paralleled with the open
        // R2), and the noise voltage at n_out is `i_n · R1`, so
        // `S_V = R1² · 4kT/R1 = 4kTR1`.
        //
        // For the test we make R2 = 1 PΩ so the open approximation
        // is exact to many digits; we then *only* count R1's
        // contribution (R2's PSD `4kT/R2` is ~1e-12 times R1's and
        // negligible).
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", r_ohms);
        add_resistor(&mut b, "R2", "n_out", "0", 1.0e15);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        // Look up n_out's NodeId.
        let out_id = g
            .elements()
            .iter()
            .find(|e| e.name().as_str() == "R1")
            .expect("R1 present")
            .terminals()[1];
        (fs, g, sys, out_id)
    }

    fn approx(a: f64, b: f64, rel: f64) -> bool {
        let scale = a.abs().max(b.abs()).max(1.0e-300);
        (a - b).abs() / scale <= rel
    }

    // ---------- API contracts --------------------------------------------

    #[test]
    fn rejects_empty_sweep() {
        let (fs, g, sys, out_id) = single_resistor_to_ground(1.0e3);
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[],
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        assert_eq!(noise_analysis(req), Err(NoiseAnalysisError::EmptySweep));
    }

    #[test]
    fn rejects_non_positive_frequency() {
        let (fs, g, sys, out_id) = single_resistor_to_ground(1.0e3);
        // f = 0 is rejected because 1/f noise diverges there.
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[0.0],
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let err = noise_analysis(req).unwrap_err();
        match err {
            NoiseAnalysisError::NonPositiveFrequency { frequency_hz } => {
                assert_eq!(frequency_hz.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPositiveFrequency, got {other:?}"),
        }
        // NaN also rejected.
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[f64::NAN],
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        assert!(matches!(
            noise_analysis(req).unwrap_err(),
            NoiseAnalysisError::NonPositiveFrequency { .. }
        ));
    }

    #[test]
    fn rejects_non_physical_temperature() {
        let (fs, g, sys, out_id) = single_resistor_to_ground(1.0e3);
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0e3],
            output: out_id,
            temperature_k: 0.0,
            ground: None,
        };
        assert!(matches!(
            noise_analysis(req).unwrap_err(),
            NoiseAnalysisError::NonPhysicalTemperature { .. }
        ));
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0e3],
            output: out_id,
            temperature_k: f64::NAN,
            ground: None,
        };
        assert!(matches!(
            noise_analysis(req).unwrap_err(),
            NoiseAnalysisError::NonPhysicalTemperature { .. }
        ));
    }

    #[test]
    fn rejects_output_node_out_of_range() {
        let (fs, g, sys, _out_id) = single_resistor_to_ground(1.0e3);
        let nc = sys.node_count();
        let bad = NodeId::new(nc + 5);
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0e3],
            output: bad,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        match noise_analysis(req).unwrap_err() {
            NoiseAnalysisError::OutputNodeOutOfRange { node, node_count } => {
                assert_eq!(node, bad);
                assert_eq!(node_count, nc);
            }
            other => panic!("expected OutputNodeOutOfRange, got {other:?}"),
        }
    }

    #[test]
    fn rejects_output_node_is_ground() {
        let (fs, g, sys, _out_id) = single_resistor_to_ground(1.0e3);
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0e3],
            output: NodeId::GROUND,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        assert!(matches!(
            noise_analysis(req).unwrap_err(),
            NoiseAnalysisError::OutputNodeIsGround { .. }
        ));
    }

    // ---------- failed-OP scenario witness -------------------------------

    #[test]
    fn failed_operating_point_short_circuits() {
        // Scenario:
        // noise-spectral-density#noise-analysis-on-circuit-with-failed-operating-point.
        let (fs, g, sys, out_id) = single_resistor_to_ground(1.0e3);
        let bad_status = diverged_status();
        let req = NoiseAnalysisRequest {
            dc_status: bad_status,
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &[1.0e3, 10.0e3, 100.0e3],
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let res = noise_analysis(req).expect("loop returns Ok wrapping the Failed variant");
        match res {
            NoiseAnalysisResult::Failed { dc_status } => {
                assert!(dc_status.is_failure());
                assert_eq!(dc_status, bad_status);
            }
            NoiseAnalysisResult::Ok(_) => panic!("expected Failed, got Ok"),
        }
    }

    // ---------- resistive-circuit witness --------------------------------

    #[test]
    fn spec_scenario_resistor_only_thermal_noise_matches_4ktr() {
        // Scenario:
        // noise-spectral-density#noise-analysis-on-a-resistive-circuit.
        //
        // Topology: V1 (1 V) → R1 (10 kΩ) → n_out → R2 (1 PΩ) → gnd.
        //
        // The dominant noise contribution at n_out is R1's thermal
        // PSD `4·k_B·T / R1` driving the AC impedance seen at
        // n_out, which for R2 → ∞ is just R1 (V1 is an AC short).
        // Therefore S_V(f) ≈ R1 · 4·k_B·T = 4·k_B·T·R1 — the
        // textbook Johnson-Nyquist voltage PSD of a resistor.
        //
        // R2 contributes its own thermal source `4·k_B·T / R2`,
        // but the AC impedance from R2's port to n_out is also
        // ≈ R1 (R2 itself is huge, parallel R1 dominates), so its
        // contribution is `R1² · 4·k_B·T / R2`, smaller than R1's
        // by a factor of `R1 / R2` = 10⁻¹¹. Negligible.
        let r1 = 10.0e3;
        let (fs, g, sys, out_id) = single_resistor_to_ground(r1);
        let f_axis: Vec<f64> = vec![1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6];
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &f_axis,
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let res = noise_analysis(req).expect("converges on a linear circuit");
        let data = res.data().expect("data present on Ok");
        assert_eq!(data.len(), f_axis.len());

        let expected = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K * r1;
        // Thermal noise is white: every sample equals 4kTR within a
        // tight relative tolerance (the LU solver returns close to
        // bit-exact results on a 3×3 conductance matrix).
        for (i, &s_v) in data.spectral_density_v2_per_hz.iter().enumerate() {
            assert!(
                approx(s_v, expected, 1.0e-6),
                "f[{i}]={} Hz: expected ~{expected:.6e} V²/Hz, got {s_v:.6e}",
                f_axis[i]
            );
        }
    }

    #[test]
    fn output_psd_is_white_across_decades_for_purely_resistive() {
        // Same witness, expanded: assert that the PSD value is
        // *frequency-independent* (the resistor noise is white).
        let r1 = 1.0e3;
        let (fs, g, sys, out_id) = single_resistor_to_ground(r1);
        let f_axis: Vec<f64> = vec![1.0e-1, 1.0, 1.0e1, 1.0e2, 1.0e3, 1.0e6, 1.0e9];
        let req = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &f_axis,
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let data = noise_analysis(req).unwrap().data().cloned().unwrap();
        let first = data.spectral_density_v2_per_hz[0];
        assert!(first > 0.0, "first PSD sample must be strictly positive");
        for &s in &data.spectral_density_v2_per_hz[1..] {
            assert!(
                approx(s, first, 1.0e-9),
                "white-noise invariant: every sample equals the first ({first:.6e}), got {s:.6e}"
            );
        }
    }

    #[test]
    fn output_psd_scales_linearly_with_resistance() {
        // Doubling R doubles the PSD: 4kT·(2R) = 2 · (4kT·R).
        let f_axis = [1.0e3];

        let (fs1, g1, sys1, out1) = single_resistor_to_ground(1.0e3);
        let req1 = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys1,
            structure: &fs1,
            graph: &g1,
            frequencies_hz: &f_axis,
            output: out1,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let s1 = noise_analysis(req1)
            .unwrap()
            .data()
            .unwrap()
            .spectral_density_v2_per_hz[0];

        let (fs2, g2, sys2, out2) = single_resistor_to_ground(2.0e3);
        let req2 = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys2,
            structure: &fs2,
            graph: &g2,
            frequencies_hz: &f_axis,
            output: out2,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let s2 = noise_analysis(req2)
            .unwrap()
            .data()
            .unwrap()
            .spectral_density_v2_per_hz[0];

        assert!(
            approx(s2, 2.0 * s1, 1.0e-6),
            "doubling R should double S_V: 2·{s1:.6e} ≈ {s2:.6e}"
        );
    }

    #[test]
    fn output_psd_scales_linearly_with_temperature() {
        // Doubling T doubles the PSD: 4k(2T)R = 2 · (4kTR).
        let f_axis = [1.0e3];
        let r1 = 1.0e3;
        let (fs, g, sys, out_id) = single_resistor_to_ground(r1);

        let req1 = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &f_axis,
            output: out_id,
            temperature_k: ROOM_TEMPERATURE_K,
            ground: None,
        };
        let s1 = noise_analysis(req1)
            .unwrap()
            .data()
            .unwrap()
            .spectral_density_v2_per_hz[0];

        let req2 = NoiseAnalysisRequest {
            dc_status: converged_status(),
            system: &sys,
            structure: &fs,
            graph: &g,
            frequencies_hz: &f_axis,
            output: out_id,
            temperature_k: 2.0 * ROOM_TEMPERATURE_K,
            ground: None,
        };
        let s2 = noise_analysis(req2)
            .unwrap()
            .data()
            .unwrap()
            .spectral_density_v2_per_hz[0];

        assert!(
            approx(s2, 2.0 * s1, 1.0e-6),
            "doubling T should double S_V: 2·{s1:.6e} ≈ {s2:.6e}"
        );
    }

    #[test]
    fn collect_noise_sources_emits_one_injection_per_resistor() {
        let (_fs, g, _sys, _out) = single_resistor_to_ground(2.0e3);
        let injs =
            collect_noise_sources(&g, ROOM_TEMPERATURE_K).expect("collection succeeds on linear");
        // The fixture has two resistors (R1, R2); both produce
        // thermal sources.
        assert_eq!(injs.len(), 2);
        // R1 = 2 kΩ → PSD = 4·k·T / R1.
        let expected_psd_r1 = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K / 2.0e3;
        // R2 = 1e15 Ω → ~4e-38 A²/Hz, negligible.
        let r1_inj = injs
            .iter()
            .find(|i| approx(i.white_psd, expected_psd_r1, 1.0e-9))
            .expect("R1's injection present");
        // Direction is irrelevant for PSD; just check the white
        // component. The flicker numerator was initialized to
        // exact bit-zero by `resistor_thermal_noise`, so a
        // bit-pattern equality is the right comparison here.
        assert_eq!(r1_inj.flicker_coeff.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn collect_noise_sources_rejects_non_physical_resistance() {
        // Construct a graph with a NaN resistor; the topology
        // checker will not actually catch this for us in
        // CircuitBuilder, so we have to hand-craft. The DC
        // assemble would reject it, but collect_noise_sources is
        // entered without re-running DC, so the noise layer must
        // catch it on its own.
        //
        // We can sidestep this by skipping the DC build entirely
        // and just calling collect_noise_sources on a graph with
        // a zero-ohm resistor. CircuitBuilder accepts the value;
        // assemble rejects it.
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "n_out", 0.0);
        let g = b.build().expect("builder accepts the value structurally");
        match collect_noise_sources(&g, ROOM_TEMPERATURE_K).unwrap_err() {
            NoiseAnalysisError::NonPhysicalResistance {
                resistance_ohms, ..
            } => {
                assert_eq!(resistance_ohms.to_bits(), 0.0_f64.to_bits());
            }
            other => panic!("expected NonPhysicalResistance, got {other:?}"),
        }
    }

    #[test]
    fn noise_injection_psd_at_handles_white_and_flicker() {
        let inj = NoiseInjection {
            node_pos: NodeId::new(1),
            node_neg: NodeId::GROUND,
            white_psd: 1.0e-18,
            flicker_coeff: 1.0e-15,
        };
        // At f = 1 Hz: white + flicker = 1e-18 + 1e-15.
        let s1 = inj.psd_at(1.0);
        assert!(approx(s1, 1.0e-18 + 1.0e-15, 1.0e-12));
        // At f = 1 kHz: white + 1e-15 / 1e3 = 1e-18 + 1e-18 = 2e-18.
        let s_k = inj.psd_at(1.0e3);
        assert!(approx(s_k, 2.0e-18, 1.0e-9));
        // At f → ∞: → white_psd.
        let s_inf = inj.psd_at(1.0e30);
        assert!(approx(s_inf, 1.0e-18, 1.0e-9));
        assert!(!inj.is_silent());
        let silent = NoiseInjection {
            white_psd: 0.0,
            flicker_coeff: 0.0,
            ..inj
        };
        assert!(silent.is_silent());
    }

    #[test]
    fn result_helpers_distinguish_ok_from_failed() {
        let ok = NoiseAnalysisResult::Ok(NoiseAnalysisData::default());
        assert!(ok.is_ok() && !ok.is_failed());
        assert!(ok.data().is_some());

        let failed = NoiseAnalysisResult::Failed {
            dc_status: diverged_status(),
        };
        assert!(failed.is_failed() && !failed.is_ok());
        assert!(failed.data().is_none());
    }
}
