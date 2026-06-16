//! Mutable MNA matrix view passed to [`DeviceModel`](crate::traits::DeviceModel) stamp methods.
//!
//! [`MnaMatrix`] is the writable interface that the nonlinear stamp
//! methods operate on.  It exposes only the two operations a device
//! stamp ever needs:
//!
//! - [`MnaMatrix::add_element`] — accumulate a value into matrix entry
//!   `(row, col)`.
//! - [`MnaMatrix::add_rhs`] — accumulate a value into the right-hand-side
//!   vector at index `row`.
//!
//! This thin wrapper exists so that the [`crate::traits::DeviceModel`]
//! trait does not carry a dependency on the `numeric-solver` crate's
//! dense `MnaSystem` type, which would create a circular dependency
//! (`device-modeling` ↔ `numeric-solver`).  A caller in `numeric-solver`
//! can cheaply wrap its internal `Vec<f64>` buffers in an `MnaMatrix`
//! reference for each stamp call.
//!
//! # Layout
//!
//! The matrix is stored externally by the caller as a flat row-major
//! `Vec<f64>` of length `dim * dim`, and the RHS as a `Vec<f64>` of
//! length `dim`.  `MnaMatrix` holds mutable slices into those buffers
//! with the stride (`dim`) embedded so that `add_element(r, c, v)`
//! can compute the flat index `r * dim + c` directly.
//!
//! Out-of-bounds accesses are detected via `debug_assert!` and cause a
//! panic in debug builds; in release builds the slice write is bounds-
//! checked by Rust's normal slice indexing.

/// Mutable view over an MNA matrix and its RHS vector, passed to
/// [`DeviceModel::stamp_linear`](crate::traits::DeviceModel::stamp_linear) and
/// [`DeviceModel::stamp_nonlinear`](crate::traits::DeviceModel::stamp_nonlinear).
pub struct MnaMatrix<'a> {
    /// Flat row-major backing slice of length `dim * dim`.
    a: &'a mut [f64],
    /// Right-hand-side backing slice of length `dim`.
    b: &'a mut [f64],
    /// Stride (== total dimension of the square matrix).
    dim: usize,
}

impl<'a> MnaMatrix<'a> {
    /// Construct an `MnaMatrix` from mutable slices and the stride.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if `a.len() != dim * dim` or
    /// `b.len() != dim`.
    #[must_use]
    pub fn new(a: &'a mut [f64], b: &'a mut [f64], dim: usize) -> Self {
        debug_assert_eq!(
            a.len(),
            dim.saturating_mul(dim),
            "MnaMatrix: a.len() must equal dim*dim"
        );
        debug_assert_eq!(b.len(), dim, "MnaMatrix: b.len() must equal dim");
        Self { a, b, dim }
    }

    /// The stride / total dimension of the system.
    #[must_use]
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Accumulate `value` into matrix entry `(row, col)`.
    ///
    /// `row` and `col` are zero-based indices into the full MNA matrix
    /// (node rows first, then branch rows).
    ///
    /// # Panics
    ///
    /// Panics if `row >= dim` or `col >= dim`.
    #[inline]
    pub fn add_element(&mut self, row: usize, col: usize, value: f64) {
        let idx = row * self.dim + col;
        self.a[idx] += value;
    }

    /// Accumulate `value` into right-hand-side entry `row`.
    ///
    /// # Panics
    ///
    /// Panics if `row >= dim`.
    #[inline]
    pub fn add_rhs(&mut self, row: usize, value: f64) {
        self.b[row] += value;
    }

    /// Read matrix entry `(row, col)`.  Intended for tests and
    /// diagnostic tooling; stamp code should only need [`add_element`](Self::add_element).
    ///
    /// # Panics
    ///
    /// Panics if `row >= dim` or `col >= dim`.
    #[must_use]
    #[inline]
    pub fn element(&self, row: usize, col: usize) -> f64 {
        self.a[row * self.dim + col]
    }

    /// Read RHS entry `row`.
    ///
    /// # Panics
    ///
    /// Panics if `row >= dim`.
    #[must_use]
    #[inline]
    pub fn rhs(&self, row: usize) -> f64 {
        self.b[row]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_element_accumulates() {
        let mut a = vec![0.0_f64; 4]; // 2x2
        let mut b = vec![0.0_f64; 2];
        let mut m = MnaMatrix::new(&mut a, &mut b, 2);
        m.add_element(0, 0, 1.0);
        m.add_element(0, 0, 0.5);
        assert_eq!(m.element(0, 0), 1.5);
    }

    #[test]
    fn add_rhs_accumulates() {
        let mut a = vec![0.0_f64; 4];
        let mut b = vec![0.0_f64; 2];
        let mut m = MnaMatrix::new(&mut a, &mut b, 2);
        m.add_rhs(1, 3.0);
        m.add_rhs(1, -1.0);
        assert_eq!(m.rhs(1), 2.0);
    }

    #[test]
    fn off_diagonal_is_independent() {
        let mut a = vec![0.0_f64; 9]; // 3x3
        let mut b = vec![0.0_f64; 3];
        let mut m = MnaMatrix::new(&mut a, &mut b, 3);
        m.add_element(0, 2, 5.0);
        assert_eq!(m.element(0, 2), 5.0);
        assert_eq!(m.element(2, 0), 0.0);
    }
}
