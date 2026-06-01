//! Scenario-level integration witness for
//! `transient-time-domain#adaptive-timestepping-rejects-and-re-solves`.
//!
//! Per the executable specification:
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
//! Position of this test in the implementation pipeline
//! =====================================================
//!
//! tasks.md slices the work for this scenario across four primitive
//! tasks that have already merged to trunk:
//!
//! - **#29** — Backward Euler companion models (`integration::backward_euler`).
//! - **#32** — Adaptive timestepping LTE estimator + step-decision
//!   controller (`integration::adaptive`).
//! - **#35** — `circuit_solver_types::TransientResult` envelope +
//!   `TimestepHistoryMetadata` lift, with `From<&TimestepHistory>`
//!   conversion landed in `numeric-solver`.
//! - **#14** — Pass-2 MNA assembly, **#15** sub-view extractor,
//!   **#23** faer complex LU — supporting primitives for the
//!   forthcoming full transient outer loop (**#33**).
//!
//! The reviewer of tasks.md #32 noted (`t_e1f5f0e3`,
//! verdict APPROVE WITH NOTES, note 5): the unit-level e2e test in
//! `integration::adaptive::tests` *hand-stipulates* the post-shrink
//! samples because that test's job is to pin the controller's local
//! arithmetic, not the full scenario. The true scenario witness
//! belongs at the integration-test boundary. **This file is that
//! witness.** It drives the controller in a closed loop against a
//! synthetic node-voltage trajectory that models the "Circuit with
//! rapidly switching inputs" Given clause, advancing per
//! `StepOutcome::Accept`, shrinking and re-solving per
//! `StepOutcome::Reject`, and asserts every Then clause directly
//! against the `TransientResult` the orchestration layer (tasks.md
//! #33) will eventually return.
//!
//! Why a synthetic trajectory rather than a full MNA solve
//! -------------------------------------------------------
//!
//! The Gherkin Given says "a Circuit with rapidly switching inputs"
//! — the *circuit* is the producer of fast-changing node voltages,
//! not the protagonist of the test. The scenario's observable
//! protagonists are (1) the controller's reject decision, (2) the
//! controller's recovery via smaller-h re-solve, (3) the Result's
//! filtering of rejected attempts out of the Waveform time axis, and
//! (4) the Result's preservation of the full attempt history in the
//! metadata. None of those four observables require a real MNA-NR
//! inner solve; they require *node voltages that change fast enough
//! to trip the LTE envelope at h=1 ns and slow enough to satisfy it
//! at a smaller h*. We supply those voltages directly as a function
//! of (t, h), so this test exercises the same `step_decision` /
//! `TimestepHistory` / `TransientResult` types the full outer loop
//! (tasks.md #33) will exercise, but without coupling the scenario
//! witness to MNA-#14 + NR-#17 + flatten-#6 + DC-#20 (none of which
//! are required to gate this scenario per its parent task fanout).
//!
//! This division of labor is structurally identical to the AC
//! scenario witness `tests/ac_lowpass_sweep.rs`: that test exercises
//! `FaerComplexSolver` against a hand-stamped 1×1 system, not the
//! full AC analysis control loop (#25). Both tests pin the
//! *publicly-visible* contract their scenario exposes, leaving the
//! deeper orchestration to its own dedicated task.
//!
//! ADR alignment
//! -------------
//!
//! - **ADR-0006** (Dual NR convergence) — vacuous; this is a
//!   Result-shape + controller-decision witness, no NR solve.
//! - **ADR-0007** (ZOH A/D boundary) — vacuous; no analog-digital
//!   boundary surface here.
//! - **ADR-0008** (max(rel, abs) tolerance envelope) — directly
//!   exercised: the test uses `LteToleranceEnvelope::transient_default()`,
//!   which encodes the QAS-2 row (`rel = 1 %`, `abs = 1 mV`), and
//!   asserts the controller actually rejects when the second
//!   difference puts the LTE above this envelope.
//! - **ADR-0009** (Topology checker) — vacuous; no topology surface.
//! - **ADR-0010** (Unstable v1 API) — implicitly exercised: the test
//!   is in `tests/`, so it consumes only public exports of
//!   `numeric-solver` and `circuit-solver-types`. If any of those
//!   public names are renamed before stabilization, this test pins
//!   them.

