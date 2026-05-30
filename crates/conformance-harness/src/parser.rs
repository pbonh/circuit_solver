//! ngspice ASCII rawfile parser.
//!
//! Parses the textual rawfile format that ngspice emits via the
//! interactive `write filename.raw` command (without the `-b` binary
//! flag). The format is:
//!
//! ```text
//! Title: <free text>
//! Date: <date>
//! Plotname: <plotname — used to classify SweepKind>
//! Flags: real          (we only handle `real`; complex is rejected
//!                       for v1 — AC magnitude/phase tests pre-extract
//!                       mag/phase real columns)
//! No. Variables: N
//! No. Points:    M
//! Variables:
//!   0   time              time
//!   1   v(n1)             voltage
//!   2   v(out)            voltage
//!   ...
//! Values:
//!  0  0.000000e+00 0.000000e+00 0.000000e+00
//!  1  1.000000e-09 1.234000e-01 2.345000e-01
//!  ...
//! ```
//!
//! The leading integer on each row of the `Values:` block is the
//! point index (0-based, matching `M` total). The columns that follow
//! are the variables in declaration order; column 0 is the sweep
//! axis.
//!
//! # What this parser is NOT
//!
//! - It does **not** handle the binary rawfile format. A follow-up
//!   task may add `load_ngspice_binary` against the same
//!   [`crate::GoldenReference`] target type; the comparator code does
//!   not need to change.
//! - It does **not** parse the `complex` flag. AC analyses that need
//!   magnitude/phase conformance must export each as a separate real
//!   variable (`vdb(out)` / `vp(out)`) — these are the names ngspice
//!   actually uses for the conformance scenarios in the spec.
//! - It does **not** support `Compose`-style chained plots. Each
//!   golden file holds exactly one plot.

use crate::golden::{GoldenReference, GoldenVariable, SweepKind};
use std::fmt;
use std::fs;
use std::path::Path;

