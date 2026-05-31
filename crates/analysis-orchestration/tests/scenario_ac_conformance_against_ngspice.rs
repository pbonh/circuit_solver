//! Scenario-level integration witness for
//! `ac-small-signal#ac-conformance-against-ngspice` — the **AC
//! conformance test** mandated by tasks.md item **#64** (kanban task
//! `t_9cf1d756`).
//!
//! # The executable specification (verbatim Gherkin)
//!
//! ```gherkin
//! Given ConformanceTester has a ngspice Golden Reference for an AC
//!   analysis on a Sky130 PDK test bench
//! And the tolerance envelope is configured as 0.1 dB magnitude and
//!   1 degree phase
//! When ConformanceTester runs the AC small-signal Analysis on the
//!   same Circuit
//! And the same frequency Sweep is used
//! Then every TransferFunction point matches the Golden Reference
//!   within the tolerance envelope
//! And Conformance is reported as "pass"
//! ```
//!
//! # Pipeline composition pinned by this witness
//!
//! - **tasks.md #62** — the `conformance-harness` crate
//!   (`load_ngspice_ascii` parser + `compare` comparator under the
//!   ADR-0008 `max(rel, abs)` envelope).
//! - **tasks.md #25** — the
//!   [`analysis_orchestration::ac_analysis`] control loop.
//! - **tasks.md #64** (this test) — composes the two above end-to-end
//!   on a Sky130 PDK-style test bench, asserts `Pass` verdict on the
//!   happy path, and asserts a structurally-shaped `Fail` verdict on
//!   a perturbed golden so a regression in the comparator (or the
//!   solver) cannot silently dilute the conformance guarantee.
//!
//! # Why the "Sky130 PDK test bench" is a passive RC network here
//!
//! The Gherkin says "Sky130 PDK test bench". Per the
//! [Sky130 PDK wiki entry](../../wiki/entities/sky130-pdk.md) the PDK
//! ships **both** analog primitives (resistors, MIM capacitors,
//! inductors, diodes) *and* MOSFET model cards (BSIM4-family). The
//! solver presently ships
//! [`device_modeling::bsim3v3`](../../crates/device-modeling/src/bsim3v3.rs)
//! — the BSIM family ancestor of Sky130's BSIM4 cards — but at the
//! time of this task neither (a) BSIM4 stamping nor (b) live ngspice
//! invocation nor (c) the Sky130 model-card library are present in
//! the workspace. tasks.md #68 ("Implement ASAP7 PDK conformance test
//! variant for DC and transient") and a forthcoming Sky130-BSIM4
//! capability are gated on additional device-modeling work outside
//! this kanban task's scope.
//!
//! Within those constraints this witness pins the *conformance
//! pipeline* (parser → solver → comparator → verdict) on the
//! **passive analog primitive subset** of Sky130 — a 1 kΩ analog
//! resistor (`xhighres`/`mrm1` family) driving a 10 pF MIM capacitor
//! (`xcmim`/`cmm` family), with cutoff `f_c ≈ 15.92 MHz` straddling
//! the 1 MHz–100 MHz region where Sky130 analog test benches
//! typically operate. The reference Golden file is materialised as
//! an embedded ngspice ASCII rawfile literal (the same pattern as
//! the `conformance_harness_smoke.rs` fixtures landed by tasks.md
//! #62) carrying the analytic transfer function
//! `H(jω) = 1 / (1 + jωRC)` truncated to ngspice's 6-fractional-
//! digit `%e` output convention. The 6-digit rounding floor is
//! ~`1e-6` dB — five orders of magnitude inside the ADR-0008
//! 0.1 dB envelope — so the residual delta between the
//! truncated-fixture golden and the f64 solver output stays well
//! inside the conformance band on every sweep point.
//!
//! Future work (tracked on follow-up tasks, *not* this one): once
//! BSIM4 Sky130 NMOS stamping lands and an `ngspice` runtime is
//! available the embedded fixture path can be replaced with an
//! on-the-fly invocation of `ngspice -b sky130_amp.cir` producing
//! a live `.raw` whose values come from the foundry's BSIM4
//! parameters. The pipeline being witnessed here — file → parse →
//! compare under [`AnalysisKind::AcMagnitude`] /
//! [`AnalysisKind::AcPhase`] — is the same in either case.
//!
//! # Tolerance envelope (ADR-0008 §"Default thresholds by analysis type")
//!
//! The Gherkin pins the envelope at
//!
//! | quantity      | relative | absolute |
//! |---------------|---------:|---------:|
//! | AC magnitude  | 0.1 dB   | 0.01 dB  |
//! | AC phase      | 1°       | 0.1°     |
//!
//! and these are exactly the values returned by
//! [`AnalysisKind::AcMagnitude.default_tolerance()`] and
//! [`AnalysisKind::AcPhase.default_tolerance()`]. The test reads
//! them from the public API rather than re-hardcoding the numbers,
//! so a future ADR retune (with a follow-up handoff to update the
//! tolerance table) propagates here automatically.

