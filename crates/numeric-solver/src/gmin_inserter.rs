//! Gmin diagonal shunting for nonlinear-device terminals (US-020).
//!
//! This module implements **Gmin insertion** — a Jacobian
//! regularization technique that adds a small conductance
//! `g_min` to every MNA diagonal entry that corresponds to a
//! terminal node of a nonlinear (semiconductor) device.
//!
//! # Why only nonlinear terminals?
//!
//! The full-circuit Gmin shunt in [`SubViewBuilder::with_gmin`]
//! adds `g_min` to *every* non-ground node row. That is sufficient
//! for homotopy (where `g_min` is walked back to zero), but for
//! the **static regularization** case — where a small residual
//! `g_min` is permanently left in the Jacobian to guard against
//! ill-conditioning at a single operating point — it is more
//! targeted and physically natural to shunt only the nodes that
//! are terminals of nonlinear devices. Linear passive elements
//! already contribute finite diagonal conductance; nonlinear
//! devices can produce near-zero or zero Jacobian diagonal
//! entries near singularities (e.g., a reverse-biased diode
//! whose dynamic conductance is negligible).
//!
//! # Design
//!
//! [`GminInserter`] is a plain configuration value. Its
//! [`GminInserter::apply`] method takes a reference to an
//! existing [`MnaSystem`] and returns a **new** `MnaSystem`
//! with the Gmin entries added. The original system is
//! **not modified** — `apply` clones the matrix and RHS
//! before augmenting. This makes `GminInserter` safe to
//! call multiple times (e.g., once per NR iteration if the
//! assembler re-stamps the Jacobian).
//!
//! # Scope of the shunt
//!
//! For each element whose [`netlist_graph::ElementKind`] is
//! [`netlist_graph::ElementKind::Semiconductor`], every terminal
//! node listed in its [`FlattenedStructure`] incidence record
//! receives `+g_min` on its matrix diagonal. Terminal nodes
//! that appear in more than one nonlinear device receive the
//! shunt once per device (i.e., the shunt is **per-terminal-per-
//! device**, not per-node), reflecting that each device
//! independently contributes to the node's KCL equation.
//!
//! # Example
//!
//! ```no_run
//! # use numeric_solver::assemble::assemble;
//! # use numeric_solver::flatten::flatten;
//! # use numeric_solver::gmin_inserter::GminInserter;
//! # use netlist_graph::CircuitBuilder;
//! # fn main() { }
//! ```
//!
//! # Honored ADRs
//!
//! - **ADR-0003** — the shunt is applied *after* assembly and
//!   before the sub-view extractor, so the two-pass boundary is
//!   preserved.
//! - **ADR-0005** — the closed-enum dispatch on
//!   `ElementKind::Semiconductor` mirrors the assembler's own
//!   dispatch, keeping the two in sync.
//! - **ADR-0010** — the exported types are part of the v1
//!   unstable public surface.

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::ElementId;
use netlist_graph::{CircuitGraph, ElementKind};

use crate::assemble::MnaSystem;

/// Default Gmin regularization conductance in siemens (1 pS).
///
/// This matches the SPICE-conventional residual floor used by
/// [`crate::gmin_stepping::GminSchedule::SPICE_DEFAULTS`] and
/// most commercial SPICE implementations.
pub const DEFAULT_GMIN_SIEMENS: f64 = 1e-12;

/// Gmin insertion configuration for nonlinear-device terminal
/// diagonal regularization.
///
/// # Construction
///
/// Use [`GminInserter::default()`] for the SPICE-conventional
/// 1 pS shunt, or supply an explicit value with
/// [`GminInserter::new`].
///
/// # Validity
///
/// `gmin_siemens` must be finite and non-negative. Zero is
/// allowed (a no-op inserter); negative values are rejected
/// by [`GminInserter::apply`] at call time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GminInserter {
    /// Conductance in siemens added to the MNA diagonal for
    /// each nonlinear-device terminal. Default: `1e-12`.
    pub gmin_siemens: f64,
}

