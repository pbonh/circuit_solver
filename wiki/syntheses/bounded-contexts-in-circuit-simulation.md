---
title: Bounded Contexts in Circuit Simulation
type: claim
id: claim-synthesis-bounded-contexts-in-circuit-simulation
tags:
- ddd
- circuit-simulation
- architecture
- bounded-context
created: 2026-05-17
updated: 2026-05-17
sources:
- concepts/graph
- concepts/modified-nodal-analysis
- concepts/device-modeling
- concepts/dc-analysis
- concepts/ac-analysis
- concepts/transient-analysis
- concepts/boolean-satisfiability
- concepts/mixed-level-simulation
- concepts/symbolic-analysis
- concepts/model-order-reduction
- concepts/floorplanning
- concepts/discrete-event-system-specification
confidence:
  base: 0.65
---

## Comparison

| Context | Core Concepts | Key Invariants | Boundary Artifacts |
|---------|---------------|----------------|--------------------|
| **Netlist & Graph Representation** | [[concepts/graph]], [[concepts/modified-nodal-analysis]], [[concepts/branch-stamping]], [[concepts/adjacency-matrix]], [[concepts/cutset-matrix]], [[concepts/hypergraph]] | Kirchhoff's laws as graph constraints; element stamp templates map terminals to matrix positions | Sparse matrix (G, C), netlist file, node/branch index maps |
| **Device Modeling & Compact Models** | [[concepts/device-modeling]], [[concepts/diode-model]], [[concepts/bjt-model]], [[concepts/fet-model]], [[concepts/mosfet-small-signal-model]], [[concepts/ebers-moll-model]], [[concepts/conservative-model]], [[concepts/charge-conservation]] | I=f(V) or Q(V) constitutive relations; parameter validity ranges; derivative continuity | Companion model stamps, Jacobian entries, temperature-scaled parameters |
| **Analog Numerical Solver Engine** | [[concepts/dc-analysis]], [[concepts/ac-analysis]], [[concepts/transient-analysis]], [[concepts/noise-analysis]], [[concepts/newton-raphson-method]], [[concepts/integration-method]], [[concepts/backward-euler]], [[concepts/gear-bdf]], [[concepts/sparse-matrix]], [[concepts/homotopy-method]], [[concepts/damped-newton]], [[concepts/stiff-circuit]], [[concepts/numerical-damping]] | KCL/KVL satisfaction within tolerance; charge/flux conservation; LTE control per timestep; convergence from nodeset | Operating point, waveform vectors, frequency-response curves, Newton iterates |
| **Digital Logic & Verification** | [[concepts/boolean-satisfiability]], [[concepts/and-inverter-graph]], [[concepts/binary-decision-diagram]], [[concepts/robdd]], [[concepts/zero-suppressed-bdd]], [[concepts/digital-network-analysis]], [[concepts/finite-state-machine]] | Boolean consistency; equivalence under miter; reachability/bounded model checking | SAT instance, BDD/ROBDD, AIG netlist, property-check result |
| **Mixed-Signal Integration** | [[concepts/mixed-level-simulation]], [[concepts/ahdl-mshdl]], [[concepts/conservative-model]], [[concepts/signal-flow-model]], [[concepts/clock-phase-formulation]] | Pin-accurate behavioral abstractions; conservative vs. signal-flow domain discipline; A↔D interface element consistency | Verilog-AMS/VHDL-AMS netlist, mixed-signal waveform, interface-element insertion log |
| **Symbolic Analysis & Model Order Reduction** | [[concepts/symbolic-analysis]], [[concepts/determinant-decision-diagram]], [[concepts/graph-pair-decision-diagram]], [[concepts/hierarchical-symbolic-analysis]], [[concepts/model-order-reduction]], [[concepts/balanced-truncation]], [[concepts/krylov-subspace-mor]] | Exact symbolic determinant expansion; passivity preservation; moment matching; second-order structure preservation | DDD/GPDD graph, reduced-order transfer function, moment vector, port-level admittance matrix |
| **VLSI Physical Design & EDA** | [[concepts/floorplanning]], [[concepts/placement]], [[concepts/interconnect-routing]], [[concepts/clock-tree-synthesis]], [[concepts/electronic-design-automation]], [[concepts/graph-partitioning]], [[concepts/vlsi-design]] | Geometric non-overlap; timing closure; HPWL minimization; power-integrity constraints | Layout (GDS-II), timing graph, DEF/LEF, clock-skew report |
| **Discrete-Event Simulation Framework** | [[concepts/discrete-event-system-specification]], [[concepts/atomic-devs-model]], [[concepts/coupled-devs-model]], [[concepts/system-entity-structure]], [[concepts/experimental-frame]] | Hierarchical composition closure; common time base for cross-formalism integration; message causality | DEVS model specification, event trace, experimental frame, coupled-model hierarchy |

## Analysis

The wiki currently contains **1,149 concept pages** with no explicit `wiki/contexts/` or `wiki/context-maps/` entries, yet the tag co-occurrence and citation graph reveals at least eight cohesive bounded contexts. Four structural observations stand out:

1. **Shared kernel: the netlist graph.** The Netlist & Graph Representation context supplies the `[[concepts/modified-nodal-analysis]]` matrix and `[[concepts/branch-stamping]]` rules that both the Analog Numerical Solver and the Symbolic Analysis contexts consume. This is a classic `published-language` / `shared-kernel` hybrid: the stamp templates are fixed (published language), but the sparse-matrix ordering and fill-in heuristics live inside the solver context (not shared).

