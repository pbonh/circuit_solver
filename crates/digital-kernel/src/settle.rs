//! Delta-cycle combinational settling with oscillation detection.
//!
//! After the kernel processes events at a simulation time point, zero-delay
//! combinational logic may produce further net-value changes at the *same*
//! time. These are resolved through **delta cycles**: iterative evaluation
//! rounds that continue until the net state stabilizes (settles) or
//! oscillation is detected.
//!
//! # Delta-cycle model
//!
//! A delta cycle is a zero-delay evaluation step at the current simulation
//! time. Within a single real-time point T:
//!
//! 1. The kernel applies all scheduled events at T to the net state.
//! 2. The [`CombinationalEvaluator`] is invoked with the list of changed nets.
//! 3. If the evaluator produces new assignments, they are applied and the
//!    changed nets are fed back into step 2 (next delta cycle).
//! 4. This repeats until either:
//!    - The evaluator returns no new assignments (**settled**), or
//!    - The delta-cycle count exceeds [`SettleConfig::max_delta_cycles`]
//!      (**oscillation detected**), or
//!    - A previously-seen net-state snapshot recurs (**cycle detected**).
//!
//! # Oscillation detection
//!
//! Two mechanisms guarantee the kernel **never hangs**:
//!
//! - **Hard limit**: `max_delta_cycles` caps the number of delta cycles.
//!   Exceeding it immediately reports [`SettleOutcome::Oscillating`].
//! - **State hashing**: after each delta cycle, the full net-state vector
//!   is compared against previously observed snapshots. A repeat indicates
//!   an oscillating cycle (e.g., an inverter feeding back on itself).
//!
//! Both mechanisms are always active. The hard limit is the safety net; the
//! state-hashing check typically detects oscillation earlier.
//!
//! # Integration with `DigitalKernel`
//!
//! When a [`CombinationalEvaluator`] is installed on the kernel, `run_until`
//! processes events **one time point at a time** and invokes settling after
//! each time point's events are applied. Without an evaluator, `run_until`
//! behaves exactly as in task #11 (backward compatible).

use std::collections::HashSet;

use crate::event_queue::{LogicValue, NetId};
use crate::kernel::NetState;

// ---------------------------------------------------------------------------
// Combinational evaluator
// ---------------------------------------------------------------------------

/// A combinational evaluator that propagates net-value changes through
/// zero-delay logic.
///
/// Given the current [`NetState`] and a list of nets that changed in the
/// current delta cycle, the evaluator returns the set of new `(net, value)`
/// assignments to apply in the next delta cycle. Returning an empty `Vec`
/// signals that the combinational network has settled.
///
/// # Contract
///
/// - **Pure**: calling with the same inputs must produce the same outputs.
/// - **Causal**: the evaluator may only depend on net values present in
///   `net_state`; it must not introduce new dependencies.
/// - **No time advance**: all returned assignments are zero-delay — they
///   occur at the same simulation time as the triggering change.
///
/// # Implementing
///
/// Most circuits implement this by looking up the fan-out of each changed
/// net, evaluating the downstream gates, and returning the results. For
/// simple cases, [`FnEvaluator`] wraps a closure.
pub trait CombinationalEvaluator {
    /// Evaluate combinational logic given the current net state and
    /// the nets that changed in the current delta cycle.
    ///
    /// Returns a (possibly empty) list of `(NetId, LogicValue)` pairs
    /// to apply in the next delta cycle. The order of the list does not
    /// matter — each assignment is applied independently.
    fn evaluate(&self, net_state: &NetState, changed_nets: &[NetId]) -> Vec<(NetId, LogicValue)>;
}

