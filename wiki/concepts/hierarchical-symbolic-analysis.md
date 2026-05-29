---
title: Hierarchical Symbolic Analysis
type: claim
id: claim-hierarchical-symbolic-analysis
tags:
- analog
- symbolic
- hierarchical
- advanced
- scalability
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt
confidence:
  base: 0.85
---

## Definition

Hierarchical symbolic analysis decomposes a large circuit into multi-port subcircuits, derives compact symbolic models (typically BDD-encoded) for each, and composes them at higher levels — allowing exact symbolic analysis of circuits that would be intractable for a flat method.

## How It Works

Each subcircuit is characterized as a port-level admittance or transfer matrix whose entries are themselves symbolic expressions stored as BDD/DDD/GPDD subgraphs. Composition at the next level reuses these subgraphs and applies BDD operations to combine them, sharing common subexpressions across the hierarchy. Partitioning may be guided by topology (cones, channels) or by the natural module boundaries of analog design.

## Key Parameters

- Partition granularity and balancing.
- Choice of intra-block representation (DDD vs. GPDD).
- Inter-level interface variables and their order.

## When To Use

- Large analog/mixed-signal designs where flat DDD/GPDD does not scale.
- Reused IP blocks where modular characterization can be cached.

## Risks & Pitfalls

- Interface variable explosion if partitions have many ports.
- Loss of cancellation sharing across block boundaries.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/graph-pair-decision-diagram]]
- [[concepts/symbolic-analysis]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-07-part-ii-methods]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