use circuit_solver_types::{
    NodeId, SimulationTime, TransientResult, TransientStepOutcome, Waveform,
};
use numeric_solver::{
    next_step_size, step_decision, LteEstimator, LteToleranceEnvelope, NodeHistorySample,
    StepDecision, StepOutcome, StepSizeBounds, TimestepHistory, TimestepRecord,
};

// -----------------------------------------------------------------------
// Synthetic "rapidly switching" node trajectory
// -----------------------------------------------------------------------

/// A logistic step centred at `T_SWITCH` with characteristic rise
/// time `TAU`. Smooth and infinitely differentiable, so the
/// second-difference LTE estimator produces well-defined values, but
/// the slope `dV/dt` is so steep around `T_SWITCH` that a 1 ns
/// timestep straddles the entire transition and the second
/// difference is large enough to violate the ADR-0008 transient
/// envelope (1 % rel / 1 mV abs).
///
/// Values are in volts; the asymptotic levels are 0 V at `t→-∞` and
/// 3.3 V at `t→+∞` (a representative CMOS rail step, mirroring the
/// 3.3 V example in `transient::tests::transient_result_carries_both_waveforms_and_metadata`).
const T_SWITCH_S: f64 = 1.5e-9; // 1.5 ns — the transition is centred *inside* the first attempted 1 ns step
const TAU_S: f64 = 5.0e-11; // 50 ps — much faster than 1 ns; the controller must shrink
const V_HIGH: f64 = 3.3;

/// Synthetic "node voltage at time `t`" for the rapidly-switching
/// circuit. The full transient outer loop (tasks.md #33) will
/// replace this with an actual NR-converged solve at every `t`;
/// from the controller's perspective the two are interchangeable.
fn node_voltage_at(t_seconds: f64) -> f64 {
    let x = (t_seconds - T_SWITCH_S) / TAU_S;
    // Numerically stable logistic.
    let sigmoid = if x >= 0.0 {
        let e = (-x).exp();
        1.0 / (1.0 + e)
    } else {
        let e = x.exp();
        e / (1.0 + e)
    };
    V_HIGH * sigmoid
}

// -----------------------------------------------------------------------
// Outer-loop driver: controller-in-the-loop adaptive integration
// -----------------------------------------------------------------------

/// One observed node — our synthetic switching input. Mirrors the
/// indexing the full outer loop (tasks.md #33) will use when it
/// passes per-node histories into the controller.
const NODE_INDEX: usize = 0;

