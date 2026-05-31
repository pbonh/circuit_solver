//! PyO3-adjacent result container types for the frontend crate.
//!
//! This module provides Rust-level result containers that carry raw data
//! vectors in a layout suitable for zero-copy transfer to NumPy ownership
//! at the Python boundary. The types here are pure Rust — no `pyo3` or
//! `numpy` dependency is needed — because the actual `Py<PyArray1<f64>>`
//! construction happens in the binding crate (`circuit-solver-py`).
//!
//! # Why this module exists (task #26)
//!
//! The spec scenario `frontend-contract#results-zero-copy-numpy` requires
//! that when Python accesses analysis results as NumPy arrays, the arrays
//! are zero-copy views backed by Rust-owned memory. The existing
//! `PyAnalysisResult` stored DC scalars (node voltages, branch currents)
//! as `BTreeMap<String, f64>`, which does not admit zero-copy NumPy access
//! because the values are scattered across the map's tree structure.
//!
//! This module introduces [`NamedScalarData`] — parallel `(names, values)`
//! vectors — which provides a contiguous `values: Vec<f64>` that the
//! binding crate can transfer to NumPy ownership via
//! `numpy::PyArray1::from_vec` in a single ownership move, yielding a
//! zero-copy `numpy.ndarray` view.
//!
//! # ADR alignment
//!
//! - **ADR-0001** — The binding crate depends only on the frontend crate.
//!   `SimulationResult` lives in the frontend so the binding can import it
//!   without reaching into `analysis-orchestration` directly.
//! - **ADR-0010** — The public Rust API surface is unstable for v1. These
//!   types are `pub` within the workspace but not SemVer-guaranteed.

use std::collections::BTreeMap;

/// Named scalar data — parallel `(names, values)` vectors for a DC scalar
/// channel (node voltages or branch currents).
///
/// The `values` vector is contiguous heap memory that can be transferred
/// to NumPy ownership via `numpy::PyArray1::from_vec`, yielding a zero-copy
/// `numpy.ndarray(dtype=float64)` view. The `names` vector provides the
/// corresponding node/element names in the same index order, enabling
/// by-name lookup on the Python side.
///
/// # Invariants
///
/// - `names.len() == values.len()` (maintained by construction).
/// - `names` is in sorted order (derived from `BTreeMap` iteration).
/// - All values are finite (validated at the orchestration boundary).
#[derive(Debug, Clone)]
pub struct NamedScalarData {
    /// Node or element names, in sorted order.
    pub names: Vec<String>,
    /// Corresponding values (volts for nodes, amperes for branches).
    pub values: Vec<f64>,
}

impl NamedScalarData {
    /// Construct from a name-keyed map, extracting names and values in
    /// sorted order (matching the `BTreeMap` iteration order).
    ///
    /// # Panics
    ///
    /// Does not panic; the input map is assumed to contain only finite
    /// values (validated upstream).
    #[must_use]
    pub fn from_sorted_map(map: &BTreeMap<String, f64>) -> Self {
        let mut names = Vec::with_capacity(map.len());
        let mut values = Vec::with_capacity(map.len());
        for (name, value) in map {
            names.push(name.clone());
            values.push(*value);
        }
        Self { names, values }
    }

    /// Convert into a name-keyed `BTreeMap` (inverse of
    /// [`Self::from_sorted_map`]).
    #[must_use]
    pub fn into_map(self) -> BTreeMap<String, f64> {
        self.names.into_iter().zip(self.values).collect()
    }

