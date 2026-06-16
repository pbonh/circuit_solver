//! Sparse MNA (Modified Nodal Analysis) matrix using a COO accumulator that
//! converts to CSR on demand.
//!
//! The typical workflow is:
//! 1. Create an `MnaMatrix` with the desired dimension.
//! 2. Call `stamp` / `stamp_rhs` for every circuit element.
//! 3. Call `to_csr()` to obtain a `CsrMatrix` for the solver.
//! 4. Call `reset()` between simulation steps to reuse allocated storage.

/// A COO (Coordinate) triplet accumulating the MNA conductance matrix and
/// right-hand-side vector.
#[derive(Debug, Default)]
pub struct MnaMatrix {
    /// Number of rows / columns (square matrix).
    size: usize,
    /// Raw COO storage: (row, col, val) triples.  Multiple entries for the
    /// same (row, col) pair are summed during CSR conversion.
    entries: Vec<(usize, usize, f64)>,
    /// Right-hand-side vector; indexed by row.
    rhs: Vec<f64>,
}

impl MnaMatrix {
    /// Create a new zero-valued `MnaMatrix` of dimension `size × size`.
    pub fn new(size: usize) -> Self {
        MnaMatrix {
            size,
            entries: Vec::new(),
            rhs: vec![0.0; size],
        }
    }

    /// Number of rows (and columns) in the matrix.
    pub fn size(&self) -> usize {
        self.size
    }

    /// **Accumulate** `val` into entry `(row, col)`.
    ///
    /// Unlike a direct write, repeated stamps on the same entry are summed —
    /// this is the correct MNA behaviour (e.g. two resistors sharing a node).
    ///
    /// # Panics
    /// Panics in debug builds if `row` or `col` ≥ `self.size()`.
    pub fn stamp(&mut self, row: usize, col: usize, val: f64) {
        debug_assert!(row < self.size, "row {row} out of range (size={})", self.size);
        debug_assert!(col < self.size, "col {col} out of range (size={})", self.size);
        self.entries.push((row, col, val));
    }

    /// **Accumulate** `val` into the right-hand-side vector at index `row`.
    ///
    /// # Panics
    /// Panics in debug builds if `row` ≥ `self.size()`.
    pub fn stamp_rhs(&mut self, row: usize, val: f64) {
        debug_assert!(row < self.size, "row {row} out of range (size={})", self.size);
        self.rhs[row] += val;
    }

    /// Clear all accumulated values **without** reallocating.
    ///
    /// After `reset()` the matrix is logically zero; the internal `Vec`
    /// capacity is preserved so subsequent stamps avoid heap allocations.
    pub fn reset(&mut self) {
        self.entries.clear();
        for v in &mut self.rhs {
            *v = 0.0;
        }
    }

    /// Convert the accumulated COO entries to a [`CsrMatrix`].
    ///
    /// Duplicate `(row, col)` entries are **summed** (standard MNA stamp
    /// behaviour).
    pub fn to_csr(&self) -> CsrMatrix {
        let n = self.size;

        // --- step 1: count nnz per row -----------------------------------
        let mut row_nnz = vec![0usize; n];
        for &(r, c, _) in &self.entries {
            // Count each unique (r,c) pair once by using a temporary dense
            // approach: first sum into a dense n×n buffer, then compress.
            // For the matrix sizes typical in SPICE-class circuits this is
            // acceptable; a production path would use hash-maps or sorted
            // triplets.
            let _ = c; // suppress lint; used via dense_vals below
            row_nnz[r] += 1; // overcount — corrected after dedup below
        }

        // --- step 2: accumulate into a dense temporary -------------------
        // Using a flat Vec<f64> of length n*n avoids a HashMap dependency.
        // For large circuits (n > ~10 000) a caller should provide a
        // pre-sorted COO; this path is correct for all sizes.
        let mut dense = vec![0.0f64; n * n];
        for &(r, c, v) in &self.entries {
            dense[r * n + c] += v;
        }

        // --- step 3: compress to CSR ------------------------------------
        let mut row_ptr = vec![0usize; n + 1];
        for r in 0..n {
            let mut cnt = 0usize;
            for c in 0..n {
                if dense[r * n + c] != 0.0 {
                    cnt += 1;
                }
            }
            row_ptr[r + 1] = row_ptr[r] + cnt;
        }
        let nnz = row_ptr[n];
        let mut col_idx = vec![0usize; nnz];
        let mut values = vec![0.0f64; nnz];
        let mut ptr = 0usize;
        for r in 0..n {
            for c in 0..n {
                let v = dense[r * n + c];
                if v != 0.0 {
                    col_idx[ptr] = c;
                    values[ptr] = v;
                    ptr += 1;
                }
            }
        }

        CsrMatrix {
            size: n,
            row_ptr,
            col_idx,
            values,
            rhs: self.rhs.clone(),
        }
    }
}