/// Drive the adaptive controller from `t_start` to `t_stop` with an
/// initial step `h0`, recording every attempted step (accepted or
/// rejected). This is a *minimal* outer loop, tightly scoped to the
/// scenario's four Then-clauses; the full DC-or-UIC + NR + MNA loop
/// is tasks.md #33's responsibility.
///
/// Returns `(history, accepted_times_seconds, accepted_values_volts)`
/// suitable for direct construction of a `TransientResult`.
///
/// # Semantics matched against the controller's contract
///
/// - The first two accepted steps don't have two prior accepted
///   samples to feed the second-difference estimator, so they're
///   accepted unconditionally (per the
///   `integration::adaptive::NodeHistorySample` docstring's
///   "Sentinel for not enough history yet" paragraph and reviewer
///   note 1 on tasks.md #32: "first-two-steps sentinel is contract
///   not invariant — #33 implementer must respect").
/// - On `StepOutcome::Reject`, the *current attempt is discarded*:
///   the time pointer does not advance, and the next attempt
///   re-solves at `decision.next_h` (which the controller has
///   already shrunk via the proportional rule).
/// - On `StepOutcome::Accept`, the time pointer advances by the
///   *attempted* step (the one that was just accepted), and the
///   next step is the controller's `decision.next_h`.
fn drive_adaptive(t_start: f64, t_stop: f64, h0: f64) -> (TimestepHistory, Vec<f64>, Vec<f64>) {
    // Outer-loop safety: an unconditional cap. The proportional rule
    // is bounded so it cannot stall; this cap is a backstop so a
    // bug in the test cannot produce an infinite loop.
    const MAX_STEPS: usize = 10_000;

    assert!(t_stop > t_start && h0 > 0.0, "test setup");

    let estimator = LteEstimator::backward_euler();
    let envelope = LteToleranceEnvelope::transient_default();
    let bounds = StepSizeBounds::transient_default()
        .validate()
        .expect("transient_default bounds are valid");

    let mut history = TimestepHistory::new();
    let mut accepted_times = vec![t_start];
    let mut accepted_values = vec![node_voltage_at(t_start)];

    let mut t = t_start;
    let mut h = h0;

    for _ in 0..MAX_STEPS {
        if t >= t_stop {
            break;
        }
        // Clip the attempt to t_stop so we don't overshoot.
        let h_attempt = h.min(t_stop - t);
        let t_attempt = t + h_attempt;
        let v_curr = node_voltage_at(t_attempt);

        let decision = if accepted_times.len() >= 2 {
            // Enough history to estimate LTE — let the controller
            // make the decision.
            let v_prev = *accepted_values.last().expect("nonempty");
            let v_prev_prev = accepted_values[accepted_values.len() - 2];
            let sample = NodeHistorySample {
                v_prev_prev,
                v_prev,
                v_curr,
            };
            step_decision(estimator, &[sample], envelope, h_attempt, bounds)
                .expect("inputs are validated")
        } else {
            // First-two-steps contract: no LTE estimate possible,
            // unconditional accept, propose-next-step at the
            // proportional-rule's "perfect step" cap.
            let next_h = next_step_size(h_attempt, 0.0, estimator.order, bounds);
            StepDecision {
                outcome: StepOutcome::Accept,
                next_h,
                worst_ratio: 0.0,
                worst_index: Some(NODE_INDEX),
            }
        };

        history.record(TimestepRecord::from_decision(
            t_attempt, h_attempt, &decision,
        ));

        match decision.outcome {
            StepOutcome::Accept => {
                t = t_attempt;
                accepted_times.push(t_attempt);
                accepted_values.push(v_curr);
                h = decision.next_h;
            }
            StepOutcome::Reject => {
                // Do *not* advance t. Re-solve at the shrunk h.
                h = decision.next_h;
            }
        }
    }

    (history, accepted_times, accepted_values)
}

// -----------------------------------------------------------------------
// Scenario witness
// -----------------------------------------------------------------------

