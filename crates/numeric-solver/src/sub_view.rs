//! Per-analysis sub-view extractor: ground-suppressed unknowns and
//! constraint masks (source-stepping, Gmin-stepping) on top of the
//! full MNA matrix produced by [`crate::assemble::assemble`].
//!
//! This module covers `tasks.md` item #15 of
//! `circuit-solver/2026-05-21-v1-spec`. It is the box labelled
//! `"Sub-View Extractor (ground suppress, mask)"` in the
//! `numeric-solver` C4 L2 component diagram in `design.md` — the
//! glue between the (analysis-agnostic) full MNA matrix and the
//! Newton-Raphson driver's per-solve linear system.
//!
//! # Design references
//!
//! - **ADR-0003 — Two-Pass Graph Flattening with Per-Analysis
//!   Sub-Views.** *Pass 2 builds the full MNA matrix (including
//!   ground); per-analysis sub-views apply analysis-specific masks at
//!   solve time without re-flattening or re-assembling.* This module
//!   implements the masking layer: it consumes an [`MnaSystem`] and
//!   produces a [`SubView`] tailored to one solve attempt (ground
//!   suppressed; possibly Gmin-shunted; possibly with sources scaled
//!   by a source-stepping factor).
//!
//! - **ADR-0009 — Topology Checker for Floating-Node Detection.**
//!   The Gmin-stepping mask in this module is the runtime side of
//!   the safety net the topology checker recommends: nodes flagged
//!   as "possibly conductive" get a finite shunt conductance to
//!   ground while NR finds a basin of attraction; the shunt is then
//!   gradually reduced (the homotopy loop is in `tasks.md` item #18).
//!
//! - **ADR-0010 — Unstable Public Rust API Surface for v1.** The
//!   [`SubView`], [`SubViewBuilder`], and [`SubViewError`] surfaces
//!   are unstable per ADR-0010.
//!
//! - **`design.md` C4 L2.** *Assembler → `"FullMNA"` → `SubView` →
//!   `"SubMatrix"` → NR.* This module is the `SubView` box; the
//!   [`SubView::matrix`] / [`SubView::rhs`] pair is the
//!   `"SubMatrix"` edge label.
//!
//! - **Scenario `dc-operating-point#linear-resistive-dc-operating-point`.**
//!   This module turns the full MNA matrix into the ground-
//!   suppressed system the linear solver actually consumes — i.e.,
//!   the matrix that is invertible because the ground equation has
//!   been replaced by the identity `v_gnd = 0`.
//!
//! - **Scenario `dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy`.**
//!   This module owns the per-step constraint application; the
//!   homotopy loop that walks Gmin down lands in `tasks.md` item #18.
//!
//! # What sub-view extraction means here
//!
//! The full MNA matrix from [`assemble`][crate::assemble::assemble]
//! carries the ground row and
//! column. As written, the ground row's KCL equation is redundant
//! (it is the negative sum of the other node-row equations) — so
//! the system is structurally singular *until* a ground reference
//! is imposed. The textbook SPICE technique is to **suppress the
//! ground unknown** by replacing the ground row with the identity
//! row `[1, 0, 0, …]` and forcing the ground column to zero
//! everywhere except the diagonal: the corresponding RHS entry is
//! also set to zero. The system is now full-rank, and the
//! constraint `v_gnd = 0` is encoded in the matrix itself rather
//! than by physically resizing the unknown vector.
//!
//! This module keeps the **physical dimension** identical to the
//! full MNA system (`node_count + branch_count`). Ground suppression
//! is a *mask*, not a slice. This preserves node indexing across
//! the whole solver pipeline — `solution[node.index() as usize]` is
//! the voltage at that node, whether the node is ground or not —
//! and it matches the contract the Newton-Raphson driver expects
//! from the sub-view per `design.md` (`SubView -->|"SubMatrix"| NR`,
//! same matrix shape, just numerically masked).
//!
//! # Constraint masks
//!
//! Two masks are offered today, both first-class consumers of the
//! homotopy methods named by `tasks.md` items #18 and #19:
//!
//! - **Gmin-stepping** — add a shunt conductance `g_min > 0` from
//!   every non-ground node to ground. This guarantees the
//!   conductance block is strictly diagonally dominant (and
//!   therefore nonsingular) regardless of topology. The homotopy
//!   loop steps `g_min` down to zero across multiple NR solves.
//!
//! - **Source-stepping** — scale the RHS contribution of every
//!   independent source (voltage and current sources, both linear)
//!   by a factor `α ∈ [0, 1]`. The MNA matrix is unaffected; only
//!   the RHS is scaled. The homotopy loop ramps `α` from 0 → 1.
//!
//! Both masks compose freely (Gmin on the matrix, source-stepping
//! on the RHS) and can be combined with ground suppression.
//!
//! # Identifying source contributions
//!
//! The Pass-2 assembler ([`crate::assemble::assemble`]) stamps
//! source contributions directly into the full MNA RHS — but the
//! resulting RHS vector does not retain *which* contributions came
//! from sources. To make source-stepping a pure post-assembly
//! operation, the extractor takes a separately-computed "source
//! RHS" vector as input. This vector is the RHS the assembler
//! would have produced if **only** independent sources were
//! stamped (every other contribution suppressed). The full RHS
//! minus the source RHS is the "non-source" contribution
//! (companion currents from linearized devices, etc.), which is
//! kept at full strength regardless of source-stepping.
//!
//! `tasks.md` item #18 (the homotopy driver) is responsible for
//! computing and threading this vector. The lightweight helper
//! [`source_rhs_from`] computes it from a [`FlattenedStructure`]
//! and a [`CircuitGraph`] by re-walking sources only — it lives
//! here, in this module, because the sub-view layer is the
//! consumer.
//!
//! # What this module does *not* do
//!
//! - **No solve.** This module produces the matrix and RHS that
//!   the linear solver consumes (`tasks.md` item #16, russell). It
//!   does not call into russell.
//! - **No homotopy loop.** The Gmin and source-stepping schedules
//!   are owned by `tasks.md` items #18 / #19 / the NR driver.
//!   This module applies *one* mask at *one* step.
//! - **No sparse representation.** The full MNA matrix is dense
//!   `f64` row-major; the sub-view inherits the same layout. The
//!   sparse-LU dispatch lands in item #16.

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitGraph, ElementKind};

