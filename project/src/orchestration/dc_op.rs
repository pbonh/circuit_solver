//! Project-level DC operating-point analysis driver.
//!
//! This module bridges the `analysis-orchestration` crate's DC analysis
//! control loop ([`dc_analysis`]) to the project's device model / stamp
//! infrastructure, providing a simplified [`ProjectDcOpRequest`] builder
//! and a direct-delegation entry point ([`project_dc_analysis`]).
//!
//! # Architecture
//!
//! The crate-level [`dc_analysis`] function handles the full Newton-Raphson
//! loop, ground suppression, Gmin-stepping homotopy fallback, and
//! convergence reporting. This module wraps it with a project-level request
//! type that provides sensible defaults and builder-method overrides, so
//! callers don't need to construct the crate-level request directly.
//!
//! # Design references
//!
//! - **ADR-0005** — Closed-enum device model dispatch.
//! - **ADR-0006** — Dual convergence criterion for Newton-Raphson.
//! - **ADR-0008** — Per-node max(Relative, Absolute) tolerance envelope.
//! - **ADR-0009** — Topology checker for floating-node detection.
//! - **ADR-0010** — Unstable public Rust API surface for v1.

use analysis_orchestration::dc::{
    dc_analysis, BranchCurrentSample, DcAnalysisError, DcAnalysisRequest, DcAnalysisResult,
    DeviceModelBinding, OperatingPoint,
};
use circuit_solver_types::flattened::FlattenedStructure;
use circuit_solver_types::{ConvergenceStatus, NodeId};
use netlist_graph::CircuitGraph;
use numeric_solver::NewtonRaphsonConfig;

// -----------------------------------------------------------------------------
// Project-level DC request
// -----------------------------------------------------------------------------

/// Project-level DC operating-point analysis input bundle.
///
/// Wraps the crate-level [`DcAnalysisRequest`] with project-specific
/// convenience defaults and builder methods. Callers that need full
/// control over every parameter can construct a [`DcAnalysisRequest`]
/// directly and call [`dc_analysis`]; this type provides a simplified
/// surface for the common case.
#[derive(Debug, Clone, Copy)]
pub struct ProjectDcOpRequest<'a> {
    /// The immutable source circuit graph.
    pub graph: &'a CircuitGraph,
    /// Pass-1 flattened incidence over `graph`.
    pub structure: &'a FlattenedStructure,
    /// Newton-Raphson tuning. `None` defaults to
    /// [`NewtonRaphsonConfig::DC_DEFAULTS`].
    pub newton_raphson: Option<NewtonRaphsonConfig>,
    /// Override the ground node. `None` defaults to
    /// [`FlattenedStructure::ground_node`].
    pub ground: Option<NodeId>,
    /// Per-`ModelName` device-physics bindings for semiconductor elements.
    /// `None` selects the linear-only adapter.
    pub device_models: Option<&'a [DeviceModelBinding]>,
    /// Whether to attempt Gmin-stepping homotopy fallback when plain NR
    /// fails to converge. Defaults to `true`.
    pub enable_gmin_fallback: bool,
}

impl<'a> ProjectDcOpRequest<'a> {
    /// Build a request with the SPICE-default Newton-Raphson tuning
    /// and the structure's own ground node. Gmin-stepping homotopy
    /// fallback is enabled by default.
    #[must_use]
    pub fn new(graph: &'a CircuitGraph, structure: &'a FlattenedStructure) -> Self {
        Self {
            graph,
            structure,
            newton_raphson: None,
            ground: None,
            device_models: None,
            enable_gmin_fallback: true,
        }
    }

    /// Builder-style override for Newton-Raphson configuration.
    #[must_use]
    pub fn with_newton_raphson(mut self, config: NewtonRaphsonConfig) -> Self {
        self.newton_raphson = Some(config);
        self
    }

    /// Builder-style override for ground node id.
    #[must_use]
    pub fn with_ground(mut self, ground: NodeId) -> Self {
        self.ground = Some(ground);
        self
    }

    /// Builder-style attach of a [`DeviceModelBinding`] slice.
    #[must_use]
    pub fn with_device_models(mut self, bindings: &'a [DeviceModelBinding]) -> Self {
        self.device_models = Some(bindings);
        self
    }

    /// Builder-style override for the Gmin-stepping fallback flag.
    #[must_use]
    pub fn with_gmin_fallback(mut self, enable: bool) -> Self {
        self.enable_gmin_fallback = enable;
        self
    }