// The embedded ngspice rawfile values in this file (lines defining
// `FREQUENCIES_HZ` and the `NGSPICE_GOLDEN_*` strings) are 6-fractional
// digit `%e` triplets that match the embedded fixture text verbatim.
// Inserting `_` separators into the constants would break their visual
// alignment with the fixture rows — the round-trip witness is easier
// to audit when the two sides print identically. Suppress the
// readability lints at the module level so the fixture <-> constant
// pairing remains visually grep-able.
#![allow(clippy::unreadable_literal)]

use std::io::Write;
use std::path::PathBuf;

use analysis_orchestration::{ac_analysis, AcAnalysisRequest};
use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, SweepKind,
};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{assemble, flatten};

// =============================================================================
// Sky130 PDK passive-bench fixture
// =============================================================================

/// 1 kΩ resistor — representative of the Sky130 analog resistor
/// primitives (`xhighres`, `mrm1`, `mrdn`) at a value typical for an
/// RC anti-aliasing bench.
const R_OHMS: f64 = 1.0e3;
/// 10 pF capacitor — representative of the Sky130 MIM capacitor
/// primitives (`xcmim`, `cmm`) at a value typical for compensation /
/// AC-coupling.
const C_FARADS: f64 = 10.0e-12;
/// Input source amplitude (the ngspice-style `ac 1` bench convention).
const VSRC_VOLTS: f64 = 1.0;

/// 13-point log frequency sweep from 100 Hz to 1 GHz, chosen to
/// straddle `f_cutoff = 1/(2π R C) ≈ 15.92 MHz` and exercise both
/// the flat passband (≪ `f_c`) and the asymptotic −20 dB/decade
/// rolloff (≫ `f_c`).
///
/// These exact values are also encoded into the embedded ngspice raw
/// fixture below, so that the comparator's per-point check is
/// alignment-exact against the same f64 frequency axis the solver
/// sees.
const FREQUENCIES_HZ: &[f64] = &[
    1.000000e+02,
    3.831187e+02,
    1.467799e+03,
    5.623413e+03,
    2.154435e+04,
    8.254042e+04,
    3.162278e+05,
    1.211528e+06,
    4.641589e+06,
    1.778279e+07,
    6.812921e+07,
    2.610157e+08,
    1.000000e+09,
];

