//! PyO3 extension module: circuit_solver_delta_py
//!
//! Exposes SimResult (wrapping TransientSolution) to Python/NumPy as PyResult.

use circuit_solver_delta::linear_elements::{Capacitor, Resistor};
use circuit_solver_delta::stamper::stamp_voltage_source;
use circuit_solver_delta::traits::DeviceModel;
use circuit_solver_delta::transient::{IntegratorConfig, TransientAnalysis, TransientSolution};
use circuit_solver_delta::{MnaMatrix, VarMap};
use numpy::{IntoPyArray, PyArray1};
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;

// ── Internal voltage-source device for test/demo circuits ─────────────────────

struct VSourceDevice {
    node_pos: String,
    branch: String,
    voltage: f64,
}

impl DeviceModel for VSourceDevice {
    fn terminals(&self) -> Vec<String> {
        vec![self.node_pos.clone()]
    }
    fn stamp_linear(&self, matrix: &mut MnaMatrix, var_map: &VarMap) {
        let np = var_map.node_index(&self.node_pos);
        let br = var_map.node_index(&self.branch).expect("branch must be in varmap");
        let to_row = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };
        stamp_voltage_source(matrix, to_row(np), None, br - 1, self.voltage);
    }
    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, _state: &[f64]) {
        self.stamp_linear(matrix, var_map);
    }
    fn is_smooth(&self) -> bool {
        true
    }
}

// ── PyResult ──────────────────────────────────────────────────────────────────

/// Python wrapper around a completed transient simulation result.
///
/// Obtain via `run_rc_transient()` or construct from Rust.
/// All returned arrays are NumPy arrays with dtype=float64.
#[pyclass(name = "PyResult")]
pub struct SimResult {
    inner: TransientSolution,
}

#[pymethods]
impl SimResult {
    /// Returns the simulation timepoints as a 1-D NumPy array.
    ///
    /// Shape: ``(n_timepoints,)``
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.times.clone().into_pyarray(py)
    }

    /// Returns the node voltage waveform for the given node name.
    ///
    /// Shape: ``(n_timepoints,)``
    ///
    /// :raises KeyError: if the node name is not found in the result.
    fn voltage<'py>(&self, py: Python<'py>, node: &str) -> pyo3::PyResult<Bound<'py, PyArray1<f64>>> {
        match self.inner.waveforms.get(node) {
            Some(v) => Ok(v.clone().into_pyarray(py)),
            None => Err(PyKeyError::new_err(format!(
                "node '{node}' not found in simulation result; available: {:?}",
                {
                    let mut keys: Vec<_> = self.inner.waveforms.keys().cloned().collect();
                    keys.sort();
                    keys
                }
            ))),
        }
    }

    /// Returns the branch current waveform for the given element name.
    ///
    /// The key is looked up as ``I(<element>)``, e.g. ``current("V1")`` looks
    /// up the ``I(V1)`` waveform produced during transient analysis.
    ///
    /// Shape: ``(n_timepoints,)``
    ///
    /// :raises KeyError: if the element name is not found in the result.
    fn current<'py>(&self, py: Python<'py>, element: &str) -> pyo3::PyResult<Bound<'py, PyArray1<f64>>> {
        let key = format!("I({element})");
        match self.inner.waveforms.get(&key) {
            Some(v) => Ok(v.clone().into_pyarray(py)),
            None => Err(PyKeyError::new_err(format!(
                "element '{element}' (key '{key}') not found in simulation result"
            ))),
        }
    }

    fn __repr__(&self) -> String {
        let mut keys: Vec<_> = self.inner.waveforms.keys().cloned().collect();
        keys.sort();
        format!(
            "PyResult(n_timepoints={}, waveforms=[{}])",
            self.inner.times.len(),
            keys.join(", ")
        )
    }
}

impl SimResult {
    /// Wrap a TransientSolution; callable from Rust.
    pub fn from_solution(sol: TransientSolution) -> Self {
        SimResult { inner: sol }
    }
}

// ── Public simulation helper ───────────────────────────────────────────────────

/// Run a simple RC low-pass circuit and return a PyResult.
///
/// Circuit: V1 (``v_in`` V DC) → R (``r`` Ω) → C (``c`` F) → GND.
/// Node names: ``"in"`` (Vsource terminal), ``"out"`` (RC junction).
/// Branch current: ``"V1"``.
///
/// The simulation steps from ``t=0`` to ``t_stop`` with initial timestep
/// ``h_initial``.  Returns a ``PyResult`` containing:
///
/// * ``time()``        → timepoints
/// * ``voltage('out')``→ capacitor voltage waveform
/// * ``voltage('in')`` → source node (≈ v_in everywhere)
/// * ``current('V1')`` → source branch current
///
/// :param r: Resistance in ohms (default 1000.0)
/// :param c: Capacitance in farads (default 1e-9)
/// :param v_in: DC source voltage (default 1.0 V)
/// :param t_stop: Simulation end time in seconds (default 5e-9)
/// :param h_initial: Initial timestep in seconds (default 2e-10)
/// :raises RuntimeError: if the transient solver fails.
#[pyfunction]
#[pyo3(
    signature = (r=1000.0, c=1e-9, v_in=1.0, t_stop=5e-9, h_initial=2e-10),
    text_signature = "(r=1000.0, c=1e-9, v_in=1.0, t_stop=5e-9, h_initial=2e-10)"
)]
fn run_rc_transient(
    r: f64,
    c: f64,
    v_in: f64,
    t_stop: f64,
    h_initial: f64,
) -> pyo3::PyResult<SimResult> {
    // Build VarMap: ground(0), "in"(1), "out"(2), branch V1(3).
    let mut vm = VarMap::new();
    vm.add_node("in");
    vm.add_node("out");
    vm.add_branch("V1");

    // Devices.
    let devices: Vec<Box<dyn DeviceModel>> = vec![
        Box::new(VSourceDevice {
            node_pos: "in".into(),
            branch: "V1".into(),
            voltage: v_in,
        }),
        Box::new(Resistor::new("in", "out", r)),
        Box::new(Capacitor::new("out", "0", c)),
    ];

    let mut analysis = TransientAnalysis::builder(0.0, t_stop, &vm, devices)
        .h_initial(h_initial)
        .h_max(h_initial * 10.0)
        .integrator(IntegratorConfig::default())
        .build();

    analysis
        .run()
        .map(SimResult::from_solution)
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "transient analysis failed at t={:.3e}, lte={:.3e}, h={:.3e}",
                e.t, e.lte, e.h
            ))
        })
}

// ── Module ────────────────────────────────────────────────────────────────────

/// circuit_solver_delta_py — Python bindings for circuit_solver_delta.
#[pymodule]
fn circuit_solver_delta_py(m: &Bound<'_, PyModule>) -> pyo3::PyResult<()> {
    m.add_class::<SimResult>()?;
    m.add_function(wrap_pyfunction!(run_rc_transient, m)?)?;
    Ok(())
}
