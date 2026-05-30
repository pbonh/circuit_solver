//! Integration tests for `conformance-harness`.
//!
//! These tests exercise the public surface (`load_ngspice_ascii` →
//! `compare` → `ConformanceReport`) end-to-end, the way the
//! per-analysis conformance tests in tasks.md #63–#68 will use it.
//! They use real on-disk fixture files (created at test runtime in a
//! temp directory) so the I/O path is covered, not just the in-memory
//! parser.

use std::io::Write;

use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, SweepKind,
};

/// Realistic ngspice ASCII transient golden — a simple two-node RC
/// divider charging on a 3.3 V step. Five sweep points.
const TRANSIENT_GOLDEN_RAW: &str = "Title: rc-divider-step\n\
    Date: Thu Jun  5 14:00:00 2025\n\
    Plotname: Transient Analysis\n\
    Flags: real\n\
    No. Variables: 3\n\
    No. Points: 5\n\
    Variables:\n\
    \t0\ttime\ttime\n\
    \t1\tv(in)\tvoltage\n\
    \t2\tv(out)\tvoltage\n\
    Values:\n\
    \t0\t0.000000e+00\t0.000000e+00\t0.000000e+00\n\
    \t1\t1.000000e-09\t3.300000e+00\t1.041500e+00\n\
    \t2\t2.000000e-09\t3.300000e+00\t1.945000e+00\n\
    \t3\t3.000000e-09\t3.300000e+00\t2.560000e+00\n\
    \t4\t4.000000e-09\t3.300000e+00\t2.951000e+00\n";

/// Realistic ngspice operating-point golden — bias node and supply
/// current of the same divider at quiescence.
const OP_GOLDEN_RAW: &str = "Title: rc-divider-op\n\
    Plotname: Operating Point\n\
    Flags: real\n\
    No. Variables: 3\n\
    No. Points: 1\n\
    Variables:\n\
    \t0\tv-sweep\tvoltage\n\
    \t1\tv(out)\tvoltage\n\
    \t2\ti(vdd)\tcurrent\n\
    Values:\n\
    \t0\t0.000000e+00\t3.300000e+00\t-3.300000e-04\n";

fn write_temp_fixture(name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("conformance-harness-it");
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create fixture");
    f.write_all(body.as_bytes()).expect("write fixture");
    path
}

// ---------- Transient happy path ----------

/// Witness for the ADR-0008 contract: a solver result exactly matching
/// ngspice's golden values passes the transient default envelope and
/// the verdict is Pass.
#[test]
fn transient_exact_match_passes_default_envelope() {
    let path = write_temp_fixture("transient-rc.raw", TRANSIENT_GOLDEN_RAW);
    let golden = load_ngspice_ascii(&path).expect("parse golden");
    assert_eq!(golden.sweep_kind, SweepKind::Transient);
    assert_eq!(golden.n_points(), 5);

    // Pretend the solver returned exactly the golden numbers.
    let actual_in: Vec<f64> = vec![0.0, 3.3, 3.3, 3.3, 3.3];
    let actual_out: Vec<f64> = vec![0.0, 1.0415, 1.945, 2.56, 2.951];

    let report = compare(
        &golden,
        [
            ("v(in)", actual_in.as_slice()),
            ("v(out)", actual_out.as_slice()),
        ],
        AnalysisKind::Transient.default_tolerance(),
        16,
    );
    assert_eq!(report.verdict, ConformanceVerdict::Pass);
    assert_eq!(report.n_failed_variables, 0);
    assert_eq!(report.n_variables, 2);
}

/// Witness for ADR-0008 §"Near-zero absolute floor": a node whose
/// reference is exactly 0.0 V passes when the solver returns a tiny
/// noise-floor value (`< 1 mV`) — the relative term collapses but the
/// 1 mV absolute floor takes over.
#[test]
fn transient_near_zero_passes_via_absolute_floor() {
    let path = write_temp_fixture("transient-nearzero.raw", TRANSIENT_GOLDEN_RAW);
    let golden = load_ngspice_ascii(&path).expect("parse golden");

    // v(in) = 0.0 at t=0 in the golden. Solver returns 0.5 mV from
    // numerical noise — would fail a pure-relative check, must pass
    // the max(rel, abs) envelope.
    let actual_in: Vec<f64> = vec![5e-4, 3.3, 3.3, 3.3, 3.3];
    let actual_out: Vec<f64> = vec![0.0, 1.0415, 1.945, 2.56, 2.951];

    let report = compare(
        &golden,
        [
            ("v(in)", actual_in.as_slice()),
            ("v(out)", actual_out.as_slice()),
        ],
        AnalysisKind::Transient.default_tolerance(),
        16,
    );
    assert_eq!(report.verdict, ConformanceVerdict::Pass);
}

