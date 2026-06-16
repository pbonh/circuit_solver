//! The [`DeviceModel`] trait — uniform interface for Newton-Raphson stamping.
//!
//! This trait lets the nonlinear solver iterate over any mix of element types
//! without matching on a closed enum.  It is the complement to the
//! [`crate::model::DeviceModel`] closed enum defined in [`crate::model`]:
//! the closed enum is the hot-path zero-cost dispatch path (ADR-0005);
//! this trait is the open-ended interface used by the `dyn`-dispatch path
//! when the Newton-Raphson engine needs to hold a heterogeneous collection
//! of devices without being coupled to the concrete variant list.
//!
//! # Trait-object safety
//!
//! Every method in the trait takes `&self` or `&mut self` with no
//! type-parameter arguments and no associated types, so `dyn DeviceModel`
//! is legal (the compiler can build a vtable for it).
//!
//! # Method contract summary
//!
//! | Method | When called | What it does |
//! |---|---|---|
//! | [`terminals`] | once, at setup | Returns the list of circuit node ids this device is connected to. |
//! | [`stamp_linear`] | every NR iteration | Adds the *operating-point-independent* (linear) contribution to the MNA matrix. |
//! | [`stamp_nonlinear`] | every NR iteration | Adds the *operating-point-dependent* (nonlinear) Jacobian and companion current to the MNA matrix, given the current solution vector. |
//! | [`is_smooth`] | once, at setup or on demand | Returns `true` when the device's I-V characteristic is everywhere differentiable (C¹). Used by convergence heuristics and step-size controllers. |
//!
//! [`terminals`]: DeviceModel::terminals
//! [`stamp_linear`]: DeviceModel::stamp_linear
//! [`stamp_nonlinear`]: DeviceModel::stamp_nonlinear
//! [`is_smooth`]: DeviceModel::is_smooth

use circuit_solver_types::NodeId;

use crate::mna_matrix::MnaMatrix;
use crate::var_map::VarMap;

/// Uniform interface for devices in the Newton-Raphson stamp loop.
///
/// Implementors include:
///
/// - **Linear elements** (resistors, capacitors, inductors): implement
///   [`stamp_linear`](DeviceModel::stamp_linear) to add conductance or
///   susceptance stamps; [`stamp_nonlinear`](DeviceModel::stamp_nonlinear)
///   can be a no-op.
/// - **Nonlinear elements** (diodes, BJTs, MOSFETs): implement both; the
///   linear stamp can add any bias-independent contribution; the nonlinear
///   stamp adds the Jacobian and equivalent current source from
///   linearization around the current iterate.
///
/// # Trait-object safety
///
/// `dyn DeviceModel` is legal and usable as a fat pointer.  The Newton-Raphson
/// engine can hold a `Vec<Box<dyn DeviceModel>>` or iterate over
/// `&dyn DeviceModel` slices without knowing the concrete device type.
pub trait DeviceModel {
    /// Returns the node identifiers of this device's terminals, in
    /// device-local order.
    ///
    /// Terminal order is device-specific and must be documented by
    /// each implementor (e.g. `[anode, cathode]` for a Diode,
    /// `[drain, gate, source, bulk]` for a MOSFET).  The caller never
    /// reorders this slice; it is used only to check that every terminal
    /// maps to a row/column in the `VarMap` before stamping begins.
    ///
    /// The returned slice must be stable across calls (same nodes, same
    /// order) for the lifetime of the device value.
    fn terminals(&self) -> &[NodeId];

