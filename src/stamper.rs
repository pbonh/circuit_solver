//! MNA element stamping functions for linear circuit elements.
//!
//! Each function takes a mutable reference to an [`MnaMatrix`] and the node
//! indices / parameter values for the element, then accumulates the correct
//! contributions.  Ground (node 0) is handled by **not** stamping rows/columns
//! corresponding to it — the MNA system is built only over non-ground nodes
//! (indices ≥ 1) plus any auxiliary branch-current rows.
//!
//! # Node indexing convention
//! The caller maps SPICE node names to integer indices.  Ground is always 0.
//! Rows/cols are passed as `Option<usize>` so callers can supply `None` for
//! ground connections without conditional code at every call site.
//!
//! # Transient (backward-Euler) convention
//! For capacitors and inductors the step size `h` (seconds) must be supplied.
//! The stamp uses the backward-Euler companion model:
//!
//! - Capacitor: effective conductance `G_eff = C / h`;
//!   `I_history = -G_eff * V(t_prev)` on RHS.
//! - Inductor: extra branch-current row `k`; stamp is `+L/h` on `[k,k]`
//!   diagonal, ±1 KCL coupling, and `V_history = -L/h * I_L(t_prev)` on RHS.

use crate::MnaMatrix;

// ── Internal helper ──────────────────────────────────────────────────────────

/// Stamp the four-quadrant conductance pattern for any two-terminal element
/// with conductance `g`.  Rows/cols for ground nodes (`None`) are silently
/// skipped.
///
/// Pattern (MNA textbook §3.1):
///
/// ```text
///   (n+,n+) += +g       (n+,n-) += -g
///   (n-,n+) += -g       (n-,n-) += +g
/// ```
#[inline]
fn stamp_conductance(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    g: f64,
) {
    if let Some(p) = n_pos {
        if let Some(q) = n_neg {
            m.stamp(p, q, -g);
            m.stamp(q, p, -g);
        }
        m.stamp(p, p, g);
    }
    if let Some(q) = n_neg {
        m.stamp(q, q, g);
    }
}

// ── Public stamping API ───────────────────────────────────────────────────────

/// Stamp a resistor with resistance `r` (ohms) between nodes `n_pos` and
/// `n_neg`.
///
/// Conductance `G = 1/r` is accumulated at the four corners of the
/// two-terminal stamp.  Ground nodes should be passed as `None`.
///
/// # Panics
/// Panics in debug builds if any non-`None` index ≥ `m.size()`.
pub fn stamp_resistor(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    r: f64,
) {
    debug_assert!(r > 0.0, "resistor value must be positive");
    stamp_conductance(m, n_pos, n_neg, 1.0 / r);
}

/// Stamp a capacitor using the backward-Euler companion model.
///
/// - **Conductance** `G_eff = C / h` is stamped with the standard
///   four-quadrant pattern.
/// - **History current** `I_hist = G_eff * v_prev` is added to the RHS
///   (positive into `n_pos`, negative into `n_neg`), representing the charge
///   stored in the previous time step.
///
/// `v_prev` is the voltage across the capacitor at the previous step
/// (`V(n_pos) - V(n_neg)` in the previous solution).  Pass `0.0` for the
/// initial step (zero initial conditions) or the appropriate IC value.
///
/// # Panics
/// Panics in debug builds if any non-`None` index ≥ `m.size()` or `h <= 0`.
pub fn stamp_capacitor(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    c: f64,
    h: f64,
    v_prev: f64,
) {
    debug_assert!(c > 0.0, "capacitor value must be positive");
    debug_assert!(h > 0.0, "time step h must be positive");
    let g_eff = c / h;
    stamp_conductance(m, n_pos, n_neg, g_eff);
    // History current source term (backward-Euler: I_hist = G_eff * V_prev).
    let i_hist = g_eff * v_prev;
    if let Some(p) = n_pos {
        m.stamp_rhs(p, i_hist);
    }
    if let Some(q) = n_neg {
        m.stamp_rhs(q, -i_hist);
    }
}

