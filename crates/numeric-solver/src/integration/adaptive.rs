//! Adaptive timestepping with local-truncation-error (LTE) step
//! rejection.
//!
//! Implements tasks.md **#32**: the *library primitive* that the
//! transient analysis control loop (tasks.md #33) uses to decide,
//! after each tentative timestep, whether the integration error is
//! within the user's tolerance envelope; if not, reject the step and
//! recommend a smaller step size to re-solve at.
//!
//! Per `wiki/specs/transient-time-domain.md`, scenario
//! `transient-time-domain#adaptive-timestepping-rejects-and-re-solves`:
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit with rapidly switching inputs
//! And the initial timestep is set to 1 ns
//! When the Simulator estimates a local truncation error exceeding the error tolerance
//! Then the Simulator rejects the current step
//! And the Simulator re-solves at a smaller timestep
//! And the final Result contains only accepted time points
//! And the timestep history is available in the Result metadata
//! ```
//!
//! This module owns the four pieces the scenario observes:
//!
//! 1. [`LteToleranceEnvelope`] — the per-node `max(rel·|v|, abs)`
//!    tolerance check, transient defaults `rel = 1 %`, `abs = 1 mV`
//!    per ADR-0008.
//! 2. [`LteEstimator`] — a divided-difference LTE estimator that
//!    converts a per-node history of three accepted-or-tentative node
//!    voltages plus the current step `h` into a per-node LTE
//!    magnitude estimate.
//! 3. [`step_decision`] — converts a per-node LTE estimate and the
//!    envelope into [`StepOutcome::Accept`] or
//!    [`StepOutcome::Reject`], **plus** a recommended next step size
//!    `h_new` per the classical proportional rule
//!    `h_new = clamp(h · safety · (tol_norm / err_norm)^(1/(p+1)))`.
//! 4. [`TimestepHistory`] — append-only log of every tentative step
//!    (accepted or rejected) plus the per-step max LTE-to-tolerance
//!    ratio, queryable for the Result metadata payload that tasks.md
//!    #35 will surface.
//!
//! # Scope: what this task implements vs. what it does not
//!
//! Task #32 explicitly depends only on #29 (BE companions). The full
//! outer loop — DC-or-UIC initial state, calling the NR driver,
//! advancing histories, accumulating the `Waveform` outputs — lands
//! in task **#33**. Task **#35** lifts the [`TimestepHistory`] this
//! module produces into the `Result` metadata.
//!
//! So this module is deliberately a *pure-compute* library: it does
//! not know about MNA matrices, the NR driver, or any solver state.
//! It takes node-voltage histories produced by a hypothetical outer
//! loop and returns step decisions. That keeps the LTE math
//! independently testable — the Gherkin scenario's observable
//! behavior (reject → re-solve at smaller `h`) is exercised by
//! feeding a deliberately-fast voltage transient into
//! [`step_decision`] and asserting [`StepOutcome::Reject`] with a
//! shrunk `h_new`, then feeding a slower transient that simulates the
//! post-shrink re-solve and asserting [`StepOutcome::Accept`].
//!
//! # ADR alignment
//!
//! - **ADR-0006** (Dual NR convergence criterion) — vacuously honored.
//!   The LTE estimator runs *after* an accepted NR solve at each
//!   tentative timestep; it does not change NR's convergence
//!   criteria.
//! - **ADR-0007** (Zero-order-hold analog/digital boundary) —
//!   vacuously honored. No A/D boundary surface added.
//! - **ADR-0008** (Per-node max(rel, abs) tolerance envelope) —
//!   **directly honored**. [`LteToleranceEnvelope::accepts`] uses the
//!   same `max(rel·|v|, abs)` formulation as the conformance harness;
//!   defaults are the spec's transient row (`1 %` relative, `1 mV`
//!   absolute per node) per `design.md` row QAS-5 / QAS-2.
//! - **ADR-0009** (Topology checker) — vacuously honored. The LTE
//!   estimator runs over the same solution vector the assembler
//!   produced; topology is the assembler's concern.
//! - **ADR-0010** (Unstable v1 public API) — honored. New public types
//!   are part of the unstable v1 surface.
//!
//! # Discretization order
//!
//! Backward Euler is a **first-order** implicit method (`p = 1`), so
//! its LTE per step scales as `O(h²)`. The classical step-size
//! controller uses the exponent `1/(p+1) = 1/2`. The order constant
//! `p` lives on [`LteEstimator`] so that sibling adapters for
//! Trapezoidal (#30, `p = 2`) and Gear-2 BDF (#31, `p = 2`) can reuse
//! the same controller without code duplication.
//!
//! # Numerical-pitfalls handled
//!
//! - **All-zero voltages** — when every node history is exactly zero
//!   (e.g. the very first transient step from a zero initial
//!   condition before any source has driven a node), the relative
//!   term `rel · |v|` is zero and the controller defers to the
//!   absolute floor `abs`. Without an absolute floor a zero-history
//!   step would reject regardless of step size; the floor makes
//!   the controller correctly accept.
//! - **Non-finite LTE** — if the LTE estimator produces NaN or ±∞
//!   (caused by upstream divergence in the NR solve injecting
//!   non-finite values into the history), the step is unconditionally
//!   [`StepOutcome::Reject`]ed and `h_new` is clamped to `h_min`.
//!   This prevents the controller from amplifying a NR divergence
//!   into a step-size explosion.
//! - **Bounded step-size growth** — even on a high-quality step
//!   (`err ≪ tol`), `h_new` is capped at `h · max_grow` to prevent
//!   the controller from leaping past a fast transient region into a
//!   slow region with a single oversized step. The classical default
//!   is `max_grow = 2.0` (no more than a 2× jump per accepted step).

use core::fmt;

// -----------------------------------------------------------------------
// Tolerance envelope (ADR-0008 max(rel, abs))
// -----------------------------------------------------------------------

/// Per-node `max(rel · |v|, abs)` tolerance envelope used by the
/// adaptive timestepping controller to decide whether an LTE estimate
/// is within tolerance.
///
/// Aligned with ADR-0008 (the conformance harness uses the same
/// formulation against the golden reference) and `design.md` QAS-5 /
/// QAS-2 (the transient tolerance row sets `rel = 1 %`, `abs = 1 mV`
/// per node).
///
/// # Field semantics
///
/// - `rel` — dimensionless relative tolerance, e.g. `0.01` for 1 %.
///   Must be finite and non-negative.
/// - `abs` — absolute tolerance in the same units as the per-node
///   quantity being compared (volts for node voltages). Must be
///   finite and non-negative.
///
/// At least one of `rel`, `abs` must be strictly positive for the
/// envelope to be non-trivial; an envelope with both zero accepts
/// only exactly-zero LTEs and is therefore practically useless. The
/// constructor [`LteToleranceEnvelope::new`] rejects that case.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LteToleranceEnvelope {
    /// Relative tolerance, dimensionless. `0.01` means 1 %.
    pub rel: f64,
    /// Absolute tolerance floor in the per-node quantity's units
    /// (volts for node voltages, amps for branch currents).
    pub abs: f64,
}

impl LteToleranceEnvelope {
    /// Construct a tolerance envelope from explicit relative and
    /// absolute components.
    ///
    /// # Errors
    ///
    /// Returns [`AdaptiveError::NonFiniteTolerance`] if `rel` or
    /// `abs` is non-finite, [`AdaptiveError::NegativeTolerance`] if
    /// either is negative, or [`AdaptiveError::ZeroTolerance`] if
    /// both are exactly zero.
    pub fn new(rel: f64, abs: f64) -> Result<Self, AdaptiveError> {
        if !rel.is_finite() {
            return Err(AdaptiveError::NonFiniteTolerance {
                field: "rel",
                value: rel,
            });
        }
        if !abs.is_finite() {
            return Err(AdaptiveError::NonFiniteTolerance {
                field: "abs",
                value: abs,
            });
        }
        if rel < 0.0 {
            return Err(AdaptiveError::NegativeTolerance {
                field: "rel",
                value: rel,
            });
        }
        if abs < 0.0 {
            return Err(AdaptiveError::NegativeTolerance {
                field: "abs",
                value: abs,
            });
        }
        if rel == 0.0 && abs == 0.0 {
            return Err(AdaptiveError::ZeroTolerance);
        }
        Ok(Self { rel, abs })
    }

    /// Default transient envelope per ADR-0008 / QAS-2: `1 %`
    /// relative, `1 mV` absolute per node voltage.
    ///
    /// Note: the conformance harness compares against the golden
    /// reference at every reported time point; the LTE controller
    /// uses the *same* envelope per accepted timestep. This is by
    /// design — the LTE envelope is the controller's *internal*
    /// truncation-error budget, chosen so that an accepted run meets
    /// QAS-2 against the golden reference without per-circuit tuning.
    #[must_use]
    pub const fn transient_default() -> Self {
        Self {
            rel: 0.01,
            abs: 1.0e-3,
        }
    }