/// Gherkin: `adaptive-timestepping-rejects-and-re-solves`.
///
/// Drives a full adaptive-controller loop against the synthetic
/// switching trajectory above with an initial 1 ns step and asserts
/// every Then-clause of the scenario.
///
/// # Why this function is long
///
/// The Gherkin scenario is a *single* behavior — five Then-clauses
/// chained through one Given/When pair — and splitting the witness
/// into multiple `#[test]` functions would require either rerunning
/// the controller in each (slow, duplicative, and would make the
/// test diagnose-by-divergence between identical drivers) or sharing
/// state through a `OnceLock` (couples runtime ordering to test
/// ordering, which `cargo test` does not promise). The
/// `clippy::too_many_lines` lint is silenced here on the principle
/// that *one scenario = one witness*; the per-Then `assert!`s with
/// their comments are the scenario itself, not noise to extract.
#[allow(clippy::too_many_lines)]
#[test]
fn adaptive_timestepping_rejects_and_re_solves() {
    // Given: CircuitDesigner has constructed a Circuit with rapidly
    //        switching inputs.
    //   → modelled by `node_voltage_at`: a 0→3.3 V logistic with a
    //     50 ps rise time centred at t = 1.5 ns. The transition is
    //     deliberately straddled by the first 1 ns attempted step.
    // And:   the initial timestep is set to 1 ns.
    let h0 = 1.0e-9_f64;
    let t_start = 0.0_f64;
    let t_stop = 3.0e-9_f64; // run through the transition + slack

    // When: the Simulator estimates a local truncation error
    //       exceeding the error tolerance.
    //   → drive_adaptive runs the controller; the third attempted
    //     step (the first one with a two-point prior history) will
    //     straddle the transition and produce a second difference
    //     that puts the LTE far above the 1 mV absolute floor.
    let (history, accepted_times, accepted_values) = drive_adaptive(t_start, t_stop, h0);

    // Then 1: the Simulator rejects the current step.
    //   → at least one rejection must appear in the recorded history.
    let (accepted_count, rejected_count) = history.counts();
    assert!(
        rejected_count >= 1,
        "scenario contract: the controller must reject at least one step \
         when fed a 1 ns straddle of a 50 ps logistic transition. \
         counts = (accepted={accepted_count}, rejected={rejected_count})"
    );

    // Locate the first reject so we can verify Then 2 against the
    // *very next* attempt.
    let first_reject_idx = history
        .records()
        .iter()
        .position(|r| r.outcome == StepOutcome::Reject)
        .expect("rejection asserted above");
    let first_reject = history.records()[first_reject_idx];

    // Then 2: the Simulator re-solves at a smaller timestep.
    //   → the attempt immediately after the first reject must use a
    //     strictly smaller h_attempt at the same t_attempt point
    //     (the controller re-solves the *current* time, not the
    //     forward time, per the StepDecision::next_h semantic on
    //     Reject).
    let next_after_reject = *history
        .records()
        .get(first_reject_idx + 1)
        .expect("controller must attempt a re-solve after a reject");
    assert!(
        next_after_reject.h_attempt < first_reject.h_attempt,
        "scenario contract: the re-solve must use a smaller timestep. \
         rejected: t={:e}s h={:e}s ratio={:.3e}; \
         next attempt: t={:e}s h={:e}s",
        first_reject.t_attempt,
        first_reject.h_attempt,
        first_reject.worst_ratio,
        next_after_reject.t_attempt,
        next_after_reject.h_attempt,
    );
    // And the LTE/threshold ratio of the rejected attempt must
    // genuinely exceed 1.0 — proving "exceeding the error tolerance"
    // in the Gherkin When, not just a controller bug that rejected
    // a fine step.
    assert!(
        first_reject.worst_ratio > 1.0,
        "rejected attempt's LTE/threshold ratio must exceed 1.0 \
         (it was {:.3e})",
        first_reject.worst_ratio,
    );

    // Lift the controller's history into the user-facing Result
    // metadata via the public `From<&TimestepHistory>` impl that
    // tasks.md #35 landed. This is the exact conversion the
    // orchestration layer (tasks.md #33) will perform at end-of-run.
    let metadata: circuit_solver_types::TimestepHistoryMetadata = (&history).into();

    // Build the Result envelope using only-accepted time points,
    // mirroring what the orchestration layer does after each
    // `StepOutcome::Accept`.
    let waveform = Waveform::new(
        NodeId::new(1),
        accepted_times
            .iter()
            .map(|t_seconds| seconds_to_simulation_time(*t_seconds))
            .collect(),
        accepted_values.clone(),
    );
    let result = TransientResult::new(vec![waveform], metadata);

    // Then 3: the final Result contains only accepted time points.
    //   → the Waveform's time axis must have exactly `accepted_count
    //     + 1` entries (the +1 is the t_start seed at index 0,
    //     which the outer loop appends unconditionally before any
    //     controller decision).
    assert_eq!(
        result.waveforms[0].times.len(),
        accepted_count + 1,
        "scenario contract: Result Waveform time axis must contain \
         only accepted time points (got {} samples, expected {} \
         accepted + 1 seed)",
        result.waveforms[0].times.len(),
        accepted_count,
    );
    // And every Waveform sample's time must equal the t_attempt of
    // some `Accept` entry in the metadata (or the t_start seed).
    let accepted_t_attempts: Vec<f64> = result.timestep_history.accepted_times();
    assert_eq!(
        accepted_t_attempts.len(),
        accepted_count,
        "metadata.accepted_times() must match the controller's accept count",
    );
    for (i, recorded_t) in accepted_t_attempts.iter().enumerate() {
        // The Waveform time at index i+1 (skipping the t_start seed
        // at index 0) must match the i-th accepted-attempt time, to
        // within the picosecond grid of SimulationTime.
        //
        // Cast is bounded: `recorded_t` is in seconds in the test's
        // `[t_start, t_stop] = [0, 3] ns` interval, so
        // `recorded_t * 1e12` is at most 3000 — three orders of
        // magnitude smaller than i64::MAX. Clippy can't prove that
        // statically; we silence the truncation lint locally.
        #[allow(clippy::cast_possible_truncation)]
        let want_picoseconds = (*recorded_t * 1.0e12).round() as i64;
        let got = result.waveforms[0].times[i + 1].as_picoseconds();
        assert_eq!(
            got, want_picoseconds,
            "Waveform time axis must align with metadata.accepted_times() \
             at accepted-attempt index {i} (got {got} ps, want {want_picoseconds} ps)",
        );
    }
    // Crucially: the count of waveform samples must reflect *only*
    // accepted attempts (plus the t_start seed). The structural
    // proof of "rejected attempts are filtered out of the Waveform"
    // is the count equality already asserted above
    // (`result.waveforms[0].times.len() == accepted_count + 1`)
    // combined with the per-index alignment to
    // `metadata.accepted_times()` — a rejected attempt cannot enter
    // the Waveform because the orchestration layer never appended
    // it. We additionally verify the metadata-side invariant: every
    // Reject entry's `outcome` is preserved through the
    // `From<&TimestepHistory>` conversion, so a downstream
    // diagnostic consumer can always recover the full attempt log
    // even though the Waveform itself only shows accepted points.
    let metadata_reject_count = result
        .timestep_history
        .entries()
        .iter()
        .filter(|e| e.outcome == TransientStepOutcome::Reject)
        .count();
    assert_eq!(
        metadata_reject_count, rejected_count,
        "scenario contract: every rejected attempt must appear in \
         metadata.entries() with outcome=Reject. The Waveform itself \
         carries only accepted-attempt times (asserted via the \
         count + per-index alignment above)."
    );

    // Then 4: the timestep history is available in the Result metadata.
    //   → metadata.entries() must contain *both* accepted and
    //     rejected attempts, preserving the controller's full
    //     trajectory (per tasks.md #35's TimestepHistoryMetadata
    //     contract).
    assert_eq!(
        result.timestep_history.entries().len(),
        accepted_count + rejected_count,
        "metadata must record every attempt (accepted + rejected)",
    );
    assert_eq!(
        result.timestep_history.counts(),
        (accepted_count, rejected_count),
        "metadata.counts() must agree with the controller's tally",
    );
    assert!(
        result.timestep_history.had_rejection(),
        "metadata.had_rejection() must reflect Then 1 (a reject occurred)",
    );
    assert!(
        result.had_rejection(),
        "convenience flag on TransientResult must reflect the same",
    );

    // The first rejected entry visible to the user must carry the
    // same outcome we saw in the controller's history (tasks.md
    // #35's TransientStepOutcome::Reject).
    let first_metadata_reject = result
        .timestep_history
        .entries()
        .iter()
        .find(|e| e.outcome == TransientStepOutcome::Reject)
        .expect("at least one metadata entry must be Reject");
    assert!(
        first_metadata_reject.worst_ratio > 1.0,
        "the metadata-side worst_ratio of a rejected step must exceed 1.0 \
         (got {:.3e})",
        first_metadata_reject.worst_ratio,
    );
    // The reviewer of tasks.md #32 note 2 + tasks.md #35
    // chained-resolution: worst_node identity must survive the
    // From<&TimestepHistory> conversion. Verify here so a future
    // refactor that drops it gets caught.
    assert_eq!(
        first_metadata_reject.worst_node,
        Some(NODE_INDEX),
        "worst_node identity must be plumbed through the controller → \
         metadata conversion (tasks.md #32 reviewer note 2 / #35 \
         chained resolution)",
    );
}

