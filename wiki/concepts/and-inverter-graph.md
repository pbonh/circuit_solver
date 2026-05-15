---
title: "And-Inverter Graph (AIG)"
type: concept
tags: [graph, digital, boolean, eda, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

An And-Inverter Graph (AIG) is a directed acyclic graph that represents a Boolean function using only two-input AND nodes and inverters (represented as edge attributes). Primary inputs have zero indegree, primary outputs have zero outdegree, and internal nodes are AND gates.

## How It Works

Because AND and NOT form a complete logic basis, any combinational circuit can be transformed into an AIG. Edges may be marked "complemented" to indicate signal inversion, avoiding explicit inverter nodes. AIG size grows linearly with circuit size (unlike worst-case-exponential ROBDDs). Structural hashing produces Functionally Reduced AIGs (FRAIGs), in which no two nodes compute the same function — a semi-canonical form.

## Key Parameters

- Number of AND nodes.
- Maximum depth (logic levels).
- Complemented-edge bookkeeping.

## When To Use

- Logic synthesis and optimization (ABC, Berkeley).
- Equivalence checking via the miter construction and SAT solving.
- Path balancing in deep pipelines.

## Risks & Pitfalls

- Not canonical — equivalence is not structural identity.
- Structural-hashing maintenance overhead during transformations.

## Related Concepts

- [[concepts/ordered-binary-decision-diagram]]
- [[concepts/boolean-satisfiability]]
- [[concepts/directed-acyclic-graph]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