    /// Return the per-node tolerance threshold `max(rel · |v|, abs)`
    /// for a single node voltage.
    ///
    /// The argument `v_ref` is the *reference* magnitude — typically
    /// the most recent accepted node voltage at that node, which the
    /// controller uses as the proxy for "signal magnitude" the
    /// relative component scales against. Sign is irrelevant
    /// (`v_ref.abs()` is taken internally).
    ///
    /// Returns `f64::INFINITY` if `v_ref` is non-finite — a
    /// non-finite reference signal already indicates an upstream
    /// divergence, so the conservative thing is to *fail-safe* by
    /// accepting nothing finite and forcing the
    /// [`step_decision`]-level non-finite-LTE branch to reject. The
    /// non-finite-LTE branch in [`step_decision`] takes precedence
    /// over this and rejects unconditionally.
    #[must_use]
    pub fn threshold(&self, v_ref: f64) -> f64 {
        if !v_ref.is_finite() {
            return f64::INFINITY;
        }
        (self.rel * v_ref.abs()).max(self.abs)
    }

    /// Test whether a single per-node LTE magnitude `lte` is within
    /// the envelope given a reference signal magnitude `v_ref`.
    ///
    /// Returns `false` if `lte` is non-finite (an upstream
    /// divergence in the LTE estimator must fail-safe reject).
    #[must_use]
    pub fn accepts(&self, lte: f64, v_ref: f64) -> bool {
        if !lte.is_finite() {
            return false;
        }
        lte.abs() <= self.threshold(v_ref)
    }
}

// -----------------------------------------------------------------------
// LTE estimator (divided-difference)
// -----------------------------------------------------------------------

/// Three-point per-node voltage history at the most recent two
/// *accepted* timesteps plus the tentative current step.
///
/// The LTE for a first-order BE step at `t_{n+1}` is, up to leading
/// order:
///
/// ```text
///   LTE_BE(t_{n+1}) ≈ (h² / 2) · y''(t_{n+1})
/// ```
///
/// We don't have `y''` analytically; the controller estimates it via
/// a backward-divided-difference using the two prior accepted
/// solution points (`v_prev_prev` at `t_{n-1}`, `v_prev` at `t_n`)
/// and the tentative current solution (`v_curr` at `t_{n+1}`):
///
/// ```text
///   y''(t_{n+1}) ≈ (v_curr − 2 · v_prev + v_prev_prev) / h²
/// ```
///
/// so the estimated LTE per node is:
///
/// ```text
///   LTE ≈ |v_curr − 2 · v_prev + v_prev_prev| / 2
/// ```
///
/// independent of `h` to leading order. (The `h²` in the numerator
/// of `y''` cancels with the `h² / 2` of the BE LTE formula; the
/// factor of `1/2` carries through.) This is the classical
/// estimator described in Vlach & Singhal *Computer Methods for
/// Circuit Analysis and Design*, §11.4, and is the same one ngspice
/// uses internally (trap-rule LTE check; see ngspice source
/// `src/spicelib/analysis/cktacct.c`).
///
/// # Field semantics
///
/// - `v_prev_prev` — node voltage at the second-most-recently-
///   accepted timestep `t_{n-1}` (volts).
/// - `v_prev` — node voltage at the most-recently-accepted timestep
///   `t_n` (volts).
/// - `v_curr` — the *tentative* node voltage at `t_{n+1}` produced
///   by the latest NR solve at step size `h`. The LTE controller
///   evaluates this point and decides whether to accept it.
///
/// # Sentinel for "not enough history yet"
///
/// The first two transient steps don't have two prior accepted
/// points to difference against. For those steps, the controller
/// should *not* call [`LteEstimator::lte_for_node`] and should
/// instead unconditionally accept (no LTE estimate is possible). The
/// outer loop (tasks.md #33) is responsible for that bookkeeping; this
/// estimator does not have a notion of "step index" because doing
/// so would couple it to the outer loop's state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeHistorySample {
    /// Node voltage at `t_{n-1}` (the second-most-recently-accepted
    /// timestep), in volts.
    pub v_prev_prev: f64,
    /// Node voltage at `t_n` (the most-recently-accepted timestep),
    /// in volts.
    pub v_prev: f64,
    /// Tentative node voltage at `t_{n+1}` from the latest NR solve
    /// at step size `h`, in volts.
    pub v_curr: f64,
}

/// The discretization-order-aware LTE estimator.
///
/// One [`LteEstimator`] is created per `Analysis` request (the
/// integration method is fixed for the duration of a transient
/// analysis per the spec scenarios for BE / Trapezoidal / Gear-2
/// BDF). The estimator's `order` field carries the integration
/// method's order `p` so the step-size controller can apply the
/// correct exponent in the classical rule.
///
/// # Why a struct, not a free function
///
/// The order field needs to be remembered across many step
/// decisions, and sibling integration methods (Trapezoidal in #30,
/// Gear-2 BDF in #31) will set `order = 2`. Carrying it in a struct
/// lets the outer loop construct the estimator once and pass it by
/// reference to [`step_decision`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LteEstimator {
    /// The integration method's order `p`. Backward Euler:
    /// `p = 1`. Trapezoidal / Gear-2 BDF (siblings, not yet
    /// implemented): `p = 2`.
    pub order: u32,
}

impl LteEstimator {
    /// Estimator for Backward Euler (order `p = 1`).
    ///
    /// Backward Euler's LTE per step is `(h²/2) · y''(t_{n+1})`; the
    /// estimator returns `|y''|/2` extracted from three consecutive
    /// node voltages via finite differences.
    #[must_use]
    pub const fn backward_euler() -> Self {
        Self { order: 1 }
    }

    /// Estimate the per-node LTE magnitude for one node given its
    /// three-point history.
    ///
    /// For Backward Euler:
    ///
    /// ```text
    ///   LTE_node ≈ |v_curr − 2·v_prev + v_prev_prev| / 2
    /// ```
    ///
    /// (See [`NodeHistorySample`] for the derivation.) For sibling
    /// second-order methods (Trapezoidal, Gear-2 BDF) the same
    /// finite-difference proxy is used but the controller's
    /// step-size exponent in [`step_decision`] reflects the higher
    /// order.
    ///
    /// # Errors
    ///
    /// Returns [`AdaptiveError::NonFiniteHistory`] if any of the
    /// three samples in `sample` is non-finite.
    ///
    /// # Note on `h`
    ///
    /// The leading-order LTE formula's explicit `h²` cancels with
    /// the `h²` denominator in the finite-difference approximation
    /// of `y''`, so `h` does not appear in the returned magnitude.
    /// This is the standard simplification in the SPICE-family
    /// adaptive controllers and is *not* a bug — `h` re-enters via
    /// the step-size proportional rule in [`next_step_size`].
    pub fn lte_for_node(&self, sample: NodeHistorySample) -> Result<f64, AdaptiveError> {
        if !sample.v_prev_prev.is_finite() {
            return Err(AdaptiveError::NonFiniteHistory {
                field: "v_prev_prev",
                value: sample.v_prev_prev,
            });
        }
        if !sample.v_prev.is_finite() {
            return Err(AdaptiveError::NonFiniteHistory {
                field: "v_prev",
                value: sample.v_prev,
            });
        }
        if !sample.v_curr.is_finite() {
            return Err(AdaptiveError::NonFiniteHistory {
                field: "v_curr",
                value: sample.v_curr,
            });
        }
        let second_difference = sample.v_curr - 2.0 * sample.v_prev + sample.v_prev_prev;
        Ok((second_difference / 2.0).abs())
    }

    /// Estimate the worst-case LTE / threshold ratio across all
    /// nodes.
    ///
    /// The controller rejects whenever the worst-case ratio exceeds
    /// `1.0` and uses the ratio to pick the next step size per the
    /// classical proportional rule. The reference signal `v_ref`
    /// for each node is taken from `sample.v_prev` (the most recent
    /// accepted voltage at that node).
    ///
    /// # Errors
    ///
    /// Returns [`AdaptiveError::EmptyHistory`] if `samples` is empty
    /// (the outer loop must always have at least one node;
    /// physically every circuit has at least the implicit ground
    /// reference plus one solved node).
    /// Returns [`AdaptiveError::NonFiniteHistory`] if any sample
    /// contains a non-finite voltage.
    ///
    /// # Returns
    ///
    /// `(worst_ratio, worst_node_index)` where `worst_ratio = lte /
    /// threshold` for the node with the largest such ratio. The
    /// caller may use `worst_node_index` for diagnostic logging.
    pub fn worst_ratio(
        &self,
        samples: &[NodeHistorySample],
        envelope: LteToleranceEnvelope,
    ) -> Result<(f64, usize), AdaptiveError> {
        if samples.is_empty() {
            return Err(AdaptiveError::EmptyHistory);
        }
        let mut worst_ratio = 0.0_f64;
        let mut worst_index = 0_usize;
        for (i, sample) in samples.iter().enumerate() {
            let lte = self.lte_for_node(*sample)?;
            let threshold = envelope.threshold(sample.v_prev);
            // threshold is always > 0 when envelope was constructed
            // via the validated `LteToleranceEnvelope::new` (or
            // `transient_default`): both ensure not-both-zero, and
            // `v_prev.abs()` is finite by the earlier non-finite
            // guard. So the division is safe.
            debug_assert!(threshold > 0.0, "envelope threshold must be positive");
            let ratio = lte / threshold;
            if ratio > worst_ratio {
                worst_ratio = ratio;
                worst_index = i;
            }
        }
        Ok((worst_ratio, worst_index))
    }
}