impl GminInserter {
    /// Construct a [`GminInserter`] with the given conductance.
    ///
    /// # Errors
    ///
    /// Returns [`GminInserterError::NonFiniteGmin`] if
    /// `gmin_siemens` is NaN or ±∞.
    /// Returns [`GminInserterError::NegativeGmin`] if
    /// `gmin_siemens` is negative.
    pub fn new(gmin_siemens: f64) -> Result<Self, GminInserterError> {
        if !gmin_siemens.is_finite() {
            return Err(GminInserterError::NonFiniteGmin { gmin_siemens });
        }
        if gmin_siemens < 0.0 {
            return Err(GminInserterError::NegativeGmin { gmin_siemens });
        }
        Ok(Self { gmin_siemens })
    }

    /// Apply Gmin insertion to the full MNA system.
    ///
    /// Returns a new [`MnaSystem`] that is a copy of `system`
    /// with `gmin_siemens` added to the diagonal entry
    /// `A[t, t]` for every terminal node `t` of every
    /// [`ElementKind::Semiconductor`] element in `graph`.
    ///
    /// The original `system` is **not modified**; the returned
    /// system is a fresh allocation with the shunt values folded
    /// in additively.
    ///
    /// # Arguments
    ///
    /// * `system` — full MNA matrix produced by
    ///   [`crate::assemble::assemble`].
    /// * `flat` — [`FlattenedStructure`] from Pass 1, used to
    ///   map element indices to their terminal [`NodeId`]s.
    /// * `graph` — [`CircuitGraph`] used to identify which
    ///   elements are `Semiconductor`.
    ///
    /// # Errors
    ///
    /// Returns [`GminInserterError::NonFiniteGmin`] or
    /// [`GminInserterError::NegativeGmin`] if the configured
    /// value is invalid (construction via [`GminInserter::new`]
    /// prevents this; can only fire if a caller mutated
    /// `gmin_siemens` directly after construction).
    /// Returns [`GminInserterError::GraphFlattenMismatch`] if
    /// `flat` and `graph` have a different element count.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if a terminal node index falls outside
    /// the MNA matrix dimension (which would indicate a corrupted
    /// [`FlattenedStructure`]). Also panics if the element count of
    /// `flat` exceeds `u32::MAX` (not expected in practice).
    ///
    /// # Example
    ///
    /// A diode circuit whose zero-linearization Jacobian is
    /// singular (all-zero diagonal at the diode nodes) becomes
    /// non-singular after [`GminInserter::apply`]:
    ///
    /// ```no_run
    /// # use numeric_solver::assemble::assemble;
    /// # use numeric_solver::flatten::flatten;
    /// # use numeric_solver::gmin_inserter::GminInserter;
    /// # use netlist_graph::CircuitBuilder;
    /// # fn main() { }
    /// ```
    pub fn apply(
        &self,
        system: &MnaSystem,
        flat: &FlattenedStructure,
        graph: &CircuitGraph,
    ) -> Result<MnaSystem, GminInserterError> {
        // Validate inputs.
        if !self.gmin_siemens.is_finite() {
            return Err(GminInserterError::NonFiniteGmin {
                gmin_siemens: self.gmin_siemens,
            });
        }
        if self.gmin_siemens < 0.0 {
            return Err(GminInserterError::NegativeGmin {
                gmin_siemens: self.gmin_siemens,
            });
        }

        let flat_count = flat.element_count() as usize;
        let graph_count = graph.elements().len();
        if flat_count != graph_count {
            return Err(GminInserterError::GraphFlattenMismatch {
                flat_count: u32::try_from(flat_count)
                    .unwrap_or(u32::MAX),
                graph_count: u32::try_from(graph_count)
                    .unwrap_or(u32::MAX),
            });
        }

        // Clone the matrix and RHS — do not modify the original.
        let mut a = system.matrix().to_vec();
        let dim = system.dim() as usize;

        if self.gmin_siemens == 0.0 || dim == 0 {
            // No-op: return a fresh clone with the same contents.
            return Ok(system.clone_with_matrix(a, system.rhs().to_vec()));
        }

        // Walk every element in the graph. For each Semiconductor
        // element, retrieve its terminal nodes from the flattened
        // structure and add gmin to the corresponding diagonal.
        for (elem_idx, element) in graph.elements().iter().enumerate() {
            if !matches!(element.kind(), ElementKind::Semiconductor) {
                continue;
            }
            let elem_id = ElementId::new(u32::try_from(elem_idx).expect("element index fits u32"));
            // O(1) lookup via ElementId — flat and graph are
            // validated to have the same element count above.
            let incidence = flat
                .element(elem_id)
                .expect("element index in flat is in range");

            for &node in &incidence.nodes {
                let i = node.index() as usize;
                // node.index() < node_count ≤ dim by construction
                // (FlattenedStructure validates node indices at
                // build time). A diagnostic assertion is used in
                // debug builds; in release, the bounds check on
                // `a[diag]` provides the safety guarantee.
                debug_assert!(i < dim, "terminal node index {i} out of dim {dim}");
                let diag = i * dim + i;
                a[diag] += self.gmin_siemens;
            }
        }

        Ok(system.clone_with_matrix(a, system.rhs().to_vec()))
    }
}

