//! `conformance-harness` — load ngspice golden-reference files and
//! compare circuit-solver results against them under
//! [ADR-0008](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0008-per-node-max-relative-absolute-tolerance-envelope.md)'s
//! `max(relative, absolute)` per-node tolerance envelope (tasks.md item
//! #62).
//!
//! # What this crate is
//!
//! This is the **shared infrastructure** for conformance testing. Per
//! tasks.md it has no scenario witness of its own: the per-analysis
//! conformance tests (tasks.md items #63 DC, #64 AC, #65 transient,
//! #66 noise, #67 mixed-signal, #68 ASAP7) are the *consumers* of this
//! crate. Each of those tests loads an ngspice golden file via
//! [`load_ngspice_ascii`] (or the binary analogue, when added) and
//! calls [`compare()`] to obtain a [`ConformanceReport`].
//!
//! # ADR-0008 in one paragraph
//!
//! For every (variable, sweep-point) pair, the result `v_actual` passes
//! against the golden value `v_ref` iff
//!
//! ```text
//! |v_actual - v_ref|  <=  max( rel_pct * |v_ref| , abs_threshold )
//! ```
//!
//! At large signals the relative term dominates (proportional
//! accuracy); near zero the absolute floor dominates (no false failure
//! on tool-noise nodes). ADR-0008 explicitly rejects pure-relative,
//! pure-absolute, and additive (relative + absolute) variants.
//!
//! # Module map
//!
//! - [`tolerance`] — the [`Tolerance`] envelope (relative / absolute
//!   pair) and per-analysis defaults from ADR-0008's "Default
//!   thresholds by analysis type" table.
//! - [`golden`] — the [`GoldenReference`] data model: a set of named
//!   variables, each with a parallel `sweep_axis` and `values` vector.
//! - [`parser`] — load ngspice ASCII rawfiles (the format emitted by
//!   `write rawfile.raw` in interactive ngspice or `wrdata` in batch
//!   mode) into a [`GoldenReference`].
//! - [`mod@compare`] — given an actual set of named series and a
//!   [`GoldenReference`], produce a [`ConformanceReport`] listing per-
//!   variable per-point pass/fail and the worst-case margin.
//!
//! # Glossary fidelity
//!
//! Terms used here track the inlined Glossary from the
//! `2026-05-21-v1-spec` change:
//!
//! - **Golden Reference** — "a trusted external simulator against
//!   which results are compared."
//! - **Conformance** — "passing the tolerance-bounded comparison
//!   against a golden reference."
//! - **Tolerance envelope** — the `max(rel, abs)` formulation chosen
//!   by ADR-0008.
//! - **`ConformanceTester`** — "an automated agent or engineer who
//!   compares solver results against golden references and reports
//!   pass/fail." (This crate is what the automated form runs.)
//!
//! # Stability
//!
//! Per [ADR-0010](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public Rust API is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod compare;
pub mod golden;
pub mod parser;
pub mod tolerance;

pub use compare::{compare, ConformanceReport, ConformanceVerdict, PointFailure, VariableSummary};
pub use golden::{GoldenReference, GoldenVariable, SweepKind};
pub use parser::{load_ngspice_ascii, ParseError};
pub use tolerance::{AnalysisKind, Tolerance};