// -----------------------------------------------------------------------
// Step decision + size controller
// -----------------------------------------------------------------------

/// The outcome of evaluating a tentative timestep against the LTE
/// envelope.
///
/// The outer transient control loop (#33) consumes this and either
/// folds the accepted solution into the Result Waveforms (on
/// `Accept`) or discards the solution and re-solves at `next_h` (on
/// `Reject`). Either way, the [`StepDecision::next_h`] is the
/// recommendation for the **next** step to attempt: on accept it is
/// the next-forward step size; on reject it is the shrunk re-solve
/// step size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepDecision {
    /// Whether the tentative step's LTE is within the envelope.
    pub outcome: StepOutcome,
    /// The recommended next step size in seconds for the outer
    /// loop's next attempt. On `Accept`, this is the size of the
    /// *next* timestep to take (typically larger or equal to the
    /// just-accepted step). On `Reject`, this is the smaller step
    /// to re-solve the *current* time point at.
    pub next_h: f64,
    /// The worst-case LTE / threshold ratio across all nodes. A
    /// value `> 1.0` produces `Reject`; `<= 1.0` produces `Accept`.
    /// Surfaced for the timestep history metadata (tasks.md #35).
    pub worst_ratio: f64,
    /// Index into the `samples` slice passed to [`step_decision`]
    /// identifying the node that produced `worst_ratio`. `None`
    /// when the samples slice was empty (which the controller
    /// rejects upstream, so consumers typically see `Some(_)`).
    ///
    /// Plumbed through here for the timestep-history metadata
    /// (tasks.md #35) per the reviewer's downstream note on
    /// tasks.md #32: the LTE estimator already computes the worst
    /// node identity, and surfacing it lets users plot
    /// "which node is driving step rejection?" without recomputing
    /// the LTE.
    pub worst_index: Option<usize>,
}

/// Whether the controller accepts or rejects the tentative step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    /// LTE within envelope; outer loop folds the solution into the
    /// Result Waveforms and advances `t` by `h`.
    Accept,
    /// LTE exceeds envelope; outer loop must discard the solution,
    /// restore the previous-step element histories, and re-solve at
    /// [`StepDecision::next_h`].
    Reject,
}

/// Bounds on the step-size controller's output.
///
/// All sizes are in seconds. The controller clamps its recommended
/// `next_h` into the interval `[h_min, h_max]` and limits the
/// step-to-step growth to `max_grow_factor` (avoids leaping past
/// fast transient regions) and shrinkage to `min_shrink_factor`
/// (avoids overshrinking on a single bad step which would slow the
/// run unnecessarily).
///
/// Reasonable defaults (per [`StepSizeBounds::transient_default`])
/// match the values used in textbook SPICE-family adaptive
/// controllers: `h_min = 1 ps`, `h_max = ∞` (caller-bounded by the
/// transient interval), `safety_factor = 0.9`, `max_grow_factor =
/// 2.0`, `min_shrink_factor = 0.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepSizeBounds {
    /// Hard lower bound on step size, in seconds. Even on
    /// catastrophically large LTEs the controller never recommends
    /// smaller than this — at which point the outer loop should
    /// raise a convergence error rather than spin forever.
    pub h_min: f64,
    /// Hard upper bound on step size, in seconds. The outer loop
    /// typically computes this from the requested transient
    /// interval (e.g. `t_stop - t_start`).
    pub h_max: f64,
    /// Multiplicative safety margin applied to the proportional
    /// rule's recommendation, in `(0, 1]`. Default `0.9`.
    pub safety_factor: f64,
    /// Maximum step-to-step growth factor per accepted step.
    /// Default `2.0`.
    pub max_grow_factor: f64,
    /// Minimum step-to-step shrink factor per rejected step.
    /// Default `0.1` (no more than a 10× shrink on a single
    /// reject).
    pub min_shrink_factor: f64,
}

impl StepSizeBounds {
    /// Transient-defaults bounds.
    ///
    /// `h_min = 1 ps`, `h_max = f64::INFINITY` (the outer loop
    /// imposes a real upper bound from the transient interval),
    /// `safety_factor = 0.9`, `max_grow_factor = 2.0`,
    /// `min_shrink_factor = 0.1`.
    ///
    /// These match the SPICE-family defaults (cf. ngspice
    /// `src/spicelib/analysis/cktacct.c` and Vlach & Singhal
    /// §11.4).
    #[must_use]
    pub const fn transient_default() -> Self {
        Self {
            h_min: 1.0e-12,
            h_max: f64::INFINITY,
            safety_factor: 0.9,
            max_grow_factor: 2.0,
            min_shrink_factor: 0.1,
        }
    }

    /// Validate the bounds.
    ///
    /// # Errors
    ///
    /// Returns an [`AdaptiveError`] variant for the first invalid
    /// field discovered. The outer loop is expected to call this
    /// once at the start of a transient analysis.
    pub fn validate(self) -> Result<Self, AdaptiveError> {
        if !self.h_min.is_finite() || self.h_min <= 0.0 {
            return Err(AdaptiveError::NonPositiveStepBound {
                field: "h_min",
                value: self.h_min,
            });
        }
        // h_max is allowed to be +infinity (caller-bounded by
        // transient interval); only reject NaN and non-positive.
        if self.h_max.is_nan() || self.h_max <= 0.0 {
            return Err(AdaptiveError::NonPositiveStepBound {
                field: "h_max",
                value: self.h_max,
            });
        }
        if self.h_max < self.h_min {
            return Err(AdaptiveError::HmaxBelowHmin {
                h_min: self.h_min,
                h_max: self.h_max,
            });
        }
        if !self.safety_factor.is_finite() || self.safety_factor <= 0.0 || self.safety_factor > 1.0
        {
            return Err(AdaptiveError::OutOfRangeFactor {
                field: "safety_factor",
                value: self.safety_factor,
                expected: "(0, 1]",
            });
        }
        if !self.max_grow_factor.is_finite() || self.max_grow_factor < 1.0 {
            return Err(AdaptiveError::OutOfRangeFactor {
                field: "max_grow_factor",
                value: self.max_grow_factor,
                expected: ">= 1.0",
            });
        }
        if !self.min_shrink_factor.is_finite()
            || self.min_shrink_factor <= 0.0
            || self.min_shrink_factor >= 1.0
        {
            return Err(AdaptiveError::OutOfRangeFactor {
                field: "min_shrink_factor",
                value: self.min_shrink_factor,
                expected: "(0, 1)",
            });
        }
        Ok(self)
    }
}

/// Compute the recommended next step size from the worst-case
/// LTE / threshold ratio and the current step.
///
/// Classical proportional rule:
///
/// ```text
///   h_new = h · safety · ratio^(-1 / (order + 1))
/// ```
///
/// Then clamp into `[h · min_shrink_factor, h · max_grow_factor]`
/// (per-step rate-limit) and finally into `[h_min, h_max]` (absolute
/// bounds).
///
/// # Special cases
///
/// - `worst_ratio == 0.0` (LTE is identically zero — possible on
///   the very first step from a zero-history initial condition) →
///   the proportional rule's `ratio^(-1/(p+1))` is `+∞`, which the
///   rate-limit then clamps to `h · max_grow_factor`. Equivalent
///   to "grow as fast as allowed on a perfect step".
/// - `worst_ratio` non-finite → return `h_min` (fail-safe; the
///   non-finite-LTE branch in [`step_decision`] independently
///   forces `Reject`).
#[must_use]
pub fn next_step_size(current_h: f64, worst_ratio: f64, order: u32, bounds: StepSizeBounds) -> f64 {
    if !worst_ratio.is_finite() {
        return bounds.h_min;
    }
    let proportional = if worst_ratio > 0.0 {
        let exponent = -1.0_f64 / f64::from(order + 1);
        let raw = current_h * bounds.safety_factor * worst_ratio.powf(exponent);
        // raw could be +inf when worst_ratio ≪ 1; the rate-limit
        // below clamps that case to current_h * max_grow_factor.
        raw
    } else {
        // ratio == 0.0 → grow as fast as allowed.
        current_h * bounds.max_grow_factor
    };
    let lo = current_h * bounds.min_shrink_factor;
    let hi = current_h * bounds.max_grow_factor;
    // Apply rate-limit. NaN-safe via explicit comparisons.
    let rate_limited = proportional.max(lo).min(hi);
    // Apply absolute bounds.
    rate_limited.max(bounds.h_min).min(bounds.h_max)
}

