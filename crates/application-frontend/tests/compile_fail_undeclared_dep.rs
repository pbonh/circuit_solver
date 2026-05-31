/// Compile-fail test: verifies that the Cargo workspace enforces ADR-0008
/// boundaries. application-frontend must not import from crates it does not
/// declare as dependencies. The compiler must reject such access.
///
/// ADR-0001 allows:  frontend → netlist-graph, frontend → analysis-orchestration
/// ADR-0008 forbids: frontend → numeric-solver, frontend → device-modeling,
///                    frontend → digital-kernel
///
/// If any of these start compiling, it means an undeclared dependency was
/// silently added somewhere — a boundary violation.

#[test]
fn undeclared_peer_access_is_rejected() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