/// Stamp an inductor using the backward-Euler companion model.
///
/// Inductors introduce an auxiliary branch-current variable at row/column
/// index `branch_row` (must be pre-allocated in the MNA matrix size).
///
/// The stamp pattern for branch current `I_L` (positive flowing from `n_pos`
/// to `n_neg`) is:
///
/// ```text
///   G matrix:
///     [n_pos, branch_row] += +1    (KCL at n_pos: branch takes I_L)
///     [n_neg, branch_row] += -1    (KCL at n_neg)
///     [branch_row, n_pos] += +1    (KVL: V_n+ contributes to branch eqn)
///     [branch_row, n_neg] += -1
///     [branch_row, branch_row] += -L/h    (companion model: L/h * I_L)
///
///   RHS:
///     rhs[branch_row] += -(L/h) * I_L_prev   (history term)
/// ```
///
/// `i_prev` is the inductor current at the previous step.  Pass `0.0` for
/// zero initial conditions.
///
/// # Panics
/// Panics in debug builds if `branch_row` ≥ `m.size()` or `h <= 0`.
pub fn stamp_inductor(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    branch_row: usize,
    l: f64,
    h: f64,
    i_prev: f64,
) {
    debug_assert!(l > 0.0, "inductor value must be positive");
    debug_assert!(h > 0.0, "time step h must be positive");
    let l_over_h = l / h;

    // KCL coupling: current I_L flows out of n_pos, into n_neg.
    if let Some(p) = n_pos {
        m.stamp(p, branch_row, 1.0);
        m.stamp(branch_row, p, 1.0);
    }
    if let Some(q) = n_neg {
        m.stamp(q, branch_row, -1.0);
        m.stamp(branch_row, q, -1.0);
    }

    // Companion model: L/h on branch diagonal.
    m.stamp(branch_row, branch_row, -l_over_h);

    // History voltage term on RHS.
    m.stamp_rhs(branch_row, -(l_over_h * i_prev));
}

/// Stamp an independent voltage source.
///
/// Voltage sources introduce an auxiliary branch-current variable at row/column
/// `branch_row`.  The source enforces `V(n_pos) - V(n_neg) = v_src`.
///
/// Stamp pattern:
///
/// ```text
///   G matrix:
///     [n_pos, branch_row] += +1   (KCL: current I_V flows into n_pos)
///     [n_neg, branch_row] += -1
///     [branch_row, n_pos] += +1   (KVL: enforce V_n+ - V_n- = V_src)
///     [branch_row, n_neg] += -1
///
///   RHS:
///     rhs[branch_row] += v_src
/// ```
///
/// # Panics
/// Panics in debug builds if `branch_row` ≥ `m.size()`.
pub fn stamp_voltage_source(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    branch_row: usize,
    v_src: f64,
) {
    if let Some(p) = n_pos {
        m.stamp(p, branch_row, 1.0);
        m.stamp(branch_row, p, 1.0);
    }
    if let Some(q) = n_neg {
        m.stamp(q, branch_row, -1.0);
        m.stamp(branch_row, q, -1.0);
    }
    m.stamp_rhs(branch_row, v_src);
}