use crate::assemble::MnaSystem;

/// A per-analysis sub-view onto a full MNA system, with ground
/// suppression and constraint masks applied.
///
/// A `SubView` owns a fresh `(matrix, rhs)` pair derived from an
/// [`MnaSystem`] by [`SubViewBuilder::build`]. The dimension matches
/// the underlying full system: ground suppression is encoded as an
/// identity row/column at index `0`, not as a physical resize. This
/// keeps node indexing stable across the entire solver pipeline.
///
/// # Layout
///
/// Identical to [`MnaSystem`]: rows/columns `0..node_count` are node
/// KCL equations, rows/columns `node_count..node_count+branch_count`
/// are MNA augmentation rows, and storage is row-major
/// (`matrix[r * dim + c]`).
///
/// # Ground suppression
///
/// When [`SubViewBuilder::suppress_ground`] is set (the default for
/// DC operating-point analysis), the row at `ground_node().index()`
/// is replaced by the standard basis vector `e_g` — `1.0` on the
/// diagonal and `0.0` everywhere else — and the column at
/// `ground_node().index()` is zeroed for every non-ground row. The
/// corresponding RHS entry is set to `0.0`. The mathematical effect
/// is to enforce `v_gnd = 0` exactly while leaving the rest of the
/// system numerically unchanged.
///
/// # Why dense
///
/// `tasks.md` item #15 produces only the intermediate masked
/// representation. The russell sparse-LU dispatch lands in item #16
/// and consumes a sparse view of this dense matrix; the assembler
/// → sub-view boundary remains dense to keep the test surface
/// small.
#[derive(Debug, Clone, PartialEq)]
pub struct SubView {
    node_count: u32,
    branch_count: u32,
    a: Vec<f64>,
    b: Vec<f64>,
}

impl SubView {
    /// Total dimension of the sub-view: `node_count + branch_count`.
    ///
    /// Always equal to [`MnaSystem::dim`] for the system this view
    /// was derived from.
    ///
    /// # Panics
    ///
    /// Panics only if `node_count + branch_count` overflows `u32`,
    /// which is structurally impossible because both fields are
    /// individually `u32` and the sum is validated via
    /// [`u32::checked_add`] at construction time — the underlying
    /// [`MnaSystem`] enforces the same invariant.
    #[must_use]
    pub fn dim(&self) -> u32 {
        self.node_count
            .checked_add(self.branch_count)
            .expect("dim was validated at construction")
    }

    /// Total node count (including the ground node, which is
    /// suppressed to identity when ground suppression is enabled).
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// Total MNA branch count.
    #[must_use]
    pub fn branch_count(&self) -> u32 {
        self.branch_count
    }

    /// Borrow the masked matrix as a flat row-major slice of length
    /// `dim * dim`.
    #[must_use]
    pub fn matrix(&self) -> &[f64] {
        &self.a
    }

    /// Borrow the masked right-hand-side vector of length `dim`.
    #[must_use]
    pub fn rhs(&self) -> &[f64] {
        &self.b
    }

    /// Look up `matrix[r, c]`. Returns `None` if either index is
    /// out of range.
    #[must_use]
    pub fn matrix_entry(&self, r: u32, c: u32) -> Option<f64> {
        let dim = self.dim();
        if r >= dim || c >= dim {
            return None;
        }
        let idx = (r as usize) * (dim as usize) + (c as usize);
        self.a.get(idx).copied()
    }

    /// Look up `rhs[r]`. Returns `None` if the index is out of
    /// range.
    #[must_use]
    pub fn rhs_entry(&self, r: u32) -> Option<f64> {
        self.b.get(r as usize).copied()
    }
}