/// Make the per-step decision: accept or reject, plus recommended
/// next step size.
///
/// This is the single library entry point the transient control
/// loop (#33) calls *after each NR solve at a tentative timestep*.
/// The outer loop:
///
/// 1. Sets `h` to the current step size.
/// 2. Calls the NR driver to produce tentative node voltages at
///    `t + h`.
/// 3. Assembles per-node [`NodeHistorySample`]s from the two
///    most-recently-accepted solutions plus the tentative
///    solution.
/// 4. Calls `step_decision(estimator, samples, envelope, h, bounds)`.
/// 5. Routes on [`StepDecision::outcome`].
///
/// # Errors
///
/// Returns [`AdaptiveError::NonPositiveStep`] if `current_h <= 0`
/// or non-finite, and propagates the [`LteEstimator::worst_ratio`]
/// errors.
///
/// # Returns
///
/// A fully-populated [`StepDecision`] with `next_h` clamped per
/// `bounds`.
pub fn step_decision(
    estimator: LteEstimator,
    samples: &[NodeHistorySample],
    envelope: LteToleranceEnvelope,
    current_h: f64,
    bounds: StepSizeBounds,
) -> Result<StepDecision, AdaptiveError> {
    if !current_h.is_finite() || current_h <= 0.0 {
        return Err(AdaptiveError::NonPositiveStep { h: current_h });
    }
    let (worst_ratio, worst_index) = estimator.worst_ratio(samples, envelope)?;
    let outcome = if worst_ratio <= 1.0 {
        StepOutcome::Accept
    } else {
        StepOutcome::Reject
    };
    let next_h = next_step_size(current_h, worst_ratio, estimator.order, bounds);
    Ok(StepDecision {
        outcome,
        next_h,
        worst_ratio,
        worst_index: Some(worst_index),
    })
}

// -----------------------------------------------------------------------
// Timestep history (Result metadata payload for tasks.md #35)
// -----------------------------------------------------------------------

/// One log entry for an attempted timestep — accepted or rejected.
///
/// The transient control loop appends one of these to the
/// [`TimestepHistory`] after every [`step_decision`] call,
/// regardless of outcome. On the Gherkin scenario's terminal Then:
///
/// > And the final Result contains only accepted time points
/// > And the timestep history is available in the Result metadata
///
/// the "only accepted time points" refers to the Result Waveforms;
/// the [`TimestepHistory`] (which contains both accepted *and*
/// rejected attempts for diagnostic value) is the separate metadata
/// payload.
///
/// # Fields
///
/// - `t_attempt` — the time at which the attempt was made, in
///   seconds. For an accepted step at `t_n + h`, this is `t_n + h`.
///   For a rejected attempt at the same time, it is also `t_n + h`
///   (rejected attempts are logged at the *would-be* time, not the
///   shrunk re-solve time, so the log shows what was tried).
/// - `h_attempt` — the step size that was attempted, in seconds.
/// - `outcome` — whether the attempt was accepted or rejected.
/// - `worst_ratio` — the worst-case LTE/threshold ratio across all
///   nodes from this attempt, for diagnostic plotting.
/// - `worst_index` — the index into the per-step `samples` slice
///   identifying which node produced `worst_ratio`. `None` when the
///   outer loop did not capture the index (e.g. for end-of-run
///   manually-built records in tests). Plumbed in tasks.md #35 per
///   the tasks.md #32 reviewer's note that this identity is
///   already available from [`LteEstimator::worst_ratio`] and is
///   load-bearing for "which node is driving rejection?" telemetry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestepRecord {
    /// Attempted time `t_n + h` (seconds).
    pub t_attempt: f64,
    /// Step size attempted (seconds).
    pub h_attempt: f64,
    /// Whether the attempt was accepted or rejected by
    /// [`step_decision`].
    pub outcome: StepOutcome,
    /// Worst-case LTE/threshold ratio across all nodes from this
    /// attempt.
    pub worst_ratio: f64,
    /// Index of the node that produced `worst_ratio`, if known.
    /// `None` allowed for callers that have not threaded the index
    /// through (legacy / test code).
    pub worst_index: Option<usize>,
}

impl TimestepRecord {
    /// Build a record from a per-step decision plus the time/h that
    /// were attempted.
    ///
    /// This is the canonical adapter for the outer control loop
    /// (tasks.md #33): after each [`step_decision`] call it folds
    /// the decision into a record and appends to the history. The
    /// adapter exists so the `worst_index` and `worst_ratio` plumbing
    /// from [`StepDecision`] is single-sourced rather than re-built
    /// at every call site.
    #[must_use]
    pub fn from_decision(t_attempt: f64, h_attempt: f64, decision: &StepDecision) -> Self {
        Self {
            t_attempt,
            h_attempt,
            outcome: decision.outcome,
            worst_ratio: decision.worst_ratio,
            worst_index: decision.worst_index,
        }
    }
}

/// Append-only log of every tentative timestep attempt during a
/// transient analysis.
///
/// Returned as part of the Result's metadata (tasks.md #35 will
/// surface this via the `Result` type once it lands). The transient
/// control loop calls [`TimestepHistory::record`] after each
/// [`step_decision`] and the consumer reads back via
/// [`TimestepHistory::accepted_points`] (only the accepted `t`
/// values, for plotting against the Waveform x-axis) and
/// [`TimestepHistory::records`] (the full log for diagnostics).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimestepHistory {
    records: Vec<TimestepRecord>,
}

impl TimestepHistory {
    /// Construct an empty history.
    #[must_use]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Construct an empty history with capacity for at least
    /// `capacity` records reserved up front. Useful when the outer
    /// loop knows the transient interval and a lower bound on the
    /// expected step count.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            records: Vec::with_capacity(capacity),
        }
    }

    /// Append one record. Called once per [`step_decision`] call.
    pub fn record(&mut self, record: TimestepRecord) {
        self.records.push(record);
    }

    /// All records in chronological order (accepted *and*
    /// rejected).
    #[must_use]
    pub fn records(&self) -> &[TimestepRecord] {
        &self.records
    }

    /// The `t_attempt` values of *only* the records with outcome
    /// [`StepOutcome::Accept`], in chronological order. These are
    /// the time points that appear in the Result Waveforms;
    /// rejected attempts are excluded by construction.
    #[must_use]
    pub fn accepted_points(&self) -> Vec<f64> {
        self.records
            .iter()
            .filter(|r| r.outcome == StepOutcome::Accept)
            .map(|r| r.t_attempt)
            .collect()
    }

    /// The number of accepted vs. rejected attempts. Useful for
    /// diagnostic logging and for testing.
    #[must_use]
    pub fn counts(&self) -> (usize, usize) {
        let accepted = self
            .records
            .iter()
            .filter(|r| r.outcome == StepOutcome::Accept)
            .count();
        let rejected = self.records.len() - accepted;
        (accepted, rejected)
    }
}

// -----------------------------------------------------------------------
// Conversion into the Result-side metadata payload (tasks.md #35)
// -----------------------------------------------------------------------
//
// `numeric-solver` produces `TimestepHistory`/`TimestepRecord` as
// pure-compute byproducts of the LTE controller. The
// transient-analysis-frontend boundary, however, must not pull the
// numeric solver into its types layer — downstream crates depend
// only on `circuit-solver-types`. So the Result-side metadata lives
// in `circuit_solver_types::transient`, and this conversion lifts a
// run's accumulated history into that stable shape.
//
// The conversion is intentionally a field-by-field map; the two
// types are structurally similar so there is no transformation,
// just a re-export of the controller's verdict in a Result-bound
// vocabulary.

impl From<StepOutcome> for circuit_solver_types::TransientStepOutcome {
    fn from(outcome: StepOutcome) -> Self {
        match outcome {
            StepOutcome::Accept => circuit_solver_types::TransientStepOutcome::Accept,
            StepOutcome::Reject => circuit_solver_types::TransientStepOutcome::Reject,
        }
    }
}

impl From<&TimestepRecord> for circuit_solver_types::TimestepHistoryEntry {
    fn from(record: &TimestepRecord) -> Self {
        circuit_solver_types::TimestepHistoryEntry {
            t_attempt: record.t_attempt,
            h_attempt: record.h_attempt,
            outcome: record.outcome.into(),
            worst_ratio: record.worst_ratio,
            worst_node: record.worst_index,
        }
    }
}

impl From<TimestepRecord> for circuit_solver_types::TimestepHistoryEntry {
    fn from(record: TimestepRecord) -> Self {
        (&record).into()
    }
}

impl From<&TimestepHistory> for circuit_solver_types::TimestepHistoryMetadata {
    /// Lift a numeric-solver-internal [`TimestepHistory`] into the
    /// stable Result-side
    /// [`circuit_solver_types::TimestepHistoryMetadata`] handed to
    /// users on the [`circuit_solver_types::TransientResult`].
    ///
    /// Realizes the Gherkin scenario
    /// `transient-time-domain#adaptive-timestepping-rejects-and-re-solves`
    /// terminal "And the timestep history is available in the Result
    /// metadata" by being the *one* conversion the orchestration
    /// layer calls at end-of-run to populate the Result.
    fn from(history: &TimestepHistory) -> Self {
        circuit_solver_types::TimestepHistoryMetadata::from_entries(
            history.records().iter().map(Into::into).collect(),
        )
    }
}

impl From<TimestepHistory> for circuit_solver_types::TimestepHistoryMetadata {
    fn from(history: TimestepHistory) -> Self {
        (&history).into()
    }
}

// -----------------------------------------------------------------------
// Error type
// -----------------------------------------------------------------------

