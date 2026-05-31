// application-frontend must not import from digital-kernel.
// Per ADR-0001/ADR-0006, frontend accesses the digital kernel only through
// analysis-orchestration (which provides the in-process run-until API).
// Per ADR-0008, the compiler must reject undeclared cross-crate access.
// compile-fail

use digital_kernel::kernel;

fn main() {}