2. **Customer–supplier: Device Modeling → Analog Solver.** The Device Modeling context produces companion-model stamps and Jacobians; the Analog Solver context consumes them. The contract is the element stamp template, but the Device Modeling team speaks semiconductor physics (doping, mobility, subthreshold slope) while the Solver team speaks numerical analysis (Newton damping, LTE, stiffness). Collapsing these into one context risks anemic abstractions where physicists write NR loops or numerical analysts tweak BSIM parameters.

3. **Anticorruption layer: Mixed-Signal Integration.** `[[concepts/mixed-level-simulation]]` sits between the Analog Numerical Solver (continuous-time DAE kernel) and the Digital Logic & Verification (discrete-event Boolean kernel). It translates conservative analog nodes into signal-flow or event-driven digital ports and back. Without an explicit anticorruption layer, the same word "node" drifts: in analog it means a KCL vertex with voltage state; in digital it means a logic-level signal transition event.

4. **False cognates already present in the corpus.**
   - **"Node"** — graph vertex in netlist context (KCL equation index) vs. physical layout coordinate in VLSI Physical Design.
   - **"Model"** — compact device equations (I=f(V)) in Device Modeling vs. behavioral abstraction (pin-accurate Verilog-AMS module) in Mixed-Signal Integration.
   - **"Simulation"** — continuous-time DAE integration in Analog Solver vs. discrete-event state-transition sequence in DEVS Framework.
   - **"Conservative"** — KCL-based analog connection discipline in `[[concepts/conservative-model]]` vs. energy-conservation physics in semiconductor device equations.
   - **"Branch"** — edge in the circuit graph with stamp template vs. control-flow branch in digital logic or software concepts.

5. **Largely separate-ways: VLSI Physical Design vs. Simulation.** Floorplanning, placement, and routing operate on geometric and timing-graph abstractions that are downstream of the netlist. They exchange netlists and timing constraints as a `published-language`, but their internal algorithms (graph partitioning, shortest paths, spanning trees) form an almost entirely separate context from the transient or AC solver.

6. **DEVS Framework as a meta-context.** The Discrete-Event Simulation Framework is not specific to circuits; it is a Systems-of-Systems modeling layer that can embed both analog and digital submodels via `[[concepts/coupled-devs-model]]`. It therefore sits above the other contexts rather than beside them, acting as an integration orchestrator.

## Recommendations

- **For a unified analog simulator codebase**, keep Netlist Representation, Device Modeling, and Analog Solver as three distinct internal modules even if one team maintains them. The stamp template is the boundary contract; the solver should not know about doping concentrations, and the device model should not know about LTE control.
- **For mixed-signal tool development**, elevate Mixed-Signal Integration to its own bounded context with an explicit `conservative ↔ signal-flow ↔ discrete-event` translation table. This prevents A↔D boundary bugs that only surface at full-chip simulation.
- **For EDA flow integration**, treat VLSI Physical Design as a downstream consumer of the netlist published language. Do not let placement heuristics leak into the solver's sparse-matrix ordering logic.
- **Next step**: run `/wiki-strategy circuit-simulation` to formalize these clusters into explicit `wiki/contexts/<slug>.md` pages and a `wiki/context-maps/circuit-simulation.md` with the translation tables and integration-pattern assignments documented above.

## Pages Compared

- [[concepts/graph]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/branch-stamping]]
- [[concepts/adjacency-matrix]]
- [[concepts/cutset-matrix]]
- [[concepts/hypergraph]]
- [[concepts/device-modeling]]
- [[concepts/diode-model]]
- [[concepts/bjt-model]]
- [[concepts/fet-model]]
- [[concepts/mosfet-small-signal-model]]
- [[concepts/ebers-moll-model]]
- [[concepts/conservative-model]]
- [[concepts/charge-conservation]]
- [[concepts/dc-analysis]]
- [[concepts/ac-analysis]]
- [[concepts/transient-analysis]]
- [[concepts/noise-analysis]]
- [[concepts/newton-raphson-method]]
- [[concepts/integration-method]]
- [[concepts/backward-euler]]
- [[concepts/gear-bdf]]
- [[concepts/sparse-matrix]]
- [[concepts/homotopy-method]]
- [[concepts/damped-newton]]
- [[concepts/stiff-circuit]]
- [[concepts/numerical-damping]]
- [[concepts/boolean-satisfiability]]
- [[concepts/and-inverter-graph]]
- [[concepts/binary-decision-diagram]]
- [[concepts/robdd]]
- [[concepts/zero-suppressed-bdd]]
- [[concepts/digital-network-analysis]]
- [[concepts/finite-state-machine]]
- [[concepts/mixed-level-simulation]]
- [[concepts/ahdl-mshdl]]
- [[concepts/signal-flow-model]]
- [[concepts/clock-phase-formulation]]
- [[concepts/symbolic-analysis]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]
- [[concepts/hierarchical-symbolic-analysis]]
- [[concepts/model-order-reduction]]
- [[concepts/balanced-truncation]]
- [[concepts/krylov-subspace-mor]]
- [[concepts/floorplanning]]
- [[concepts/placement]]
- [[concepts/interconnect-routing]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/electronic-design-automation]]
- [[concepts/graph-partitioning]]
- [[concepts/vlsi-design]]
- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]
- [[concepts/system-entity-structure]]
- [[concepts/experimental-frame]]