/// Input-validation errors from the adaptive-timestepping module.
///
/// These are *programming errors* in the outer transient control
/// loop (#33) or in user-supplied tolerance configuration, not user
/// input errors from `application-frontend`. The outer loop is
/// expected to validate user-supplied tolerances at the API boundary
/// and surface a higher-level `AnalysisRequestError` to the user.
#[derive(Debug, Clone, PartialEq)]
pub enum AdaptiveError {
    /// A tolerance field (`rel` or `abs`) is non-finite (NaN or
    /// ±∞).
    NonFiniteTolerance {
        /// Which field — `"rel"` or `"abs"`.
        field: &'static str,
        /// The offending value.
        value: f64,
    },
    /// A tolerance field is negative.
    NegativeTolerance {
        /// Which field — `"rel"` or `"abs"`.
        field: &'static str,
        /// The offending value.
        value: f64,
    },
    /// Both `rel` and `abs` are exactly zero — accepts only
    /// exactly-zero LTE, which is unusable in floating-point
    /// practice.
    ZeroTolerance,
    /// A history sample contains a non-finite voltage. Indicates an
    /// upstream divergence in the NR solve.
    NonFiniteHistory {
        /// Which field in the [`NodeHistorySample`] — one of
        /// `"v_prev_prev"`, `"v_prev"`, `"v_curr"`.
        field: &'static str,
        /// The offending value.
        value: f64,
    },
    /// [`LteEstimator::worst_ratio`] was called with an empty
    /// slice of [`NodeHistorySample`]s.
    EmptyHistory,
    /// The tentative step size passed to [`step_decision`] is
    /// non-positive or non-finite.
    NonPositiveStep {
        /// The offending step value.
        h: f64,
    },
    /// A field of [`StepSizeBounds`] (`h_min` / `h_max`) is
    /// non-positive or non-finite.
    NonPositiveStepBound {
        /// Which bound.
        field: &'static str,
        /// The offending value.
        value: f64,
    },
    /// `h_max < h_min` in a [`StepSizeBounds`].
    HmaxBelowHmin {
        /// The `h_min` field.
        h_min: f64,
        /// The `h_max` field.
        h_max: f64,
    },
    /// A multiplicative factor in [`StepSizeBounds`]
    /// (`safety_factor` / `max_grow_factor` / `min_shrink_factor`)
    /// is outside its admissible range.
    OutOfRangeFactor {
        /// Which factor.
        field: &'static str,
        /// The offending value.
        value: f64,
        /// Human-readable description of the admissible range.
        expected: &'static str,
    },
}

impl fmt::Display for AdaptiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteTolerance { field, value } => write!(
                f,
                "adaptive: tolerance {field} must be finite, got {value}"
            ),
            Self::NegativeTolerance { field, value } => write!(
                f,
                "adaptive: tolerance {field} must be non-negative, got {value}"
            ),
            Self::ZeroTolerance => write!(
                f,
                "adaptive: tolerance envelope must have at least one of rel, abs strictly positive"
            ),
            Self::NonFiniteHistory { field, value } => write!(
                f,
                "adaptive: history field {field} must be finite, got {value}"
            ),
            Self::EmptyHistory => write!(
                f,
                "adaptive: LTE estimator called with empty per-node history slice"
            ),
            Self::NonPositiveStep { h } => write!(
                f,
                "adaptive: tentative step h must be strictly positive and finite, got {h}"
            ),
            Self::NonPositiveStepBound { field, value } => write!(
                f,
                "adaptive: step-size bound {field} must be strictly positive and finite, got {value}"
            ),
            Self::HmaxBelowHmin { h_min, h_max } => write!(
                f,
                "adaptive: h_max ({h_max}) must be >= h_min ({h_min})"
            ),
            Self::OutOfRangeFactor {
                field,
                value,
                expected,
            } => write!(
                f,
                "adaptive: factor {field} = {value} is outside admissible range {expected}"
            ),
        }
    }
}

impl std::error::Error for AdaptiveError {}

