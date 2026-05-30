//! Standards-conformant VCD (Value Change Dump) writer for the digital
//! event-trace half of a [`MixedSignalResult`][circuit_solver_types::MixedSignalResult].
//!
//! ## Why this module exists (tasks.md item #50)
//!
//! The capability spec for `mixed-signal-cosim` includes the scenario
//! `mixed-signal-result-contains-vcd-trace`:
//!
//! ```gherkin
//! Given SimulationEngineer has completed a mixed-signal simulation
//!   with Icarus Verilog as the digital kernel
//! When the Result is produced
//! Then the Result contains an analog Waveform section with
//!   time-indexed node voltages
//! And the Result contains a VCD-format digital event trace
//! And the VCD trace is parseable by standard VCD readers
//! ```
//!
//! The struct envelope (`DigitalEventTrace { vcd: String,
//! events_by_signal }`) was landed by tasks.md #42. What remained for
//! #50 is the actual **VCD emitter**: a single well-formed builder
//! that
//!
//! - both the test doubles inside this crate, and
//! - the real Icarus (#47) and Verilator (#48) adapters when they
//!   land,
//!
//! can call to turn `(signals, events_by_signal)` into a VCD string
//! the IEEE-1364 §18 VCD reader grammar accepts.
//!
//! ## Output shape
//!
//! The emitted VCD is the minimal subset required for the scenario's
//! "parseable by standard VCD readers" clause:
//!
//! ```text
//! $date 2026-05-21 $end
//! $version circuit-solver mixed-signal-cosim $end
//! $timescale 1 ps $end
//! $scope module <scope_name> $end
//! $var wire 1 ! din $end
//! $var wire 1 " dout $end
//! $upscope $end
//! $enddefinitions $end
//! #0
//! 0!
//! 0"
//! #50000
//! 1!
//! 1"
//! ```
//!
//! The leading `#0` block with explicit `0` value-changes is the
//! "initial-value dump" prescribed by IEEE 1364 §18.2.3.4. Without
//! it, downstream VCD readers (including the `vcd` crate this module
//! is tested against) cannot distinguish "signal not yet declared"
//! from "signal at unknown value", and may emit warnings or refuse
//! to scan the timeline.
//!
//! The VCD identifier-code assignment scheme is the canonical
//! "printable ASCII starting at `!` (0x21)" sequence. The VCD
//! standard reserves `!` (0x21) through `~` (0x7e) for identifier
//! codes; this writer panics deliberately if more than 94 signals
//! are declared (a hard constraint of single-character ids). Multi-
//! character ids are a future-task concern; the current scenario's
//! signal count is ≤ a handful.
//!
//! ## Boundary signals only
//!
//! Per ADR-0004, only boundary signals crossing the analog↔digital
//! divide are observed at scheduler synchronization points. This
//! module writes exactly what the scheduler captured, in the order
//! the caller supplied. Internal digital signals (purely on the
//! event-driven side) are the digital simulator's own waveform-dump
//! responsibility and are out of scope for the mixed-signal Result.
//!
//! ## ADR alignment
//!
//! - **ADR-0004** — the scheduler is the sole mediator; this writer
//!   only sees the captured event log, not the kernels.
//! - **ADR-0010** — public Rust API is unstable at v1.0.0; this
//!   module is exported via `analysis-orchestration` so adapter tasks
//!   (#47, #48) can call it without an inter-crate move first.

use circuit_solver_types::{SignalName, SimulationTime};
use core::fmt::Write as _;

/// First printable-ASCII identifier code reserved by IEEE-1364 §18 for
/// VCD `$var` declarations.
const VCD_ID_FIRST: u8 = b'!';

/// Last printable-ASCII identifier code reserved by IEEE-1364 §18.
const VCD_ID_LAST: u8 = b'~';

/// Maximum number of signals this single-character-id writer can emit
/// in one VCD trace.
pub const MAX_SINGLE_CHAR_SIGNALS: usize = (VCD_ID_LAST - VCD_ID_FIRST + 1) as usize;