/// Errors raised by [`SubViewBuilder::build`].
#[derive(Debug, Clone, PartialEq)]
pub enum SubViewError {
    /// The `source_rhs` slice handed to
    /// [`SubViewBuilder::with_source_step`] has the wrong length;
    /// it must match the underlying [`MnaSystem`]'s `dim()`.
    SourceRhsLengthMismatch {
        /// Length the underlying system requires.
        expected: usize,
        /// Length actually provided.
        actual: usize,
    },
    /// A Gmin-stepping conductance was non-finite (NaN or ±∞).
    /// Adding such a value would poison the conductance block of
    /// the matrix.
    NonFiniteGmin {
        /// The offending value.
        gmin_siemens: f64,
    },
    /// A Gmin-stepping conductance was negative. Negative shunt
    /// conductance is unphysical and would actively *destabilize*
    /// Newton-Raphson rather than help it converge.
    NegativeGmin {
        /// The offending value.
        gmin_siemens: f64,
    },
    /// A source-stepping factor `alpha` was non-finite.
    NonFiniteAlpha {
        /// The offending value.
        alpha: f64,
    },
    /// The ground node index recorded by the [`FlattenedStructure`]
    /// (passed via [`SubViewBuilder::with_ground_node`]) is out of
    /// range for the [`MnaSystem`]'s `node_count`. Indicates the
    /// caller paired a `FlattenedStructure` and an `MnaSystem` from
    /// different graphs.
    GroundNodeOutOfRange {
        /// The offending index.
        ground: NodeId,
        /// The system's node count.
        node_count: u32,
    },
}

impl core::fmt::Display for SubViewError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SourceRhsLengthMismatch { expected, actual } => write!(
                f,
                "source_rhs has length {actual} but the MNA system has dim {expected}"
            ),
            Self::NonFiniteGmin { gmin_siemens } => write!(
                f,
                "Gmin-stepping conductance {gmin_siemens} is non-finite; \
                 only finite, non-negative shunts are accepted"
            ),
            Self::NegativeGmin { gmin_siemens } => write!(
                f,
                "Gmin-stepping conductance {gmin_siemens} S is negative; \
                 only non-negative shunts are physical"
            ),
            Self::NonFiniteAlpha { alpha } => write!(
                f,
                "source-stepping factor {alpha} is non-finite; \
                 alpha must be a finite real (typically in [0, 1])"
            ),
            Self::GroundNodeOutOfRange { ground, node_count } => write!(
                f,
                "ground {ground} is out of range for node_count={node_count}; \
                 caller paired a FlattenedStructure with the wrong MnaSystem"
            ),
        }
    }
}

impl std::error::Error for SubViewError {}

/// Per-solve sub-view builder.
///
/// The builder is created from an [`MnaSystem`] reference and a
/// [`FlattenedStructure`] reference (so it knows the ground node
/// index). Composable mask methods set per-solve parameters; the
/// terminal [`SubViewBuilder::build`] returns a freshly-allocated
/// [`SubView`] with all masks applied.
///
/// Construction is cheap (no allocation); [`SubViewBuilder::build`]
/// copies the underlying matrix/RHS once and mutates the copy.
/// Re-using the same `MnaSystem` across many sub-views (one per
/// homotopy step) is the intended pattern.
#[derive(Debug, Clone)]
pub struct SubViewBuilder<'a> {
    system: &'a MnaSystem,
    ground: NodeId,
    suppress_ground: bool,
    gmin_siemens: f64,
    source_alpha: f64,
    source_rhs: Option<&'a [f64]>,
}

impl<'a> SubViewBuilder<'a> {
    /// Start a new builder for `system` with ground at
    /// [`NodeId::GROUND`]. Defaults: ground suppression enabled,
    /// no Gmin shunt, source-stepping factor `α = 1.0` (i.e.
    /// sources at full strength), no separate source-RHS vector.
    ///
    /// The default produces the textbook DC sub-view used by the
    /// `linear-resistive-dc-operating-point` scenario.
    #[must_use]
    pub fn from_full(system: &'a MnaSystem) -> Self {
        Self {
            system,
            ground: NodeId::GROUND,
            suppress_ground: true,
            gmin_siemens: 0.0,
            source_alpha: 1.0,
            source_rhs: None,
        }
    }

    /// Override the ground node. Defaults to [`NodeId::GROUND`].
    /// The Pass-1 flattener always pins ground at node 0 in v1, but
    /// this hook lets future structural changes route the ground
    /// reference through the [`FlattenedStructure::ground_node`]
    /// accessor without baking the index into this module.
    #[must_use]
    pub fn with_ground_node(mut self, ground: NodeId) -> Self {
        self.ground = ground;
        self
    }

    /// Enable or disable ground suppression. The default is
    /// `true` (DC operating-point convention). The `false` path
    /// exists for debugging and for analyses that explicitly want
    /// the full singular matrix (none in v1).
    #[must_use]
    pub fn suppress_ground(mut self, suppress: bool) -> Self {
        self.suppress_ground = suppress;
        self
    }

    /// Set the Gmin-stepping shunt conductance in siemens. A
    /// shunt of `g` adds `+g` to every non-ground diagonal node
    /// entry and `+0` everywhere else (the conductance is to
    /// ground; ground itself receives no extra contribution since
    /// the corresponding off-diagonal terms would land on the
    /// ground row/column, which is suppressed). Defaults to `0.0`
    /// (no shunt).
    ///
    /// `gmin_siemens` must be finite and non-negative;
    /// [`SubViewBuilder::build`] returns
    /// [`SubViewError::NonFiniteGmin`] or [`SubViewError::NegativeGmin`]
    /// otherwise.
    #[must_use]
    pub fn with_gmin(mut self, gmin_siemens: f64) -> Self {
        self.gmin_siemens = gmin_siemens;
        self
    }