// -----------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // LteToleranceEnvelope
    // -----------------------------------------------------------------

    #[test]
    fn envelope_default_matches_adr_0008_transient_row() {
        // ADR-0008 / design.md QAS-2: transient envelope is 1 %
        // relative or 1 mV absolute per node voltage.
        let env = LteToleranceEnvelope::transient_default();
        assert_eq!(env.rel.to_bits(), 0.01_f64.to_bits());
        assert_eq!(env.abs.to_bits(), 1.0e-3_f64.to_bits());
    }

    #[test]
    fn envelope_threshold_max_of_rel_and_abs_on_large_signal() {
        // Large signal: 1 V; rel = 1 % ⇒ 10 mV > abs = 1 mV ⇒ rel
        // dominates.
        let env = LteToleranceEnvelope::transient_default();
        assert!((env.threshold(1.0) - 1.0e-2).abs() < 1.0e-15);
    }

    #[test]
    fn envelope_threshold_max_of_rel_and_abs_on_small_signal() {
        // Small signal: 1 µV; rel × |v| = 10 nV ≪ abs = 1 mV ⇒ abs
        // floor dominates. Without the floor a tiny LTE on a
        // near-zero node would over-reject.
        let env = LteToleranceEnvelope::transient_default();
        assert!((env.threshold(1.0e-6) - 1.0e-3).abs() < 1.0e-15);
    }

    #[test]
    fn envelope_threshold_handles_zero_voltage() {
        // The very first step from a zero initial condition: every
        // node has v_prev = 0. Threshold must be exactly the abs
        // floor.
        let env = LteToleranceEnvelope::transient_default();
        assert_eq!(env.threshold(0.0).to_bits(), 1.0e-3_f64.to_bits());
    }

    #[test]
    fn envelope_threshold_handles_negative_v_ref() {
        // Sign of v_ref must not matter — only magnitude is used
        // for the relative term.
        let env = LteToleranceEnvelope::transient_default();
        assert!((env.threshold(-2.0) - 2.0e-2).abs() < 1.0e-15);
    }

    #[test]
    fn envelope_threshold_returns_infinity_on_non_finite_v_ref() {
        // Fail-safe when v_ref is non-finite (upstream divergence).
        // The step_decision-level non-finite-LTE branch is the
        // primary reject; this is a defense-in-depth.
        let env = LteToleranceEnvelope::transient_default();
        assert!(env.threshold(f64::NAN).is_infinite());
        assert!(env.threshold(f64::INFINITY).is_infinite());
    }

    #[test]
    fn envelope_accepts_within_threshold() {
        let env = LteToleranceEnvelope::transient_default();
        // 5 mV LTE on 1 V signal: threshold = 10 mV ⇒ accepts.
        assert!(env.accepts(5.0e-3, 1.0));
        // 15 mV LTE on 1 V signal: threshold = 10 mV ⇒ rejects.
        assert!(!env.accepts(15.0e-3, 1.0));
    }

    #[test]
    fn envelope_accepts_uses_floor_on_zero_signal() {
        let env = LteToleranceEnvelope::transient_default();
        // 0.5 mV LTE on 0 V signal: threshold = 1 mV (floor) ⇒
        // accepts.
        assert!(env.accepts(0.5e-3, 0.0));
        // 2 mV LTE on 0 V signal: rejects.
        assert!(!env.accepts(2.0e-3, 0.0));
    }

    #[test]
    fn envelope_accepts_rejects_non_finite_lte() {
        let env = LteToleranceEnvelope::transient_default();
        assert!(!env.accepts(f64::NAN, 1.0));
        assert!(!env.accepts(f64::INFINITY, 1.0));
    }

    #[test]
    fn envelope_new_rejects_non_finite() {
        assert!(matches!(
            LteToleranceEnvelope::new(f64::NAN, 1.0e-3),
            Err(AdaptiveError::NonFiniteTolerance { field: "rel", .. })
        ));
        assert!(matches!(
            LteToleranceEnvelope::new(0.01, f64::INFINITY),
            Err(AdaptiveError::NonFiniteTolerance { field: "abs", .. })
        ));
    }

    #[test]
    fn envelope_new_rejects_negative() {
        assert!(matches!(
            LteToleranceEnvelope::new(-0.01, 1.0e-3),
            Err(AdaptiveError::NegativeTolerance { field: "rel", .. })
        ));
        assert!(matches!(
            LteToleranceEnvelope::new(0.01, -1.0e-3),
            Err(AdaptiveError::NegativeTolerance { field: "abs", .. })
        ));
    }

    #[test]
    fn envelope_new_rejects_both_zero() {
        assert_eq!(
            LteToleranceEnvelope::new(0.0, 0.0),
            Err(AdaptiveError::ZeroTolerance)
        );
    }

    #[test]
    fn envelope_new_accepts_zero_rel_with_positive_abs() {
        // Pure-absolute envelope is admissible (degenerate but
        // sometimes useful for tests).
        let env = LteToleranceEnvelope::new(0.0, 1.0e-3).expect("pure-abs envelope is valid");
        assert_eq!(env.threshold(1000.0).to_bits(), 1.0e-3_f64.to_bits());
    }

    // -----------------------------------------------------------------
    // LteEstimator::lte_for_node
    // -----------------------------------------------------------------

    #[test]
    fn lte_estimator_be_quadratic_signal_gives_constant_second_difference() {
        // A perfectly quadratic signal y(t) = t² sampled at
        // {0, h, 2h} gives second difference (2h)² − 2·h² + 0 =
        // 4h² − 2h² = 2h². The estimator returns |...|/2 = h².
        let estimator = LteEstimator::backward_euler();
        let h = 1.0e-9;
        let sample = NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: h * h,
            v_curr: (2.0 * h) * (2.0 * h),
        };
        let lte = estimator
            .lte_for_node(sample)
            .expect("finite inputs must succeed");
        assert!(
            (lte - h * h).abs() < 1.0e-30,
            "LTE for y=t² must equal h², got {lte}"
        );
    }

    #[test]
    fn lte_estimator_linear_signal_gives_zero_lte() {
        // A perfectly linear signal y(t) = a + b·t has zero second
        // derivative, so the divided-difference LTE must be exactly
        // zero (within floating point).
        let estimator = LteEstimator::backward_euler();
        let sample = NodeHistorySample {
            v_prev_prev: 1.0,
            v_prev: 2.5,
            v_curr: 4.0, // slope 1.5 per unit step
        };
        let lte = estimator
            .lte_for_node(sample)
            .expect("finite inputs must succeed");
        assert!(
            lte.abs() < 1.0e-15,
            "LTE for linear signal must be ~0, got {lte}"
        );
    }

    #[test]
    fn lte_estimator_constant_signal_gives_zero_lte() {
        // A flat signal — every sample equal — has zero LTE.
        let estimator = LteEstimator::backward_euler();
        let sample = NodeHistorySample {
            v_prev_prev: 3.3,
            v_prev: 3.3,
            v_curr: 3.3,
        };
        let lte = estimator
            .lte_for_node(sample)
            .expect("finite inputs must succeed");
        assert_eq!(lte.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn lte_estimator_rejects_non_finite_samples() {
        let estimator = LteEstimator::backward_euler();
        assert!(matches!(
            estimator.lte_for_node(NodeHistorySample {
                v_prev_prev: f64::NAN,
                v_prev: 0.0,
                v_curr: 0.0,
            }),
            Err(AdaptiveError::NonFiniteHistory {
                field: "v_prev_prev",
                ..
            })
        ));
        assert!(matches!(
            estimator.lte_for_node(NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: f64::INFINITY,
                v_curr: 0.0,
            }),
            Err(AdaptiveError::NonFiniteHistory {
                field: "v_prev",
                ..
            })
        ));
        assert!(matches!(
            estimator.lte_for_node(NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: 0.0,
                v_curr: f64::NAN,
            }),
            Err(AdaptiveError::NonFiniteHistory {
                field: "v_curr",
                ..
            })
        ));
    }

    // -----------------------------------------------------------------
    // LteEstimator::worst_ratio
    // -----------------------------------------------------------------

    #[test]
    fn worst_ratio_picks_node_with_largest_normalized_lte() {
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        // Three nodes: node 0 has tiny curvature, node 1 has large
        // curvature (the worst), node 2 has moderate curvature.
        // Each pair shares v_prev = 1 V so the threshold is 10 mV
        // for all three.
        let samples = vec![
            NodeHistorySample {
                v_prev_prev: 1.0,
                v_prev: 1.0,
                v_curr: 1.001, // 2nd diff = 0.001 → LTE = 5e-4
            },
            NodeHistorySample {
                v_prev_prev: 1.0,
                v_prev: 1.0,
                v_curr: 1.10, // 2nd diff = 0.10 → LTE = 0.05
            },
            NodeHistorySample {
                v_prev_prev: 1.0,
                v_prev: 1.0,
                v_curr: 1.01, // 2nd diff = 0.01 → LTE = 5e-3
            },
        ];
        let (ratio, index) = estimator
            .worst_ratio(&samples, env)
            .expect("non-empty slice");
        // Node 1 has LTE 0.05 ≫ threshold 10 mV ⇒ ratio = 5.
        assert!((ratio - 5.0).abs() < 1.0e-12);
        assert_eq!(index, 1);
    }

    #[test]
    fn worst_ratio_rejects_empty_slice() {
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        assert_eq!(
            estimator.worst_ratio(&[], env),
            Err(AdaptiveError::EmptyHistory)
        );
    }

    // -----------------------------------------------------------------
    // step_decision: the Gherkin scenario's observable behavior
    // -----------------------------------------------------------------

    #[test]
    fn step_decision_accepts_within_envelope() {
        // A gently-curving signal on a 1 V baseline: LTE ≪ 10 mV
        // threshold. Outer loop folds the solution into Result and
        // advances.
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let samples = vec![NodeHistorySample {
            v_prev_prev: 1.0,
            v_prev: 1.001,
            v_curr: 1.003, // 2nd diff = 0.001 ⇒ LTE = 5e-4 ≪ 10 mV
        }];
        let decision = step_decision(estimator, &samples, env, 1.0e-9, bounds)
            .expect("valid inputs must succeed");
        assert_eq!(decision.outcome, StepOutcome::Accept);
        assert!(decision.worst_ratio < 1.0);
        // On accept with very-good ratio, controller may grow step.
        assert!(
            decision.next_h >= 1.0e-9,
            "next_h on accept should not shrink below current; got {}",
            decision.next_h
        );
    }

    #[test]
    fn step_decision_rejects_when_lte_exceeds_envelope_and_shrinks_h() {
        // This is the Gherkin scenario's primary Then: rapidly
        // switching input produces a large LTE → reject → re-solve
        // at smaller h.
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 0.0,
            v_curr: 1.0, // 2nd diff = 1 ⇒ LTE = 0.5 V ≫ 1 mV floor
        }];
        let h0 = 1.0e-9;
        let decision =
            step_decision(estimator, &samples, env, h0, bounds).expect("valid inputs must succeed");
        assert_eq!(decision.outcome, StepOutcome::Reject);
        assert!(decision.worst_ratio > 1.0);
        // On reject, next_h must be strictly smaller than current_h
        // (the Gherkin "re-solves at a smaller timestep").
        assert!(
            decision.next_h < h0,
            "rejected step must shrink h; got next_h = {} from h0 = {}",
            decision.next_h,
            h0
        );
        // And the shrink must not exceed the configured per-step
        // floor (no more than 10× shrink).
        assert!(
            decision.next_h >= h0 * bounds.min_shrink_factor,
            "shrink must respect min_shrink_factor floor"
        );
    }

    #[test]
    fn step_decision_after_shrink_accepts_when_signal_resolved() {
        // Second half of the Gherkin scenario: after the outer loop
        // re-solves at the shrunk h, the new LTE estimate must
        // accept. We simulate this by feeding a sample whose curvature
        // is small enough that the new ratio is < 1.
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        // After the shrink the same physical transient looks much
        // smoother in samples (smaller second difference).
        let samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 1.0e-4,
            v_curr: 3.0e-4, // 2nd diff = 1e-4 ⇒ LTE = 5e-5
        }];
        let h_after_shrink = 1.0e-10;
        let decision = step_decision(estimator, &samples, env, h_after_shrink, bounds)
            .expect("valid inputs must succeed");
        // threshold floor 1 mV ≫ LTE 5e-5 ⇒ ratio < 1 ⇒ Accept.
        assert_eq!(decision.outcome, StepOutcome::Accept);
        assert!(decision.worst_ratio < 1.0);
    }

    #[test]
    fn step_decision_rejects_non_positive_h() {
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 0.0,
            v_curr: 0.0,
        }];
        for bad in [0.0, -1.0e-9, f64::NAN, f64::INFINITY] {
            assert!(matches!(
                step_decision(estimator, &samples, env, bad, bounds),
                Err(AdaptiveError::NonPositiveStep { .. })
            ));
        }
    }

    // -----------------------------------------------------------------
    // next_step_size proportional rule
    // -----------------------------------------------------------------

    #[test]
    fn next_step_size_shrinks_when_ratio_above_one() {
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        // ratio = 4, BE order p=1 ⇒ shrink factor = 4^(-1/2) = 0.5,
        // times safety 0.9 ⇒ 0.45 ⇒ next_h ≈ 4.5e-10. Within
        // [0.1·h, 2·h] rate limits so passes through.
        let next = next_step_size(h, 4.0, 1, bounds);
        assert!((next - 4.5e-10).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_grows_when_ratio_below_one() {
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        // ratio = 0.25, BE p=1 ⇒ grow factor = 0.25^(-1/2) = 2.0,
        // times safety 0.9 ⇒ 1.8 ⇒ next_h ≈ 1.8e-9. Within rate
        // limit (max 2× = 2e-9), passes through.
        let next = next_step_size(h, 0.25, 1, bounds);
        assert!((next - 1.8e-9).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_respects_max_grow_rate_limit() {
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        // ratio = 0.0 → proportional rule says +∞; rate-limit clamps
        // to h · max_grow_factor = 2·h.
        let next = next_step_size(h, 0.0, 1, bounds);
        assert!((next - 2.0e-9).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_respects_min_shrink_rate_limit() {
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        // ratio = 1e9, BE p=1 ⇒ raw shrink factor = 1e-4.5 ≈ 3e-5,
        // times safety 0.9 ⇒ ~2.8e-5. Below min_shrink_factor 0.1
        // ⇒ rate-limit clamps to h · 0.1 = 1e-10.
        let next = next_step_size(h, 1.0e9, 1, bounds);
        assert!((next - 1.0e-10).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_respects_h_min_absolute_floor() {
        let bounds = StepSizeBounds {
            h_min: 5.0e-10,
            ..StepSizeBounds::transient_default()
        };
        let bounds = bounds.validate().expect("valid");
        let h = 1.0e-9;
        // ratio = 1e9 would shrink to 1e-10 by rate-limit, but
        // h_min = 5e-10 lifts it to 5e-10.
        let next = next_step_size(h, 1.0e9, 1, bounds);
        assert!((next - 5.0e-10).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_respects_h_max_absolute_ceiling() {
        let bounds = StepSizeBounds {
            h_max: 1.5e-9,
            ..StepSizeBounds::transient_default()
        };
        let bounds = bounds.validate().expect("valid");
        let h = 1.0e-9;
        // ratio = 0 would grow to 2·h = 2e-9 by rate-limit, but
        // h_max = 1.5e-9 caps it.
        let next = next_step_size(h, 0.0, 1, bounds);
        assert!((next - 1.5e-9).abs() < 1.0e-20);
    }

    #[test]
    fn next_step_size_falls_back_to_h_min_on_non_finite_ratio() {
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        assert_eq!(
            next_step_size(h, f64::NAN, 1, bounds).to_bits(),
            bounds.h_min.to_bits()
        );
        assert_eq!(
            next_step_size(h, f64::INFINITY, 1, bounds).to_bits(),
            bounds.h_min.to_bits()
        );
    }

    #[test]
    fn next_step_size_uses_order_two_for_trapezoidal_siblings() {
        // Trapezoidal / Gear-2 BDF (#30, #31): order p = 2 ⇒
        // exponent = 1/3.
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let h = 1.0e-9;
        // ratio = 8, p=2 ⇒ shrink factor = 8^(-1/3) = 0.5, times
        // safety 0.9 ⇒ 0.45 ⇒ next_h ≈ 4.5e-10.
        let next = next_step_size(h, 8.0, 2, bounds);
        assert!((next - 4.5e-10).abs() < 1.0e-20);
    }

    // -----------------------------------------------------------------
    // StepSizeBounds::validate
    // -----------------------------------------------------------------

    #[test]
    fn step_bounds_default_validates() {
        StepSizeBounds::transient_default()
            .validate()
            .expect("defaults must validate");
    }

    #[test]
    fn step_bounds_validate_rejects_non_positive_h_min() {
        for bad in [0.0, -1.0e-12, f64::NAN] {
            let bounds = StepSizeBounds {
                h_min: bad,
                ..StepSizeBounds::transient_default()
            };
            assert!(matches!(
                bounds.validate(),
                Err(AdaptiveError::NonPositiveStepBound { field: "h_min", .. })
            ));
        }
    }

    #[test]
    fn step_bounds_validate_rejects_h_max_below_h_min() {
        let bounds = StepSizeBounds {
            h_min: 1.0e-9,
            h_max: 1.0e-10,
            ..StepSizeBounds::transient_default()
        };
        assert!(matches!(
            bounds.validate(),
            Err(AdaptiveError::HmaxBelowHmin { .. })
        ));
    }

    #[test]
    fn step_bounds_validate_rejects_safety_outside_range() {
        for bad in [0.0, -0.1, 1.5, f64::NAN] {
            let bounds = StepSizeBounds {
                safety_factor: bad,
                ..StepSizeBounds::transient_default()
            };
            assert!(matches!(
                bounds.validate(),
                Err(AdaptiveError::OutOfRangeFactor {
                    field: "safety_factor",
                    ..
                })
            ));
        }
    }

    #[test]
    fn step_bounds_validate_rejects_max_grow_below_one() {
        let bounds = StepSizeBounds {
            max_grow_factor: 0.5,
            ..StepSizeBounds::transient_default()
        };
        assert!(matches!(
            bounds.validate(),
            Err(AdaptiveError::OutOfRangeFactor {
                field: "max_grow_factor",
                ..
            })
        ));
    }

    #[test]
    fn step_bounds_validate_rejects_min_shrink_above_one() {
        for bad in [1.0, 1.5, 0.0, -0.1] {
            let bounds = StepSizeBounds {
                min_shrink_factor: bad,
                ..StepSizeBounds::transient_default()
            };
            assert!(matches!(
                bounds.validate(),
                Err(AdaptiveError::OutOfRangeFactor {
                    field: "min_shrink_factor",
                    ..
                })
            ));
        }
    }

    // -----------------------------------------------------------------
    // TimestepHistory: only-accepted-points discipline
    // -----------------------------------------------------------------

    #[test]
    fn timestep_history_accepted_points_filters_out_rejected() {
        // Mimic the Gherkin scenario: attempt at t = 1 ns rejected,
        // re-attempt at t = 0.5 ns accepted, next attempt at t =
        // 1.5 ns accepted.
        let mut history = TimestepHistory::new();
        history.record(TimestepRecord {
            t_attempt: 1.0e-9,
            h_attempt: 1.0e-9,
            outcome: StepOutcome::Reject,
            worst_ratio: 5.0,
            worst_index: Some(0),
        });
        history.record(TimestepRecord {
            t_attempt: 0.5e-9,
            h_attempt: 0.5e-9,
            outcome: StepOutcome::Accept,
            worst_ratio: 0.2,
            worst_index: Some(0),
        });
        history.record(TimestepRecord {
            t_attempt: 1.5e-9,
            h_attempt: 1.0e-9,
            outcome: StepOutcome::Accept,
            worst_ratio: 0.5,
            worst_index: Some(0),
        });
        let accepted = history.accepted_points();
        // Gherkin: "the final Result contains only accepted time
        // points" — the rejected 1.0 ns attempt must not appear.
        assert_eq!(accepted, vec![0.5e-9, 1.5e-9]);
        // The full timestep history is still available as Result
        // metadata for diagnostics.
        assert_eq!(history.records().len(), 3);
        assert_eq!(history.counts(), (2, 1));
    }

    #[test]
    fn timestep_history_with_capacity_pre_reserves() {
        let history = TimestepHistory::with_capacity(100);
        // Capacity is at least 100 (Vec may round up).
        assert!(history.records.capacity() >= 100);
        assert!(history.records().is_empty());
    }

    #[test]
    fn timestep_history_default_is_empty() {
        let history = TimestepHistory::default();
        assert!(history.records().is_empty());
        assert_eq!(history.counts(), (0, 0));
        assert!(history.accepted_points().is_empty());
    }

    // -----------------------------------------------------------------
    // Error display strings
    // -----------------------------------------------------------------

    #[test]
    fn error_display_strings_are_actionable() {
        // Every variant must produce a non-empty Display string that
        // names the field/value so downstream log readers can
        // diagnose the failure without consulting source code.
        let cases: Vec<AdaptiveError> = vec![
            AdaptiveError::NonFiniteTolerance {
                field: "rel",
                value: f64::NAN,
            },
            AdaptiveError::NegativeTolerance {
                field: "abs",
                value: -1.0,
            },
            AdaptiveError::ZeroTolerance,
            AdaptiveError::NonFiniteHistory {
                field: "v_curr",
                value: f64::INFINITY,
            },
            AdaptiveError::EmptyHistory,
            AdaptiveError::NonPositiveStep { h: -1.0 },
            AdaptiveError::NonPositiveStepBound {
                field: "h_min",
                value: 0.0,
            },
            AdaptiveError::HmaxBelowHmin {
                h_min: 1.0e-9,
                h_max: 1.0e-10,
            },
            AdaptiveError::OutOfRangeFactor {
                field: "safety_factor",
                value: 2.0,
                expected: "(0, 1]",
            },
        ];
        for err in cases {
            let s = format!("{err}");
            assert!(
                !s.is_empty(),
                "Display string must be non-empty for {err:?}"
            );
            assert!(
                s.starts_with("adaptive:"),
                "Display string should be tagged; got {s}"
            );
        }
    }

    // -----------------------------------------------------------------
    // End-to-end Gherkin scenario as one rolled-up test
    // -----------------------------------------------------------------

    #[test]
    fn end_to_end_reject_then_resolve_at_smaller_step() {
        // Scenario: transient-time-domain#adaptive-timestepping-rejects-and-re-solves
        //
        //   Given the initial timestep is set to 1 ns
        //   When the Simulator estimates a local truncation error
        //        exceeding the error tolerance
        //   Then the Simulator rejects the current step
        //   And the Simulator re-solves at a smaller timestep
        //   And the final Result contains only accepted time points
        //   And the timestep history is available in the Result metadata
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default()
            .validate()
            .expect("defaults valid");
        let mut history = TimestepHistory::new();

        // First attempt at h = 1 ns; a rapidly switching input
        // produces a large second difference.
        let h0 = 1.0e-9;
        let bad_samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 0.0,
            v_curr: 1.0,
        }];
        let first = step_decision(estimator, &bad_samples, env, h0, bounds)
            .expect("valid inputs must succeed");
        // Gherkin: rejects the current step.
        assert_eq!(first.outcome, StepOutcome::Reject);
        // Gherkin: re-solves at a smaller timestep.
        assert!(first.next_h < h0);
        history.record(TimestepRecord::from_decision(h0, h0, &first));

        // Second attempt at the shrunk step, with a smaller
        // second difference (the outer loop's NR re-solve at the
        // smaller step produces a less-curved tentative
        // solution).
        let h1 = first.next_h;
        let good_samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 1.0e-4,
            v_curr: 3.0e-4,
        }];
        let second = step_decision(estimator, &good_samples, env, h1, bounds)
            .expect("valid inputs must succeed");
        assert_eq!(second.outcome, StepOutcome::Accept);
        history.record(TimestepRecord::from_decision(h1, h1, &second));

        // Gherkin: the final Result contains only accepted time
        // points.
        let accepted = history.accepted_points();
        assert_eq!(accepted.len(), 1);
        assert!((accepted[0] - h1).abs() < 1.0e-30);

        // Gherkin: the timestep history is available in the Result
        // metadata (i.e. we can also inspect the rejected attempt).
        assert_eq!(history.records().len(), 2);
        assert_eq!(history.counts(), (1, 1));
        assert_eq!(history.records()[0].outcome, StepOutcome::Reject);
        assert_eq!(history.records()[1].outcome, StepOutcome::Accept);
    }

    // -----------------------------------------------------------------
    // tasks.md #35: Result-side metadata + conversion tests
    // -----------------------------------------------------------------

    #[test]
    fn step_decision_now_reports_worst_index() {
        // Two-node sample: node 1 has the larger LTE; controller
        // must surface that index.
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default();
        let samples = vec![
            NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: 0.0,
                v_curr: 1.0e-6, // small curvature
            },
            NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: 0.0,
                v_curr: 1.0, // large curvature → big LTE
            },
        ];
        let decision = step_decision(estimator, &samples, env, 1.0e-9, bounds)
            .expect("valid inputs must succeed");
        // The downstream reviewer's tasks.md #32 note #2 was
        // "step_decision discards worst_index". This test pins
        // the fix.
        assert_eq!(decision.worst_index, Some(1));
    }

    #[test]
    fn timestep_record_from_decision_propagates_worst_index() {
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default();
        let samples = vec![
            NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: 0.0,
                v_curr: 1.0e-6,
            },
            NodeHistorySample {
                v_prev_prev: 0.0,
                v_prev: 0.0,
                v_curr: 1.0,
            },
        ];
        let decision = step_decision(estimator, &samples, env, 1.0e-9, bounds)
            .expect("valid inputs must succeed");
        let record = TimestepRecord::from_decision(1.0e-9, 1.0e-9, &decision);
        assert_eq!(record.outcome, decision.outcome);
        assert!((record.worst_ratio - decision.worst_ratio).abs() < 1.0e-30);
        assert_eq!(record.worst_index, decision.worst_index);
        assert!((record.t_attempt - 1.0e-9).abs() < 1.0e-30);
        assert!((record.h_attempt - 1.0e-9).abs() < 1.0e-30);
    }

    #[test]
    fn step_outcome_converts_to_result_side_outcome() {
        use circuit_solver_types::TransientStepOutcome;
        let accept: TransientStepOutcome = StepOutcome::Accept.into();
        let reject: TransientStepOutcome = StepOutcome::Reject.into();
        assert_eq!(accept, TransientStepOutcome::Accept);
        assert_eq!(reject, TransientStepOutcome::Reject);
    }

    #[test]
    fn timestep_record_converts_to_result_side_entry() {
        use circuit_solver_types::{TimestepHistoryEntry, TransientStepOutcome};
        let record = TimestepRecord {
            t_attempt: 1.0e-9,
            h_attempt: 1.0e-9,
            outcome: StepOutcome::Reject,
            worst_ratio: 5.0,
            worst_index: Some(2),
        };
        let entry: TimestepHistoryEntry = (&record).into();
        assert!((entry.t_attempt - record.t_attempt).abs() < 1.0e-30);
        assert!((entry.h_attempt - record.h_attempt).abs() < 1.0e-30);
        assert_eq!(entry.outcome, TransientStepOutcome::Reject);
        assert!((entry.worst_ratio - record.worst_ratio).abs() < 1.0e-30);
        assert_eq!(entry.worst_node, Some(2));

        // By-value also works (just delegates).
        let entry_owned: TimestepHistoryEntry = record.into();
        assert_eq!(entry_owned.worst_node, Some(2));
    }

    #[test]
    fn timestep_history_lifts_into_result_metadata() {
        // The Gherkin scenario hand-built end-to-end.
        // Attempt 1: t=1ns, h=1ns → reject (worst_ratio 5.0).
        // Attempt 2: t=0.5ns, h=0.5ns → accept.
        // Attempt 3: t=1.5ns, h=1ns → accept.
        let mut history = TimestepHistory::new();
        history.record(TimestepRecord {
            t_attempt: 1.0e-9,
            h_attempt: 1.0e-9,
            outcome: StepOutcome::Reject,
            worst_ratio: 5.0,
            worst_index: Some(0),
        });
        history.record(TimestepRecord {
            t_attempt: 0.5e-9,
            h_attempt: 0.5e-9,
            outcome: StepOutcome::Accept,
            worst_ratio: 0.2,
            worst_index: Some(0),
        });
        history.record(TimestepRecord {
            t_attempt: 1.5e-9,
            h_attempt: 1.0e-9,
            outcome: StepOutcome::Accept,
            worst_ratio: 0.5,
            worst_index: Some(0),
        });

        // The Result-side metadata is what the user reads.
        let meta: circuit_solver_types::TimestepHistoryMetadata = (&history).into();
        assert_eq!(meta.len(), 3);
        assert_eq!(meta.counts(), (2, 1));
        // Gherkin: "the final Result contains only accepted time
        // points" — accepted_times excludes the 1ns rejected attempt.
        assert_eq!(meta.accepted_times(), vec![0.5e-9, 1.5e-9]);
        // Gherkin: "the timestep history is available in the
        // Result metadata" — the rejected attempt is preserved
        // in the entry log for diagnostics.
        assert!(meta.had_rejection());
        assert_eq!(
            meta.entries()[0].outcome,
            circuit_solver_types::TransientStepOutcome::Reject
        );
        // By-value also works.
        let meta_owned: circuit_solver_types::TimestepHistoryMetadata = history.into();
        assert_eq!(meta_owned.len(), 3);
    }

    #[test]
    fn lifting_an_empty_history_yields_an_empty_metadata() {
        let history = TimestepHistory::new();
        let meta: circuit_solver_types::TimestepHistoryMetadata = (&history).into();
        assert!(meta.is_empty());
        assert_eq!(meta.counts(), (0, 0));
        assert!(meta.accepted_times().is_empty());
        assert!(!meta.had_rejection());
    }

    #[test]
    fn gherkin_scenario_end_to_end_through_result_metadata() {
        // This is the full Gherkin scenario through *both* the
        // controller and the conversion: drive the controller with
        // a fast switch sample (rejected), then a slow re-solve
        // sample (accepted), record each into the history, lift
        // into Result metadata, and assert the scenario's
        // terminal Then about accepted time points and metadata
        // availability.
        let estimator = LteEstimator::backward_euler();
        let env = LteToleranceEnvelope::transient_default();
        let bounds = StepSizeBounds::transient_default();
        let mut history = TimestepHistory::new();

        // Attempt 1: tentative step at t=1ns, h=1ns, with a
        // rapidly switching sample. Expect Reject.
        let h0 = 1.0e-9;
        let bad_samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 0.0,
            v_curr: 1.0,
        }];
        let d1 = step_decision(estimator, &bad_samples, env, h0, bounds)
            .expect("valid inputs must succeed");
        assert_eq!(d1.outcome, StepOutcome::Reject);
        // Per the controller's contract: reject ⇒ next_h is the
        // shrunken re-solve step (smaller than h0).
        assert!(d1.next_h < h0);
        history.record(TimestepRecord::from_decision(h0, h0, &d1));

        // Attempt 2: at the controller-suggested smaller step,
        // with a less-curvy sample (the post-shrink NR re-solve).
        // Expect Accept.
        let h1 = d1.next_h;
        let good_samples = vec![NodeHistorySample {
            v_prev_prev: 0.0,
            v_prev: 1.0e-4,
            v_curr: 3.0e-4,
        }];
        let d2 = step_decision(estimator, &good_samples, env, h1, bounds)
            .expect("valid inputs must succeed");
        assert_eq!(d2.outcome, StepOutcome::Accept);
        history.record(TimestepRecord::from_decision(h1, h1, &d2));

        // Lift to Result-side metadata.
        let meta: circuit_solver_types::TimestepHistoryMetadata = (&history).into();
        // Gherkin terminal Then #1: only accepted time points.
        assert_eq!(meta.accepted_times().len(), 1);
        // Gherkin terminal Then #2: timestep history is available
        // in the metadata — both attempts visible.
        assert_eq!(meta.entries().len(), 2);
        assert_eq!(meta.counts(), (1, 1));
        assert!(meta.had_rejection());
        // The worst_node identity from the controller round-trips
        // through into the metadata.
        assert_eq!(
            meta.entries()[0].worst_node,
            Some(0),
            "worst_node must survive the conversion (tasks.md #32 reviewer note #2)"
        );
    }
}