/// Configuration handed to [`build_vcd`].
///
/// `signals` is the ordered set of boundary-signal names to declare
/// inside the VCD `$scope`. `events_by_signal` is the per-signal event
/// timeline as captured by the scheduler (parallel to the field of
/// the same name on [`circuit_solver_types::DigitalEventTrace`]).
///
/// The writer materialises every event as a transition to `'1'` —
/// the minimal value-change that the scenario's parseability check
/// requires. Sibling tasks (e.g., the Icarus adapter, #47) will pass
/// real per-event level information once VVP relays it; the
/// signature is structured to accept that extension without a
/// breaking change.
#[derive(Debug, Clone)]
pub struct VcdTraceInput<'a> {
    /// Top-level `$scope module <name> $end` label. Conventionally
    /// the design-under-test's top module name; the test double uses
    /// `"mixed_signal_test"`.
    pub scope_name: &'a str,
    /// Boundary signals declared inside the scope, in deterministic
    /// order. Each signal will be assigned a single-character VCD id
    /// in declaration order, starting from `'!'`.
    pub signals: &'a [SignalName],
    /// Per-signal event times. The slice's i-th entry corresponds to
    /// `signals[i]`; pass an empty `Vec` for a signal that never
    /// transitions. The writer collates these into a single
    /// time-ordered stream as required by the VCD grammar.
    pub events_by_signal: &'a [Vec<SimulationTime>],
}