/// Convert a real-valued time in seconds to the picosecond-grid
/// `SimulationTime`. Used to align the Waveform time axis with the
/// accepted-attempt times from `TimestepHistoryMetadata`.
fn seconds_to_simulation_time(t_seconds: f64) -> SimulationTime {
    // Picosecond grid: round-half-to-even via i64 cast on the
    // pre-rounded f64. The synthetic trajectory and attempted h
    // values are constructed in multiples of 50 ps so rounding is
    // exact in practice.
    //
    // Cast is bounded: callers pass `t_seconds` in the test's
    // `[t_start, t_stop] = [0, 3] ns` interval, so `t * 1e12` is at
    // most ~3000 — well inside i64. Clippy can't prove that
    // statically; we silence the truncation lint locally.
    #[allow(clippy::cast_possible_truncation)]
    let picoseconds = (t_seconds * 1.0e12).round() as i64;
    SimulationTime::from_picoseconds(picoseconds)
}

// -----------------------------------------------------------------------
// Companion assertions — guard against regressions in the supporting
// primitives the scenario relies on. Each one is small enough that a
// failure points unambiguously at the affected primitive rather than
// at the scenario test.
// -----------------------------------------------------------------------

/// Pin the ADR-0008 transient defaults: 1 % relative, 1 mV absolute.
/// If a future change loosens these without an ADR update, the
/// scenario witness would silently start accepting steps that the
/// conformance harness (ADR-0008) would later reject against the
/// golden reference.
#[test]
fn adr_0008_transient_defaults_match_qas2_row() {
    let env = LteToleranceEnvelope::transient_default();
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(env.rel, 0.01, "ADR-0008 / QAS-2: rel = 1 %");
        assert_eq!(env.abs, 1.0e-3, "ADR-0008 / QAS-2: abs = 1 mV");
    }
}