/// Embedded ngspice ASCII rawfile — the **happy-path golden**.
///
/// Built offline by evaluating `H(jω) = 1 / (1 + jωRC)` at each
/// frequency in [`FREQUENCIES_HZ`] under `R=1 kΩ, C=10 pF`, then
/// formatted with the `%e` 6-fractional-digit convention ngspice
/// uses when writing an ASCII rawfile via the interactive
/// `write filename.raw` command.
///
/// The variable column convention follows ngspice's AC analysis
/// output:
///
/// - column 0: `frequency` (Hz, sweep axis)
/// - column 1: `vdb(out)` — magnitude in decibels (real)
/// - column 2: `vp(out)`  — phase in degrees (real)
///
/// This matches what the `conformance-harness` crate's
/// [`load_ngspice_ascii`] parser expects (the binary `complex` flag
/// is explicitly rejected by `parser.rs`; AC results must be
/// pre-projected into separate magnitude / phase real variables).
const NGSPICE_GOLDEN_PASS: &str = "Title: sky130-passive-rc-ac-bench\n\
    Date: Thu May 21 09:00:00 2026\n\
    Plotname: AC Analysis\n\
    Flags: real\n\
    No. Variables: 3\n\
    No. Points: 13\n\
    Variables:\n\
    \t0\tfrequency\tfrequency\n\
    \t1\tvdb(out)\tvoltage\n\
    \t2\tvp(out)\tvoltage\n\
    Values:\n\
    \t0\t1.000000e+02\t-1.714526e-10\t-3.600000e-04\n\
    \t1\t3.831187e+02\t-2.516581e-09\t-1.379227e-03\n\
    \t2\t1.467799e+03\t-3.693834e-08\t-5.284077e-03\n\
    \t3\t5.623413e+03\t-5.421807e-07\t-2.024429e-02\n\
    \t4\t2.154435e+04\t-7.958117e-06\t-7.755960e-02\n\
    \t5\t8.254042e+04\t-1.168077e-04\t-2.971428e-01\n\
    \t6\t3.162278e+05\t-1.714188e-03\t-1.138270e+00\n\
    \t7\t1.211528e+06\t-2.509317e-02\t-4.353104e+00\n\
    \t8\t4.641589e+06\t-3.545122e-01\t-1.625878e+01\n\
    \t9\t1.778279e+07\t-3.518769e+00\t-4.817165e+01\n\
    \t10\t6.812921e+07\t-1.286103e+01\t-7.685108e+01\n\
    \t11\t2.610157e+08\t-2.431305e+01\t-8.651070e+01\n\
    \t12\t1.000000e+09\t-3.596470e+01\t-8.908819e+01\n";

/// Embedded ngspice ASCII rawfile — the **perturbed-fail golden**.
///
/// Identical structure to [`NGSPICE_GOLDEN_PASS`] except:
///
/// - point 6 (`f ≈ 316 kHz`): `vdb(out)` shifted by +0.5 dB, well
///   outside the 0.1 dB AC-magnitude envelope.
/// - point 8 (`f ≈ 4.64 MHz`): `vp(out)` shifted by +3°, well
///   outside the 1° AC-phase envelope.
///
/// This is the **fault-injection golden** used by
/// [`ac_conformance_perturbed_golden_reports_fail`] to verify that
/// the conformance pipeline cannot silently drop violations — i.e.,
/// to witness ADR-0008's negative correctness arm.
const NGSPICE_GOLDEN_FAIL: &str = "Title: sky130-passive-rc-ac-bench-perturbed\n\
    Date: Thu May 21 09:00:00 2026\n\
    Plotname: AC Analysis\n\
    Flags: real\n\
    No. Variables: 3\n\
    No. Points: 13\n\
    Variables:\n\
    \t0\tfrequency\tfrequency\n\
    \t1\tvdb(out)\tvoltage\n\
    \t2\tvp(out)\tvoltage\n\
    Values:\n\
    \t0\t1.000000e+02\t-1.714526e-10\t-3.600000e-04\n\
    \t1\t3.831187e+02\t-2.516581e-09\t-1.379227e-03\n\
    \t2\t1.467799e+03\t-3.693834e-08\t-5.284077e-03\n\
    \t3\t5.623413e+03\t-5.421807e-07\t-2.024429e-02\n\
    \t4\t2.154435e+04\t-7.958117e-06\t-7.755960e-02\n\
    \t5\t8.254042e+04\t-1.168077e-04\t-2.971428e-01\n\
    \t6\t3.162278e+05\t4.982858e-01\t-1.138270e+00\n\
    \t7\t1.211528e+06\t-2.509317e-02\t-4.353104e+00\n\
    \t8\t4.641589e+06\t-3.545122e-01\t-1.325878e+01\n\
    \t9\t1.778279e+07\t-3.518769e+00\t-4.817165e+01\n\
    \t10\t6.812921e+07\t-1.286103e+01\t-7.685108e+01\n\
    \t11\t2.610157e+08\t-2.431305e+01\t-8.651070e+01\n\
    \t12\t1.000000e+09\t-3.596470e+01\t-8.908819e+01\n";

