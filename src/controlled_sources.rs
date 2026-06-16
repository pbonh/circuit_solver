//! MNA stamp functions for the four SPICE controlled-source types.
//!
//! ## Background
//!
//! Modified Nodal Analysis augments the `n`-node conductance matrix `G` with
//! extra rows and columns for every element that introduces a branch-current
//! unknown.  Controlled sources are the canonical example:
//!
//! | Element | Extra rows | Stamp pattern |
//! |---------|-----------|---------------|
//! | VCCS G_m | 0 | Four cross-entries via `g_m` |
//! | VCVS E   | 1 (branch current `I_E`) | KVL + coupling |
//! | CCCS F   | 1 (branch current `I_sense` from sensing V-source) | Coupling only |
//! | CCVS H   | 2 (one per element) | Both KVL + coupling |
//!
//! The caller is responsible for allocating the `MnaMatrix` with the correct
//! extended size (number of nodes + number of branch-current unknowns) before
//! calling these functions.  Each function documents the required size.

use crate::MnaMatrix;

// ── VCCS ─────────────────────────────────────────────────────────────────────

/// Stamp a **Voltage-Controlled Current Source** (VCCS / `G`-element) into the
/// MNA conductance matrix.
///
/// The controlled current `I = g_m * (V_nc_pos - V_nc_neg)` is injected from
/// `n_neg` to `n_pos` (conventional current flows into `n_pos`).
///
/// ## Stamp pattern
///
/// ```text
///           nc+   nc-
/// n+  [  +gm   -gm ]
/// n-  [  -gm   +gm ]
/// ```
///
/// ## Parameters
/// - `mat`   — MNA conductance matrix (size ≥ max(n_pos, n_neg, nc_pos, nc_neg) + 1)
/// - `n_pos`, `n_neg`  — output node indices
/// - `nc_pos`, `nc_neg` — controlling (sense) node indices
/// - `g_m`  — transconductance in Siemens
///
/// Ground nodes (index 0) are silently skipped per standard MNA convention.
pub fn stamp_vccs(
    mat: &mut MnaMatrix,
    n_pos: usize,
    n_neg: usize,
    nc_pos: usize,
    nc_neg: usize,
    g_m: f64,
) {
    // (n_pos, nc_pos)
    if n_pos != 0 && nc_pos != 0 {
        mat.stamp(n_pos - 1, nc_pos - 1, g_m);
    }
    // (n_pos, nc_neg)
    if n_pos != 0 && nc_neg != 0 {
        mat.stamp(n_pos - 1, nc_neg - 1, -g_m);
    }
    // (n_neg, nc_pos)
    if n_neg != 0 && nc_pos != 0 {
        mat.stamp(n_neg - 1, nc_pos - 1, -g_m);
    }
    // (n_neg, nc_neg)
    if n_neg != 0 && nc_neg != 0 {
        mat.stamp(n_neg - 1, nc_neg - 1, g_m);
    }
}

// ── VCVS ─────────────────────────────────────────────────────────────────────

/// Stamp a **Voltage-Controlled Voltage Source** (VCVS / `E`-element).
///
/// The VCVS introduces a branch-current unknown `I_E`.  The MNA matrix must
/// be pre-sized to `n_nodes + (number of VCVS elements)` so that `j_row`
/// addresses a valid row/column.
///
/// ## Stamp pattern (0-indexed, using `node - 1` for non-ground nodes)
///
/// KVL constraint row (`j_row`):
/// ```text
///   V(n_pos) - V(n_neg) - gain*V(nc_pos) + gain*V(nc_neg) = 0
/// ```
///
/// Current injection:
/// ```text
///   n_pos row, j_col: +1   (I_E flows into n_pos)
///   n_neg row, j_col: -1   (I_E flows out of n_neg)
///   j_row, n_pos col: +1
///   j_row, n_neg col: -1
///   j_row, nc_pos col: -gain
///   j_row, nc_neg col: +gain
/// ```
///
/// ## Parameters
/// - `n_pos`, `n_neg`  — output node indices (1-based; 0 = ground)
/// - `nc_pos`, `nc_neg` — controlling node indices
/// - `gain`  — voltage gain (dimensionless)
/// - `j_row` — 0-indexed row/column in `mat` for this element's branch current
pub fn stamp_vcvs(
    mat: &mut MnaMatrix,
    n_pos: usize,
    n_neg: usize,
    nc_pos: usize,
    nc_neg: usize,
    gain: f64,
    j_row: usize,
) {
    // Current injection columns
    if n_pos != 0 {
        mat.stamp(n_pos - 1, j_row, 1.0);
        mat.stamp(j_row, n_pos - 1, 1.0);
    }
    if n_neg != 0 {
        mat.stamp(n_neg - 1, j_row, -1.0);
        mat.stamp(j_row, n_neg - 1, -1.0);
    }
    // Controlling voltage terms in the KVL row
    if nc_pos != 0 {
        mat.stamp(j_row, nc_pos - 1, -gain);
    }
    if nc_neg != 0 {
        mat.stamp(j_row, nc_neg - 1, gain);
    }
}