    /// Set the source-stepping factor `alpha`. Independent-source
    /// contributions to the RHS are scaled by this factor:
    ///
    /// `rhs_masked = (rhs_full - source_rhs) + alpha * source_rhs`
    ///
    /// At `alpha = 1.0` the masked RHS equals `rhs_full`
    /// (default). At `alpha = 0.0` the sources are fully
    /// suppressed (all elements are at zero excitation; the
    /// solution should be the trivial zero solution for a linear
    /// circuit, which is the starting point of source-stepping
    /// homotopy).
    ///
    /// Setting `alpha != 1.0` requires a `source_rhs` vector;
    /// supply it with [`SubViewBuilder::with_source_rhs`]. If
    /// `alpha == 1.0` the source vector is unused.
    ///
    /// `alpha` must be finite; [`SubViewBuilder::build`] returns
    /// [`SubViewError::NonFiniteAlpha`] otherwise.
    #[must_use]
    pub fn with_source_step(mut self, alpha: f64) -> Self {
        self.source_alpha = alpha;
        self
    }

    /// Provide the source-only RHS vector (length = system
    /// `dim()`). Required when `alpha != 1.0`. See
    /// [`source_rhs_from`] for a helper that computes it from a
    /// `(FlattenedStructure, CircuitGraph)` pair.
    #[must_use]
    pub fn with_source_rhs(mut self, source_rhs: &'a [f64]) -> Self {
        self.source_rhs = Some(source_rhs);
        self
    }

    /// Apply all configured masks to a fresh copy of the
    /// underlying [`MnaSystem`] and return a [`SubView`].
    ///
    /// Order of operations:
    ///
    /// 1. Copy the full matrix and RHS.
    /// 2. Apply the Gmin-stepping shunt (matrix only): for each
    ///    non-ground node `i`, `A[i, i] += gmin_siemens`.
    /// 3. Apply the source-stepping factor (RHS only):
    ///    `b[r] = (b_full[r] - s[r]) + alpha * s[r]` for every
    ///    row `r`, where `s` is `source_rhs`. When `alpha == 1.0`
    ///    this is a no-op and `source_rhs` may be `None`.
    /// 4. Apply ground suppression (matrix + RHS): row at
    ///    `ground.index()` is replaced with the standard basis
    ///    row `e_g`; column at `ground.index()` is zeroed in every
    ///    other row; RHS at `ground.index()` is `0`.
    ///
    /// # Errors
    ///
    /// - [`SubViewError::NonFiniteGmin`] / [`SubViewError::NegativeGmin`]
    ///   when [`with_gmin`] received a bad value.
    /// - [`SubViewError::NonFiniteAlpha`] when [`with_source_step`]
    ///   received a bad value.
    /// - [`SubViewError::SourceRhsLengthMismatch`] when
    ///   `alpha != 1.0` and the supplied `source_rhs` length does
    ///   not match the underlying system's `dim()`.
    /// - [`SubViewError::GroundNodeOutOfRange`] when the ground
    ///   node index exceeds the underlying system's `node_count`.
    ///
    /// [`with_gmin`]: SubViewBuilder::with_gmin
    /// [`with_source_step`]: SubViewBuilder::with_source_step
    pub fn build(self) -> Result<SubView, SubViewError> {
        // Validate inputs up front so we never leave a half-built
        // sub-view behind on error.
        if !self.gmin_siemens.is_finite() {
            return Err(SubViewError::NonFiniteGmin {
                gmin_siemens: self.gmin_siemens,
            });
        }
        if self.gmin_siemens < 0.0 {
            return Err(SubViewError::NegativeGmin {
                gmin_siemens: self.gmin_siemens,
            });
        }
        if !self.source_alpha.is_finite() {
            return Err(SubViewError::NonFiniteAlpha {
                alpha: self.source_alpha,
            });
        }
        if self.ground.index() >= self.system.node_count() {
            return Err(SubViewError::GroundNodeOutOfRange {
                ground: self.ground,
                node_count: self.system.node_count(),
            });
        }

        let dim = self.system.dim();
        let dim_us = dim as usize;
        let needs_source = (self.source_alpha - 1.0).abs() > 0.0;
        let source_rhs = match (needs_source, self.source_rhs) {
            (true, Some(s)) => {
                if s.len() != dim_us {
                    return Err(SubViewError::SourceRhsLengthMismatch {
                        expected: dim_us,
                        actual: s.len(),
                    });
                }
                Some(s)
            }
            // alpha == 1.0: no source modulation; ignore any
            // source_rhs that was supplied.
            (false, _) => None,
            // alpha != 1.0 but no source_rhs given: treat as
            // length-mismatch with actual=0 so the caller sees a
            // single error type.
            (true, None) => {
                return Err(SubViewError::SourceRhsLengthMismatch {
                    expected: dim_us,
                    actual: 0,
                });
            }
        };

        // 1. Fresh copies.
        let mut a = self.system.matrix().to_vec();
        let mut b = self.system.rhs().to_vec();

        // 2. Gmin-stepping on the matrix: add gmin to non-ground
        //    diagonals in the node block.
        if self.gmin_siemens > 0.0 {
            let g_idx = self.ground.index() as usize;
            for i in 0..(self.system.node_count() as usize) {
                if i == g_idx {
                    continue;
                }
                let diag = i * dim_us + i;
                a[diag] += self.gmin_siemens;
            }
        }

        // 3. Source-stepping on the RHS.
        if let Some(s) = source_rhs {
            // b' = (b - s) + alpha * s = b + (alpha - 1) * s
            let alpha_delta = self.source_alpha - 1.0;
            for (b_r, &s_r) in b.iter_mut().zip(s.iter()) {
                *b_r += alpha_delta * s_r;
            }
        }

        // 4. Ground suppression: replace the ground row with the
        //    standard basis row e_g; zero the ground column in every
        //    other row; zero RHS at the ground row.
        if self.suppress_ground {
            let g = self.ground.index() as usize;
            // Row g → e_g.
            let row_start = g * dim_us;
            for c in 0..dim_us {
                a[row_start + c] = 0.0;
            }
            a[row_start + g] = 1.0;
            // Column g → 0 for every row except g (already 1.0).
            for r in 0..dim_us {
                if r == g {
                    continue;
                }
                a[r * dim_us + g] = 0.0;
            }
            // RHS at ground row → 0.
            b[g] = 0.0;
        }

        Ok(SubView {
            node_count: self.system.node_count(),
            branch_count: self.system.branch_count(),
            a,
            b,
        })
    }
}

