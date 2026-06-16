# netlist-graph — agent notes

## Topology checker (US-008)

### Pattern: two-level topology API
The `topology` module exposes two distinct layers:

1. **`check_topology(&FlattenedStructure, &[ConductivityClass]) -> Result<TopologyReport, TopologyCheckError>`**
   Low-level pass-1 checker that works on the already-flattened incidence
   structure. The caller supplies one `ConductivityClass` per element and the
   function performs two union-find passes (Always, then Always∪Possibly) to
   classify each node as grounded / warning / floating per ADR-0009.
   Returns a `TopologyReport` (not an Err) even when floating nodes exist —
   the error return is only for API misuse (wrong slice length).

2. **`validate_topology(&CircuitGraph) -> Result<(), TopologyError>`**
   Higher-level pre-solve validation that works directly on the `CircuitGraph`
   (before flattening). Returns `Ok(())` iff the circuit is analyzable:
   - No floating nodes (union-find BFS over VoltageSource/Inductor/Resistor edges)
   - No voltage-source-only loop (DFS cycle detection on V-source subgraph)
   - No inductor-only loop (DFS cycle detection on inductor subgraph)
   Errors are `TopologyError::FloatingNode(Vec<NodeId>)`, `::VoltageLoop`,
   or `::InductorLoop`.

### Gotcha: `add_element` signature requires `model: Option<ModelName>` as 4th arg
`CircuitBuilder::add_element(name, kind, terminals, model)` — tests and doctests
must pass `None` explicitly; the builder API does **not** have a shorter overload.
Terminal iterators must yield `impl Into<String>` (use `Vec<String>` or
`.to_owned()` when calling from tests with string literals).

### Gotcha: `circuit-solver-py` linker failure is pre-existing
`cargo test --workspace` fails with a PyO3/Python linker error on
`circuit-solver-py`. This is unrelated to topology work; skip that crate with
`--exclude circuit-solver-py` when doing workspace-wide checks.