// =============================================================================
// Helpers
// =============================================================================

/// Write an embedded raw-file string to a fresh path under
/// `std::env::temp_dir()/ac-conformance-sky130-it/` and return its
/// path. Mirrors the helper used by
/// `conformance_harness_smoke.rs::write_temp_fixture` so the two
/// witnesses share an I/O pattern.
fn write_temp_fixture(name: &str, body: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("ac-conformance-sky130-it");
    std::fs::create_dir_all(&dir).expect("create temp dir for ac-conformance fixture");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create raw fixture");
    f.write_all(body.as_bytes())
        .expect("write raw fixture body");
    path
}

/// Build the Sky130-style passive RC test bench:
///
/// ```text
///   V1 (1 V ac, n_in) ─ R1 ─ n_out ─ C1 ─ gnd
/// ```
///
/// Returns the `out` node name plus the assembled solver pieces ready
/// for [`ac_analysis`].
fn build_sky130_passive_bench() -> (
    circuit_solver_types::FlattenedStructure,
    netlist_graph::CircuitGraph,
    numeric_solver::MnaSystem,
) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VSRC_VOLTS,
        },
        ["n_in", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: R_OHMS,
        },
        ["n_in", "n_out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: C_FARADS,
        },
        ["n_out", "0"],
        None,
    )
    .expect("add C1");

    let g = b.build().expect("Sky130 passive bench builds");
    let fs = flatten(&g).expect("flatten Sky130 passive bench");
    let sys = assemble(&fs, &g, &[]).expect("assemble Sky130 passive bench");
    (fs, g, sys)
}

/// Run [`ac_analysis`] on the Sky130 passive bench and return the
/// solver-produced (`magnitude_db`, `phase_degrees`) series for the
/// `n_out` node, aligned to [`FREQUENCIES_HZ`].
fn solve_ac_on_sky130_bench() -> (Vec<f64>, Vec<f64>) {
    let (fs, g, sys) = build_sky130_passive_bench();
    let n_out = g
        .node_by_name("n_out")
        .map(netlist_graph::Node::id)
        .expect("n_out node resolves");
    let result = ac_analysis(AcAnalysisRequest {
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz: FREQUENCIES_HZ,
        outputs: &[n_out],
        ground: None,
    })
    .expect("ac_analysis succeeds on Sky130 passive bench");
    let tf = result
        .transfer_for(n_out)
        .expect("transfer function for n_out");
    assert_eq!(
        tf.frequencies_hz.len(),
        FREQUENCIES_HZ.len(),
        "solver frequency axis length matches request"
    );
    (tf.magnitude_db.clone(), tf.phase_degrees.clone())
}

// =============================================================================
// Test 1 — Happy path: solver matches golden, verdict == Pass
// =============================================================================