// ── CCCS ─────────────────────────────────────────────────────────────────────

/// Stamp a **Current-Controlled Current Source** (CCCS / `F`-element).
///
/// The CCCS reuses the branch-current unknown `I_sense` already introduced by
/// the sensing voltage source (`Vname`).  No new branch-current row is added
/// for `F` itself; `j_sense` references that existing row.
///
/// ## Stamp pattern
///
/// ```text
///   n_pos row, j_sense col: +beta
///   n_neg row, j_sense col: -beta
/// ```
///
/// ## Parameters
/// - `n_pos`, `n_neg`  — output node indices
/// - `j_sense` — 0-indexed column of the sensing V-source's branch current
/// - `beta`    — current gain (dimensionless)
pub fn stamp_cccs(
    mat: &mut MnaMatrix,
    n_pos: usize,
    n_neg: usize,
    j_sense: usize,
    beta: f64,
) {
    if n_pos != 0 {
        mat.stamp(n_pos - 1, j_sense, beta);
    }
    if n_neg != 0 {
        mat.stamp(n_neg - 1, j_sense, -beta);
    }
}

// ── CCVS ─────────────────────────────────────────────────────────────────────

/// Stamp a **Current-Controlled Voltage Source** (CCVS / `H`-element).
///
/// A CCVS introduces *its own* branch-current unknown `I_H` (at `j_row`) and
/// also reads the sensing branch current `I_sense` at `j_sense`.  Both
/// voltage-source rows must be pre-allocated in the matrix.
///
/// ## Stamp pattern
///
/// KVL constraint row (`j_row`):
/// ```text
///   V(n_pos) - V(n_neg) - r_m * I_sense = 0
/// ```
///
/// Current injection:
/// ```text
///   n_pos row, j_row col: +1
///   n_neg row, j_row col: -1
///   j_row, n_pos col:     +1
///   j_row, n_neg col:     -1
///   j_row, j_sense col:   -r_m
/// ```
///
/// ## Parameters
/// - `n_pos`, `n_neg`  — output node indices
/// - `j_row`   — 0-indexed row/column for this CCVS's branch current `I_H`
/// - `j_sense` — 0-indexed column for the sensing V-source's branch current
/// - `r_m`     — transresistance in Ohms
pub fn stamp_ccvs(
    mat: &mut MnaMatrix,
    n_pos: usize,
    n_neg: usize,
    j_row: usize,
    j_sense: usize,
    r_m: f64,
) {
    if n_pos != 0 {
        mat.stamp(n_pos - 1, j_row, 1.0);
        mat.stamp(j_row, n_pos - 1, 1.0);
    }
    if n_neg != 0 {
        mat.stamp(n_neg - 1, j_row, -1.0);
        mat.stamp(j_row, n_neg - 1, -1.0);
    }
    // Transresistance coupling: -r_m * I_sense in KVL row
    mat.stamp(j_row, j_sense, -r_m);
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── VCCS ─────────────────────────────────────────────────────────────────

    /// Sedra-Smith Example: VCCS with g_m = 0.04 S between nodes 1,2 controlled
    /// by node voltage at nodes 3,4.
    ///
    /// Expected stamp (1-indexed textbook notation):
    ///   G[0,2] += +gm    (row n_pos-1=0, col nc_pos-1=2)
    ///   G[0,3] += -gm    (row n_pos-1=0, col nc_neg-1=3)
    ///   G[1,2] += -gm    (row n_neg-1=1, col nc_pos-1=2)
    ///   G[1,3] += +gm    (row n_neg-1=1, col nc_neg-1=3)
    ///
    /// All other entries must remain zero.
    #[test]
    fn vccs_sedra_smith_stamp() {
        let g_m = 0.04_f64; // 40 mS
        // 4 nodes (indices 1-4 → matrix size 4)
        let n_pos = 1usize;
        let n_neg = 2usize;
        let nc_pos = 3usize;
        let nc_neg = 4usize;

        let mut mat = MnaMatrix::new(4);
        stamp_vccs(&mut mat, n_pos, n_neg, nc_pos, nc_neg, g_m);
        let csr = mat.to_csr();

        // Cross-entries as per Sedra-Smith MNA stamp table
        assert!(
            (csr.get(n_pos - 1, nc_pos - 1) - g_m).abs() < 1e-12,
            "G[n+,nc+] should be +gm"
        );
        assert!(
            (csr.get(n_pos - 1, nc_neg - 1) + g_m).abs() < 1e-12,
            "G[n+,nc-] should be -gm"
        );
        assert!(
            (csr.get(n_neg - 1, nc_pos - 1) + g_m).abs() < 1e-12,
            "G[n-,nc+] should be -gm"
        );
        assert!(
            (csr.get(n_neg - 1, nc_neg - 1) - g_m).abs() < 1e-12,
            "G[n-,nc-] should be +gm"
        );

        // Diagonal entries must be untouched (VCCS does not stamp diagonal)
        assert!((csr.get(0, 0)).abs() < 1e-12, "diagonal should be 0");
        assert!((csr.get(1, 1)).abs() < 1e-12, "diagonal should be 0");
    }

    /// VCCS with ground nodes skips those stamps.
    #[test]
    fn vccs_ground_nodes_skipped() {
        let g_m = 0.5_f64;
        // n_neg = 0 (ground), nc_neg = 0 (ground)
        // Only two non-ground stamps should appear: (n_pos-1, nc_pos-1)
        let mut mat = MnaMatrix::new(2);
        stamp_vccs(&mut mat, 1, 0, 2, 0, g_m);
        let csr = mat.to_csr();
        assert!((csr.get(0, 1) - g_m).abs() < 1e-12);
        // All other entries zero
        assert!((csr.get(1, 0)).abs() < 1e-12);
        assert!((csr.get(0, 0)).abs() < 1e-12);
        assert!((csr.get(1, 1)).abs() < 1e-12);
    }

    // ── VCVS ─────────────────────────────────────────────────────────────────

    /// VCVS with gain=10: verify KVL row and current-injection column entries.
    ///
    /// Circuit: nodes 1,2 (output), nodes 3,4 (control).
    /// j_row = 4 (index into extended matrix, 0-based).
    /// Matrix size = 5 (4 nodes + 1 branch current).
    #[test]
    fn vcvs_stamp_entries() {
        let gain = 10.0_f64;
        let n_pos = 1usize;
        let n_neg = 2usize;
        let nc_pos = 3usize;
        let nc_neg = 4usize;
        let j_row = 4usize; // extended branch-current row

        let mut mat = MnaMatrix::new(5);
        stamp_vcvs(&mut mat, n_pos, n_neg, nc_pos, nc_neg, gain, j_row);
        let csr = mat.to_csr();

        // Current injection: n_pos↔j_row
        assert!((csr.get(n_pos - 1, j_row) - 1.0).abs() < 1e-12, "G[n+, j] = +1");
        assert!((csr.get(j_row, n_pos - 1) - 1.0).abs() < 1e-12, "G[j, n+] = +1");
        // Current injection: n_neg↔j_row
        assert!((csr.get(n_neg - 1, j_row) + 1.0).abs() < 1e-12, "G[n-, j] = -1");
        assert!((csr.get(j_row, n_neg - 1) + 1.0).abs() < 1e-12, "G[j, n-] = -1");
        // KVL controlling voltage: -gain at (j_row, nc_pos-1)
        assert!(
            (csr.get(j_row, nc_pos - 1) + gain).abs() < 1e-12,
            "G[j, nc+] = -gain"
        );
        // KVL controlling voltage: +gain at (j_row, nc_neg-1)
        assert!(
            (csr.get(j_row, nc_neg - 1) - gain).abs() < 1e-12,
            "G[j, nc-] = +gain"
        );
    }

    // ── CCCS ─────────────────────────────────────────────────────────────────

    /// CCCS with beta=5: verify output current injection via j_sense column.
    ///
    /// Circuit: nodes 1,2 (output), j_sense = 2 (0-indexed branch current col).
    /// Matrix size = 3 (2 nodes + 1 sensing branch current).
    #[test]
    fn cccs_stamp_entries() {
        let beta = 5.0_f64;
        let n_pos = 1usize;
        let n_neg = 2usize;
        let j_sense = 2usize; // 0-based index in extended matrix

        let mut mat = MnaMatrix::new(3);
        stamp_cccs(&mut mat, n_pos, n_neg, j_sense, beta);
        let csr = mat.to_csr();

        assert!((csr.get(n_pos - 1, j_sense) - beta).abs() < 1e-12, "G[n+, j] = +beta");
        assert!((csr.get(n_neg - 1, j_sense) + beta).abs() < 1e-12, "G[n-, j] = -beta");
    }

    // ── CCVS ─────────────────────────────────────────────────────────────────

    /// CCVS with r_m=100: verify KVL row and current injection.
    ///
    /// Circuit: nodes 1,2 (output). j_row=2, j_sense=3.
    /// Matrix size = 4 (2 nodes + 2 branch currents).
    #[test]
    fn ccvs_stamp_entries() {
        let r_m = 100.0_f64;
        let n_pos = 1usize;
        let n_neg = 2usize;
        let j_row = 2usize;
        let j_sense = 3usize;

        let mut mat = MnaMatrix::new(4);
        stamp_ccvs(&mut mat, n_pos, n_neg, j_row, j_sense, r_m);
        let csr = mat.to_csr();

        // Current injection
        assert!((csr.get(n_pos - 1, j_row) - 1.0).abs() < 1e-12, "G[n+, j_row]=+1");
        assert!((csr.get(j_row, n_pos - 1) - 1.0).abs() < 1e-12, "G[j_row, n+]=+1");
        assert!((csr.get(n_neg - 1, j_row) + 1.0).abs() < 1e-12, "G[n-, j_row]=-1");
        assert!((csr.get(j_row, n_neg - 1) + 1.0).abs() < 1e-12, "G[j_row, n-]=-1");
        // Transresistance coupling
        assert!((csr.get(j_row, j_sense) + r_m).abs() < 1e-12, "G[j_row, j_sense]=-r_m");
    }
}
