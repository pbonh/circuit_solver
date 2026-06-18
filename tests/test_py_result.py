"""
Pytest tests for the circuit_solver_delta_py PyO3 extension module.

These tests verify the PyResult class wraps TransientSolution correctly
and exposes time(), voltage(), and current() as NumPy arrays.
"""

import numpy as np
import pytest

import circuit_solver_delta_py
from circuit_solver_delta_py import PyResult, run_rc_transient


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------


@pytest.fixture
def rc_result() -> PyResult:
    """Run a known RC circuit and return the PyResult.

    Circuit: V1 (1 V DC) -> R (1 kOhm) -> C (1 nF) -> GND.
    tau = RC = 1 us.  We simulate for 5 ns (0.5% of tau), enough for
    a clean numeric solution without running for microseconds.
    Nodes: 'in' (source terminal), 'out' (RC junction).
    """
    return run_rc_transient(r=1000.0, c=1e-9, v_in=1.0, t_stop=5e-9, h_initial=2e-10)


# ---------------------------------------------------------------------------
# Module-level smoke test
# ---------------------------------------------------------------------------


def test_import_succeeds():
    """The extension module must be importable."""
    assert hasattr(circuit_solver_delta_py, "PyResult")
    assert hasattr(circuit_solver_delta_py, "run_rc_transient")


# ---------------------------------------------------------------------------
# PyResult.time()
# ---------------------------------------------------------------------------


def test_time_returns_ndarray(rc_result: PyResult):
    t = rc_result.time()
    assert isinstance(t, np.ndarray)


def test_time_shape(rc_result: PyResult):
    t = rc_result.time()
    n = t.shape[0]
    assert t.shape == (n,), f"expected 1-D array, got shape {t.shape}"
    assert n > 0, "time array must not be empty"


def test_time_monotone(rc_result: PyResult):
    t = rc_result.time()
    assert np.all(np.diff(t) > 0), "timepoints must be strictly increasing"


def test_time_dtype(rc_result: PyResult):
    t = rc_result.time()
    assert t.dtype == np.float64


# ---------------------------------------------------------------------------
# PyResult.voltage()
# ---------------------------------------------------------------------------


def test_voltage_returns_ndarray(rc_result: PyResult):
    v = rc_result.voltage("out")
    assert isinstance(v, np.ndarray)


def test_voltage_shape(rc_result: PyResult):
    """voltage('out').shape must equal (n_timepoints,)."""
    t = rc_result.time()
    n_timepoints = t.shape[0]
    v = rc_result.voltage("out")
    assert v.shape == (n_timepoints,), (
        f"voltage('out').shape={v.shape} != time shape {(n_timepoints,)}"
    )


def test_voltage_in_shape(rc_result: PyResult):
    """voltage('in') shape must equal (n_timepoints,)."""
    t = rc_result.time()
    n_timepoints = t.shape[0]
    v = rc_result.voltage("in")
    assert v.shape == (n_timepoints,)


def test_voltage_source_node_near_v_in(rc_result: PyResult):
    """The 'in' node is directly driven by a 1 V source."""
    v = rc_result.voltage("in")
    assert np.allclose(v, 1.0, atol=1e-3), (
        f"voltage('in') should be ~1 V everywhere, got range [{v.min():.4f}, {v.max():.4f}]"
    )


def test_voltage_capacitor_charging(rc_result: PyResult):
    """Capacitor voltage must be non-negative and strictly increasing."""
    v = rc_result.voltage("out")
    assert np.all(v >= 0.0), "capacitor voltage should not go negative"
    assert np.all(np.diff(v) > 0), "capacitor should be charging (monotone increase)"


def test_voltage_dtype(rc_result: PyResult):
    v = rc_result.voltage("out")
    assert v.dtype == np.float64


def test_voltage_unknown_node_raises_key_error(rc_result: PyResult):
    with pytest.raises(KeyError):
        rc_result.voltage("does_not_exist")


# ---------------------------------------------------------------------------
# PyResult.current()
# ---------------------------------------------------------------------------


def test_current_returns_ndarray(rc_result: PyResult):
    i = rc_result.current("V1")
    assert isinstance(i, np.ndarray)


def test_current_shape(rc_result: PyResult):
    """current('V1').shape must equal (n_timepoints,)."""
    t = rc_result.time()
    n_timepoints = t.shape[0]
    i = rc_result.current("V1")
    assert i.shape == (n_timepoints,), (
        f"current('V1').shape={i.shape} != time shape {(n_timepoints,)}"
    )


def test_current_dtype(rc_result: PyResult):
    i = rc_result.current("V1")
    assert i.dtype == np.float64


def test_current_unknown_element_raises_key_error(rc_result: PyResult):
    with pytest.raises(KeyError):
        rc_result.current("no_such_element")


# ---------------------------------------------------------------------------
# Consistency: all waveforms must have matching length
# ---------------------------------------------------------------------------


def test_all_waveforms_same_length(rc_result: PyResult):
    t = rc_result.time()
    n = t.shape[0]
    v_out = rc_result.voltage("out")
    v_in = rc_result.voltage("in")
    i_v1 = rc_result.current("V1")

    assert v_out.shape[0] == n, f"voltage('out') length {v_out.shape[0]} != {n}"
    assert v_in.shape[0] == n, f"voltage('in') length {v_in.shape[0]} != {n}"
    assert i_v1.shape[0] == n, f"current('V1') length {i_v1.shape[0]} != {n}"


# ---------------------------------------------------------------------------
# repr smoke test
# ---------------------------------------------------------------------------


def test_repr(rc_result: PyResult):
    r = repr(rc_result)
    assert "PyResult" in r
    assert "n_timepoints=" in r