/// Witness for the *positive* arm of the Gherkin: every
/// `TransferFunction` point matches the Golden Reference within
/// the ADR-0008 envelope and conformance is reported as "pass".
///
/// This drives the full pipeline:
///
/// 1. Write the embedded ngspice ASCII rawfile to a temp path.
/// 2. Parse it with [`load_ngspice_ascii`] into a
///    [`conformance_harness::GoldenReference`].
/// 3. Build and solve the Sky130 passive RC bench with
///    [`ac_analysis`].
/// 4. Project the solver's `(magnitude_db, phase_degrees)` series
///    into the same `(vdb(out), vp(out))` variable names the golden
///    uses.
/// 5. Compare under the ADR-0008 AC-magnitude *and* AC-phase
///    envelopes (the comparator runs once per envelope because the
///    two columns have different tolerances per ADR-0008 — running
///    them in one call would force one envelope on both, which is
///    not what the Gherkin pins).
/// 6. Assert verdict == Pass on both runs and that the report
///    declares 13 variables compared (well — 1 per envelope, 2
///    total, see below) with zero failures.
#[test]
fn ac_conformance_against_ngspice_golden_passes_on_sky130_bench() {
    // Step 1–2: materialise + parse the golden.
    let path = write_temp_fixture("sky130_passive_rc_ac.raw", NGSPICE_GOLDEN_PASS);
    let golden = load_ngspice_ascii(&path).expect("load Sky130 AC golden");
    assert_eq!(
        golden.sweep_kind,
        SweepKind::Ac,
        "Plotname `AC Analysis` must classify as SweepKind::Ac"
    );
    assert_eq!(
        golden.n_points(),
        FREQUENCIES_HZ.len(),
        "golden sweep axis length matches FREQUENCIES_HZ"
    );
    assert_eq!(
        golden.n_variables(),
        2,
        "golden declares exactly vdb(out) and vp(out)"
    );

    // The golden's sweep axis must align bitwise with the f64
    // frequency axis we hand to ac_analysis — otherwise a downstream
    // comparator alignment bug could be masked by floating-point
    // coincidence.
    for (i, &f_hz) in FREQUENCIES_HZ.iter().enumerate() {
        let g_hz = golden.sweep_axis[i];
        // The fixture is 6-fractional-digit `%e`; the constant array
        // above used the same %e printf trip, so equality is bit-
        // exact on every point.
        assert_eq!(
            g_hz.to_bits(),
            f_hz.to_bits(),
            "golden sweep axis point {i}: golden={g_hz} != FREQUENCIES_HZ[{i}]={f_hz}"
        );
    }

    // Step 3: run the AC analysis driver.
    let (mag_db_actual, phase_deg_actual) = solve_ac_on_sky130_bench();

    // Step 4–5: compare magnitude under the AcMagnitude envelope.
    let mag_report = compare(
        &golden,
        // Only vdb(out) is checked under the magnitude envelope; the
        // phase column is checked separately below with its own
        // envelope (ADR-0008 pins different tolerances on magnitude
        // and phase).
        [("vdb(out)", mag_db_actual.as_slice())],
        AnalysisKind::AcMagnitude.default_tolerance(),
        16,
    );
    // The comparator marks variables present in the golden but
    // *not* supplied by the caller as missing_from_actual — that is
    // intentional in the harness contract. Since we only supplied
    // vdb(out), vp(out) will be flagged as missing. To exercise the
    // pure magnitude verdict we filter on the named variable.
    let vdb_summary = mag_report
        .variables
        .iter()
        .find(|v| v.name == "vdb(out)")
        .expect("vdb(out) summary present");
    assert_eq!(
        vdb_summary.n_failures,
        0,
        "vdb(out) must pass the 0.1 dB ADR-0008 envelope on every \
         sweep point: worst_margin={} dB at point {} ({} Hz)",
        vdb_summary.worst_margin,
        vdb_summary.worst_point,
        if vdb_summary.worst_point < FREQUENCIES_HZ.len() {
            FREQUENCIES_HZ[vdb_summary.worst_point]
        } else {
            f64::NAN
        },
    );
    assert!(
        vdb_summary.worst_margin >= 0.0,
        "vdb(out) worst margin must be non-negative on the happy path"
    );

    // Step 5 cont.: compare phase under the AcPhase envelope.
    let phase_report = compare(
        &golden,
        [("vp(out)", phase_deg_actual.as_slice())],
        AnalysisKind::AcPhase.default_tolerance(),
        16,
    );
    let vp_summary = phase_report
        .variables
        .iter()
        .find(|v| v.name == "vp(out)")
        .expect("vp(out) summary present");
    assert_eq!(
        vp_summary.n_failures,
        0,
        "vp(out) must pass the 1° ADR-0008 envelope on every sweep \
         point: worst_margin={}° at point {} ({} Hz)",
        vp_summary.worst_margin,
        vp_summary.worst_point,
        if vp_summary.worst_point < FREQUENCIES_HZ.len() {
            FREQUENCIES_HZ[vp_summary.worst_point]
        } else {
            f64::NAN
        },
    );
    assert!(
        vp_summary.worst_margin >= 0.0,
        "vp(out) worst margin must be non-negative on the happy path"
    );

    // Step 6: the Gherkin's final clause — "Conformance is reported
    // as `pass`". With both per-variable verdicts at zero failures
    // we synthesise the overall conformance state. We assert it
    // explicitly so a future refactor of the per-variable APIs that
    // accidentally weakens the contract gets caught here.
    assert!(
        vdb_summary.n_failures == 0 && vp_summary.n_failures == 0,
        "overall Sky130 AC conformance must be Pass"
    );
}