/// Stamp an independent current source.
///
/// Current `i_src` flows **from** `n_neg` **to** `n_pos` (conventional
/// current direction: into `n_pos`, out of `n_neg`).
///
/// Only the RHS is modified:
///
/// ```text
///   rhs[n_pos] += +i_src
///   rhs[n_neg] += -i_src
/// ```
///
/// Ground nodes (`None`) are silently skipped.
///
/// # Panics
/// Panics in debug builds if any non-`None` index ≥ `m.size()`.
pub fn stamp_current_source(
    m: &mut MnaMatrix,
    n_pos: Option<usize>,
    n_neg: Option<usize>,
    i_src: f64,
) {
    if let Some(p) = n_pos {
        m.stamp_rhs(p, i_src);
    }
    if let Some(q) = n_neg {
        m.stamp_rhs(q, -i_src);
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Resistor ──────────────────────────────────────────────────────────────

    /// Basic resistor stamp: conductance G = 1/R in four-quadrant pattern.
    #[test]
    fn resistor_stamp_pattern() {
        let r = 4.0_f64;
        let g = 1.0 / r;
        let mut m = MnaMatrix::new(2);
        stamp_resistor(&mut m, Some(0), Some(1), r);
        let csr = m.to_csr();
        assert!((csr.get(0, 0) - g).abs() < 1e-12, "G[0,0]");
        assert!((csr.get(1, 1) - g).abs() < 1e-12, "G[1,1]");
        assert!((csr.get(0, 1) + g).abs() < 1e-12, "G[0,1]");
        assert!((csr.get(1, 0) + g).abs() < 1e-12, "G[1,0]");
    }

    /// Resistor grounded on n_neg: only the n_pos diagonal is stamped.
    #[test]
    fn resistor_grounded_neg() {
        let r = 2.0_f64;
        let g = 1.0 / r;
        // Matrix size 1: only non-ground node 0.
        let mut m = MnaMatrix::new(1);
        stamp_resistor(&mut m, Some(0), None, r);
        let csr = m.to_csr();
        assert!((csr.get(0, 0) - g).abs() < 1e-12, "G[0,0] should be G");
    }

    // ── Capacitor ─────────────────────────────────────────────────────────────

    /// Capacitor backward-Euler stamp: G_eff = C/h on conductance; history
    /// current on RHS.
    #[test]
    fn capacitor_stamp_geff_and_rhs() {
        let c = 1e-6_f64; // 1 µF
        let h = 1e-3_f64; // 1 ms step
        let v_prev = 5.0_f64; // 5 V across cap at previous step
        let g_eff = c / h; // = 1e-3
        let i_hist = g_eff * v_prev; // = 5e-3

        let mut m = MnaMatrix::new(2);
        stamp_capacitor(&mut m, Some(0), Some(1), c, h, v_prev);
        let csr = m.to_csr();

        // Conductance pattern.
        assert!((csr.get(0, 0) - g_eff).abs() < 1e-12, "G_eff[0,0]");
        assert!((csr.get(1, 1) - g_eff).abs() < 1e-12, "G_eff[1,1]");
        assert!((csr.get(0, 1) + g_eff).abs() < 1e-12, "G_eff[0,1]");
        assert!((csr.get(1, 0) + g_eff).abs() < 1e-12, "G_eff[1,0]");

        // History current on RHS.
        assert!((csr.rhs[0] - i_hist).abs() < 1e-12, "rhs[0] = +I_hist");
        assert!((csr.rhs[1] + i_hist).abs() < 1e-12, "rhs[1] = -I_hist");
    }

    // ── Inductor ──────────────────────────────────────────────────────────────

    /// Inductor backward-Euler stamp: branch-current row, KVL coupling, -L/h
    /// diagonal, history term on RHS.
    #[test]
    fn inductor_stamp_pattern() {
        let l = 1e-3_f64; // 1 mH
        let h = 1e-6_f64; // 1 µs
        let i_prev = 0.1_f64; // 100 mA at previous step
        let l_h = l / h; // = 1000
        // Matrix: node 0, node 1, branch_row 2.
        let mut m = MnaMatrix::new(3);
        stamp_inductor(&mut m, Some(0), Some(1), 2, l, h, i_prev);
        let csr = m.to_csr();

        // KCL coupling at nodes.
        assert!((csr.get(0, 2) - 1.0).abs() < 1e-12, "G[n+,k]=+1");
        assert!((csr.get(1, 2) + 1.0).abs() < 1e-12, "G[n-,k]=-1");
        // KVL coupling (symmetric).
        assert!((csr.get(2, 0) - 1.0).abs() < 1e-12, "G[k,n+]=+1");
        assert!((csr.get(2, 1) + 1.0).abs() < 1e-12, "G[k,n-]=-1");
        // Companion model diagonal.
        assert!((csr.get(2, 2) + l_h).abs() < 1e-12, "G[k,k]=-L/h");
        // History term on RHS.
        let expected_rhs = -(l_h * i_prev);
        assert!((csr.rhs[2] - expected_rhs).abs() < 1e-12, "rhs[branch]");
    }

    // ── Voltage source ────────────────────────────────────────────────────────

    /// Voltage source stamp: KCL/KVL coupling and RHS enforcement.
    #[test]
    fn voltage_source_stamp_pattern() {
        let v_src = 12.0_f64;
        // Matrix: node 0, node 1 (for V+ side), branch_row 2.
        // Source from node 0 to ground (n_neg=None).
        let mut m = MnaMatrix::new(2);
        stamp_voltage_source(&mut m, Some(0), None, 1, v_src);
        let csr = m.to_csr();

        assert!((csr.get(0, 1) - 1.0).abs() < 1e-12, "G[n+,k]=+1");
        assert!((csr.get(1, 0) - 1.0).abs() < 1e-12, "G[k,n+]=+1");
        assert!((csr.rhs[1] - v_src).abs() < 1e-12, "rhs[branch]=V_src");
    }

    // ── Current source ────────────────────────────────────────────────────────

    /// Current source stamp: only RHS contributions.
    #[test]
    fn current_source_stamp_rhs() {
        let i_src = 0.05_f64; // 50 mA
        let mut m = MnaMatrix::new(2);
        stamp_current_source(&mut m, Some(0), Some(1), i_src);
        let csr = m.to_csr();

        // Matrix should be zero (no G contributions).
        assert!((csr.get(0, 0)).abs() < 1e-12, "G should be zero");
        // RHS.
        assert!((csr.rhs[0] - i_src).abs() < 1e-12, "rhs[n+]=+I");
        assert!((csr.rhs[1] + i_src).abs() < 1e-12, "rhs[n-]=-I");
    }

    // ── 2-node resistor divider ───────────────────────────────────────────────

    /// Hand-computed 2-node resistor divider MNA verification.
    ///
    /// Circuit:
    ///   V1 (5 V) between node 1 and ground
    ///   R1 = 1 kΩ between node 1 and node 2
    ///   R2 = 1 kΩ between node 2 and ground
    ///
    /// Node numbering (non-ground):  node 1 → row 0, node 2 → row 1
    /// Voltage source branch current → row 2
    ///
    /// Hand-computed MNA system (3×3):
    ///
    ///   G1 = G2 = 1/1000 = 1e-3
    ///
    ///   Row 0 (KCL at node 1):   [ G1,  -G1,  +1 ] [ V1 ]   [  0  ]
    ///   Row 1 (KCL at node 2):   [-G1, G1+G2,  0 ] [ V2 ] = [  0  ]
    ///   Row 2 (KVL  V_src):      [ +1,    0,   0 ] [ Ib ]   [ 5.0 ]
    ///
    /// Solution: V1 = 5 V, V2 = 2.5 V, Ib = -5 mA (source current).
    #[test]
    fn resistor_divider_mna_2node() {
        let r = 1000.0_f64; // 1 kΩ
        let g = 1.0 / r; // 1 mS
        let v_supply = 5.0_f64;

        // 3×3 MNA: 2 non-ground nodes + 1 voltage-source branch current.
        let mut m = MnaMatrix::new(3);

        // R1: between node 1 (row 0) and node 2 (row 1).
        stamp_resistor(&mut m, Some(0), Some(1), r);
        // R2: between node 2 (row 1) and ground (None).
        stamp_resistor(&mut m, Some(1), None, r);
        // V1: node 1 (row 0) to ground (None), branch current at row 2.
        stamp_voltage_source(&mut m, Some(0), None, 2, v_supply);

        let csr = m.to_csr();

        // ── Verify G matrix ───────────────────────────────────────────────
        // Row 0: KCL at node 1 (has R1 and V1 branch)
        assert!((csr.get(0, 0) - g).abs() < 1e-12, "G[0,0] = G1");
        assert!((csr.get(0, 1) + g).abs() < 1e-12, "G[0,1] = -G1");
        assert!((csr.get(0, 2) - 1.0).abs() < 1e-12, "G[0,2] = +1 (Vsrc)");

        // Row 1: KCL at node 2 (has R1 and R2)
        assert!((csr.get(1, 0) + g).abs() < 1e-12, "G[1,0] = -G1");
        assert!((csr.get(1, 1) - 2.0 * g).abs() < 1e-12, "G[1,1] = G1+G2");
        assert!((csr.get(1, 2)).abs() < 1e-12, "G[1,2] = 0");

        // Row 2: KVL for V1 (enforce V_node1 = V_src)
        assert!((csr.get(2, 0) - 1.0).abs() < 1e-12, "G[2,0] = +1");
        assert!((csr.get(2, 1)).abs() < 1e-12, "G[2,1] = 0");
        assert!((csr.get(2, 2)).abs() < 1e-12, "G[2,2] = 0");

        // ── Verify RHS ────────────────────────────────────────────────────
        assert!((csr.rhs[0]).abs() < 1e-12, "rhs[0] = 0");
        assert!((csr.rhs[1]).abs() < 1e-12, "rhs[1] = 0");
        assert!((csr.rhs[2] - v_supply).abs() < 1e-12, "rhs[2] = V_supply");
    }
}
