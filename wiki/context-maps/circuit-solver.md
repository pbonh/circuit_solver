---
title: "Circuit Solver Context Map"
type: context-map
tags: [circuit-solver, context-map, domain-driven-design]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "contexts/netlist-graph"
  - "contexts/device-modeling"
  - "contexts/numeric-solver"
  - "contexts/analysis-orchestration"
  - "contexts/application-frontend"
confidence: high
---

## Contexts

- [[contexts/netlist-graph]]
- [[contexts/device-modeling]]
- [[contexts/numeric-solver]]
- [[contexts/analysis-orchestration]]
- [[contexts/application-frontend]]

## Translations

| Term in A | Context A | Term in B | Context B | Notes |
|-----------|-----------|-----------|-----------|-------|
| Node | netlist-graph | Node | numeric-solver | In netlist-graph, a node is any electrical vertex. In numeric-solver, a node index is a matrix row after ground suppression and MNA augmentation; the two sets may differ in size. |
| Model | netlist-graph | Model | device-modeling | In netlist-graph, a model is a string reference on an element. In device-modeling, it is the full constitutive equation set and parameter set. |
| Stamp | device-modeling | Stamp | numeric-solver | In device-modeling, a stamp is the template of matrix entries a device type contributes. In numeric-solver, it is the assembled numerical values placed into the MNA matrix. |
| Operating Point | analysis-orchestration | Operating Point | device-modeling | In analysis-orchestration, it is the converged DC solution vector. In device-modeling, it is the terminal bias voltages/currents used to evaluate a single device. |
| Convergence | analysis-orchestration | Convergence | numeric-solver | In analysis-orchestration, convergence is the overall analysis success/failure. In numeric-solver, it is the per-Newton-iteration update/residual criterion. |

## False Cognates

- **Node** — In `netlist-graph` it is any electrical vertex (including ground). In `numeric-solver` it is a matrix variable index after ground suppression and MNA augmentation; not all graph nodes become solver variables, and some extra variables (branch currents) are added. Conflating them causes off-by-one index errors in MNA assembly. ([[concepts/false-cognate]])
- **Model** — In `netlist-graph` it is a lightweight string key (`ModelName`) on an element instance. In `device-modeling` it is the full mathematical description (equations, parameters, derivatives) of a device class. Passing a `ModelName` where a `DeviceModel` is expected produces no useful stamps. ([[concepts/false-cognate]])
- **Operating Point** — In `analysis-orchestration` it is the global circuit solution vector. In `device-modeling` it is the local terminal bias of one device. Using a global node voltage as a device terminal voltage is valid, but calling the global vector an "operating point" in the device context misses the per-device Jacobian computation requirement. ([[concepts/false-cognate]])
- **Convergence** — In `analysis-orchestration` it signals that the entire analysis (including sweep loops and timestep adaptation) succeeded. In `numeric-solver` it means one Newton iteration met its local tolerance. Reusing the solver-level convergence flag as the analysis-level success criterion forgets that the analysis may still fail due to timestep rejection or sweep limits. ([[concepts/false-cognate]])

## Integration Patterns

- `netlist-graph` ↔ `device-modeling`: **shared-kernel** — Both contexts share the definition of `Element` and `ModelName`; the netlist graph owns the element list, while device modeling owns the model library. Neither can drift the `Element` structure without the other.
- `device-modeling` → `numeric-solver`: **customer-supplier** — The numeric solver is the customer of device-modeling stamps. Device modeling supplies `LinearizedModel` stamps; the solver consumes them. Device modeling must not break the stamp contract.
- `numeric-solver` ↔ `analysis-orchestration`: **shared-kernel** — Both contexts share the `SolutionVector`, `JacobianMatrix`, and `ConvergenceStatus` types. The solver provides the engine; orchestration provides the control loop. Tight coupling is acceptable because they evolve together in the same codebase.
- `analysis-orchestration` → `application-frontend`: **open-host-service** — The analysis-orchestration context exposes a stable `AnalysisRequest` / `AnalysisResult` interface that the frontend consumes. New analysis types can be added without changing the frontend contract.
- `application-frontend` → `analysis-orchestration`: **conformist** — The frontend conforms to the analysis API; it does not dictate analysis semantics.
