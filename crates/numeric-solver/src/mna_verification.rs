//! MNA formulation verification tests — US-010.
//!
//! These tests confirm that the assembled MNA matrix is structurally
//! correct before any device model or Newton-Raphson machinery builds
//! on top of it. Three acceptance criteria are exercised here:
//!
//! 1. **Resistor divider** — 10 kΩ / 10 kΩ, V = 5 V.
//!    The output node must be 2.5 V (within 1e-9) after solving the
//!    ground-suppressed sub-view via `faer` dense partial-pivot LU.
//!
//! 2. **Voltage-source KVL loop** — two ideal voltage sources wired
//!    in a conflicting loop; [`validate_topology`] must return
//!    [`TopologyError::VoltageLoop`].
//!
//! 3. **Floating node** — a node with no DC path to ground (only a
//!    capacitor connects it); [`validate_topology`] must return
//!    [`TopologyError::FloatingNode`].

#[cfg(test)]
mod tests {
    use faer::prelude::SpSolver;
    use faer::Mat;
    use netlist_graph::topology::{validate_topology, TopologyError};
    use netlist_graph::{CircuitBuilder, ElementKind};

    use crate::assemble::assemble;
    use crate::flatten::flatten;
    use crate::sub_view::SubViewBuilder;

    // ------------------------------------------------------------------
    // Helper: add a voltage source to a `CircuitBuilder`.
    // ------------------------------------------------------------------

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
    // Helper: add a resistor to a `CircuitBuilder`.
    // ------------------------------------------------------------------

    fn add_res(b: &mut CircuitBuilder, name: &str, a: &str, z: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor { resistance_ohms: ohms },
            [a, z],
            None,
        )
        .expect("add resistor");
    }

    // ------------------------------------------------------------------
    // Helper: add a capacitor to a `CircuitBuilder`.
    // ------------------------------------------------------------------

    fn add_cap(b: &mut CircuitBuilder, name: &str, a: &str, z: &str, farads: f64) {
        b.add_element(
            name,
            ElementKind::Capacitor { capacitance_farads: farads },
            [a, z],
            None,
        )
        .expect("add capacitor");
    }

    // ------------------------------------------------------------------
    // Acceptance criterion 1: resistor divider (US-010 §1)
    //
    // Circuit: V1 = 5 V (n1 → gnd), R1 = 10 kΩ (n1 → n2),
    //          R2 = 10 kΩ (n2 → gnd).
    //
    // After ground suppression the analytic solution is:
    //   v_n1 = 5 V  (enforced by V1)
    //   v_n2 = 2.5 V (resistor divider midpoint)
    //
    // The test assembles the full MNA, extracts a ground-suppressed
    // sub-view, packs that sub-view into a `faer` dense matrix, and
    // solves via `Mat::partial_piv_lu().solve(rhs)`.
    // ------------------------------------------------------------------

    #[test]
    fn resistor_divider_output_node_is_2_5v() {
        let mut b = CircuitBuilder::default();
        // V1 from n1 (+) to ground (−): establishes v_n1 = 5 V.
        add_vsrc(&mut b, "V1", "n1", "0", 5.0);
        // Ladder: n1 → R1 → n2 → R2 → gnd.
        add_res(&mut b, "R1", "n1", "n2", 10_000.0);
        add_res(&mut b, "R2", "n2", "0", 10_000.0);

        let g = b.build().expect("build ok");
        let fs = flatten(&g).expect("flatten ok");
        let sys = assemble(&fs, &g, &[]).expect("assemble ok");

        // Build ground-suppressed sub-view (no Gmin, no source-stepping).
        let sv = SubViewBuilder::from_full(&sys)
            .suppress_ground(true)
            .build()
            .expect("sub-view ok");

        let dim = sv.dim() as usize;

        // Pack sub-view matrix and RHS into faer dense types.
        let a_mat = Mat::<f64>::from_fn(dim, dim, |r, c| {
            sv.matrix_entry(
                u32::try_from(r).expect("fits"),
                u32::try_from(c).expect("fits"),
            )
            .unwrap_or(0.0)
        });
        let rhs_mat = Mat::<f64>::from_fn(dim, 1, |r, _| {
            sv.rhs_entry(u32::try_from(r).expect("fits"))
                .unwrap_or(0.0)
        });

        // Solve via dense partial-pivot LU.
        let sol = a_mat.partial_piv_lu().solve(&rhs_mat);

        // Identify node indices. The CircuitBuilder assigns ids in
        // insertion order: gnd = 0, n1 = 1, n2 = 2. Branch-current
        // variable for V1 sits at index node_count = 3.
        // SubView keeps the same physical indexing as the full system.
        let v_n2 = sol.read(2, 0);

        assert!(
            (v_n2 - 2.5_f64).abs() < 1e-9,
            "expected v_n2 = 2.5 V, got {v_n2}"
        );
    }

    // ------------------------------------------------------------------
    // Acceptance criterion 2: voltage-source KVL loop (US-010 §2)
    //
    // Circuit: V1 (n1 → gnd) and V2 (gnd → n1).
    // Both sources force the same pair of nodes but with conflicting
    // orientations, forming a KVL loop.  `validate_topology` must
    // return `TopologyError::VoltageLoop`.
    // ------------------------------------------------------------------

    #[test]
    fn voltage_source_kvl_loop_triggers_voltage_loop_error() {
        let mut b = CircuitBuilder::default();
        // V1: n1 (+) → gnd (−).
        add_vsrc(&mut b, "V1", "n1", "0", 1.0);
        // V2: gnd (+) → n1 (−) — forms a KVL loop with V1.
        add_vsrc(&mut b, "V2", "0", "n1", 2.0);

        let g = b.build().expect("build ok");
        assert_eq!(
            validate_topology(&g),
            Err(TopologyError::VoltageLoop),
            "two voltage sources forming a loop must trigger VoltageLoop"
        );
    }

    // ------------------------------------------------------------------
    // Acceptance criterion 3: floating node (US-010 §3)
    //
    // Circuit: V1 (n1 → gnd) + C1 (n1 → n2).
    // Node n2 is connected to n1 only through a capacitor, which is
    // open at DC — so n2 has no DC path to ground.
    // `validate_topology` must return `TopologyError::FloatingNode`.
    // ------------------------------------------------------------------

    #[test]
    fn single_floating_node_triggers_floating_node_error() {
        let mut b = CircuitBuilder::default();
        // V1 grounds n1.
        add_vsrc(&mut b, "V1", "n1", "0", 1.0);
        // C1 connects n1 to n2 — but capacitors are open at DC.
        add_cap(&mut b, "C1", "n1", "n2", 1e-9);

        let g = b.build().expect("build ok");

        match validate_topology(&g) {
            Err(TopologyError::FloatingNode(nodes)) => {
                assert!(
                    !nodes.is_empty(),
                    "expected at least one floating node but got an empty list"
                );
            }
            other => panic!("expected TopologyError::FloatingNode, got {other:?}"),
        }
    }
}