/// Errors that can surface from [`load_ngspice_ascii`].
///
/// All variants carry enough context (line number, expected/found
/// counts, header key) for a per-test conformance harness in
/// tasks.md #63–#68 to print an actionable diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The underlying file could not be read.
    Io {
        /// Path the parser tried to open, in `Display` form.
        path: String,
        /// Error message from `std::fs::read_to_string`.
        message: String,
    },
    /// A required header key was missing (e.g., `Plotname`,
    /// `No. Variables`, `Values:`).
    MissingHeader {
        /// Header key the parser expected.
        key: String,
    },
    /// A header value could not be parsed as the expected type.
    BadHeader {
        /// Header key whose value was malformed.
        key: String,
        /// The raw value seen.
        value: String,
    },
    /// The `Variables:` block did not contain `n_vars` lines.
    BadVariablesBlock {
        /// Expected number of variable rows from `No. Variables:`.
        expected: usize,
        /// Number actually parsed before the next section started.
        actual: usize,
    },
    /// A `Values:` row was malformed (wrong column count, non-numeric).
    BadValuesRow {
        /// 0-based logical row index (point number).
        point: usize,
        /// Free-text message.
        message: String,
    },
    /// The file declared `Flags: complex` (not supported in v1 — see
    /// module docs).
    ComplexFlagUnsupported,
    /// The file declared `No. Points: 0`, which the conformance
    /// harness rejects (no data to compare against).
    EmptyValuesBlock,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, message } => write!(f, "I/O error reading {path}: {message}"),
            Self::MissingHeader { key } => write!(f, "missing required rawfile header `{key}`"),
            Self::BadHeader { key, value } => {
                write!(f, "rawfile header `{key}` has malformed value `{value}`")
            }
            Self::BadVariablesBlock { expected, actual } => write!(
                f,
                "rawfile `Variables:` block declared {expected} variables but {actual} were parsed"
            ),
            Self::BadValuesRow { point, message } => {
                write!(f, "rawfile `Values:` row {point}: {message}")
            }
            Self::ComplexFlagUnsupported => {
                write!(f, "rawfile `Flags: complex` is not supported (export magnitude/phase as separate real variables)")
            }
            Self::EmptyValuesBlock => {
                write!(f, "rawfile declared `No. Points: 0` (no data to compare)")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Load an ngspice ASCII rawfile from disk.
///
/// # Errors
///
/// Returns [`ParseError::Io`] on filesystem failure and one of the
/// structured variants on malformed content. The parser does not
/// attempt recovery — a malformed golden file is a hard fault in the
/// conformance harness.
pub fn load_ngspice_ascii<P: AsRef<Path>>(path: P) -> Result<GoldenReference, ParseError> {
    let path_ref = path.as_ref();
    let body = fs::read_to_string(path_ref).map_err(|e| ParseError::Io {
        path: path_ref.display().to_string(),
        message: e.to_string(),
    })?;
    parse_ngspice_ascii(&body)
}

/// In-memory analogue of [`load_ngspice_ascii`] for tests and for
/// callers that already have the bytes in hand (e.g., a fixture
/// embedded via `include_str!`).
///
/// # Errors
///
/// Same structured variants as [`load_ngspice_ascii`], minus
/// [`ParseError::Io`].
#[allow(clippy::too_many_lines)] // Single-file parser: header scan + values scan
                                 // are tightly coupled; splitting would require
                                 // either shared mutable state or a builder type
                                 // that obscures more than it clarifies.
pub fn parse_ngspice_ascii(body: &str) -> Result<GoldenReference, ParseError> {
    // --- Header pass: scan line-by-line until we hit `Values:` ----------

    let mut title: Option<String> = None;
    let mut plotname: Option<String> = None;
    let mut flags: Option<String> = None;
    let mut n_vars: Option<usize> = None;
    let mut n_points: Option<usize> = None;
    let mut variables_decl: Vec<(String, String)> = Vec::new(); // (name, kind)
                                                                // Position into `lines` (0-based) where the `Values:` block starts.
    let mut values_start: Option<usize> = None;

    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        let raw = lines[i];
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        // The `Variables:` header line itself ends with `:` and is
        // followed by `n_vars` indented rows of `idx name kind`.
        if let Some(rest) = strip_header_prefix(trimmed, "Title:") {
            title = Some(rest.to_string());
        } else if let Some(rest) = strip_header_prefix(trimmed, "Plotname:") {
            plotname = Some(rest.to_string());
        } else if let Some(rest) = strip_header_prefix(trimmed, "Flags:") {
            flags = Some(rest.to_string());
        } else if let Some(rest) = strip_header_prefix(trimmed, "No. Variables:") {
            n_vars = Some(parse_count("No. Variables", rest)?);
        } else if let Some(rest) = strip_header_prefix(trimmed, "No. Points:") {
            n_points = Some(parse_count("No. Points", rest)?);
        } else if trimmed.eq_ignore_ascii_case("Variables:") {
            // Consume the next n_vars rows.
            let n = n_vars.ok_or(ParseError::MissingHeader {
                key: "No. Variables".into(),
            })?;
            i += 1;
            let mut collected = 0usize;
            while i < lines.len() && collected < n {
                let row = lines[i].trim();
                if row.is_empty() {
                    i += 1;
                    continue;
                }
                if row.eq_ignore_ascii_case("Values:")
                    || row.eq_ignore_ascii_case("Binary:")
                    || strip_header_prefix(row, "Title:").is_some()
                {
                    // Hit next section early — bail.
                    break;
                }
                let toks: Vec<&str> = row.split_whitespace().collect();
                // Expected: `<idx>  <name>  <kind>`. The idx field is
                // discarded — column order is positional.
                if toks.len() < 3 {
                    return Err(ParseError::BadHeader {
                        key: "Variables row".into(),
                        value: row.to_string(),
                    });
                }
                let name = toks[1].to_string();
                let kind = toks[2..].join(" "); // kind can be multi-word in
                                                // rare ngspice builds
                variables_decl.push((name, kind));
                collected += 1;
                i += 1;
            }
            if collected != n {
                return Err(ParseError::BadVariablesBlock {
                    expected: n,
                    actual: collected,
                });
            }
            continue; // do not increment i twice
        } else if trimmed.eq_ignore_ascii_case("Values:") {
            values_start = Some(i + 1);
            break;
        } else if trimmed.eq_ignore_ascii_case("Binary:") {
            return Err(ParseError::BadHeader {
                key: "Binary".into(),
                value: "binary rawfile not supported (use `write` not `write -b`)".into(),
            });
        }
        i += 1;
    }

    let n_vars = n_vars.ok_or(ParseError::MissingHeader {
        key: "No. Variables".into(),
    })?;
    let n_points = n_points.ok_or(ParseError::MissingHeader {
        key: "No. Points".into(),
    })?;
    let plotname = plotname.ok_or(ParseError::MissingHeader {
        key: "Plotname".into(),
    })?;
    let title = title.unwrap_or_default();

    if let Some(f) = &flags {
        // ngspice flags are space-separated tokens; reject `complex`
        // explicitly, accept `real` and anything else (some builds
        // emit `real forward`).
        if f.split_whitespace()
            .any(|t| t.eq_ignore_ascii_case("complex"))
        {
            return Err(ParseError::ComplexFlagUnsupported);
        }
    }

    if n_points == 0 {
        return Err(ParseError::EmptyValuesBlock);
    }
    if variables_decl.len() != n_vars {
        return Err(ParseError::BadVariablesBlock {
            expected: n_vars,
            actual: variables_decl.len(),
        });
    }
    let values_start = values_start.ok_or(ParseError::MissingHeader {
        key: "Values:".into(),
    })?;

    // --- Values pass: parse n_points × n_vars samples -------------------

    // Per-column accumulators in declaration order. Index 0 is the
    // sweep axis.
    let mut columns: Vec<Vec<f64>> = vec![Vec::with_capacity(n_points); n_vars];
    let mut points_parsed = 0usize;
    let mut k = values_start;

    while points_parsed < n_points {
        // Read enough non-empty tokens to assemble one row of
        // `1 + n_vars` numbers (point-index prefix + sweep + deps).
        // ngspice typically emits the point-index on the same line as
        // the sweep value and dep #0, with continuation lines for
        // wide rows. We collect tokens until we have `1 + n_vars`.
        let mut tokens: Vec<&str> = Vec::with_capacity(1 + n_vars);
        while tokens.len() < 1 + n_vars {
            if k >= lines.len() {
                return Err(ParseError::BadValuesRow {
                    point: points_parsed,
                    message: format!(
                        "ran out of input — wanted {} tokens, got {} ({} points parsed of {})",
                        1 + n_vars,
                        tokens.len(),
                        points_parsed,
                        n_points
                    ),
                });
            }
            let line = lines[k].trim();
            k += 1;
            if line.is_empty() {
                continue;
            }
            tokens.extend(line.split_whitespace());
        }
        // The first token of the row is the point index — discard it
        // (we trust positional order). Some emitters omit it on
        // continuation lines but always include it on the first line
        // of each row.
        let _point_idx = tokens.remove(0);
        if tokens.len() != n_vars {
            return Err(ParseError::BadValuesRow {
                point: points_parsed,
                message: format!(
                    "expected {n_vars} sample tokens (1 sweep + {} deps), got {}",
                    n_vars - 1,
                    tokens.len()
                ),
            });
        }
        for (col_idx, tok) in tokens.iter().enumerate() {
            let v: f64 = tok.parse().map_err(|_| ParseError::BadValuesRow {
                point: points_parsed,
                message: format!("token `{tok}` in column {col_idx} is not a finite number"),
            })?;
            columns[col_idx].push(v);
        }
        points_parsed += 1;
    }

    // --- Assemble GoldenReference --------------------------------------

    let sweep_kind = SweepKind::from_plotname(&plotname);
    let (sweep_name, sweep_unit) = variables_decl[0].clone();
    let mut golden = GoldenReference::new(title, sweep_kind, sweep_name, sweep_unit);
    golden.sweep_axis = std::mem::take(&mut columns[0]);

    for (col_idx, (name, kind)) in variables_decl.into_iter().enumerate().skip(1) {
        let values = std::mem::take(&mut columns[col_idx]);
        let var = GoldenVariable { name, kind, values };
        // The values vec is guaranteed length n_points by construction,
        // matching sweep_axis.len() set above, so push_variable cannot
        // fail — but we surface the error explicitly anyway to keep
        // the invariant local.
        golden
            .push_variable(var)
            .map_err(|()| ParseError::BadValuesRow {
                point: 0,
                message: "internal: variable length parity violated post-parse".into(),
            })?;
    }
    Ok(golden)
}

