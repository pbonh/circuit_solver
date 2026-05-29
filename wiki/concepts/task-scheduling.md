---
title: Task Scheduling
type: claim
id: claim-task-scheduling
tags:
- graph
- algorithm
- well-established
- parallel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.65
---

## Definition

Task scheduling determines an order (and, for parallel systems, an assignment to processors) of executing tasks subject to precedence constraints, minimizing a target metric such as makespan, latency, or throughput. The task graph is a directed acyclic graph (DAG) where nodes are tasks and edges encode dependency.

## How It Works

For sequential scheduling, any topological order of the task DAG is valid; choice may influence cache behavior. For parallel/heterogeneous scheduling (NP-hard), classical algorithms include HEFT (Heterogeneous Earliest Finish Time), which traverses the DAG in reverse order inserting tasks where they finish earliest, and CPOP (Critical Path on a Processor), which pins critical-path tasks to a single processor and assigns others to minimize makespan. Both run in O(|E||V|).

## Key Parameters

- Number of processors and their heterogeneity.
- Task execution times per processor.
- Inter-task communication costs.
- Critical path length (lower bound on makespan).

## When To Use

- Compiler instruction scheduling.
- High-level synthesis in EDA.
- Parallel/distributed computing workloads (job shops, scientific computing).

## Risks & Pitfalls

- NP-hard in general — heuristics give no optimality guarantee.
- Communication costs can dominate computation for fine-grained tasks.

## Related Concepts

- [[concepts/directed-acyclic-graph]]
- [[concepts/topological-sort]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