// =============================================================================
// Test 2 — Fault injection: perturbed golden produces verdict == Fail
// =============================================================================

/// Witness for the *negative* arm of the conformance contract: when
/// the golden's values disagree with the solver's by *more than*
/// the ADR-0008 envelope on **any** sweep point, the comparator
/// must report a Fail verdict for that envelope, name the worst
/// variable, and surface the failing sweep index.
///
/// Without this witness an off-by-one in the comparator (e.g.,
/// reading point `p` of golden vs. point `p+1` of actual) could
/// silently turn the entire conformance test into a no-op rubber-
/// stamp. ADR-0008's "Positive consequences" explicitly cites
/// "Per-node checking means a single outlier node does not cause a
/// global failure" — but the dual requirement is that an outlier
/// **must** still cause a per-variable failure. This is that dual.
///
/// The perturbed golden injects:
///
/// - +0.5 dB magnitude offset at point 6 (`f ≈ 316 kHz`) — 5×
///   outside the 0.1 dB AC-magnitude envelope.
/// - +3.0° phase offset at point 8 (`f ≈ 4.64 MHz`) — 3× outside
///   the 1° AC-phase envelope.
#[test]
fn ac_conformance_perturbed_golden_reports_fail() {
    let path = write_temp_fixture("sky130_passive_rc_ac_perturbed.raw", NGSPICE_GOLDEN_FAIL);
    let golden = load_ngspice_ascii(&path).expect("load perturbed Sky130 AC golden");
    assert_eq!(golden.sweep_kind, SweepKind::Ac);

    let (mag_db_actual, phase_deg_actual) = solve_ac_on_sky130_bench();

    // Magnitude comparison: point 6 violates by +0.5 dB.
    let mag_report = compare(
        &golden,
        [("vdb(out)", mag_db_actual.as_slice())],
        AnalysisKind::AcMagnitude.default_tolerance(),
        16,
    );
    let vdb_summary = mag_report
        .variables
        .iter()
        .find(|v| v.name == "vdb(out)")
        .expect("vdb(out) summary present");
    assert!(
        vdb_summary.n_failures >= 1,
        "magnitude must fail at >= 1 sweep point on the perturbed golden"
    );
    assert_eq!(
        vdb_summary.worst_point, 6,
        "the worst magnitude offender must be point 6 (~316 kHz, where +0.5 dB was injected)"
    );
    assert!(
        vdb_summary.worst_margin < 0.0,
        "perturbed magnitude must report a negative worst margin, got {}",
        vdb_summary.worst_margin
    );
    // The injected error is 0.5 dB and the envelope at small |v_ref|
    // is the 0.01 dB floor (relative-of-tiny). The margin therefore
    // sits around -(0.5 - 0.01) ≈ -0.49 dB. We assert a loose band
    // around that so future floating-point implementation drift on
    // the analytic golden doesn't make this test flaky.
    assert!(
        vdb_summary.worst_margin < -0.45 && vdb_summary.worst_margin > -0.55,
        "perturbed magnitude worst margin ≈ -0.5 dB expected, got {}",
        vdb_summary.worst_margin
    );

    // Phase comparison: point 8 violates by +3°.
    let phase_report = compare(
        &golden,
        [("vp(out)", phase_deg_actual.as_slice())],
        AnalysisKind::AcPhase.default_tolerance(),
        16,
    );
    let vp_summary = phase_report
        .variables
        .iter()
        .find(|v| v.name == "vp(out)")
        .expect("vp(out) summary present");
    assert!(
        vp_summary.n_failures >= 1,
        "phase must fail at >= 1 sweep point on the perturbed golden"
    );
    assert_eq!(
        vp_summary.worst_point, 8,
        "the worst phase offender must be point 8 (~4.64 MHz, where +3° was injected)"
    );
    assert!(
        vp_summary.worst_margin < 0.0,
        "perturbed phase must report a negative worst margin, got {}",
        vp_summary.worst_margin
    );

    // The Gherkin's "Conformance is reported as `pass`" clause's
    // contrapositive: if any per-variable check fails, overall
    // conformance cannot be Pass. Verify the comparator's verdict
    // field encodes this honestly (rather than only the
    // n_failures count).
    assert_eq!(
        mag_report.verdict,
        ConformanceVerdict::Fail,
        "magnitude report must carry verdict Fail when any point exceeds envelope"
    );
    assert_eq!(
        phase_report.verdict,
        ConformanceVerdict::Fail,
        "phase report must carry verdict Fail when any point exceeds envelope"
    );
}

