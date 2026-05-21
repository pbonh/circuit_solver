---
title: "Grill: Circuit Solver"
type: grill
tags: [grill, design, circuit-solver]
sources:
  - "wiki/vision/circuit-solver"
  - "contexts/application-frontend"
  - "contexts/analysis-orchestration"
updated: 2026-05-17
---

## Decision Tree

1. **Rust-to-Python binding mechanism** — How does the `application-frontend` context call into the Rust core?
   1.1. Q1: In-process vs. out-of-process?
   → A1: In-process PyO3 (option 1)
   1.2. Q2: Stateful session vs. stateless functional vs. read-only graph + mutable analysis state vs. builder pattern?
   → A2: Read-only graph + mutable analysis state (option 3)
   1.3. Q3: How does Python pass circuit data into Rust?
   → A3: Builder API via PyO3 (option 3)
2. **Sparse direct solver backend** — Which crate/library backs `numeric-solver`’s `LinearSolver`?
   2.1. Q4: Pure-Rust vs. FFI vs. hybrid?
   → A4: Russell pure-Rust (via https://github.com/cpmech/russell)
   2.2. Q5: Russell for all analyses, or hybrid with another backend for complex AC?
   → A5: Russell for DC/Transient, `faer` for AC (option 2)
3. **Graph-to-matrix assembly pattern** — Does the solver traverse the graph every Newton iteration or flatten once?
   3.1. Q6: Flatten once vs. per-iteration traversal vs. lazy flatten vs. stamp-table?
   → A6: Flatten once, numeric refactor only (option 1)
   3.2. Q7: How is the ground reference handled?
   → A7: Configurable per-analysis type (option 4)
   3.3. Q8: Who owns the suppression decision and how does it propagate?
   → A8: Two-pass: full matrix always built, analysis context extracts sub-view (option 4)
4. **Mixed-signal analog-digital coupling strategy** — How is time synchronized between event-driven digital and continuous analog?
   4.1. Q9: Lockstep vs. optimistic vs. quantized vs. event-driven analog?
   → A9: Optimistic (analog runs ahead, rolls back on digital events) (option 2)
   4.2. Q10: How does the analog solver implement rollback?
   → A10: Sparse checkpointing at predicted digital event boundaries (option 2)
   4.3. Q11: Who predicts event boundaries and how does lookahead cross the boundary?
   → A11: Shared scheduler process (option 4)
5. **Newton-Raphson convergence criterion style** — SPICE-style ΔI/ΔV or Spectre-style KCL residue check?
   5.1. Q12: Which convergence check for NewtonRaphsonSolver?
   → A12: Hybrid: ΔI/ΔV primary + KCL guard (option 3)
6. **Device model dispatch architecture** — Trait objects, closed enum, or hybrid registry?
   6.1. Q13: How are concrete device models dispatched in Rust?
   → A13: Closed enum (option 2)

## Q&A Log

Q4: Which crate or library should back the `numeric-solver` context’s `LinearSolver`?
1. Pure-Rust `faer` crate
2. FFI wrapper around KLU (SuiteSparse)
3. FFI wrapper around SuperLU or UMFPACK
4. Hybrid: KLU for DC/Transient, pure-Rust fallback for AC
5. other → free text
A4: 2. Use https://github.com/cpmech/russell

Q5: `russell` provides pure-Rust sparse direct solvers with LAPACK/BLAS backends. Should it serve **all** analysis types, or do we use a hybrid where a different backend handles the complex-valued AC matrices?
1. Russell for everything
2. Russell for DC/Transient, `faer` for AC
3. Russell for DC/Transient, custom small dense for AC
4. Russell with MUMPS/SuperLU fallback
5. other → free text
A5: 2

Q9: For the in-scope mixed-signal co-simulation hooks, how is time synchronized between the event-driven digital kernel and the continuous-time analog solver?
1. Lockstep (analog solver pauses at digital events)
2. Optimistic (analog runs ahead and rolls back on digital events)
3. Quantized breakpoints (master-slave with fixed digital quanta)
4. Event-driven analog (analog solver only runs when digital signals cross thresholds)
5. other → free text
A9: 2

Q10: How does the analog solver implement rollback when a digital event arrives at a time earlier than its current point?
1. Full state snapshot at every accepted timestep
2. Sparse checkpointing at predicted digital event boundaries
3. Incremental state delta logging
4. Predictor-corrector with digital event prediction
5. other → free text
A10: 2

Q11: With sparse checkpointing at predicted digital event boundaries, who predicts the event boundaries and how does the lookahead schedule cross the context boundary?
1. Fixed quantum schedule
2. Digital kernel exposes next-event-time API
3. Hard time ceiling from digital kernel
4. Shared scheduler process
5. other → free text
A11: 4

Q13: In the `device-modeling` context, how are concrete device models dispatched to produce stamps in Rust?
1. Trait objects (`dyn DeviceModel`)
2. Closed enum
3. Hybrid: closed enum for core models + trait-object registry for extensions
4. Proc-macro/codegen closed dispatch
5. other → free text
A13: 2

Q12: Which convergence check should the `numeric-solver` implement for its `NewtonRaphsonSolver`?
1. SPICE-style ΔI/ΔV checks
2. Spectre-style KCL residue check
3. Hybrid: ΔI/ΔV primary + KCL guard
4. Hybrid with adaptive switching
5. other → free text
A12: 3

Q6: Does the `numeric-solver` traverse the `CircuitGraph` on every Newton iteration, or flatten the graph into a static sparse matrix structure with cached index mappings?
1. Flatten once, numeric refactor only
2. Graph traversal per iteration
3. Lazy flatten with topology-change detection
4. Stamp-table dispatch
5. other → free text
A6: 1

Q7: During the one-time flattening, how is the ground reference node handled in the matrix structure?
1. Ground node suppressed during flattening
2. Ground node retained with a Dirichlet constraint
3. Ground suppressed for DC/transient, retained for AC
4. Configurable per-analysis type
5. other → free text
A7: 4

Q8: With per-analysis configurability, who owns the suppression decision and how does it propagate from `analysis-orchestration` to `numeric-solver`?
1. Analysis-orchestration passes a `MatrixConfig` struct
2. Numeric-solver owns the policy table
3. Graph context owns ground semantics
4. Two-pass: full matrix always built, analysis context extracts sub-view
5. other → free text
A8: 4

Q1: Should the `application-frontend` Python layer bind to the Rust core in-process for zero-copy ergonomics, or cross a process boundary for isolation and multi-tenancy?
1. In-process PyO3
2. In-process C FFI + cffi
3. Out-of-process gRPC / IPC
4. Hybrid
5. other → free text
A1: 1 in-process pyo3

Q2: With PyO3, should the Python API expose a **stateful session** (Python holds a mutable handle to the Rust circuit graph and can incrementally modify / re-solve), or a **stateless functional API** (pass a netlist dict/graph on every call, get an immutable result back)?
1. Stateful session
2. Stateless functional
3. Read-only graph + mutable analysis state
4. Builder pattern from Python
5. other → free text
A2: 3

Q3: How does Python pass circuit data into Rust to construct the immutable `Circuit` graph?
1. Netlist string
2. Python dict/list tree
3. Builder API via PyO3
4. Graph interchange format
5. other → free text
A3: 3

## Decisions Made

**Rust-to-Python binding mechanism** — PyO3 in-process extension module with an immutable `Circuit` graph built via a Rust-backed builder API exposed to Python, and per-request mutable analysis state (options / sweep definitions) passed from Python. Rationale: zero-copy NumPy-compatible results with full Rust ownership discipline; Python gets ergonomic incremental construction without exposing mutable shared state.

**Sparse direct solver backend** — `russell` (https://github.com/cpmech/russell) for DC and transient real-valued sparse LU, and `faer` for AC complex-valued small-signal LU. Rationale: `russell` gives a pure-Rust, LAPACK/BLAS-backed solver tuned for general sparse systems; `faer` provides a pure-Rust complex LU without FFI, matching the typically smaller and less repeatedly-factorized AC matrices.

**Graph-to-matrix assembly pattern** — Flatten the `CircuitGraph` once into a static sparse matrix structure; on each Newton iteration only recompute nonzero values and refactor. The full matrix (including ground) is always built during flattening; the `analysis-orchestration` context extracts the relevant sub-view or applies constraint masks at solve time, avoiding rebuilds when analysis needs differ. Rationale: standard SPICE performance pattern with flexibility for per-analysis matrix views.

**Mixed-signal analog-digital coupling** — Optimistic time synchronization with sparse checkpointing at predicted digital event boundaries, coordinated by a shared scheduler process that owns both kernels and issues "run-until" commands. Rationale: maximizes analog solver efficiency via adaptive timestepping while avoiding rollback memory explosion through sparse boundary checkpoints; neither kernel queries the other directly, keeping the context boundary clean.

**Newton-Raphson convergence criterion** — ΔI/ΔV update check as the primary iteration criterion (cheap, values available from the LU solve), with a Spectre-style KCL residue verification triggered only when ΔI/ΔV claims convergence. If KCL fails, NR continues iterating. Rationale: pays the expensive residue cost only on the final iterations, gaining robustness against false convergence without the full per-iteration overhead.

**Device model dispatch architecture** — Closed enum (`enum DeviceModel { Diode(...), BJT(...), MOSFET(...), ... }`) for all in-scope core semiconductor models. Rationale: zero-cost dispatch, cache-friendly stamp evaluation in tight Newton loops, and the vision explicitly bounds device scope to core models (diode, BJT, MOSFET Level-1 through BSIM4-level); no runtime extensibility requirement is stated.

## Open Questions

*None yet.*

## Cross-Links

- [[vision/circuit-solver]]
- [[contexts/application-frontend]]
- [[contexts/analysis-orchestration]]
- [[architecture/circuit-solver]]
- [[specs/circuit-solver]]

## Related Decisions

- [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler|ADR-0004]] — Formal ADR capturing the optimistic mixed-signal synchronization commitment surfaced in the grill under decision-tree item 4.
- [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] — Formal ADR capturing the closed-enum device model dispatch commitment surfaced in the grill under decision-tree item 6.

## Status

done