impl Default for GminInserter {
    fn default() -> Self {
        Self {
            gmin_siemens: DEFAULT_GMIN_SIEMENS,
        }
    }
}

/// Errors returned by [`GminInserter::apply`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GminInserterError {
    /// `gmin_siemens` was NaN or ±∞.
    NonFiniteGmin {
        /// The offending value.
        gmin_siemens: f64,
    },
    /// `gmin_siemens` was negative.
    NegativeGmin {
        /// The offending value.
        gmin_siemens: f64,
    },
    /// The supplied [`FlattenedStructure`] and [`CircuitGraph`]
    /// have a different element count, indicating a mismatch.
    GraphFlattenMismatch {
        /// Element count reported by the flattened structure.
        flat_count: u32,
        /// Element count reported by the graph.
        graph_count: u32,
    },
}

impl core::fmt::Display for GminInserterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteGmin { gmin_siemens } => write!(
                f,
                "gmin-inserter: gmin_siemens {gmin_siemens} is non-finite"
            ),
            Self::NegativeGmin { gmin_siemens } => write!(
                f,
                "gmin-inserter: gmin_siemens {gmin_siemens} is negative"
            ),
            Self::GraphFlattenMismatch {
                flat_count,
                graph_count,
            } => write!(
                f,
                "gmin-inserter: flat element count {flat_count} != graph element count {graph_count}"
            ),
        }
    }
}

impl std::error::Error for GminInserterError {}

#[cfg(test)]
mod tests {
    use device_modeling::stamp::{DiodeLinearization, LinearizedModel};
    use netlist_graph::{CircuitBuilder, ElementKind};

    use crate::assemble::assemble;
    use crate::flatten::flatten;

    use super::*;

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------

    fn add_semiconductor(b: &mut CircuitBuilder, name: &str, terminals: &[&str], model: &str) {
        b.add_element(
            name,
            ElementKind::Semiconductor,
            terminals.to_vec(),
            Some(model.into()),
        )
        .expect("add semiconductor");
    }