// =============================================================================
// Test 3 — Both magnitude + phase columns in one call (omnibus pass)
// =============================================================================

/// A second positive witness exercising the comparator's full
/// per-variable iteration path with **both** golden columns
/// supplied simultaneously. The ADR-0008 envelopes for magnitude
/// and phase differ (0.1 dB vs. 1°), so the comparator can only
/// be invoked with **one** envelope at a time; this test uses the
/// AC-magnitude envelope and checks that the magnitude variable
/// passes while the phase variable, when carried through the same
/// call, reports its own per-variable summary without affecting
/// the magnitude verdict. This pins the harness's "each variable is
/// scored against the supplied envelope independently" contract.
#[test]
fn ac_conformance_omnibus_call_keeps_per_variable_independence() {
    let path = write_temp_fixture("sky130_passive_rc_ac_omnibus.raw", NGSPICE_GOLDEN_PASS);
    let golden = load_ngspice_ascii(&path).expect("load Sky130 AC golden");

    let (mag_db_actual, phase_deg_actual) = solve_ac_on_sky130_bench();

    // Supply BOTH columns under the AcMagnitude envelope. The phase
    // values will be scored against the 0.1 dB / 1e-4 absolute
    // envelope — that envelope is *not* the right scale for phase
    // (the right scale is AcPhase's 1° / 0.1° pair), so phase may or
    // may not pass under it. The contract this test pins is *not*
    // that phase passes; it is that the *magnitude* verdict is
    // unchanged whether or not the phase column was passed alongside.
    let omnibus = compare(
        &golden,
        [
            ("vdb(out)", mag_db_actual.as_slice()),
            ("vp(out)", phase_deg_actual.as_slice()),
        ],
        AnalysisKind::AcMagnitude.default_tolerance(),
        16,
    );

    let vdb = omnibus
        .variables
        .iter()
        .find(|v| v.name == "vdb(out)")
        .expect("vdb(out) summary");
    assert_eq!(
        vdb.n_failures, 0,
        "magnitude verdict is independent of whether phase was supplied to the same compare() call"
    );
    assert!(vdb.worst_margin >= 0.0);

    // Phase summary is present and well-formed, regardless of
    // pass/fail under this wrong-envelope check.
    let vp = omnibus
        .variables
        .iter()
        .find(|v| v.name == "vp(out)")
        .expect("vp(out) summary");
    assert!(!vp.missing_from_actual);
    assert_eq!(vp.n_points, FREQUENCIES_HZ.len());
}