/// If `s` starts with `prefix` (case-insensitive), return the trimmed
/// remainder. Used for header keys like `"Plotname:"` →
/// `"Transient Analysis"`.
fn strip_header_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() >= prefix.len() && s[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(s[prefix.len()..].trim())
    } else {
        None
    }
}

fn parse_count(key: &'static str, value: &str) -> Result<usize, ParseError> {
    value
        .trim()
        .parse::<usize>()
        .map_err(|_| ParseError::BadHeader {
            key: key.to_string(),
            value: value.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // A minimal transient golden file with two dependent variables.
    // Mirrors the shape `ngspice -b` writes for a 3-point transient.
    const TRANSIENT_RAW: &str = "Title: tb-rc-divider\n\
        Date: Thu Jun  5 14:00:00 2025\n\
        Plotname: Transient Analysis\n\
        Flags: real\n\
        No. Variables: 3\n\
        No. Points: 3\n\
        Variables:\n\
        \t0\ttime\ttime\n\
        \t1\tv(n1)\tvoltage\n\
        \t2\tv(out)\tvoltage\n\
        Values:\n\
        \t0\t0.000000e+00\t0.000000e+00\t0.000000e+00\n\
        \t1\t1.000000e-09\t3.300000e+00\t1.650000e+00\n\
        \t2\t2.000000e-09\t3.300000e+00\t3.300000e+00\n";

    const OP_RAW: &str = "Title: op-bias\n\
        Plotname: Operating Point\n\
        Flags: real\n\
        No. Variables: 3\n\
        No. Points: 1\n\
        Variables:\n\
        \t0\tv-sweep\tvoltage\n\
        \t1\tv(n1)\tvoltage\n\
        \t2\ti(v1)\tcurrent\n\
        Values:\n\
        \t0\t0.000000e+00\t3.300000e+00\t-1.234500e-03\n";

    const AC_RAW: &str = "Title: rc-ac\n\
        Plotname: AC Analysis\n\
        Flags: real\n\
        No. Variables: 2\n\
        No. Points: 2\n\
        Variables:\n\
        \t0\tfrequency\tfrequency\n\
        \t1\tvdb(out)\tvoltage\n\
        Values:\n\
        \t0\t1.000000e+03\t-3.010300e+00\n\
        \t1\t1.000000e+04\t-2.001000e+01\n";

    // ---------- Happy-path shape ----------

    #[test]
    fn parses_transient_three_point_two_variable() {
        let g = parse_ngspice_ascii(TRANSIENT_RAW).expect("parse should succeed");
        assert_eq!(g.title, "tb-rc-divider");
        assert_eq!(g.sweep_kind, SweepKind::Transient);
        assert_eq!(g.sweep_name, "time");
        assert_eq!(g.sweep_axis, vec![0.0, 1e-9, 2e-9]);
        assert_eq!(g.n_variables(), 2);
        let v_n1 = g.variable("v(n1)").expect("v(n1) present");
        assert_eq!(v_n1.kind, "voltage");
        assert_eq!(v_n1.values, vec![0.0, 3.3, 3.3]);
        let v_out = g.variable("v(out)").expect("v(out) present");
        assert_eq!(v_out.values, vec![0.0, 1.65, 3.3]);
    }

    #[test]
    fn parses_operating_point_single_row() {
        let g = parse_ngspice_ascii(OP_RAW).expect("parse should succeed");
        assert_eq!(g.sweep_kind, SweepKind::OperatingPoint);
        assert_eq!(g.n_points(), 1);
        assert!((g.variable("v(n1)").unwrap().values[0] - 3.3).abs() < 1e-12);
        assert!((g.variable("i(v1)").unwrap().values[0] - (-1.2345e-3)).abs() < 1e-15);
    }

    #[test]
    fn parses_ac_two_point_one_variable() {
        let g = parse_ngspice_ascii(AC_RAW).expect("parse should succeed");
        assert_eq!(g.sweep_kind, SweepKind::Ac);
        assert_eq!(g.sweep_axis, vec![1e3, 1e4]);
        let vdb = g.variable("vdb(out)").unwrap();
        assert!((vdb.values[0] - (-3.0103)).abs() < 1e-9);
        assert!((vdb.values[1] - (-20.01)).abs() < 1e-9);
    }

    // ---------- Header errors ----------

    #[test]
    fn rejects_complex_flag() {
        let body = TRANSIENT_RAW.replace("Flags: real", "Flags: complex");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert_eq!(err, ParseError::ComplexFlagUnsupported);
    }

    #[test]
    fn rejects_missing_plotname() {
        let body = TRANSIENT_RAW.replace("Plotname: Transient Analysis\n", "");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert_eq!(
            err,
            ParseError::MissingHeader {
                key: "Plotname".into()
            }
        );
    }

    #[test]
    fn rejects_zero_points() {
        let body = TRANSIENT_RAW.replace("No. Points: 3", "No. Points: 0");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert_eq!(err, ParseError::EmptyValuesBlock);
    }

    #[test]
    fn rejects_malformed_count() {
        let body = TRANSIENT_RAW.replace("No. Variables: 3", "No. Variables: three");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert!(matches!(
            err,
            ParseError::BadHeader { ref key, .. } if key == "No. Variables"
        ));
    }

    #[test]
    fn rejects_binary_section() {
        let body = TRANSIENT_RAW.replace("Values:", "Binary:");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert!(matches!(err, ParseError::BadHeader { ref key, .. } if key == "Binary"));
    }

    #[test]
    fn rejects_short_variables_block() {
        // Only declare 2 variables when the header said 3.
        let body = TRANSIENT_RAW.replace(
            "Variables:\n\t0\ttime\ttime\n\t1\tv(n1)\tvoltage\n\t2\tv(out)\tvoltage\n",
            "Variables:\n\t0\ttime\ttime\n\t1\tv(n1)\tvoltage\n",
        );
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert!(matches!(
            err,
            ParseError::BadVariablesBlock {
                expected: 3,
                actual: 2
            }
        ));
    }

    #[test]
    fn rejects_non_numeric_value() {
        let body =
            TRANSIENT_RAW.replace("3.300000e+00\t1.650000e+00", "NaN-but-text\t1.650000e+00");
        let err = parse_ngspice_ascii(&body).unwrap_err();
        assert!(matches!(err, ParseError::BadValuesRow { point: 1, .. }));
    }

    // ---------- I/O ----------

    #[test]
    fn load_from_disk_returns_io_error_on_missing_file() {
        let err = load_ngspice_ascii("/definitely/not/a/real/path.raw").unwrap_err();
        assert!(matches!(err, ParseError::Io { .. }));
    }

    #[test]
    fn load_from_disk_round_trips_transient_fixture() {
        // Use a temp file to exercise the disk path, not just the
        // in-memory parser.
        let dir = std::env::temp_dir();
        let path = dir.join("conformance-harness-test-fixture.raw");
        std::fs::write(&path, TRANSIENT_RAW).unwrap();
        let g = load_ngspice_ascii(&path).expect("load_ngspice_ascii happy path");
        assert_eq!(g.sweep_axis.len(), 3);
        let _ = std::fs::remove_file(&path);
    }
}
