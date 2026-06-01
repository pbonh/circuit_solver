---
title: "ADR-0009: Topology Checker for Floating-Node Detection in Pass 1"
adr_id: ADR-0009
status: accepted
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
supersedes: []
superseded_by: null
asr:
  - "The netlist-graph Pass 1 flattening must detect floating nodes and disconnected subgraphs before solve, preventing Newton-Raphson from encountering structurally singular MNA matrices."
tags: [netlist-graph, topology, floating-node, dc-analysis, convergence, numeric-solver]
created: 2025-07-18
---

# ADR-0009: Topology Checker for Floating-Node Detection in Pass 1

## Y-Statement

**In the context of** the netlist-graph crate's Pass 1 structure flattening,
**facing** the risk that circuits with floating nodes (no DC path to ground) produce structurally singular MNA matrices that cause Newton-Raphson to fail or report false convergence,
**we decided for** embedding a topology checker in Pass 1 that traverses the flattened incidence structure and flags nodes with no DC path to the ground reference,
**and against** relying solely on runtime Gmin-stepping homotopy to rescue singular systems, or deferring topology checks to the numeric solver,
**to achieve** early, deterministic failure reporting before the solver attempts an expensive and ultimately futile factorization,
**accepting** that the topology checker adds O(N) traversal cost to Pass 1 and may produce false positives on circuits where a valid DC path exists through nonlinear devices (e.g., diodes) that the checker models as open circuits.

## Architecturally Significant Requirement

Floating nodes are the most common cause of DC convergence failure in SPICE simulators. The [[concepts/dc-analysis]] pitfall states: "Non-isolated equilibria (floating nodes, loops of shorts, parallel LC tanks) are not reachable by NR — topology checkers and Gmin are the defensive tools." The [[concepts/newton-raphson-method]] pitfall confirms: "Non-isolated solutions (floating nodes, loops of shorts) cannot be reached by standard NR." Detecting these conditions before entering the solver loop is architecturally significant because it prevents wasted computation and provides actionable diagnostics to the user.

## Options Considered

### Option A — Rely on Gmin-stepping only
Skip topology checking; let Newton-Raphson fail, then apply Gmin-stepping homotopy to add shunt conductances to all nodes, gradually reducing them.

- **Pros:** No extra code in netlist-graph; Gmin-stepping already handles floating nodes per ADR-0003's support for constraint masks.
- **Cons:** Gmin-stepping is expensive (multiple factorizations); convergence is not guaranteed; the user receives a generic "convergence failed" message rather than a specific "floating node n5" diagnostic; the solver may waste many iterations before Gmin-stepping kicks in.

### Option B — Topology check in numeric solver (Pass 2)
After building the full MNA matrix, check for zero-diagonal rows (indicating nodes with no DC path to ground).

- **Pros:** Zero-diagonal detection is numerically precise; catches all floating nodes including those created by stamp-level effects.
- **Cons:** Topology check is delayed until after matrix assembly; if the check fails, the user has already paid the cost of building the matrix; does not distinguish between a truly floating node and a node that is grounded through a nonlinear device at the current operating point; the check is coupled to the numeric solver rather than the graph structure.

### Option C — Topology checker in Pass 1 (chosen)
After Pass 1 structure flattening, traverse the incidence graph and compute DC connectivity to the ground node. Flag any node that has no path through conductive elements (resistors, voltage sources, inductor DC path) to ground.

- **Pros:** Early detection before any matrix assembly; deterministic O(N) graph traversal; provides specific node names in diagnostics; aligns with the [[concepts/dc-analysis]] recommendation for topology checkers.
- **Cons:** May produce false positives on nodes grounded through nonlinear devices (diodes, MOSFET channels) that the checker models as open circuits at DC; the checker must be conservative (warn, not error) for such cases; adds code to the netlist-graph crate that must be maintained alongside the flattening logic.
- **False-positive mitigation:** The topology checker classifies elements as "always conductive" (resistors, voltage sources, inductor DC short), "possibly conductive" (diodes, MOSFETs when on), and "never conductive at DC" (capacitors, current sources open at DC). A node grounded only through "possibly conductive" elements receives a warning rather than a hard error; the solver proceeds with Gmin-stepping enabled as a safety net.

## Consequences

- **Positive:** Users receive immediate, specific diagnostics ("node n5 has no DC path to ground") instead of a generic convergence failure, dramatically reducing debugging time for the most common SPICE failure mode.
- **Positive:** The topology checker runs once per `CircuitGraph` in Pass 1, consistent with ADR-0003's one-pass flattening strategy; no re-checking on analysis switches.
- **Positive:** The three-tier classification (always / possibly / never conductive) balances early detection with tolerance for circuits that are connected only through nonlinear devices.
- **Negative:** The "possibly conductive" classification requires element-type awareness in the netlist-graph crate, creating a minor dependency on device-model categorization that must be kept in sync with the `DeviceModel` enum in ADR-0005.
- **Negative:** The checker cannot detect loops of shorts (another non-isolated equilibrium type) without a more sophisticated algebraic analysis; that check is deferred to a future ADR if needed.
- **Follow-up:** The topology checker result is attached to the `FlattenedStructure` as a `TopologyReport` (list of floating nodes, list of warning nodes). The `AnalysisOrchestrator` reads the report and auto-enables Gmin-stepping when warning nodes are present. The Python frontend surfaces the report in the `CircuitGraph` object for user inspection.

## Supersession

This ADR does not supersede any prior ADR. It complements ADR-0003 (two-pass flattening) by adding a validation step in Pass 1, and ADR-0002 (sparse-direct solver) by preventing attempts to factorize structurally singular matrices.