    /// Stamp the operating-point-independent (linear) contribution of
    /// this device into `matrix`.
    ///
    /// Called once (or once per transient timestep for companion models)
    /// before the Newton-Raphson iteration starts.  For purely nonlinear
    /// devices, this can be a no-op.  For linear devices (resistors,
    /// etc.) this is where the full conductance stamp goes.
    ///
    /// `var_map` provides the mapping from the device's [`NodeId`]
    /// terminals to the integer row/column offsets in `matrix`.
    ///
    /// Implementors should only call [`MnaMatrix::add_element`] and
    /// [`MnaMatrix::add_rhs`] — they must not read back entries from
    /// `matrix` (the matrix may be partially filled by other devices).
    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap);

    /// Stamp the operating-point-dependent (nonlinear) Jacobian and
    /// companion current for this device at the current iterate `x`.
    ///
    /// Called on every Newton-Raphson iteration.  `x` is the current
    /// solution vector (length `var_map.dim()`); entry `i` is the
    /// voltage at node `i` (or the branch current at branch variable `i`).
    ///
    /// For linear devices this is typically a no-op (the stamp is fully
    /// captured by [`stamp_linear`](DeviceModel::stamp_linear)).
    ///
    /// For nonlinear devices the implementor:
    ///
    /// 1. Reads the terminal voltages from `x` via `var_map`.
    /// 2. Evaluates the device's tangent conductance `g = dI/dV` at
    ///    those voltages.
    /// 3. Calls `matrix.add_element(…)` for Jacobian entries.
    /// 4. Calls `matrix.add_rhs(…)` for the companion current
    ///    `I_eq = I(V) - g * V`.
    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, x: &[f64]);

    /// Returns `true` if this device's I-V characteristic is everywhere
    /// C¹ (continuously differentiable) over the voltage range of
    /// interest.
    ///
    /// A device with discontinuities or kinks (e.g. an ideal switch,
    /// a piecewise-linear model with corners) should return `false`.
    /// Smooth devices allow the NR engine to apply aggressive step-size
    /// heuristics; non-smooth devices force smaller, more conservative
    /// steps.
    fn is_smooth(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mna_matrix::MnaMatrix;
    use crate::var_map::VarMap;
    use circuit_solver_types::NodeId;

    // ---------- minimal stub implementor used in tests ----------

    /// A fixed-value resistor between `n0` and `n1` with conductance `g`.
    struct StubResistor {
        terminals: [NodeId; 2],
        g: f64,
    }

    impl DeviceModel for StubResistor {
        fn terminals(&self) -> &[NodeId] {
            &self.terminals
        }

        fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
            let i = var_map.node_index(self.terminals[0]).unwrap();
            let j = var_map.node_index(self.terminals[1]).unwrap();
            matrix.add_element(i, i, self.g);
            matrix.add_element(j, j, self.g);
            matrix.add_element(i, j, -self.g);
            matrix.add_element(j, i, -self.g);
        }

        fn stamp_nonlinear(&self, _matrix: &mut MnaMatrix<'_>, _var_map: &VarMap, _x: &[f64]) {
            // linear device: no operating-point-dependent contribution
        }

        fn is_smooth(&self) -> bool {
            true
        }
    }

    // ---------- trait-object safety witness ----------

    #[test]
    fn dyn_device_model_is_usable() {
        // This function would fail to compile if DeviceModel is not
        // object-safe.
        fn accepts_dyn(_: &dyn DeviceModel) {}

        let r = StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            g: 0.001, // 1 kΩ
        };
        accepts_dyn(&r);
    }

    #[test]
    fn vec_of_boxed_dyn_device_model_compiles() {
        let r: Box<dyn DeviceModel> = Box::new(StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            g: 0.001,
        });
        // Can call trait methods through the fat pointer.
        assert_eq!(r.terminals().len(), 2);
        assert!(r.is_smooth());
    }

    // ---------- stamp correctness ----------

    #[test]
    fn stub_resistor_stamps_correctly() {
        // 2-node system: [GND=0, n1=1], 1 kΩ resistor (g = 1e-3).
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 4]; // 2×2
        let mut b = vec![0.0_f64; 2];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        let r = StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            g: 1e-3,
        };

        r.stamp_linear(&mut matrix, &var_map);

        // (0,0) and (1,1) should be +g; (0,1) and (1,0) should be -g.
        assert!((matrix.element(0, 0) - 1e-3).abs() < f64::EPSILON);
        assert!((matrix.element(1, 1) - 1e-3).abs() < f64::EPSILON);
        assert!((matrix.element(0, 1) + 1e-3).abs() < f64::EPSILON);
        assert!((matrix.element(1, 0) + 1e-3).abs() < f64::EPSILON);
        // RHS untouched for a resistor.
        assert_eq!(matrix.rhs(0), 0.0);
        assert_eq!(matrix.rhs(1), 0.0);
    }

    #[test]
    fn stamp_nonlinear_is_noop_for_linear_device() {
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 4];
        let mut b = vec![0.0_f64; 2];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);

        let r = StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            g: 1e-3,
        };

        // stamp_nonlinear should be a pure no-op for a resistor.
        let x = [0.0_f64, 5.0_f64];
        r.stamp_nonlinear(&mut matrix, &var_map, &x);

        // Nothing was stamped.
        for &v in a.iter() {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn terminals_returns_correct_nodes() {
        let r = StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(3)],
            g: 1.0,
        };
        let t = r.terminals();
        assert_eq!(t.len(), 2);
        assert_eq!(t[0], NodeId::GROUND);
        assert_eq!(t[1], NodeId::new(3));
    }

    #[test]
    fn smooth_flag_reported_correctly() {
        let r = StubResistor {
            terminals: [NodeId::GROUND, NodeId::new(1)],
            g: 1.0,
        };
        assert!(r.is_smooth());
    }
}
