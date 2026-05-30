//! Scenario-level integration test for
//! `python-frontend#gil-release-during-simulation` (tasks.md #59).
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_cde95dec`:
//!
//! ```gherkin
//! Given PythonDeveloper has two Python threads
//! And   thread A submits a transient AnalysisRequest that takes
//!       several seconds
//! And   thread B increments a Python counter in a loop
//! When  thread A's simulation begins executing
//! Then  thread B's counter continues to increment without being
//!       blocked by thread A
//! And   thread A eventually receives its Result
//! ```
//!
//! ## Why the witness drives `parse_netlist` / `build` instead of
//!    a Simulator submission
//!
//! At the time this scenario landed (tasks.md #59, parent `t_e34f8ed6`
//! merged tasks.md #57's `Result` value object), the dedicated
//! `Simulator.run` / submission entry point that consumes an
//! `AnalysisRequest` + `CircuitGraph` and emits a `Result` does *not*
//! yet exist on the Python surface (see the docstring on
//! `crate::result::PyAnalysisResult` — "Specifically out of scope for
//! #57: Simulator submission"). The scenario's
//! "transient `AnalysisRequest` that takes several seconds" is therefore
//! realised by the only **native-solver entry points** currently
//! exposed: [`circuit_solver::parse_netlist`] and
//! [`circuit_solver::CircuitBuilder.build`]. Both delegate to
//! `netlist_graph` under a [`pyo3::Python::detach`] (the pyo3 0.28
//! successor to `allow_threads`) so the GIL is dropped for the
//! duration of the native call — exactly the contract the scenario
//! pins.
//!
//! When the Simulator-submission entry point lands in a downstream
//! task, an additional `detach`-protected call site will appear on
//! that method, and this witness's structural assertions will extend
//! verbatim (same harness, same counter pattern, same Gherkin
//! mapping) to that surface.
//!
//! ## Test architecture (deterministic, not timing-based)
//!
//! Naïvely asserting "thread B's counter is large" is flaky under
//! `cargo test --jobs N` because `CPython`'s default `sys.setswitchinterval`
//! (5 ms) means thread B can make *some* progress even without
//! `detach` — the interpreter periodically yields the GIL on its
//! own. To distinguish "B advanced *because the GIL was released*"
//! from "B advanced *because the switch-interval ticked*", the
//! witness does **two** measurements:
//!
//!   1. **Concurrent run.** Thread A loops `build()` over a topology
//!      large enough that the cumulative native time is ≥ 200 ms.
//!      Thread B counts in a pure-Python loop *during* A's window.
//!      Record B's count at A's `t_end`: `ticks_concurrent`.
//!
//!   2. **Solo run.** Reset B's counter, run B alone for the same
//!      wall-clock duration A took, with no A. Record
//!      `ticks_solo`.
//!
//! The acceptance assertion is `ticks_concurrent >= ticks_solo / 5`.
//! Rationale:
//!   - If the GIL is held throughout A's native work, B gets at most
//!     ~one tick per `setswitchinterval` window (~5 ms), so for a
//!     ~200 ms window B accumulates only ~40 ticks — orders of
//!     magnitude below `ticks_solo` (which counts millions in
//!     pure Python).
//!   - If the GIL is released via `Python::detach`, B runs nearly
//!     unimpeded — `ticks_concurrent` ≈ `ticks_solo`. A 5× safety
//!     margin tolerates scheduler jitter on busy CI without
//!     letting the "GIL-held" failure mode slip through.
//!
//! The `Then` clause "thread A eventually receives its Result" is
//! pinned by asserting A's returned `CircuitGraph` has the expected
//! element/node count — a smoke check that `build()` succeeded.
//!
//! ## `cfg`-gate rationale
//!
//! Identical to the sibling test binaries: `extension-module` is
//! incompatible with linking the Python ABI directly into a test
//! binary. The whole file is skipped under default features, so
//! `cargo test --workspace` still passes. The per-crate recipe is:
//!
//! ```text
//! cargo test -p circuit-solver-py --no-default-features
//! ```

#![cfg(not(feature = "extension-module"))]

use std::time::{Duration, Instant};

use circuit_solver::PyCircuitBuilder;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

/// Number of resistor elements seeded into the builder before each
/// `build()` call. The product of (elements × inner-loop iterations)
/// must give a cumulative native time well above `CPython`'s
/// `setswitchinterval` (5 ms default) so the "GIL held" failure mode
/// is unambiguously detectable, but bounded enough that the test
/// completes in under a couple seconds on a slow CI box. 2,000
/// resistors per build × 100 builds ≈ ~200 ms cumulative on a
/// modern laptop and ~600 ms on a slow CI VM — both well above the
/// 5 ms switch-interval floor.
const ELEMENTS_PER_BUILD: usize = 2000;

