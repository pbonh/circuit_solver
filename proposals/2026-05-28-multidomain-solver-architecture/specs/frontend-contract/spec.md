---
capability: frontend-contract
created: 2026-05-28
---

# Feature: Python Frontend Contract (PyO3, Immutable Graph)

The in-process PyO3 frontend (ADR-0001) exposes an immutable CircuitGraph and zero-copy results, and releases the GIL during long solves. The GIL-release behavior is verified empirically rather than assumed (grill ha-gil-release).

## Scenarios

<!-- traces-grill: ha-gil-release -->
**Scenario: The GIL is released during a long solve**
```gherkin
Given a long-running analysis invoked from Python on a background thread
And a second Python thread doing independent work
When the Rust solve is executing
Then the second Python thread makes measurable progress (the GIL is released for the duration of the solve)
```

### Scenario: Results are exposed zero-copy to NumPy
```gherkin
Given a completed analysis producing a result vector
When Python accesses the result as a NumPy array
Then it is a zero-copy view backed by Rust-owned memory (no element-wise copy)
```

### Scenario: The circuit graph is immutable from Python
```gherkin
Given a CircuitGraph built via the PyO3 builder API
When Python attempts to mutate the graph after construction
Then the mutation is rejected, preserving the immutable-graph invariant
```

### Scenario: The PyO3 binding crate declares only the frontend crate as a direct dependency
```gherkin
Given the PyO3 binding crate's Cargo.toml
When its [dependencies] are inspected
Then circuit-solver-frontend is the only domain crate listed as a direct path dependency; no other domain crate (netlist, numeric, devices, digital, orchestration) appears as a direct dep
```
