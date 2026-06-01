//! The [`GoldenReference`] data model.
//!
//! A golden reference is a set of *named variables*, each carrying a
//! parallel `sweep_axis` and `values` vector. The sweep axis kind
//! (time, frequency, sweep parameter, or none for DC operating point)
//! is captured at the file level by [`SweepKind`], so per-point
//! cross-checks (e.g., "the actual transient ran out to the same time
//! interval the golden reports") are possible without per-variable
//! re-tagging.
//!
//! # Why this shape
//!
//! ngspice's rawfile format is fundamentally:
//!
//! ```text
//! Header:
//!   Title:      <free text>
//!   `Plotname`: <"DC transfer characteristic" | "Transient analysis"
//!                | "AC analysis" | "Noise spectral density" | ...>
//!   Flags:      real | complex
//!   No. Variables: N
//!   No. Points:    M
//!   Variables:
//!     0   time              time
//!     1   v(n1)             voltage
//!     ...
//!   Values:
//!     0  t_0  v0_0  v1_0  ...  v_{N-2}_0
//!     1  t_1  v0_1  ...
//!     ...
//! ```
//!
//! The first variable (index 0) is the *sweep axis*. The remaining
//! variables are the dependent quantities the harness compares against.
//! This module preserves exactly that shape: [`GoldenReference`] holds
//! the sweep-axis name + values once, then one
//! [`GoldenVariable`] per dependent variable.

/// What the sweep axis means for the variables in a
/// [`GoldenReference`]. Determined from the rawfile's `Plotname`
/// header by [`crate::parser::load_ngspice_ascii`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SweepKind {
    /// DC operating point — there is no sweep; the variable holds a
    /// single value (`sweep_axis` is length 1, typically the placeholder
    /// `0.0`). The plotname matches `op` / `operating point`.
    OperatingPoint,
    /// Transient analysis — sweep axis is time in seconds. Matches
    /// plotname containing `transient`.
    Transient,
    /// AC small-signal — sweep axis is frequency in Hz. Matches
    /// plotname containing `ac` (and not `transient` / `dc`).
    Ac,
    /// DC sweep over a source — sweep axis is the swept-source value
    /// (volts or amperes depending on the source). Matches plotname
    /// containing `dc transfer characteristic` (the ngspice phrasing).
    DcSweep,
    /// Noise spectral density — sweep axis is frequency in Hz, but the
    /// dependent values are V/√Hz, not volts. Matches plotname
    /// containing `noise`.
    Noise,
    /// Anything else (custom user plotname) — the harness still parses
    /// and compares but cannot apply analysis-specific default
    /// tolerances. Callers must supply an explicit
    /// [`crate::tolerance::Tolerance`].
    Unknown,
}

impl SweepKind {
    /// Classify an ngspice `Plotname:` header value. Case-insensitive
    /// substring match in the order DC-operating-point → noise → AC →
    /// transient → DC-sweep → unknown so that the more specific
    /// plotnames win.
    ///
    /// # Examples
    ///
    /// ```
    /// use conformance_harness::SweepKind;
    /// assert_eq!(SweepKind::from_plotname("Transient Analysis"), SweepKind::Transient);
    /// assert_eq!(SweepKind::from_plotname("AC Analysis"), SweepKind::Ac);
    /// assert_eq!(SweepKind::from_plotname("Noise Spectral Density Curves"), SweepKind::Noise);
    /// assert_eq!(SweepKind::from_plotname("DC transfer characteristic"), SweepKind::DcSweep);
    /// assert_eq!(SweepKind::from_plotname("Operating Point"), SweepKind::OperatingPoint);
    /// assert_eq!(SweepKind::from_plotname("custom"), SweepKind::Unknown);
    /// ```
    #[must_use]
    pub fn from_plotname(plotname: &str) -> Self {
        let lower = plotname.to_ascii_lowercase();
        if lower.contains("operating point") || lower.trim() == "op" {
            Self::OperatingPoint
        } else if lower.contains("noise") {
            // Noise must be checked before "ac" because some ngspice
            // builds emit "ac noise" / "noise analysis (ac)" — we want
            // these classed as Noise.
            Self::Noise
        } else if lower.contains("transient") {
            Self::Transient
        } else if lower.contains("dc transfer") || lower.contains("dc sweep") {
            Self::DcSweep
        } else if lower.contains("ac") {
            Self::Ac
        } else {
            Self::Unknown
        }
    }
}

/// One dependent variable inside a [`GoldenReference`]. Its `values`
/// vector is parallel to the parent reference's `sweep_axis` vector.
///
/// # Invariants
///
/// - `values.len()` always equals the parent `GoldenReference`'s
///   `sweep_axis.len()`. The constructor [`GoldenReference::push_variable`]
///   enforces this.
/// - `name` is the verbatim ngspice variable name (e.g., `v(n1)`,
///   `i(v1)`, `v(out)`). The harness does not normalize the name —
///   the comparator caller must match the actual series' name against
///   what ngspice emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenVariable {
    /// Verbatim ngspice variable name. Case-preserved.
    pub name: String,
    /// ngspice type token (`voltage`, `current`, `frequency`,
    /// `onoise_spectrum`, `time`, etc.). Kept verbatim so callers can
    /// validate units; the harness itself does not interpret it.
    pub kind: String,
    /// Sample values in same order as the parent's `sweep_axis`.
    pub values: Vec<f64>,
}

