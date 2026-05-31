// application-frontend must not import from device-modeling.
// Per ADR-0001, frontend accesses device models only through analysis-orchestration.
// Per ADR-0008, the compiler must reject undeclared cross-crate access.
// compile-fail

use device_modeling::model;

fn main() {}
