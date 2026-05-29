---
title: Maiter
type: entity
id: entities/maiter
tags:
- graph
- distributed-systems
- big-data
- asynchronous
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

Maiter (Zhang et al., IEEE TPDS 2014) is an asynchronous graph-processing framework based on delta-based accumulative iterative computation (DAIC). It guarantees exact results despite asynchronous prioritized execution, addressing GraphLab's approximate-result weakness while avoiding ghost replication and remote locking. Implemented on MapReduce; not publicly released.

## Characteristics

- DAIC update model: each vertex maintains a(v) and Δa(v); delta messages accumulate into Δa(v) and are applied/propagated when v runs.
- Requires update function to be expressible as `a^{i+1}(v) = (⊕ g_{u,v}(a^i(u))) ⊕ c(v)` with g distributive over ⊕.
- Supports prioritized execution: e.g., choose top-1% of vertices by |Δa(v)| per round.
- Termination by periodic global progress requests from master.

## Common Strategies

- Express algorithms with associative-commutative aggregation operators (sum, min) and distributive edge functions.
- Use prioritization to converge asymmetric workloads (PageRank) much faster than vanilla BSP.
- Combine with block-centric execution to amortize message cost inside blocks.

## Related Entities

- [[entities/graphlab]]
- [[entities/powergraph]]
- [[entities/pregel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