    /// Number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether the channel carries no data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Complete simulation result carrying all four output channels.
///
/// This is the Rust-level container that flows from the analysis
/// orchestration layer through the frontend crate to the PyO3 binding.
/// The `values` vectors in each [`NamedScalarData`] and the `Vec<f64>`
/// elements of waveform/transfer-function channels are transferred to
/// NumPy ownership at the Python boundary, yielding zero-copy
/// `numpy.ndarray` views per the `frontend-contract#results-zero-copy-numpy`
/// spec scenario.
///
/// # Ownership flow
///
/// ```text
/// analysis-orchestration (Rust)
///   → SimulationResult (this type, in frontend crate)
///     → PyAnalysisResult (in binding crate, via from_simulation_result)
///       → Python: result.node_voltages_array() → numpy.ndarray view
/// ```
///
/// The `Vec<f64>` → `Py<PyArray1<f64>>` transfer is a single ownership
/// move: `numpy::PyArray1::from_vec(py, vec)` takes the `Vec`'s heap
/// buffer and wraps it as a NumPy array. No element-wise copy occurs.
#[derive(Debug)]
pub struct SimulationResult {
    /// DC node voltages as parallel (names, values) vectors.
    pub node_voltages: NamedScalarData,
    /// DC branch currents as parallel (names, values) vectors.
    pub branch_currents: NamedScalarData,
    /// Time-domain waveforms: node name → (times, values).
    pub waveforms: BTreeMap<String, (Vec<f64>, Vec<f64>)>,
    /// Frequency-domain transfer functions: node name → (freq, mag, phase).
    pub transfer_functions: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<f64>)>,
}

impl SimulationResult {
    /// Construct from individual maps, projecting scalar data into parallel
    /// name/value vectors for zero-copy NumPy transfer.
    ///
    /// The scalar maps are converted to [`NamedScalarData`] (contiguous
    /// `Vec<f64>` values alongside sorted name vectors). Waveform and
    /// transfer-function maps are passed through as-is; the binding crate
    /// transfers their inner `Vec<f64>` elements to NumPy ownership.
    #[must_use]
    pub fn from_maps(
        node_voltages: BTreeMap<String, f64>,
        branch_currents: BTreeMap<String, f64>,
        waveforms: BTreeMap<String, (Vec<f64>, Vec<f64>)>,
        transfer_functions: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<f64>)>,
    ) -> Self {
        Self {
            node_voltages: NamedScalarData::from_sorted_map(&node_voltages),
            branch_currents: NamedScalarData::from_sorted_map(&branch_currents),
            waveforms,
            transfer_functions,
        }
    }

    /// Construct an empty result (all channels vacant).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            node_voltages: NamedScalarData {
                names: Vec::new(),
                values: Vec::new(),
            },
            branch_currents: NamedScalarData {
                names: Vec::new(),
                values: Vec::new(),
            },
            waveforms: BTreeMap::new(),
            transfer_functions: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_scalar_data_from_sorted_map_roundtrip() {
        let mut map = BTreeMap::new();
        map.insert("n2".to_string(), 3.3);
        map.insert("n1".to_string(), 5.0);
        map.insert("n3".to_string(), 0.0);

        let data = NamedScalarData::from_sorted_map(&map);
        // BTreeMap iterates in key order
        assert_eq!(data.names, vec!["n1", "n2", "n3"]);
        assert_eq!(data.values, vec![5.0, 3.3, 0.0]);
        assert_eq!(data.len(), 3);
        assert!(!data.is_empty());

        let recovered = data.into_map();
        assert_eq!(recovered, map);
    }

    #[test]
    fn named_scalar_data_empty() {
        let data = NamedScalarData::from_sorted_map(&BTreeMap::new());
        assert!(data.is_empty());
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn simulation_result_from_maps() {
        let mut nv = BTreeMap::new();
        nv.insert("n1".to_string(), 5.0);
        let mut bc = BTreeMap::new();
        bc.insert("V1".to_string(), 1e-3);
        let result = SimulationResult::from_maps(nv.clone(), bc.clone(), BTreeMap::new(), BTreeMap::new());
        assert_eq!(result.node_voltages.names, vec!["n1"]);
        assert_eq!(result.node_voltages.values, vec![5.0]);
        assert_eq!(result.branch_currents.names, vec!["V1"]);
        assert_eq!(result.branch_currents.values, vec![1e-3]);
        assert!(result.waveforms.is_empty());
        assert!(result.transfer_functions.is_empty());
    }

    #[test]
    fn simulation_result_empty() {
        let result = SimulationResult::empty();
        assert!(result.node_voltages.is_empty());
        assert!(result.branch_currents.is_empty());
        assert!(result.waveforms.is_empty());
        assert!(result.transfer_functions.is_empty());
    }
}