/// A CSR (Compressed Sparse Row) matrix produced by [`MnaMatrix::to_csr`].
///
/// Holds both the conductance matrix and the right-hand-side vector so the
/// solver receives a single self-contained object.
#[derive(Debug, Clone)]
pub struct CsrMatrix {
    /// Dimension (square).
    pub size: usize,
    /// Row pointer array (length `size + 1`).
    pub row_ptr: Vec<usize>,
    /// Column indices of non-zero entries.
    pub col_idx: Vec<usize>,
    /// Non-zero values (same length as `col_idx`).
    pub values: Vec<f64>,
    /// Right-hand-side vector (length `size`).
    pub rhs: Vec<f64>,
}

impl CsrMatrix {
    /// Return the value at `(row, col)`, or `0.0` if the entry is structural
    /// zero.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        let start = self.row_ptr[row];
        let end = self.row_ptr[row + 1];
        for i in start..end {
            if self.col_idx[i] == col {
                return self.values[i];
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stamp a 2-node resistor (conductance G) in MNA form and verify the
    /// resulting CSR matrix.
    ///
    /// For a resistor between node 0 (ground) and node 1 with conductance G:
    ///
    ///   G matrix stamp:
    ///     [+G  -G]
    ///     [-G  +G]
    ///
    /// (In a grounded circuit only the 1×1 submatrix [+G] matters, but we
    /// stamp the full 2×2 here to exercise both stamp directions.)
    #[test]
    fn resistor_stamp_2x2() {
        let g = 0.5_f64; // conductance = 1 / 2 Ω
        let mut m = MnaMatrix::new(2);

        // Positive conductance on diagonal entries.
        m.stamp(0, 0, g);
        m.stamp(1, 1, g);
        // Negative off-diagonal entries.
        m.stamp(0, 1, -g);
        m.stamp(1, 0, -g);

        let csr = m.to_csr();

        assert_eq!(csr.size, 2);
        assert!((csr.get(0, 0) - g).abs() < 1e-12, "G[0,0] should be {g}");
        assert!((csr.get(1, 1) - g).abs() < 1e-12, "G[1,1] should be {g}");
        assert!((csr.get(0, 1) + g).abs() < 1e-12, "G[0,1] should be -{g}");
        assert!((csr.get(1, 0) + g).abs() < 1e-12, "G[1,0] should be -{g}");
    }

    /// Stamping the same entry twice must sum (not overwrite).
    #[test]
    fn stamp_accumulates() {
        let mut m = MnaMatrix::new(2);
        m.stamp(0, 0, 1.0);
        m.stamp(0, 0, 2.0);
        let csr = m.to_csr();
        assert!((csr.get(0, 0) - 3.0).abs() < 1e-12, "should accumulate to 3.0");
    }

    /// stamp_rhs should accumulate into the rhs vector.
    #[test]
    fn stamp_rhs_accumulates() {
        let mut m = MnaMatrix::new(3);
        m.stamp_rhs(1, 5.0);
        m.stamp_rhs(1, 3.0);
        let csr = m.to_csr();
        assert!((csr.rhs[1] - 8.0).abs() < 1e-12, "rhs[1] should be 8.0");
        assert!((csr.rhs[0]).abs() < 1e-12, "rhs[0] should be 0.0");
    }

    /// reset() must clear accumulated values without reallocating.
    #[test]
    fn reset_clears_values() {
        let mut m = MnaMatrix::new(2);
        m.stamp(0, 0, 1.0);
        m.stamp_rhs(0, 2.0);
        m.reset();
        let csr = m.to_csr();
        assert!((csr.get(0, 0)).abs() < 1e-12, "matrix should be zero after reset");
        assert!((csr.rhs[0]).abs() < 1e-12, "rhs should be zero after reset");
    }

    /// reset() preserves Vec capacity (no reallocation).
    #[test]
    fn reset_preserves_capacity() {
        let mut m = MnaMatrix::new(4);
        // Stamp many entries to force a heap allocation.
        for i in 0..4 {
            m.stamp(i, i, 1.0);
        }
        let cap_before = m.entries.capacity();
        m.reset();
        assert_eq!(m.entries.capacity(), cap_before, "capacity should not shrink");
    }
}
