// application-frontend must not import from numeric-solver.
// Per ADR-0001, frontend accesses the solver only through analysis-orchestration.
// Per ADR-0008, the compiler must reject undeclared cross-crate access.
// compile-fail

use numeric_solver::newton_raphson;

fn main() {}