    fn add_resistor(b: &mut CircuitBuilder, name: &str, a: &str, z: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor { resistance_ohms: ohms },
            [a, z],
            None,
        )
        .expect("add resistor");
    }

    fn add_vsrc(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, v: f64) {
        b.add_element(
            name,
            ElementKind::VoltageSource { voltage_volts: v },
            [plus, minus],
            None,
        )
        .expect("add voltage source");
    }

    // ------------------------------------------------------------------
    // Test: default config is valid.
    // ------------------------------------------------------------------

    #[test]
    fn default_gmin_inserter_is_1e12_siemens() {
        let gi = GminInserter::default();
        assert_eq!(gi.gmin_siemens, DEFAULT_GMIN_SIEMENS);
        assert_eq!(gi.gmin_siemens, 1e-12);
    }

    // ------------------------------------------------------------------
    // Test: validation rejects bad inputs.
    // ------------------------------------------------------------------

    #[test]
    fn new_rejects_non_finite_gmin() {
        assert!(matches!(
            GminInserter::new(f64::NAN),
            Err(GminInserterError::NonFiniteGmin { .. })
        ));
        assert!(matches!(
            GminInserter::new(f64::INFINITY),
            Err(GminInserterError::NonFiniteGmin { .. })
        ));
    }

    #[test]
    fn new_rejects_negative_gmin() {
        assert!(matches!(
            GminInserter::new(-1e-12),
            Err(GminInserterError::NegativeGmin { .. })
        ));
    }

    #[test]
    fn new_accepts_zero_gmin() {
        assert!(GminInserter::new(0.0).is_ok());
    }

    // ------------------------------------------------------------------
    // Test: zero-linearization diode circuit — singular before insertion,
    // non-singular (well-conditioned) after.
    //
    // Circuit: D1 between n_a (anode) and n_c (cathode).
    // At zero-linearization the diode stamps nothing, so the assembled
    // matrix diagonal at n_a and n_c is zero (singular Jacobian).
    //
    // This is the acceptance-criterion test from US-020: the Gmin-
    // augmented matrix of a simple diode circuit must be non-singular
    // when plain NR would be singular.
    // ------------------------------------------------------------------

    #[test]
    fn diode_circuit_diagonal_is_zero_before_insertion() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let sys = assemble(&fs, &g, &[Some(lin)]).unwrap();

        // node order: 0=gnd, 1=n_a, 2=n_c.
        assert_eq!(sys.matrix_entry(1, 1), Some(0.0));
        assert_eq!(sys.matrix_entry(2, 2), Some(0.0));
    }

    #[test]
    fn diode_circuit_diagonal_is_gmin_after_insertion() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let sys = assemble(&fs, &g, &[Some(lin)]).unwrap();

        let gi = GminInserter::default();
        let augmented = gi.apply(&sys, &fs, &g).unwrap();

        // Diagonal at anode (node 1) and cathode (node 2) are now gmin.
        assert_eq!(augmented.matrix_entry(1, 1), Some(1e-12));
        assert_eq!(augmented.matrix_entry(2, 2), Some(1e-12));
        // Ground node (0,0) is untouched.
        assert_eq!(augmented.matrix_entry(0, 0), Some(0.0));
        // Off-diagonal entries are unchanged.
        assert_eq!(augmented.matrix_entry(1, 2), sys.matrix_entry(1, 2));
        assert_eq!(augmented.matrix_entry(2, 1), sys.matrix_entry(2, 1));
    }

    #[test]
    fn original_system_is_not_modified_by_apply() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let sys = assemble(&fs, &g, &[Some(lin)]).unwrap();

        let original_diag = sys.matrix_entry(1, 1).unwrap();
        let gi = GminInserter::default();
        let _augmented = gi.apply(&sys, &fs, &g).unwrap();

        // Original is unchanged.
        assert_eq!(sys.matrix_entry(1, 1), Some(original_diag));
    }

    // ------------------------------------------------------------------
    // Test: non-singular after insertion — the augmented system is
    // solvable (produces finite solution).
    //
    // Circuit: V1 biases a diode D1 (zero-linearization) with a
    // load resistor R1. Without Gmin the diode diagonal is zero →
    // singular. With Gmin the diagonal is 1e-12 → non-singular.
    // ------------------------------------------------------------------

    #[test]
    fn gmin_augmented_diode_matrix_is_non_singular() {
        use crate::sub_view::SubViewBuilder;
        use faer::prelude::SpSolver;
        use faer::Mat;

        let mut b = CircuitBuilder::default();
        add_vsrc(&mut b, "V1", "n1", "0", 5.0);
        add_resistor(&mut b, "R1", "n2", "0", 1000.0);
        add_semiconductor(&mut b, "D1", &["n1", "n2"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();

        // Element order: V1=0, R1=1, D1=2.
        // Only the semiconductor (D1 at index 2) needs a linearization.
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let lins = vec![None, None, Some(lin)];
        let sys = assemble(&fs, &g, &lins).unwrap();

        let gi = GminInserter::default();
        let augmented = gi.apply(&sys, &fs, &g).unwrap();

        let sv = SubViewBuilder::from_full(&augmented)
            .suppress_ground(true)
            .build()
            .unwrap();

        let dim = sv.dim() as usize;
        let a_mat = Mat::<f64>::from_fn(dim, dim, |r, c| {
            sv.matrix_entry(r as u32, c as u32).unwrap_or(0.0)
        });
        let rhs_mat =
            Mat::<f64>::from_fn(dim, 1, |r, _| sv.rhs_entry(r as u32).unwrap_or(0.0));

        let sol = a_mat.partial_piv_lu().solve(&rhs_mat);
        for r in 0..dim {
            assert!(
                sol.read(r, 0).is_finite(),
                "solution at row {r} must be finite (non-singular)"
            );
        }
    }

    // ------------------------------------------------------------------
    // Test: linear-only circuit is unchanged (no semiconductor = no shunt).
    // ------------------------------------------------------------------

    #[test]
    fn linear_circuit_is_unmodified() {
        let mut b = CircuitBuilder::default();
        add_vsrc(&mut b, "V1", "n1", "0", 5.0);
        add_resistor(&mut b, "R1", "n1", "n2", 1000.0);
        add_resistor(&mut b, "R2", "n2", "0", 1000.0);
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let sys = assemble(&fs, &g, &[]).unwrap();

        let gi = GminInserter::default();
        let augmented = gi.apply(&sys, &fs, &g).unwrap();

        assert_eq!(sys.matrix(), augmented.matrix());
        assert_eq!(sys.rhs(), augmented.rhs());
    }

    // ------------------------------------------------------------------
    // Test: zero-gmin inserter is a no-op.
    // ------------------------------------------------------------------

    #[test]
    fn zero_gmin_is_a_noop() {
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n_a", "n_c"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let sys = assemble(&fs, &g, &[Some(lin)]).unwrap();

        let gi = GminInserter::new(0.0).unwrap();
        let augmented = gi.apply(&sys, &fs, &g).unwrap();
        assert_eq!(sys.matrix(), augmented.matrix());
        assert_eq!(sys.rhs(), augmented.rhs());
    }

    // ------------------------------------------------------------------
    // Test: two diodes sharing a node each contribute gmin.
    // ------------------------------------------------------------------

    #[test]
    fn two_diodes_sharing_a_node_each_contribute_gmin() {
        // D1: n1→n2, D2: n2→n3.
        // Node n2 is a terminal of both D1 and D2 → 2×gmin on diagonal.
        let mut b = CircuitBuilder::default();
        add_semiconductor(&mut b, "D1", &["n1", "n2"], "D");
        add_semiconductor(&mut b, "D2", &["n2", "n3"], "D");
        let g = b.build().unwrap();
        let fs = flatten(&g).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let lins = vec![Some(lin.clone()), Some(lin)];
        let sys = assemble(&fs, &g, &lins).unwrap();

        let gi = GminInserter::default();
        let augmented = gi.apply(&sys, &fs, &g).unwrap();

        // n1: terminal of D1 only → 1e-12.
        assert!((augmented.matrix_entry(1, 1).unwrap() - 1e-12).abs() < 1e-20);
        // n2: terminal of both D1 and D2 → 2e-12.
        assert!((augmented.matrix_entry(2, 2).unwrap() - 2e-12).abs() < 1e-20);
        // n3: terminal of D2 only → 1e-12.
        assert!((augmented.matrix_entry(3, 3).unwrap() - 1e-12).abs() < 1e-20);
    }

    // ------------------------------------------------------------------
    // Test: error surface — graph/flat mismatch is reported.
    // ------------------------------------------------------------------

    #[test]
    fn graph_flat_mismatch_returns_error() {
        let mut b1 = CircuitBuilder::default();
        add_semiconductor(&mut b1, "D1", &["n_a", "n_c"], "D");
        let g1 = b1.build().unwrap();
        let fs1 = flatten(&g1).unwrap();
        let lin = LinearizedModel::Diode(DiodeLinearization::zero());
        let sys = assemble(&fs1, &g1, &[Some(lin)]).unwrap();

        // A second graph with 2 elements.
        let mut b2 = CircuitBuilder::default();
        add_semiconductor(&mut b2, "D1", &["n_a", "n_c"], "D");
        add_semiconductor(&mut b2, "D2", &["n_c", "n_b"], "D");
        let g2 = b2.build().unwrap();

        let gi = GminInserter::default();
        let result = gi.apply(&sys, &fs1, &g2);
        assert!(matches!(
            result,
            Err(GminInserterError::GraphFlattenMismatch {
                flat_count: 1,
                graph_count: 2
            })
        ));
    }
}
