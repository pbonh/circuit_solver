---
title: "Declarative Machine-Learning Language (DML)"
type: concept
tags: [machine-learning, big-data, language, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt"]
confidence: medium
---

## Definition

DML is the high-level R-like (and Python-like, PyDML) scripting language at the heart of Apache SystemML. Users write machine-learning and graph-analytics algorithms in terms of matrices, vectors, control flow, and user-defined functions; the system compiles and optimizes the script, choosing between in-memory and distributed execution.

## How It Works

A DML script (e.g., nine lines for PageRank) goes through a parser that performs lexical, syntactic, live-variable, and matrix-dimension-compatibility checks. The compiler builds a DAG of high-level operators (HOPs), applies rule-based rewrites (CSE, constant folding, algebraic simplification, branch removal), propagates matrix-size estimates, and then applies cost-based rewrites (matrix-multiply-chain optimization, dynamic algebraic simplification). The optimizer then chooses low-level operators (LOPs) for each HOP, selecting backend (CP single-node, MR MapReduce, or Spark) based on memory and operator constraints, and finally generates a runtime program that piggybacks LOPs into composite jobs when possible.

## Key Parameters

- Backend choice (standalone, Hadoop batch, Spark batch, JMLC for scoring).
- Memory budget for the cost-based optimizer.
- Matrix-block layout (sparse vs. dense, compressed).
- Use of YARN for elastic resource negotiation.

## When To Use

- Prototyping ML or graph-analytics algorithms in linear-algebra form.
- Production pipelines that combine prototyping (R-like) with distributed execution (Spark/MR) without rewriting code.
- Scoring trained models via JMLC inside Java services.

## Risks & Pitfalls

- Heavy-duty graph operations that need vertex-level activity tracking are awkward to express.
- Cost-based optimizer estimates are sensitive to matrix sparsity statistics.
- Switching backends mid-pipeline can introduce unexpected materialization costs.

## Related Concepts

- [[concepts/matrix-based-graph-analytics]]
- [[concepts/hybrid-runtime-execution]]
- [[concepts/machine-learning]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