/// Pin the controller's reject-then-shrink semantic at the public
/// API level. If `step_decision::next_h` ever switches to "next
/// forward step" on Reject (a tempting cleanup if dual semantics get
/// split per tasks.md #32 reviewer note 3), the scenario witness's
/// Then 2 would fail in a confusing way. Catch it here with a
/// targeted assertion that uses only public types.
#[test]
fn controller_reject_recommends_smaller_h() {
    // Construct a history that yields a large second difference
    // relative to the absolute floor at a reference voltage of 0.
    let estimator = LteEstimator::backward_euler();
    let envelope = LteToleranceEnvelope::transient_default();
    let bounds = StepSizeBounds::transient_default().validate().unwrap();
    // v_prev_prev = 0, v_prev = 0, v_curr = 1.0 → 2nd diff = 1.0
    // → LTE = 0.5 V which dwarfs the 1 mV abs floor.
    let sample = NodeHistorySample {
        v_prev_prev: 0.0,
        v_prev: 0.0,
        v_curr: 1.0,
    };
    let h0 = 1.0e-9;
    let decision = step_decision(estimator, &[sample], envelope, h0, bounds).unwrap();
    assert_eq!(decision.outcome, StepOutcome::Reject);
    assert!(
        decision.next_h < h0,
        "Reject must recommend a strictly smaller next h (got next_h = {:e} \
         vs h0 = {:e})",
        decision.next_h,
        h0,
    );
    // And the worst-ratio must exceed 1 by construction.
    assert!(decision.worst_ratio > 1.0);
}