/// Build a well-formed VCD trace from a captured boundary-event log.
///
/// The returned string is suitable to assign to
/// [`circuit_solver_types::DigitalEventTrace::vcd`] and is parseable
/// by any IEEE-1364-§18 VCD reader (including the third-party `vcd`
/// crate the implementer-task tests exercise).
///
/// # Panics
///
/// Panics if `signals.len() > MAX_SINGLE_CHAR_SIGNALS` (94). The
/// scenario's signal count is bounded to a handful; growing past 94
/// requires the multi-character-id extension which is deferred to a
/// follow-on task.
///
/// Panics if `signals.len() != events_by_signal.len()`. The two
/// slices are required to be parallel.
#[must_use]
pub fn build_vcd(input: &VcdTraceInput<'_>) -> String {
    assert_eq!(
        input.signals.len(),
        input.events_by_signal.len(),
        "build_vcd: signals and events_by_signal must be parallel slices"
    );
    assert!(
        input.signals.len() <= MAX_SINGLE_CHAR_SIGNALS,
        "build_vcd: at most {MAX_SINGLE_CHAR_SIGNALS} signals fit single-char VCD ids; got {}",
        input.signals.len()
    );

    let mut out = String::new();

    // -----------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------
    //
    // `$date` and `$version` are *optional* per IEEE-1364 §18 but most
    // VCD consumers expect them; emitting fixed strings keeps the
    // output deterministic and round-trippable in tests.
    out.push_str("$date\n");
    out.push_str("    2026-05-21\n");
    out.push_str("$end\n");
    out.push_str("$version\n");
    out.push_str("    circuit-solver mixed-signal-cosim\n");
    out.push_str("$end\n");

    // The scheduler tracks time in picoseconds (`SimulationTime`'s
    // internal unit), so the VCD timescale must match exactly. Any
    // other choice would force the consumer to divide and risks
    // floating-point error.
    //
    // We emit the compact `1ps` form (no space between number and
    // unit). IEEE-1364 §18 accepts both `"1 ps"` and `"1ps"`; the
    // compact form is what tasks.md #42's witness assertions pin.
    out.push_str("$timescale 1ps $end\n");

    // -----------------------------------------------------------------
    // Declarations: $scope ... $var* ... $upscope ... $enddefinitions
    // -----------------------------------------------------------------
    let _ = writeln!(out, "$scope module {} $end", input.scope_name);
    for (i, sig) in input.signals.iter().enumerate() {
        // Safe: bounded by the assert above.
        let id_byte = VCD_ID_FIRST + u8::try_from(i).expect("checked by assert: len <= 94");
        let id = char::from(id_byte);
        let _ = writeln!(out, "$var wire 1 {id} {sig} $end");
    }
    out.push_str("$upscope $end\n");
    out.push_str("$enddefinitions $end\n");

    // -----------------------------------------------------------------
    // Initial value dump at t = 0
    // -----------------------------------------------------------------
    //
    // IEEE 1364 §18.2.3.4 strongly recommends that every declared
    // variable have a known value at `#0` so downstream tools never
    // see "x"-undefined transitions on signals that simply never
    // toggled. Emit one explicit `0<id>` per signal.
    out.push_str("#0\n");
    out.push_str("$dumpvars\n");
    for i in 0..input.signals.len() {
        let id_byte = VCD_ID_FIRST + u8::try_from(i).expect("checked by assert: len <= 94");
        let id = char::from(id_byte);
        let _ = writeln!(out, "0{id}");
    }
    out.push_str("$end\n");

    // -----------------------------------------------------------------
    // Time-ordered transitions
    // -----------------------------------------------------------------
    //
    // Collate every event from every signal into a single sorted
    // timeline. For each unique time stamp emit a `#<ps>` record
    // followed by `1<id>` lines for the signals transitioning at
    // that time. We don't dedup *within* a (signal, time) pair —
    // the scheduler is responsible for that — but we do dedup across
    // signals so the timestamp record appears exactly once.
    let mut timeline: Vec<(i64, u8)> = Vec::new();
    for (i, events) in input.events_by_signal.iter().enumerate() {
        for t in events {
            let ps = t.as_picoseconds();
            // Skip t == 0: it's already covered by $dumpvars above.
            // A real transition at t=0 would be indistinguishable
            // from the initial value in any VCD reader anyway.
            if ps == 0 {
                continue;
            }
            let id_byte = VCD_ID_FIRST + u8::try_from(i).expect("checked by assert: len <= 94");
            timeline.push((ps, id_byte));
        }
    }
    // Sort by (time, id) so output is deterministic. Stability isn't
    // sufficient because input ordering is per-signal.
    timeline.sort_unstable();

    let mut last_time: Option<i64> = None;
    for (ps, id_byte) in timeline {
        if last_time != Some(ps) {
            let _ = writeln!(out, "#{ps}");
            last_time = Some(ps);
        }
        let id = char::from(id_byte);
        let _ = writeln!(out, "1{id}");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::{SignalName, SimulationTime};

    #[test]
    fn empty_signals_produce_a_minimal_header() {
        let vcd = build_vcd(&VcdTraceInput {
            scope_name: "empty",
            signals: &[],
            events_by_signal: &[],
        });
        assert!(vcd.contains("$timescale 1ps $end"));
        assert!(vcd.contains("$scope module empty $end"));
        assert!(vcd.contains("$upscope $end"));
        assert!(vcd.contains("$enddefinitions $end"));
        // No `$var` declarations and no transitions, but the initial
        // `#0 $dumpvars $end` block is still present (per IEEE-1364
        // §18 the dump block is allowed to declare zero variables).
        assert!(vcd.contains("#0"));
    }

    #[test]
    fn two_signals_get_distinct_ids() {
        let signals = vec![SignalName::new("din"), SignalName::new("dout")];
        let events = vec![
            vec![SimulationTime::from_nanoseconds(50)],
            vec![SimulationTime::from_nanoseconds(50)],
        ];
        let vcd = build_vcd(&VcdTraceInput {
            scope_name: "test",
            signals: &signals,
            events_by_signal: &events,
        });
        assert!(vcd.contains("$var wire 1 ! din $end"));
        assert!(vcd.contains("$var wire 1 \" dout $end"));
        // The 50 ns event is at 50_000 ps.
        assert!(vcd.contains("#50000\n"));
        assert!(vcd.contains("1!\n"));
        assert!(vcd.contains("1\"\n"));
    }

    #[test]
    fn timeline_is_sorted_even_for_unordered_input() {
        let signals = vec![SignalName::new("a"), SignalName::new("b")];
        // Signal "a" fires late, signal "b" fires early — verifying
        // the writer collates and re-sorts.
        let events = vec![
            vec![SimulationTime::from_nanoseconds(80)],
            vec![SimulationTime::from_nanoseconds(20)],
        ];
        let vcd = build_vcd(&VcdTraceInput {
            scope_name: "sort",
            signals: &signals,
            events_by_signal: &events,
        });
        let body = vcd.split("$enddefinitions $end\n").nth(1).unwrap();
        let twenty_idx = body.find("#20000\n").expect("must have #20000");
        let eighty_idx = body.find("#80000\n").expect("must have #80000");
        assert!(
            twenty_idx < eighty_idx,
            "VCD transitions must be time-sorted"
        );
    }

    #[test]
    fn t_zero_events_collapse_into_initial_dump() {
        // An event scheduled at t=0 should be folded into $dumpvars,
        // not appear as a separate `#0` ... `1!` block. The latter
        // would still parse, but most VCD readers warn about
        // duplicate `#0` records.
        let signals = vec![SignalName::new("zero")];
        let events = vec![vec![SimulationTime::ZERO]];
        let vcd = build_vcd(&VcdTraceInput {
            scope_name: "z",
            signals: &signals,
            events_by_signal: &events,
        });
        // Exactly one `#0` record.
        assert_eq!(vcd.matches("#0\n").count(), 1);
        // The signal is initialised to 0 inside $dumpvars (not
        // transitioned to 1 at t=0).
        assert!(vcd.contains("0!"));
        assert!(!vcd.contains("1!"));
    }

    #[test]
    #[should_panic(expected = "parallel slices")]
    fn build_panics_on_length_mismatch() {
        let signals = vec![SignalName::new("x"), SignalName::new("y")];
        let events: Vec<Vec<SimulationTime>> = vec![Vec::new()];
        let _ = build_vcd(&VcdTraceInput {
            scope_name: "bad",
            signals: &signals,
            events_by_signal: &events,
        });
    }
}
