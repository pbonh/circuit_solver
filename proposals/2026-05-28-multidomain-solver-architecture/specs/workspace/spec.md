---
capability: workspace
created: 2026-05-30
---

# Feature: Multi-Crate Cargo Workspace

The solver is decomposed into one Rust crate per container/bounded-context, all members of a single Cargo workspace rooted at the repository root. Inter-crate dependencies are explicit Cargo path-deps; no domain logic is accessed via module re-export from a peer crate without a declared dependency. Crates live under `crates/<name>/`.

## Scenarios

### Scenario: Each bounded-context container is a workspace member crate
```gherkin
Given the Cargo workspace manifest at the repository root
When the workspace member list is resolved
Then frontend, netlist, orchestration, numeric, devices, and digital are each a member crate under crates/<name>/ with its own Cargo.toml
```

### Scenario: The full workspace builds from the root
```gherkin
Given the Cargo workspace at the repository root
When cargo build --workspace is executed
Then all six domain crates and the PyO3 binding crate compile without error
```

### Scenario: An unrelated crate is not recompiled when a peer changes
```gherkin
Given a source change in crates/digital/
When cargo build is run
Then crates/netlist/ is not recompiled (no dependency relationship exists between digital and netlist)
```

### Scenario: Inter-crate access requires an explicit Cargo dependency
```gherkin
Given two crates that are both workspace members but have no declared [dependency] on each other
When one crate attempts to use a type from the other
Then the build fails with an unresolved module error, confirming that module re-export across undeclared boundaries is not possible
```
