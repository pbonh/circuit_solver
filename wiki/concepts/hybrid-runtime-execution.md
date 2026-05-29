---
title: Hybrid Runtime Execution
type: claim
id: concepts/hybrid-runtime-execution
tags:
- distributed-systems
- big-data
- optimization
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Hybrid runtime execution mixes in-memory single-node operators with distributed (MapReduce or Spark) operators in the same execution plan, letting the optimizer pick the cheapest backend for each operator based on data size, sparsity, and cluster resources.

## How It Works

In SystemML, every low-level operator (LOP) is tagged as CP (single-node), MR, or Spark. The optimizer chooses based on hard memory constraints and cost estimates: small intermediate matrices stay in CP, large ones spill to MR/Spark. The runtime program contains both kinds of instructions; transitions move data between local memory and distributed RDDs/HDFS. For MR backends SystemML greedily piggybacks multiple LOPs into composite jobs to amortize MR startup. YARN integration adds resource-elasticity: the system can scale containers up or down for each operator based on demand.

## Key Parameters

- Per-operator memory estimates and budget.
- Backend selection thresholds.
- YARN container sizes and elasticity policy.
- MR-job piggybacking heuristics.

## When To Use

- Workloads whose operator costs span many orders of magnitude (small joins followed by huge linear-algebra ops).
- Clusters that mix small interactive jobs with batch analytics.
- Settings where data scale changes at runtime (e.g., filtering shrinks intermediate matrices).

## Risks & Pitfalls

- Inaccurate size estimates can force a switch to a slow backend.
- Mode switches incur materialization costs (CP ↔ Spark/MR).
- Memory leaks in long-running CP operators can starve subsequent distributed phases.

## Related Concepts

- [[concepts/declarative-machine-learning-language]]
- [[concepts/matrix-based-graph-analytics]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