/// A parsed golden reference file. Constructed by
/// [`crate::parser::load_ngspice_ascii`].
///
/// # Lookup
///
/// Variable names are stored verbatim. Looking up by name is `O(N)`
/// over the variable count — that's fine for the ~20-200 variables a
/// typical SPICE testbench emits; we don't add an index because the
/// per-test code in #63–#68 builds its own actual-vs-golden mapping
/// once per run.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenReference {
    /// Free-text title from the rawfile header.
    pub title: String,
    /// What the sweep axis means. See [`SweepKind`].
    pub sweep_kind: SweepKind,
    /// The independent variable's name (e.g., `time`, `frequency`).
    /// Always non-empty after a successful parse.
    pub sweep_name: String,
    /// The independent variable's ngspice type token (e.g., `time`,
    /// `frequency`).
    pub sweep_unit: String,
    /// Sweep-axis sample points. For DC operating point this has
    /// length 1 (placeholder).
    pub sweep_axis: Vec<f64>,
    /// Dependent variables, in the order the rawfile listed them
    /// (which matches the column order in the values block).
    pub variables: Vec<GoldenVariable>,
}

impl GoldenReference {
    /// Construct an empty reference with the given header data.
    /// Variables are added via [`Self::push_variable`].
    #[must_use]
    pub fn new(
        title: String,
        sweep_kind: SweepKind,
        sweep_name: String,
        sweep_unit: String,
    ) -> Self {
        Self {
            title,
            sweep_kind,
            sweep_name,
            sweep_unit,
            sweep_axis: Vec::new(),
            variables: Vec::new(),
        }
    }

    /// Number of sweep points (equivalently, the length each
    /// variable's `values` vector must have).
    #[must_use]
    pub fn n_points(&self) -> usize {
        self.sweep_axis.len()
    }

    /// Number of dependent variables.
    #[must_use]
    pub fn n_variables(&self) -> usize {
        self.variables.len()
    }

    /// Append one dependent variable.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if `var.values.len()` does not equal the
    /// current `sweep_axis.len()`. (The parser enforces this when
    /// constructing variables, so callers outside the parser rarely
    /// see this error.)
    ///
    /// # Why `Err(())`
    ///
    /// Plumbing a richer error type up to here would require either a
    /// generic error or a leaky dependency on
    /// [`crate::parser::ParseError`]. The parser builds the reference
    /// in a controlled order (sweep axis first, then variables with
    /// matched length); this assertion is a defensive guard, not a
    /// user-facing branch. The parser converts a returned `Err(())`
    /// into a structured [`crate::parser::ParseError`].
    #[allow(clippy::result_unit_err)]
    pub fn push_variable(&mut self, var: GoldenVariable) -> Result<(), ()> {
        if var.values.len() != self.sweep_axis.len() {
            return Err(());
        }
        self.variables.push(var);
        Ok(())
    }

    /// Look up a dependent variable by exact name. `O(N)` in the
    /// variable count.
    #[must_use]
    pub fn variable(&self, name: &str) -> Option<&GoldenVariable> {
        self.variables.iter().find(|v| v.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_kind_classifies_known_plotnames() {
        // Cases mirror the doc-comment examples plus the noise-vs-ac
        // precedence the parser depends on.
        assert_eq!(
            SweepKind::from_plotname("Transient Analysis"),
            SweepKind::Transient
        );
        assert_eq!(SweepKind::from_plotname("AC Analysis"), SweepKind::Ac);
        assert_eq!(
            SweepKind::from_plotname("Noise Spectral Density Curves"),
            SweepKind::Noise
        );
        // Noise-with-ac-in-name must still classify as Noise (precedence).
        assert_eq!(
            SweepKind::from_plotname("AC Noise Analysis"),
            SweepKind::Noise
        );
        assert_eq!(
            SweepKind::from_plotname("DC transfer characteristic"),
            SweepKind::DcSweep
        );
        assert_eq!(
            SweepKind::from_plotname("Operating Point"),
            SweepKind::OperatingPoint
        );
        assert_eq!(SweepKind::from_plotname("op"), SweepKind::OperatingPoint);
        assert_eq!(
            SweepKind::from_plotname("anything else"),
            SweepKind::Unknown
        );
    }

    #[test]
    fn sweep_kind_is_case_insensitive() {
        assert_eq!(
            SweepKind::from_plotname("TRANSIENT analysis"),
            SweepKind::Transient
        );
        assert_eq!(SweepKind::from_plotname("ac"), SweepKind::Ac);
    }

    #[test]
    fn push_variable_enforces_length_parity() {
        let mut g = GoldenReference::new(
            "test".into(),
            SweepKind::Transient,
            "time".into(),
            "time".into(),
        );
        g.sweep_axis = vec![0.0, 1e-9, 2e-9];
        let ok = g.push_variable(GoldenVariable {
            name: "v(n1)".into(),
            kind: "voltage".into(),
            values: vec![0.0, 1.0, 2.0],
        });
        assert!(ok.is_ok());
        let bad = g.push_variable(GoldenVariable {
            name: "v(n2)".into(),
            kind: "voltage".into(),
            values: vec![0.0, 1.0], // length mismatch
        });
        assert!(bad.is_err());
        assert_eq!(g.n_variables(), 1);
    }

    #[test]
    fn variable_lookup_by_name() {
        let mut g = GoldenReference::new(
            "t".into(),
            SweepKind::OperatingPoint,
            "v-sweep".into(),
            "voltage".into(),
        );
        g.sweep_axis = vec![0.0];
        g.push_variable(GoldenVariable {
            name: "v(n1)".into(),
            kind: "voltage".into(),
            values: vec![3.3],
        })
        .unwrap();
        assert!(g.variable("v(n1)").is_some());
        assert!(g.variable("v(missing)").is_none());
    }
}