/// Witness for the ADR-0008 §"Large-signal relative dominance" axis:
/// a 200 mV error on a 3.3 V node is 6 % — outside the 1 % envelope —
/// and the harness must report Fail with the worst variable
/// identified.
#[test]
fn transient_large_signal_violation_fails_with_correct_worst_variable() {
    let path = write_temp_fixture("transient-violation.raw", TRANSIENT_GOLDEN_RAW);
    let golden = load_ngspice_ascii(&path).expect("parse golden");

    let actual_in: Vec<f64> = vec![0.0, 3.3, 3.3, 3.3, 3.3];
    // 2.56 V → 2.76 V at p=3: 200 mV diff, envelope 25.6 mV → fail.
    let actual_out: Vec<f64> = vec![0.0, 1.0415, 1.945, 2.76, 2.951];

    let report = compare(
        &golden,
        [
            ("v(in)", actual_in.as_slice()),
            ("v(out)", actual_out.as_slice()),
        ],
        AnalysisKind::Transient.default_tolerance(),
        16,
    );
    assert_eq!(report.verdict, ConformanceVerdict::Fail);
    assert_eq!(report.worst_variable, "v(out)");
    assert!(report.worst_margin < 0.0);
    let v_out = report
        .variables
        .iter()
        .find(|s| s.name == "v(out)")
        .unwrap();
    assert_eq!(v_out.n_failures, 1);
    assert_eq!(v_out.worst_point, 3);
    assert_eq!(v_out.failures.len(), 1);
    // Diff is 200 mV, envelope 25.6 mV → margin ≈ -174.4 mV.
    assert!(v_out.failures[0].margin < -0.16);
    assert!(v_out.failures[0].margin > -0.18);
}

// ---------- Operating point ----------

/// Witness that operating-point golden files (single sweep row) are
/// classified, parsed, and compared correctly under the DC default
/// envelope.
#[test]
fn dc_operating_point_round_trips_and_passes() {
    let path = write_temp_fixture("op.raw", OP_GOLDEN_RAW);
    let golden = load_ngspice_ascii(&path).expect("parse op golden");
    assert_eq!(golden.sweep_kind, SweepKind::OperatingPoint);
    assert_eq!(golden.n_points(), 1);
    assert_eq!(golden.n_variables(), 2);

    // Solver returned tiny offsets within the 1 mV floor / 1 % envelope.
    let actual_out: Vec<f64> = vec![3.3008]; // 0.8 mV error → within 33 mV envelope
    let actual_i: Vec<f64> = vec![-3.301e-4]; // 0.3 µA error on 330 µA → 0.09 %

    let report = compare(
        &golden,
        [
            ("v(out)", actual_out.as_slice()),
            ("i(vdd)", actual_i.as_slice()),
        ],
        AnalysisKind::Dc.default_tolerance(),
        16,
    );
    assert_eq!(report.verdict, ConformanceVerdict::Pass);
}

// ---------- Missing variable diagnostics ----------

/// Witness that a missing actual variable yields the verdict Fail
/// with the variable flagged `missing_from_actual` — the per-analysis
/// tests will rely on this signal to identify naming drift between
/// the netlist and the result map.
#[test]
fn missing_actual_variable_is_diagnosed() {
    let path = write_temp_fixture("transient-missing.raw", TRANSIENT_GOLDEN_RAW);
    let golden = load_ngspice_ascii(&path).expect("parse golden");

    // Only supply v(in); v(out) is missing.
    let actual_in: Vec<f64> = vec![0.0, 3.3, 3.3, 3.3, 3.3];
    let report = compare(
        &golden,
        [("v(in)", actual_in.as_slice())],
        AnalysisKind::Transient.default_tolerance(),
        16,
    );
    assert_eq!(report.verdict, ConformanceVerdict::Fail);
    let v_out = report
        .variables
        .iter()
        .find(|s| s.name == "v(out)")
        .unwrap();
    assert!(v_out.missing_from_actual);
}

// ---------- ADR-0008 default-tolerance pinning ----------

/// Witness that the four ADR-0008 analysis-default tolerance pairs
/// (DC, Transient, AC magnitude, AC phase, Noise SD) are exposed
/// through the public `AnalysisKind::default_tolerance()` API. The
/// per-analysis tests in #63–#68 read these values directly.
#[test]
fn adr_0008_default_tolerances_are_publicly_addressable() {
    // We don't re-assert the numbers here (the unit test in
    // `tolerance::tests` already does), but we exercise the public
    // path the per-analysis tests will take, ensuring no privacy or
    // type-export regression.
    let _dc = AnalysisKind::Dc.default_tolerance();
    let _tr = AnalysisKind::Transient.default_tolerance();
    let _ac_mag = AnalysisKind::AcMagnitude.default_tolerance();
    let _ac_phase = AnalysisKind::AcPhase.default_tolerance();
    let _noise = AnalysisKind::NoiseSpectralDensity.default_tolerance();
}