/// A [`CombinationalEvaluator`] backed by a closure.
///
/// ```
/// # use digital_kernel::settle::{CombinationalEvaluator, FnEvaluator};
/// # use digital_kernel::{NetState, NetId, LogicValue};
/// let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
///     let mut out = Vec::new();
///     for &net in changed {
///         // Inverter: flip the bit
///         let v = ns.get(net);
///         if v == LogicValue::One { out.push((net, LogicValue::Zero)); }
///         else if v == LogicValue::Zero { out.push((net, LogicValue::One)); }
///     }
///     out
/// });
/// ```
pub struct FnEvaluator<F>
where
    F: Fn(&NetState, &[NetId]) -> Vec<(NetId, LogicValue)>,
{
    f: F,
}

impl<F> FnEvaluator<F>
where
    F: Fn(&NetState, &[NetId]) -> Vec<(NetId, LogicValue)>,
{
    /// Create a new closure-backed evaluator.
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> CombinationalEvaluator for FnEvaluator<F>
where
    F: Fn(&NetState, &[NetId]) -> Vec<(NetId, LogicValue)>,
{
    fn evaluate(&self, net_state: &NetState, changed_nets: &[NetId]) -> Vec<(NetId, LogicValue)> {
        (self.f)(net_state, changed_nets)
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Configuration for delta-cycle combinational settling.
///
/// The default `max_delta_cycles` of 100 is generous for real combinational
/// circuits (which typically settle in < 10 delta cycles) while still
/// guaranteeing termination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleConfig {
    /// Maximum number of delta cycles to attempt before declaring
    /// oscillation. This is the hard safety limit that guarantees
    /// the kernel **never hangs**.
    ///
    /// A value of 0 disables settling (the evaluator is never invoked).
    pub max_delta_cycles: u32,
}

impl Default for SettleConfig {
    fn default() -> Self {
        Self {
            max_delta_cycles: 100,
        }
    }
}

impl SettleConfig {
    /// Create a config with the given max delta cycles.
    #[must_use]
    pub fn with_max_delta_cycles(max: u32) -> Self {
        Self {
            max_delta_cycles: max,
        }
    }
}

// ---------------------------------------------------------------------------
// Settle outcome
// ---------------------------------------------------------------------------

/// Result of delta-cycle combinational settling at a simulation time point.
#[derive(Debug, Clone, PartialEq)]
pub enum SettleOutcome {
    /// The combinational network settled within the delta cycle limit.
    Settled {
        /// Number of delta cycles required to reach stability.
        /// 0 means no combinational evaluation was needed (no nets changed
        /// or no evaluator was configured).
        delta_cycles: u32,
    },

    /// Oscillation was detected — the network did not settle within the
    /// maximum delta cycle limit, or a previously-seen state recurred.
    Oscillating {
        /// Number of delta cycles attempted before detection.
        delta_cycles: u32,
        /// Nets whose values were still changing when oscillation was
        /// detected. Useful for diagnostic reports.
        oscillating_nets: Vec<NetId>,
    },
}

impl core::fmt::Display for SettleOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Settled { delta_cycles } => {
                write!(f, "settled after {delta_cycles} delta cycle(s)")
            }
            Self::Oscillating {
                delta_cycles,
                oscillating_nets,
            } => {
                write!(
                    f,
                    "oscillating after {delta_cycles} delta cycles, {} net(s) unstable",
                    oscillating_nets.len()
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Settle function
// ---------------------------------------------------------------------------

/// Run delta-cycle combinational settling.
///
/// After the kernel applies events at a given simulation time to the net
/// state, this function iteratively evaluates combinational logic until
/// the net state stabilizes or oscillation is detected.
///
/// # Arguments
///
/// * `net_state` — mutable reference to the net state to update in-place.
/// * `evaluator` — the combinational evaluator to invoke each delta cycle.
/// * `initially_changed` — nets whose values changed in the just-completed
///   event processing step. These seed the first delta cycle.
/// * `config` — settling configuration (max delta cycles).
///
/// # Returns
///
/// A [`SettleOutcome`] indicating whether the network settled or oscillated.
///
/// # Guarantees
///
/// This function **always terminates**: it will not hang regardless of the
/// evaluator's behavior, because the hard `max_delta_cycles` limit and
/// state-hashing check both guarantee termination.
pub fn settle(
    net_state: &mut NetState,
    evaluator: &dyn CombinationalEvaluator,
    initially_changed: Vec<NetId>,
    config: &SettleConfig,
) -> SettleOutcome {
    // Nothing changed → nothing to settle.
    if initially_changed.is_empty() {
        return SettleOutcome::Settled { delta_cycles: 0 };
    }

    // Zero limit → settling disabled, treat as settled with no cycles.
    if config.max_delta_cycles == 0 {
        return SettleOutcome::Settled { delta_cycles: 0 };
    }

    let mut changed_nets = initially_changed;
    let mut delta_cycles: u32 = 0;

    // Track previously seen net-state snapshots for cycle detection.
    // We snapshot after applying the initial events.
    let mut seen_states: HashSet<Vec<LogicValue>> = HashSet::new();
    seen_states.insert(snapshot_net_state(net_state));

    loop {
        // Ask the evaluator what should change given the current state
        // and the nets that just changed.
        let updates = evaluator.evaluate(net_state, &changed_nets);

        // No updates → settled.
        if updates.is_empty() {
            return SettleOutcome::Settled { delta_cycles };
        }

        delta_cycles += 1;

        // Hard limit exceeded → oscillation.
        if delta_cycles > config.max_delta_cycles {
            let oscillating_nets: Vec<NetId> = updates.iter().map(|(net, _)| *net).collect();
            return SettleOutcome::Oscillating {
                delta_cycles,
                oscillating_nets,
            };
        }

        // Apply updates, recording which nets actually changed value.
        let mut actually_changed: Vec<NetId> = Vec::new();
        for (net, value) in updates {
            let old = net_state.get(net);
            if old != value {
                net_state.set(net, value);
                actually_changed.push(net);
            }
        }

        // All updates were no-ops (same values already present) → settled.
        if actually_changed.is_empty() {
            return SettleOutcome::Settled { delta_cycles };
        }

        // State-hashing oscillation check: if we've seen this exact
        // net-state vector before, we're in a cycle.
        let snapshot = snapshot_net_state(net_state);
        if !seen_states.insert(snapshot) {
            return SettleOutcome::Oscillating {
                delta_cycles,
                oscillating_nets: actually_changed,
            };
        }

        changed_nets = actually_changed;
    }
}

/// Snapshot the net-state values as a Vec for cycle detection.
#[allow(clippy::cast_possible_truncation)]
fn snapshot_net_state(net_state: &NetState) -> Vec<LogicValue> {
    // We iterate over all nets by index. NetState stores values in a Vec
    // indexed by NetId::index(). We capture the full vector.
    (0..net_state.len())
        .map(|i| net_state.get(NetId::new(i as u32)))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a NetState from a slice of values.
    fn net_state_from(values: &[LogicValue]) -> NetState {
        let mut ns = NetState::new();
        for (i, &v) in values.iter().enumerate() {
            ns.set(NetId::new(i as u32), v);
        }
        ns
    }

    // -- SettleConfig tests --

    #[test]
    fn default_config_has_100_max_delta_cycles() {
        let cfg = SettleConfig::default();
        assert_eq!(cfg.max_delta_cycles, 100);
    }

    #[test]
    fn with_max_delta_cycles_overrides() {
        let cfg = SettleConfig::with_max_delta_cycles(50);
        assert_eq!(cfg.max_delta_cycles, 50);
    }

    // -- SettleOutcome display --

    #[test]
    fn settled_display() {
        let outcome = SettleOutcome::Settled { delta_cycles: 3 };
        assert_eq!(format!("{outcome}"), "settled after 3 delta cycle(s)");
    }

    #[test]
    fn oscillating_display() {
        let outcome = SettleOutcome::Oscillating {
            delta_cycles: 10,
            oscillating_nets: vec![NetId::new(0), NetId::new(1)],
        };
        let s = format!("{outcome}");
        assert!(s.contains("oscillating after 10 delta cycles"));
        assert!(s.contains("2 net(s) unstable"));
    }

    // -- settle() basic tests --

    #[test]
    fn settle_no_changes_returns_settled_zero() {
        let mut ns = NetState::new();
        let eval = FnEvaluator::new(|_, _| vec![]);
        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 0 });
    }

    #[test]
    fn settle_zero_limit_returns_settled_zero() {
        let mut ns = net_state_from(&[LogicValue::One]);
        let eval = FnEvaluator::new(|_, _| vec![(NetId::new(0), LogicValue::Zero)]);
        let cfg = SettleConfig::with_max_delta_cycles(0);
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 0 });
    }

    #[test]
    fn settle_immediately_stable() {
        // Evaluator returns no updates → settles in 0 delta cycles.
        let mut ns = net_state_from(&[LogicValue::One]);
        let eval = FnEvaluator::new(|_, _| vec![]);
        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 0 });
    }

    #[test]
    fn settle_one_delta_cycle() {
        // Net 0 changes to One; evaluator propagates to net 1 as Zero;
        // net 1 was Unknown, so it changes; next cycle evaluator returns
        // empty → settled in 1 delta cycle.
        let mut ns = NetState::with_nets(2);
        ns.set(NetId::new(0), LogicValue::One);
        // net 1 = Unknown

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) && ns.get(NetId::new(0)) == LogicValue::One {
                    out.push((NetId::new(1), LogicValue::Zero));
                }
            }
            out
        });

        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 1 });
        assert_eq!(ns.get(NetId::new(1)), LogicValue::Zero);
    }

    #[test]
    fn settle_cascading_two_delta_cycles() {
        // Net 0 → One; evaluator: net 0 → net 1 (Zero); net 1 → net 2 (One).
        // Cycle 1: net 1 changes to Zero.
        // Cycle 2: net 2 changes to One (because net 1 is Zero).
        // Cycle 3: evaluator returns empty → settled.
        let mut ns = NetState::with_nets(3);
        ns.set(NetId::new(0), LogicValue::One);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    out.push((NetId::new(1), LogicValue::Zero));
                }
                if net == NetId::new(1) && ns.get(NetId::new(1)) == LogicValue::Zero {
                    out.push((NetId::new(2), LogicValue::One));
                }
            }
            out
        });

        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 2 });
        assert_eq!(ns.get(NetId::new(0)), LogicValue::One);
        assert_eq!(ns.get(NetId::new(1)), LogicValue::Zero);
        assert_eq!(ns.get(NetId::new(2)), LogicValue::One);
    }

    // -- Oscillation detection tests --

    #[test]
    fn oscillation_detected_by_state_hash() {
        // Self-feeding inverter on net 0: evaluator always flips net 0.
        // State: X → 1 → 0 → 1 → 0 → ... (oscillates).
        // After the initial change to One, the evaluator flips it to Zero,
        // then back to One — which is a state we've seen → oscillation.
        let mut ns = NetState::with_nets(1);
        ns.set(NetId::new(0), LogicValue::One);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                let v = ns.get(net);
                let new_v = match v {
                    LogicValue::One => LogicValue::Zero,
                    LogicValue::Zero => LogicValue::One,
                    _ => LogicValue::One,
                };
                out.push((net, new_v));
            }
            out
        });

        let cfg = SettleConfig::with_max_delta_cycles(1000);
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        match outcome {
            SettleOutcome::Oscillating {
                delta_cycles,
                oscillating_nets,
            } => {
                // Should detect after 2 delta cycles (1→0, then 0→1 which
                // returns to the initial state).
                assert!(
                    delta_cycles <= 3,
                    "expected early detection, got {delta_cycles}"
                );
                assert!(oscillating_nets.contains(&NetId::new(0)));
            }
            SettleOutcome::Settled { .. } => {
                panic!("self-feeding inverter should oscillate, not settle");
            }
        }
    }

    #[test]
    fn oscillation_detected_by_hard_limit() {
        // Self-feeding inverter with a very low max_delta_cycles so
        // the hard limit fires before the state-hash check would.
        // With max=1: cycle 1 OK (1 > 1 false), cycle 2 triggers (2 > 1 true).
        let mut ns = NetState::with_nets(1);
        ns.set(NetId::new(0), LogicValue::One);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                let v = ns.get(net);
                let new_v = match v {
                    LogicValue::One => LogicValue::Zero,
                    LogicValue::Zero => LogicValue::One,
                    _ => LogicValue::One,
                };
                out.push((net, new_v));
            }
            out
        });

        let cfg = SettleConfig::with_max_delta_cycles(1);
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        match outcome {
            SettleOutcome::Oscillating {
                delta_cycles,
                oscillating_nets,
            } => {
                // Hard limit should fire at delta_cycles=2 (which exceeds max=1).
                assert_eq!(delta_cycles, 2);
                assert!(oscillating_nets.contains(&NetId::new(0)));
            }
            SettleOutcome::Settled { .. } => {
                panic!("self-feeding inverter with low limit should oscillate, not settle");
            }
        }
    }

    #[test]
    fn no_op_updates_settle_immediately() {
        // Evaluator returns updates that don't actually change any values.
        let mut ns = net_state_from(&[LogicValue::One]);
        let eval = FnEvaluator::new(|_, _| vec![(NetId::new(0), LogicValue::One)]);
        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 1 });
    }

    #[test]
    fn settle_preserves_unrelated_nets() {
        // Net 0 changes; evaluator updates net 1; net 2 should stay Unknown.
        let mut ns = NetState::with_nets(3);
        ns.set(NetId::new(0), LogicValue::One);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) && ns.get(NetId::new(0)) == LogicValue::One {
                    out.push((NetId::new(1), LogicValue::Zero));
                }
            }
            out
        });

        let cfg = SettleConfig::default();
        let _ = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(ns.get(NetId::new(2)), LogicValue::Unknown);
    }

    #[test]
    fn settle_with_four_valued_logic() {
        // Test X and Z propagation through the evaluator.
        let mut ns = NetState::with_nets(2);
        ns.set(NetId::new(0), LogicValue::Unknown);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    let v = ns.get(NetId::new(0));
                    // Pass X through as X on net 1.
                    out.push((NetId::new(1), v));
                }
            }
            out
        });

        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 1 });
        assert_eq!(ns.get(NetId::new(1)), LogicValue::Unknown);
    }

    #[test]
    fn settle_high_impedance_propagation() {
        let mut ns = NetState::with_nets(2);
        ns.set(NetId::new(0), LogicValue::HighImpedance);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    out.push((NetId::new(1), ns.get(NetId::new(0))));
                }
            }
            out
        });

        let cfg = SettleConfig::default();
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        assert_eq!(outcome, SettleOutcome::Settled { delta_cycles: 1 });
        assert_eq!(ns.get(NetId::new(1)), LogicValue::HighImpedance);
    }

    #[test]
    fn oscillation_report_contains_oscillating_nets() {
        // Two nets oscillating: net 0 flips itself.
        let mut ns = NetState::with_nets(2);
        ns.set(NetId::new(0), LogicValue::One);

        let eval = FnEvaluator::new(|ns: &NetState, changed: &[NetId]| {
            let mut out = vec![];
            for &net in changed {
                if net == NetId::new(0) {
                    let v = ns.get(net);
                    out.push((
                        net,
                        if v == LogicValue::One {
                            LogicValue::Zero
                        } else {
                            LogicValue::One
                        },
                    ));
                }
            }
            out
        });

        let cfg = SettleConfig::with_max_delta_cycles(100);
        let outcome = settle(&mut ns, &eval, vec![NetId::new(0)], &cfg);
        if let SettleOutcome::Oscillating {
            oscillating_nets, ..
        } = outcome
        {
            assert!(oscillating_nets.contains(&NetId::new(0)));
        } else {
            panic!("expected Oscillating");
        }
    }
}
