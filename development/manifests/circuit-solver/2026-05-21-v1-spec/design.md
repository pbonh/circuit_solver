---
title: "Design manifest — circuit-solver/2026-05-21-v1-spec"
type: manifest-design
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
scientia_schema: 1
wiki_snapshot: 95cbedf14044a84906326148954a97a6cad0eaf7
created: 2025-07-18
---

## 5 — In-Force ADRs

Supersession walk over `wiki/decisions/`: all five ADRs carry status
`accepted` with no `Supersedes:` or superseded-by chains. No orphan
ADR references detected; walk completes cleanly.

| ID | Title | Status | ASR |
|---|---|---|---|
| ADR-0001 | PyO3 In-Process Binding with Immutable Circuit Graph | accepted | Zero-copy NumPy results with Rust ownership discipline across the PyO3 boundary |
| ADR-0002 | Hybrid Sparse Direct Solver Backend (russell + faer) | accepted | Pure-Rust sparse LU for both real-valued and complex-valued MNA systems |
| ADR-0003 | Two-Pass Graph Flattening with Per-Analysis Sub-Views | accepted | Zero re-flattening overhead when switching analysis types on the same netlist |
| ADR-0004 | Optimistic Mixed-Signal Synchronization via Shared Scheduler | accepted | Efficient analog timestepping with rollback on digital misprediction, decoupled kernels |
| ADR-0005 | Closed Enum Device Model Dispatch | accepted | Zero-cost dispatch and cache-friendly layout in Newton-Raphson stamp loops |

## 6 — ASRs / QAS

| ASR / QAS ID | Description | Source |
|---|---|---|
| ASR-1 | Python↔Rust boundary must preserve Rust ownership; no shared mutable state | ADR-0001 |
| ASR-2 | No FFI to C/C++ in the solver stack; pure-Rust dependency tree | ADR-0002 |
| ASR-3 | Switching analysis type on the same netlist must not re-flatten the graph | ADR-0003 |
| ASR-4 | Mixed-signal kernels must be decoupled; scheduler is sole mediator | ADR-0004 |
| ASR-5 | Device model dispatch must be zero-cost (no vtable, no heap indirection) | ADR-0005 |
| QAS-1 | GIL must be released during native solver work; concurrent Python threads observe ≥ 80 % CPU utilisation | spec/python-frontend |
| QAS-2 | Conformance tolerance envelope: DC 1 % relative or 1 mV absolute per node; AC 0.1 dB magnitude / 1° phase; transient 1 % relative or 1 mV absolute per time point per node; noise 2 % relative or 1 nV/√Hz absolute per frequency point | specs (DC, AC, transient, noise conformance scenarios) |
| QAS-3 | Mixed-signal analog tolerance 1 % relative; digital event-trace equivalence at cycle boundaries | spec/mixed-signal-cosim |
| QAS-4 | OperatingPoint auto-computation when absent must complete before AC/noise proceeds; failure short-circuits to Result with Convergence "failed" | specs (AC, noise) |
| QAS-5 | Adaptive timestepping LTE rejection must re-solve at smaller step; only accepted time points in final Result | spec/transient-time-domain |

## 8 — Known Pitfalls

| Pitfall | Source concept | Design mitigation |
|---|---|---|
| Newton-Raphson false convergence under ΔI check (stall with small Δv/ΔI but large residue) | [[concepts/newton-raphson-method]] | Dual convergence criterion: require both Δv/ΔI below tolerance **and** residue norm below threshold; never accept "converged" on update-only check |
| Non-isolated equilibria (floating nodes, loops of shorts) unreachable by NR | [[concepts/dc-analysis]], [[concepts/newton-raphson-method]] | Topology checker in netlist-graph Pass 1 flags disconnected subgraphs; Gmin-stepping homotopy adds shunt conductances to ground for floating nodes |
| Trapezoidal ringing on marginally stable circuits | [[concepts/transient-analysis]] | Offer Backward Euler and Gear-2 BDF as alternatives; default to Trapezoidal but document ringing risk; LTE controller automatically shrinks h when ringing is detected |
| Numerical damping from BE / Gear-2 on LC tanks | [[concepts/transient-analysis]] | Document the tradeoff in API; allow user to select Trapezoidal for energy-conserving circuits; charge-conserving companion models reduce artificial loss |
| AC analysis unsuitable for mixers, oscillators, switched-cap circuits | [[concepts/ac-analysis]] | Raise `AnalysisTypeError` when user requests AC on a circuit containing switching elements (detected via element-type scan); documentation states LTI-only scope |
| 1/f noise model sensitivity to KF/AF parameters | [[concepts/noise-analysis]] | Warn when KF/AF differ from foundry defaults; per-device noise breakdown (optional) helps the user inspect which devices dominate low-frequency noise |
| Analog-digital boundary interpolation artifacts | [[concepts/mixed-level-simulation]] | Zero-order hold at boundary by default; linear interpolation available as option; sub-view masking at boundary ensures charge conservation |
| Overly tight tolerance envelope causes false conformance failures | [[concepts/golden-reference]] | Tolerance envelope uses max(relative, absolute) formulation; per-node checks allow local failures without global fail; conformance report lists worst-case nodes |
| Holding GIL during long Rust work blocks Python threads | [[concepts/global-interpreter-lock]] | Every analysis entry point wraps native work in `Python::allow_threads`; audit per path; integration test spawns concurrent Python thread to verify |
| Derivative discontinuities in device models cause NR convergence failures | [[concepts/device-modeling]] | Smooth model equations (e.g., limiting exponentials, continuous capacitance models); closed-enum match arms enforce exhaustive derivative coverage |