/// How many times thread A re-runs `build()` inside its native
/// window. Each call goes through `py.detach` independently, so the
/// total native-side time during which the GIL is dropped is
/// `BUILDS_IN_WINDOW × per_build_native_time`. The constant is
/// chosen large enough that the cumulative native time exceeds the
/// switch-interval window by orders of magnitude.
const BUILDS_IN_WINDOW: usize = 100;

/// Helper: produce a fresh Python-side `CircuitBuilder` instance.
fn fresh_builder(py: Python<'_>) -> Bound<'_, PyCircuitBuilder> {
    Bound::new(py, PyCircuitBuilder::new())
        .expect("constructing PyCircuitBuilder via Bound::new must succeed")
}

/// Helper: seed a fresh builder with `ELEMENTS_PER_BUILD` resistors
/// forming a chain `n0 — R0 — n1 — R1 — n2 — … — nK — ground`. The
/// resulting topology has K+1 elements, K+1 nodes, and exercises the
/// union-find / NodeId-assignment path inside `netlist_graph::build`.
fn seed_resistor_chain(py: Python<'_>, elements: usize) -> Bound<'_, PyCircuitBuilder> {
    let builder = fresh_builder(py);
    for i in 0..elements {
        let a = format!("n{i}");
        let b = if i + 1 == elements {
            "0".to_string()
        } else {
            format!("n{}", i + 1)
        };
        let kwargs = [("value", 1000.0_f64)]
            .into_py_dict(py)
            .expect("kwargs dict construction must succeed");
        let terminals = PyList::new(py, [a.as_str(), b.as_str()])
            .expect("terminal list construction must succeed");
        let name = format!("R{i}");
        builder
            .call_method(
                "add_element",
                (name.as_str(), "R", terminals),
                Some(&kwargs),
            )
            .expect("add_element on a resistor chain link must succeed");
    }
    builder
}

/// Scenario witness for
/// `python-frontend#gil-release-during-simulation` (tasks.md #59).
///
/// Walks the Gherkin steps in order:
///
/// - **Given** `PythonDeveloper` has two Python threads — realised by
///   spawning thread B via Python's `threading.Thread` and keeping
///   thread A as the calling OS thread. Both attach independently
///   to the same `CPython` interpreter via `Python::attach`.
/// - **And** thread A submits work that takes several (hundred)
///   ms — realised by `BUILDS_IN_WINDOW` consecutive `build()`
///   calls on a `ELEMENTS_PER_BUILD`-element chain.
/// - **And** thread B increments a Python counter in a loop —
///   realised by Python code `while not done: counter[0] += 1`,
///   started before A's native work and stopped after.
/// - **When** thread A's simulation begins executing — realised
///   by entering the `build()` loop after thread B has been
///   spawned and is running.
/// - **Then** thread B's counter continues to increment without
///   being blocked by thread A — realised by the deterministic
///   `ticks_concurrent >= ticks_solo / 5` comparison documented
///   in the module preamble.
/// - **And** thread A eventually receives its Result — realised
///   by asserting A's final `CircuitGraph` has the expected
///   element / node count, smoke-checking that `build()` succeeded
///   inside `detach`.
#[test]
#[allow(clippy::too_many_lines)] // Gherkin witness: linear walk through Given/When/Then is more readable in one function than as split helpers.
fn gherkin_scenario_gil_release_during_simulation() {
    Python::attach(|py| {
        // --- Given: stash a Python `threading` module reference and
        // pre-build a counter list and "done" event so both threads
        // can see them. We use a one-element list rather than a bare
        // `int` so thread B's mutation is observable from thread A's
        // post-run assertion — Python's GIL semantics protect the
        // list element mutation under either GIL-held or GIL-released
        // execution.
        let threading = py
            .import("threading")
            .expect("`threading` import must succeed in a Python::attach scope");

        // Shared state held across both threads.
        let counter_b =
            PyList::new(py, [0_u64]).expect("constructing the counter list must succeed");
        let done_event = threading
            .call_method0("Event")
            .expect("threading.Event() construction must succeed");
        let started_event = threading
            .call_method0("Event")
            .expect("threading.Event() construction must succeed");

        // The Python-side body of thread B. Increments `counter[0]`
        // in a tight pure-Python loop and exits when `done` is set.
        // Sleeping is intentionally absent: this is the workload
        // whose throughput we use as the "GIL-released" witness.
        //
        // `started.set()` lets thread A know B has actually entered
        // its loop, so A doesn't start native work before B is
        // running.
        let counter_loop_src = "\
def _b_loop(counter, started, done):
    started.set()
    while not done.is_set():
        counter[0] += 1
";
        let counter_globals = pyo3::types::PyDict::new(py);
        py.run(
            std::ffi::CString::new(counter_loop_src)
                .expect("counter loop source has no NUL bytes")
                .as_c_str(),
            Some(&counter_globals),
            None,
        )
        .expect("defining `_b_loop` must succeed");
        let b_loop = counter_globals
            .get_item("_b_loop")
            .expect("`_b_loop` is present in the globals dict")
            .expect("`_b_loop` resolves to a callable");

        // --- And thread B increments a Python counter in a loop.
        // Spawn the worker via `threading.Thread(target=_b_loop,
        // args=(counter, started, done))`. Use kwargs for clarity.
        let thread_kwargs = pyo3::types::PyDict::new(py);
        thread_kwargs
            .set_item("target", &b_loop)
            .expect("setting target kwarg must succeed");
        thread_kwargs
            .set_item(
                "args",
                pyo3::types::PyTuple::new(py, [&counter_b, &started_event, &done_event])
                    .expect("args tuple construction must succeed"),
            )
            .expect("setting args kwarg must succeed");
        let thread_b = threading
            .call_method("Thread", (), Some(&thread_kwargs))
            .expect("constructing threading.Thread must succeed");
        thread_b
            .call_method0("start")
            .expect("starting thread B must succeed");

        // Wait until B has entered its loop. `wait()` releases the
        // GIL internally so B can actually make progress before A
        // starts.
        let waited = started_event
            .call_method1("wait", (5.0_f64,))
            .expect("started.wait(timeout=5.0) must succeed");
        let waited_b: bool = waited.extract().expect("started.wait() returns a bool");
        assert!(
            waited_b,
            "thread B failed to start within 5 s — the test harness is broken, \
             not the GIL-release contract"
        );

        // --- When thread A's simulation begins executing.
        //
        // Seed a large topology once; each `build()` call replays
        // the same expansion / union-find sweep, so the cumulative
        // native time is `BUILDS_IN_WINDOW × per_build_time` and the
        // GIL is dropped for the entire union of those windows.
        let builder = seed_resistor_chain(py, ELEMENTS_PER_BUILD);

        let a_start = Instant::now();
        let mut last_graph: Option<Bound<'_, PyAny>> = None;
        for _ in 0..BUILDS_IN_WINDOW {
            let graph = builder
                .call_method0("build")
                .expect("builder.build() must succeed on a well-formed chain");
            last_graph = Some(graph);
        }
        let a_duration = a_start.elapsed();

        // Snapshot B's counter at A's t_end (before signalling done),
        // and only then signal done. The order matters: if we
        // signalled `done` first, B would exit immediately and the
        // counter read after would race; reading first pins the value
        // observed during A's native window.
        let ticks_concurrent: u64 = counter_b
            .get_item(0)
            .expect("counter list index 0 is present")
            .extract()
            .expect("counter slot is a Python int that fits u64");
        done_event
            .call_method0("set")
            .expect("done.set() must succeed");
        thread_b
            .call_method1("join", (5.0_f64,))
            .expect("thread B join must complete within 5 s");
        let b_alive: bool = thread_b
            .call_method0("is_alive")
            .expect("is_alive() must succeed")
            .extract()
            .expect("is_alive returns bool");
        assert!(
            !b_alive,
            "thread B must exit cleanly after `done.set()` — leaked thread \
             indicates a harness bug, not a GIL-release violation"
        );

        // --- Then thread A eventually receives its Result.
        // (Realised here as: A's final `build()` produced a
        // well-formed `CircuitGraph` with the expected element count.
        // The dedicated submission-pipeline `Result` will be wired
        // through on a downstream task; until then, the immutable
        // graph handle is the concrete artifact A "received".)
        let final_graph = last_graph.expect("at least one build() ran inside the window");
        let element_count: usize = final_graph
            .call_method0("element_count")
            .expect("graph.element_count() must succeed")
            .extract()
            .expect("element_count returns usize");
        assert_eq!(
            element_count, ELEMENTS_PER_BUILD,
            "A's returned CircuitGraph must contain the seeded element count — \
             smoke-check that build() actually executed inside detach()"
        );

        // --- Then thread B's counter continues to increment without
        // being blocked by thread A.
        //
        // Calibrate the "solo" baseline: rerun B's loop alone for
        // the same wall-clock duration A took, with no A native
        // work. The ratio of `ticks_concurrent / ticks_solo` is the
        // headline witness for GIL release.
        let solo_counter =
            PyList::new(py, [0_u64]).expect("constructing the solo counter list must succeed");
        let solo_done = threading
            .call_method0("Event")
            .expect("threading.Event() construction must succeed");
        let solo_started = threading
            .call_method0("Event")
            .expect("threading.Event() construction must succeed");
        let solo_kwargs = pyo3::types::PyDict::new(py);
        solo_kwargs
            .set_item("target", &b_loop)
            .expect("setting solo target kwarg must succeed");
        solo_kwargs
            .set_item(
                "args",
                pyo3::types::PyTuple::new(py, [&solo_counter, &solo_started, &solo_done])
                    .expect("solo args tuple construction must succeed"),
            )
            .expect("setting solo args kwarg must succeed");
        let solo_thread = threading
            .call_method("Thread", (), Some(&solo_kwargs))
            .expect("constructing solo threading.Thread must succeed");
        solo_thread
            .call_method0("start")
            .expect("starting solo thread must succeed");
        let solo_waited = solo_started
            .call_method1("wait", (5.0_f64,))
            .expect("solo started.wait must succeed");
        let solo_waited_b: bool = solo_waited
            .extract()
            .expect("solo started.wait returns bool");
        assert!(
            solo_waited_b,
            "solo thread B must start within 5 s — harness bug"
        );

        // Yield the GIL for exactly the same wall-clock duration A
        // took. `py.detach` is the cleanest way to model "main
        // thread does nothing native, just lets the GIL be free"
        // — same primitive the contract under test uses.
        py.detach(|| {
            // Pure Rust sleep — no Python data touched. Mirrors the
            // shape of a native solver call that has dropped the
            // GIL for `a_duration`.
            std::thread::sleep(a_duration);
        });

        let ticks_solo: u64 = solo_counter
            .get_item(0)
            .expect("solo counter index 0 is present")
            .extract()
            .expect("solo counter is a Python int that fits u64");
        solo_done
            .call_method0("set")
            .expect("solo done.set() must succeed");
        solo_thread
            .call_method1("join", (5.0_f64,))
            .expect("solo thread join must complete within 5 s");

        // Headline assertion. With `detach` engaged on every
        // `build()`, `ticks_concurrent` should be the same order of
        // magnitude as `ticks_solo` (B ran in parallel with A's
        // native work). A 5× safety margin tolerates scheduler
        // jitter on busy CI while still failing loudly if the GIL
        // is held throughout A's window.
        //
        // Failure-mode picture:
        //   - GIL released (the contract):  ticks_concurrent ≈ ticks_solo.
        //   - GIL held everywhere:          ticks_concurrent ≈
        //                                    a_duration / 5ms (switch interval),
        //                                    i.e. tens of ticks for a ~200 ms
        //                                    window — far below ticks_solo / 5.
        //
        // Sanity: both should be non-zero (otherwise the harness is
        // broken before we even get to the GIL question).
        assert!(
            ticks_solo > 0,
            "solo thread B must have incremented at least once in {a_duration:?} \
             of GIL-free time — harness bug if zero"
        );
        assert!(
            ticks_concurrent > 0,
            "concurrent thread B must have incremented at least once during A's \
             native window of {a_duration:?} — zero ticks means the GIL was \
             completely held throughout, contradicting the task #59 contract"
        );
        let lower_bound = ticks_solo / 5;
        assert!(
            ticks_concurrent >= lower_bound,
            "GIL release contract violated: ticks_concurrent ({ticks_concurrent}) \
             must be >= ticks_solo / 5 ({lower_bound}) — solo={ticks_solo}, \
             concurrent={ticks_concurrent}, a_duration={a_duration:?}. \
             Either build() / parse_netlist no longer wraps its native work in \
             Python::detach, or the topology has shrunk below the switch-interval \
             detection floor."
        );

        // A modest sanity check: A's native window must actually
        // exceed the switch-interval window by an order of
        // magnitude. If `a_duration` shrinks below ~50 ms, the test
        // can no longer distinguish GIL-released from GIL-held, so
        // grow the topology constants if this fires.
        assert!(
            a_duration >= Duration::from_millis(50),
            "a_duration ({a_duration:?}) is too short to discriminate GIL \
             release from CPython switch-interval ticks; grow \
             ELEMENTS_PER_BUILD / BUILDS_IN_WINDOW above the 5 ms \
             switch-interval floor"
        );
    });
}