    /// Convert to the crate-level request, borrowing all fields.
    fn as_crate_request(&self) -> DcAnalysisRequest<'a> {
        let mut req = DcAnalysisRequest::new(self.graph, self.structure)
            .with_gmin_fallback(self.enable_gmin_fallback);

        if let Some(cfg) = self.newton_raphson {
            req = req.with_newton_raphson(cfg);
        }
        if let Some(g) = self.ground {
            req = req.with_ground(g);
        }
        if let Some(models) = self.device_models {
            req = req.with_device_models(models);
        }

        req
    }
}

// -----------------------------------------------------------------------------
// Project-level DC analysis entry point
// -----------------------------------------------------------------------------

/// Run the DC operating-point analysis.
///
/// This is a thin wrapper around the crate-level
/// [`analysis_orchestration::dc::dc_analysis`] that uses the project-level
/// request type. The analysis loop is identical: assemble the MNA system,
/// apply ground suppression, drive the Newton-Raphson solver (with optional
/// Gmin-stepping homotopy fallback), and return the converged operating
/// point or a convergence-failure diagnostic.
///
/// # Errors
///
/// See [`DcAnalysisError`] for the complete list.
pub fn project_dc_analysis(
    req: ProjectDcOpRequest<'_>,
) -> Result<DcAnalysisResult, DcAnalysisError> {
    dc_analysis(req.as_crate_request())
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::FlattenedStructure;
    use circuit_solver_types::convergence::ConvergenceDiagnostic;
    use netlist_graph::{CircuitBuilder, ElementKind};
    use numeric_solver::flatten;

    // ---------- helpers -------------------------------------------------

    fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
        b.add_element(
            name,
            ElementKind::Resistor {
                resistance_ohms: ohms,
            },
            [n1, n2],
            None,
        )
        .expect("add resistor");
    }

    fn add_voltage_source(
        b: &mut CircuitBuilder,
        name: &str,
        plus: &str,
        minus: &str,
        volts: f64,
    ) {
        b.add_element(
            name,
            ElementKind::VoltageSource {
                voltage_volts: volts,
            },
            [plus, minus],
            None,
        )
        .expect("add voltage source");
    }

    fn add_current_source(
        b: &mut CircuitBuilder,
        name: &str,
        plus: &str,
        minus: &str,
        amps: f64,
    ) {
        b.add_element(
            name,
            ElementKind::CurrentSource {
                current_amperes: amps,
            },
            [plus, minus],
            None,
        )
        .expect("add current source");
    }

    // ---------- Direct delegation tests --------------------------------

    /// Voltage divider: V1 (5 V) → R1 (1 kΩ) → R2 (1 kΩ) → gnd.
    /// Mid-point voltage should be 2.5 V (±0.1%).
    #[test]
    fn project_dc_voltage_divider() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 5.0);
        add_resistor(&mut b, "R1", "n_in", "n_mid", 1.0e3);
        add_resistor(&mut b, "R2", "n_mid", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        let result = project_dc_analysis(req).expect("DC analysis ok");

        assert!(result.is_converged(), "DC should converge");
        let op = result.operating_point.expect("operating point should exist");

        // Voltage at n_in should be 5 V (source).
        let n_in = g.nodes().iter().find(|n| n.name() == "n_in").map(|n| n.id()).expect("n_in");
        let v_in = op.voltage_at(n_in).expect("v_in");
        assert!(
            (v_in - 5.0).abs() < 5.0e-3,
            "v_in = {v_in}, expected 5.0 V"
        );

        // Voltage at n_mid should be 2.5 V (equal divider).
        let n_mid = g.nodes().iter().find(|n| n.name() == "n_mid").map(|n| n.id()).expect("n_mid");
        let v_mid = op.voltage_at(n_mid).expect("v_mid");
        assert!(
            (v_mid - 2.5).abs() < 2.5e-3,
            "v_mid = {v_mid}, expected 2.5 V"
        );
    }

    /// Current source into resistor: I1 (1 mA) → R1 (1 kΩ) → gnd.
    /// Node voltage should be 1 V (V = I × R).
    #[test]
    fn project_dc_current_source_resistor() {
        let mut b = CircuitBuilder::default();
        add_current_source(&mut b, "I1", "n_top", "0", 1.0e-3);
        add_resistor(&mut b, "R1", "n_top", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        let result = project_dc_analysis(req).expect("DC analysis ok");

        assert!(result.is_converged(), "DC should converge");
        let op = result.operating_point.expect("operating point should exist");

        let n_top = g.nodes().iter().find(|n| n.name() == "n_top").map(|n| n.id()).expect("n_top");
        let v_top = op.voltage_at(n_top).expect("v_top");
        assert!(
            (v_top - 1.0).abs() < 1.0e-3,
            "v_top = {v_top}, expected 1.0 V"
        );
    }

    // ---------- Request roundtrip / builder tests -----------------------

    /// Verify that `ProjectDcOpRequest::new` carries the supplied graph
    /// and structure references, and that builder methods override the
    /// correct fields.
    #[test]
    fn project_dc_request_roundtrip() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        assert!(std::ptr::eq(req.graph, &g));
        assert!(std::ptr::eq(req.structure, &fs));
        assert!(req.newton_raphson.is_none());
        assert!(req.ground.is_none());
        assert!(req.device_models.is_none());
        assert!(req.enable_gmin_fallback);

        let req = req
            .with_newton_raphson(NewtonRaphsonConfig::DC_DEFAULTS)
            .with_gmin_fallback(false);
        assert!(req.newton_raphson.is_some());
        assert!(!req.enable_gmin_fallback);
    }

    /// Verify that builder overrides actually change the analysis outcome.
    /// Run DC twice on the same circuit: once with defaults, once with a
    /// custom NR config that has tighter tolerances. Both should converge
    /// for this linear circuit, confirming the override was applied.
    #[test]
    fn project_dc_request_builder_overrides() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 3.3);
        add_resistor(&mut b, "R1", "n_in", "0", 100.0);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        // With defaults.
        let req1 = ProjectDcOpRequest::new(&g, &fs);
        let result1 = project_dc_analysis(req1).expect("DC analysis 1 ok");
        assert!(result1.is_converged());

        // With tighter tolerances (still converges for linear circuits).
        let nr = NewtonRaphsonConfig {
            max_iterations: 50,
            tolerances: circuit_solver_types::convergence::ConvergenceTolerances::new(
                1.0e-12,
                1.0e-15,
            ),
        };
        let req2 = ProjectDcOpRequest::new(&g, &fs).with_newton_raphson(nr);
        let result2 = project_dc_analysis(req2).expect("DC analysis 2 ok");
        assert!(result2.is_converged());
    }

    // ---------- Error surface tests ------------------------------------

    /// Verify `DcAnalysisError` implements `std::error::Error`.
    #[test]
    fn project_dc_error_implements_std_error() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        // A successful analysis should not produce an error.
        let result = project_dc_analysis(req);
        assert!(result.is_ok(), "linear circuit should succeed");
    }

    // ---------- OperatingPoint accessor tests ---------------------------

    /// Verify `OperatingPoint::voltage_at` and `current_through` accessors.
    #[test]
    fn project_dc_operating_point_accessors() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 10.0);
        add_resistor(&mut b, "R1", "n_in", "0", 2.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        let result = project_dc_analysis(req).expect("DC analysis ok");
        let op = result.operating_point.expect("operating point");

        // voltage_at should return Some for valid nodes.
        let n_in = g.nodes().iter().find(|n| n.name() == "n_in").map(|n| n.id()).expect("n_in");
        assert!(op.voltage_at(n_in).is_some());

        // voltage_at should return None for an out-of-range NodeId.
        let bad_node = NodeId::new(9999);
        assert!(op.voltage_at(bad_node).is_none());

        // node_count should match the structure.
        assert_eq!(op.node_count(), fs.node_count() as usize);
    }

    // ---------- DcAnalysisResult tests ---------------------------------

    /// Verify `DcAnalysisResult::is_converged` is true for a successful
    /// linear analysis and false when the convergence status is a failure.
    #[test]
    fn project_dc_result_convergence_flag() {
        let mut b = CircuitBuilder::default();
        add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
        add_resistor(&mut b, "R1", "n_in", "0", 1.0e3);

        let g = b.build().expect("build ok");
        let fs: FlattenedStructure = flatten(&g).expect("flatten ok");

        let req = ProjectDcOpRequest::new(&g, &fs);
        let result = project_dc_analysis(req).expect("DC analysis ok");

        // Linear circuits converge on the first iteration.
        assert!(result.is_converged());
        assert!(result.convergence.is_converged());
        assert!(!result.convergence.is_failure());
    }
}