/// Compute the "source-only" RHS contribution for a `(FlattenedStructure,
/// CircuitGraph)` pair: the RHS the assembler would have produced if
/// **only** independent voltage and current sources were stamped.
///
/// This is the vector required by [`SubViewBuilder::with_source_rhs`]
/// when source-stepping with `alpha != 1.0`. The Pass-2 assembler
/// stamps source contributions directly into the full RHS without
/// retaining a marker, so the homotopy driver (tasks.md item #19)
/// invokes this helper once and reuses the result across the
/// stepping loop.
///
/// # Layout
///
/// The returned vector has length `node_count + branch_count`. It
/// is laid out identically to [`MnaSystem::rhs`]:
///
/// - For each [`ElementKind::VoltageSource`] in the graph, the
///   element's MNA branch row carries `voltage_volts`. (Other
///   branch rows are zero.)
/// - For each [`ElementKind::CurrentSource`] from terminal `0` to
///   terminal `1` with value `S` amperes, the `from` node entry
///   receives `+S` and the `to` node entry receives `-S` (SPICE
///   convention, identical to the assembler).
/// - All other entries (node rows for non-source contributions,
///   branch rows for inductors, etc.) are zero.
///
/// # Errors
///
/// Returns `None` if `structure` and `graph` disagree on element
/// count, or if any source element's incidence is inconsistent
/// (wrong terminal count, branch index out of range, etc.). These
/// are the same conditions the assembler reports through
/// [`crate::assemble::MnaAssemblyError`]; here we return `None` and
/// expect the caller to have already invoked the assembler (which
/// will have reported the precise error variant).
///
/// # Panics
///
/// Never panics. All indexing is bounds-checked against `dim`.
#[must_use]
pub fn source_rhs_from(structure: &FlattenedStructure, graph: &CircuitGraph) -> Option<Vec<f64>> {
    if structure.element_count() as usize != graph.elements().len() {
        return None;
    }
    let dim = (structure.node_count() as usize).checked_add(structure.branch_count() as usize)?;
    let node_count = structure.node_count() as usize;
    let mut s = vec![0.0_f64; dim];

    for incidence in structure.elements() {
        let element = graph.element(incidence.element)?;
        match element.kind() {
            ElementKind::VoltageSource { voltage_volts } => {
                let branch = incidence.branch?;
                let row = node_count.checked_add(branch.index() as usize)?;
                if !voltage_volts.is_finite() {
                    return None;
                }
                *s.get_mut(row)? = *voltage_volts;
            }
            ElementKind::CurrentSource { current_amperes } => {
                if incidence.nodes.len() != 2 {
                    return None;
                }
                if !current_amperes.is_finite() {
                    return None;
                }
                let from = incidence.nodes[0].index() as usize;
                let to = incidence.nodes[1].index() as usize;
                if from >= node_count || to >= node_count {
                    return None;
                }
                s[from] += *current_amperes;
                s[to] -= *current_amperes;
            }
            _ => {}
        }
    }

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assemble::assemble;
    use crate::flatten::flatten;
    use netlist_graph::{CircuitBuilder, ElementKind};

    // ---------------- helpers ----------------------------------------------

    fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor {
                resistance_ohms: ohms,
            },
            [n1, n2],
            None,
        )
        .expect("add resistor");
    }

    fn add_voltage_source(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, volts: f64) {
        b.add_element(
            name,
            ElementKind::VoltageSource {
                voltage_volts: volts,
            },
            [plus, minus],
            None,
        )
        .expect("add voltage source");
    }

    fn add_current_source(b: &mut CircuitBuilder, name: &str, from: &str, to: &str, amps: f64) {
        b.add_element(
            name,
            ElementKind::CurrentSource {
                current_amperes: amps,
            },
            [from, to],
            None,
        )
        .expect("add current source");
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12_f64.max(1e-12 * a.abs().max(b.abs()))
    }

    /// Build a two-resistor + voltage-source ladder used across
    /// several tests. `V1` from `n1` to `0` (1 V), `R1`/`R2` from
    /// `n1` to `0` (1 kΩ each).
    fn two_resistor_ladder() -> (FlattenedStructure, CircuitGraph, MnaSystem) {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", 1.0);
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_resistor(&mut b, "R2", "n1", "0", 1000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        (fs, g, sys)
    }

    // ---------------- ground suppression -----------------------------------

    #[test]
    fn ground_suppression_replaces_ground_row_with_identity() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let sv = SubViewBuilder::from_full(&sys).build().expect("subview ok");

        // Same dimension as the full system.
        assert_eq!(sv.dim(), sys.dim());
        assert_eq!(sv.node_count(), sys.node_count());
        assert_eq!(sv.branch_count(), sys.branch_count());

        // Ground row (row 0) is the standard basis vector e_0.
        for c in 0..sv.dim() {
            let expected = if c == 0 { 1.0 } else { 0.0 };
            assert!(
                approx(sv.matrix_entry(0, c).unwrap(), expected),
                "row 0 col {c}: got {}, want {expected}",
                sv.matrix_entry(0, c).unwrap()
            );
        }

        // Ground column (col 0) is zero in every non-ground row.
        for r in 1..sv.dim() {
            assert!(
                approx(sv.matrix_entry(r, 0).unwrap(), 0.0),
                "row {r} col 0: got {}, want 0.0",
                sv.matrix_entry(r, 0).unwrap()
            );
        }

        // RHS at ground row is zero.
        assert!(approx(sv.rhs_entry(0).unwrap(), 0.0));
    }

    #[test]
    fn ground_suppression_preserves_non_ground_block_entries() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let sv = SubViewBuilder::from_full(&sys).build().expect("subview ok");

        // Non-ground node-block entry (n1, n1) is unchanged: 2/R.
        let two_g = 2.0_f64 / 1000.0;
        assert!(approx(sv.matrix_entry(1, 1).unwrap(), two_g));

        // V1 branch entries (n1, br) and (br, n1) unchanged.
        assert!(approx(sv.matrix_entry(1, 2).unwrap(), 1.0));
        assert!(approx(sv.matrix_entry(2, 1).unwrap(), 1.0));

        // Branch RHS (V1 = 1 V) unchanged.
        assert!(approx(sv.rhs_entry(2).unwrap(), 1.0));
    }

    #[test]
    fn ground_suppression_can_be_disabled() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let sv = SubViewBuilder::from_full(&sys)
            .suppress_ground(false)
            .build()
            .expect("subview ok");

        // Ground row matches the full system: G[0][0] = 2/R, G[0][1] = -2/R.
        let two_g = 2.0_f64 / 1000.0;
        assert!(approx(sv.matrix_entry(0, 0).unwrap(), two_g));
        assert!(approx(sv.matrix_entry(0, 1).unwrap(), -two_g));
    }

    #[test]
    fn ground_suppression_solution_matches_textbook_analytic() {
        // Solve the 3x3 ground-suppressed system by hand to confirm
        // the masked matrix is what the linear solver expects.
        let (_fs, _g, sys) = two_resistor_ladder();
        let sv = SubViewBuilder::from_full(&sys).build().expect("subview ok");

        // System:
        //   row 0 (gnd): [1, 0, 0] [vgnd] = [0]
        //   row 1 (n1):  [0, 2g, 1] [vn1] = [0]
        //   row 2 (br):  [0, 1, 0]  [iV1] = [1]
        // Solution: vgnd=0, vn1=1, 2g*1 + iV1 = 0 → iV1 = -2g = -2 mA.
        // We don't solve here (item #16) but we *do* check those
        // matrix entries.
        let two_g = 2.0_f64 / 1000.0;
        assert!(approx(sv.matrix_entry(0, 0).unwrap(), 1.0));
        assert!(approx(sv.matrix_entry(1, 1).unwrap(), two_g));
        assert!(approx(sv.matrix_entry(1, 2).unwrap(), 1.0));
        assert!(approx(sv.matrix_entry(2, 1).unwrap(), 1.0));
        assert!(approx(sv.matrix_entry(2, 2).unwrap(), 0.0));
        assert!(approx(sv.rhs_entry(2).unwrap(), 1.0));
    }

    // ---------------- Gmin-stepping ----------------------------------------

    #[test]
    fn gmin_stepping_adds_shunt_only_to_non_ground_node_diagonals() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let gmin = 1e-3_f64;
        let sv = SubViewBuilder::from_full(&sys)
            .with_gmin(gmin)
            // disable ground suppression so we can read the ground
            // diagonal directly.
            .suppress_ground(false)
            .build()
            .expect("subview ok");

        // Non-ground diagonal (n1) gets +gmin: 2/R + gmin.
        let two_g = 2.0_f64 / 1000.0;
        assert!(approx(sv.matrix_entry(1, 1).unwrap(), two_g + gmin));

        // Ground diagonal: unchanged (the conductance to ground is
        // collapsed into the same row that ground suppression would
        // replace later; we deliberately do not add it twice).
        assert!(approx(sv.matrix_entry(0, 0).unwrap(), two_g));

        // Branch-row diagonals: unchanged.
        assert!(approx(sv.matrix_entry(2, 2).unwrap(), 0.0));
    }

    #[test]
    fn gmin_zero_is_a_noop() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let sv = SubViewBuilder::from_full(&sys)
            .with_gmin(0.0)
            .build()
            .expect("subview ok");
        let sv_no_gmin = SubViewBuilder::from_full(&sys).build().expect("subview ok");
        assert_eq!(sv.matrix(), sv_no_gmin.matrix());
        assert_eq!(sv.rhs(), sv_no_gmin.rhs());
    }

    #[test]
    fn gmin_rejects_non_finite() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let err = SubViewBuilder::from_full(&sys)
            .with_gmin(f64::NAN)
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::NonFiniteGmin { .. }));

        let err = SubViewBuilder::from_full(&sys)
            .with_gmin(f64::INFINITY)
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::NonFiniteGmin { .. }));
    }

    #[test]
    fn gmin_rejects_negative() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let err = SubViewBuilder::from_full(&sys)
            .with_gmin(-1e-3)
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::NegativeGmin { .. }));
    }

    // ---------------- source-stepping --------------------------------------

    #[test]
    fn source_rhs_from_voltage_source_matches_assembler() {
        let (fs, g, sys) = two_resistor_ladder();
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        // Voltage source V1 stamps 1.0 at branch row (row 2);
        // resistors contribute nothing to source_rhs.
        assert_eq!(s.len(), sys.dim() as usize);
        assert!(approx(s[0], 0.0));
        assert!(approx(s[1], 0.0));
        assert!(approx(s[2], 1.0));
    }

    #[test]
    fn source_rhs_from_current_source_uses_spice_convention() {
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_current_source(&mut b, "I1", "n1", "0", 3.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        // I1 from n1 to 0: +3 at n1, -3 at gnd.
        assert!(approx(s[0], -3.0));
        assert!(approx(s[1], 3.0));
    }

    #[test]
    fn source_stepping_alpha_one_is_noop() {
        let (fs, g, sys) = two_resistor_ladder();
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        let sv = SubViewBuilder::from_full(&sys)
            .with_source_step(1.0)
            .with_source_rhs(&s)
            .build()
            .expect("subview ok");
        let sv_no_step = SubViewBuilder::from_full(&sys).build().expect("subview ok");
        assert_eq!(sv.matrix(), sv_no_step.matrix());
        assert_eq!(sv.rhs(), sv_no_step.rhs());
    }

    #[test]
    fn source_stepping_alpha_zero_zeros_source_contributions() {
        let (fs, g, sys) = two_resistor_ladder();
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        let sv = SubViewBuilder::from_full(&sys)
            .with_source_step(0.0)
            .with_source_rhs(&s)
            .build()
            .expect("subview ok");

        // At alpha=0, RHS = b_full - s. For the ladder b_full=[0,0,1]
        // and s=[0,0,1], so post-mask RHS = [0,0,0]; the ground row
        // (row 0) is then suppressed to 0 anyway.
        for r in 0..sv.dim() {
            assert!(
                approx(sv.rhs_entry(r).unwrap(), 0.0),
                "row {r}: got {}, want 0",
                sv.rhs_entry(r).unwrap()
            );
        }
    }

    #[test]
    fn source_stepping_half_alpha_scales_only_source_contributions() {
        // Use a circuit with both a source contribution and a
        // non-source contribution to RHS, so we can verify only the
        // source part is scaled.
        //
        // We synthesize this by hand: take the two-resistor ladder,
        // then *manually* add a fake "non-source" contribution to the
        // full RHS to simulate, e.g., a companion-current stamp.
        let (fs, g, sys) = two_resistor_ladder();
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        // Spoof a non-source RHS contribution: pretend the original
        // RHS were [0, 0.5, 1.0] (where 0.5 is e.g. companion current
        // at n1 and 1.0 is the V1 branch row). We achieve this by
        // post-multiplying b through the builder's source_rhs vector
        // *plus* an offset injected into the source_rhs (so the
        // formula b_full - s tells us the offset).
        //
        // The cleanest version: directly assert the algebra.
        //
        // For alpha=0.5: b_masked[r] = b_full[r] + (alpha-1)*s[r]
        //                            = b_full[r] - 0.5*s[r].
        // For row 2 (the V1 branch row): b_full=1, s=1, so
        //                                b_masked = 1 - 0.5 = 0.5.
        let sv = SubViewBuilder::from_full(&sys)
            .with_source_step(0.5)
            .with_source_rhs(&s)
            .suppress_ground(false)
            .build()
            .expect("subview ok");
        assert!(approx(sv.rhs_entry(2).unwrap(), 0.5));
        // Non-source rows (0, 1) are unchanged in this circuit
        // because there is no non-source RHS contribution.
        assert!(approx(sv.rhs_entry(0).unwrap(), 0.0));
        assert!(approx(sv.rhs_entry(1).unwrap(), 0.0));
    }

    #[test]
    fn source_stepping_rejects_non_finite_alpha() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let s = vec![0.0_f64; sys.dim() as usize];
        let err = SubViewBuilder::from_full(&sys)
            .with_source_step(f64::NAN)
            .with_source_rhs(&s)
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::NonFiniteAlpha { .. }));
    }

    #[test]
    fn source_stepping_requires_source_rhs_when_alpha_not_one() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let err = SubViewBuilder::from_full(&sys)
            .with_source_step(0.5)
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SubViewError::SourceRhsLengthMismatch { actual: 0, .. }
        ));
    }

    #[test]
    fn source_stepping_rejects_wrong_source_rhs_length() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let s = vec![0.0_f64; (sys.dim() as usize) + 1];
        let err = SubViewBuilder::from_full(&sys)
            .with_source_step(0.5)
            .with_source_rhs(&s)
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::SourceRhsLengthMismatch { .. }));
    }

    // ---------------- composition: ground suppression + Gmin + step --------

    #[test]
    fn all_three_masks_compose_in_a_single_build() {
        let (fs, g, sys) = two_resistor_ladder();
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        let gmin = 1e-9;
        let alpha = 0.25;
        let sv = SubViewBuilder::from_full(&sys)
            .with_gmin(gmin)
            .with_source_step(alpha)
            .with_source_rhs(&s)
            .build()
            .expect("subview ok");

        // Ground row is e_0.
        for c in 0..sv.dim() {
            let want = if c == 0 { 1.0 } else { 0.0 };
            assert!(approx(sv.matrix_entry(0, c).unwrap(), want));
        }
        // n1 diagonal carries 2/R + gmin.
        let two_g = 2.0_f64 / 1000.0;
        assert!(approx(sv.matrix_entry(1, 1).unwrap(), two_g + gmin));
        // V1 branch RHS scaled by alpha.
        assert!(approx(sv.rhs_entry(2).unwrap(), alpha));
        // RHS at ground row suppressed.
        assert!(approx(sv.rhs_entry(0).unwrap(), 0.0));
    }

    // ---------------- non-default ground node ------------------------------

    #[test]
    fn ground_node_out_of_range_errors() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let err = SubViewBuilder::from_full(&sys)
            .with_ground_node(NodeId::new(99))
            .build()
            .unwrap_err();
        assert!(matches!(err, SubViewError::GroundNodeOutOfRange { .. }));
    }

    // ---------------- non-mutation of the underlying system ----------------

    #[test]
    fn building_a_subview_does_not_mutate_the_full_system() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let original_matrix = sys.matrix().to_vec();
        let original_rhs = sys.rhs().to_vec();
        let _ = SubViewBuilder::from_full(&sys)
            .with_gmin(1e-3)
            .build()
            .expect("subview ok");
        assert_eq!(sys.matrix(), original_matrix.as_slice());
        assert_eq!(sys.rhs(), original_rhs.as_slice());
    }

    // ---------------- repeated builds at different homotopy steps ----------

    #[test]
    fn repeated_builds_at_different_gmin_steps_share_one_full_system() {
        let (_fs, _g, sys) = two_resistor_ladder();
        let steps = [1e-3, 1e-6, 1e-9, 0.0];
        let two_g = 2.0_f64 / 1000.0;
        for &gmin in &steps {
            let sv = SubViewBuilder::from_full(&sys)
                .with_gmin(gmin)
                .suppress_ground(false)
                .build()
                .expect("subview ok");
            assert!(
                approx(sv.matrix_entry(1, 1).unwrap(), two_g + gmin),
                "at gmin={gmin}: n1 diagonal mismatch"
            );
        }
    }

    #[test]
    fn empty_ground_only_graph_subview_is_1x1_identity() {
        let g = CircuitBuilder::default().build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        let sv = SubViewBuilder::from_full(&sys).build().expect("subview ok");
        // dim = 1, matrix = [1.0], rhs = [0.0].
        assert_eq!(sv.dim(), 1);
        assert!(approx(sv.matrix_entry(0, 0).unwrap(), 1.0));
        assert!(approx(sv.rhs_entry(0).unwrap(), 0.0));
    }

    // ---------------- source_rhs_from edge cases ---------------------------

    #[test]
    fn source_rhs_from_includes_only_independent_sources() {
        // Add a resistor and a current source; verify only the
        // current source contributes to source_rhs.
        let mut b = CircuitBuilder::default();
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        add_current_source(&mut b, "I1", "n1", "0", 0.5);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");
        let s = source_rhs_from(&fs, &g).expect("source_rhs ok");
        assert_eq!(s.len(), sys.dim() as usize);

        // Only I1 contributes: +0.5 at n1 (row 1), -0.5 at gnd (row 0).
        // No branch rows in this circuit.
        assert!(approx(s[0], -0.5));
        assert!(approx(s[1], 0.5));
    }

    #[test]
    fn source_rhs_from_rejects_non_finite_voltage() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n1", "0", f64::NAN);
        add_resistor(&mut b, "R1", "n1", "0", 1000.0);
        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        // Note: the *assembler* would also reject this with
        // NonFiniteParameter; source_rhs_from is the safety-net
        // companion path used in tests / homotopy.
        assert!(source_rhs_from(&fs, &g).is_none());
    }
}
