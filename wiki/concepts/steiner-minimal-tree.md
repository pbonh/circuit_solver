---
title: Steiner Minimal Tree
type: claim
id: concepts/steiner-minimal-tree
tags:
- graph
- algorithm
- routing
- vlsi
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/00-preface.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A Steiner minimal tree (SMT) on a set of terminal points is a tree of minimum total edge weight that connects all terminals, possibly introducing additional intermediate vertices ("Steiner points") that reduce the overall tree cost compared with a spanning tree on terminals alone.

## How It Works

Given a graph or a metric space (rectilinear/Euclidean), the SMT problem seeks the minimum-cost tree spanning a designated set of terminals, allowing extra Steiner vertices anywhere. The problem is NP-hard, so practical implementations use heuristics (Hanan grid, batched 1-Steiner, etc.) and approximation algorithms. In VLSI, rectilinear Steiner minimum trees (RSMT) are central to interconnect routing since wires run along orthogonal layers.

## Key Parameters

- Number and locations of terminals.
- Distance metric (rectilinear vs. Euclidean vs. weighted graph).
- Allowed Steiner-vertex set.
- Solution quality vs. runtime tradeoff for heuristics.

## When To Use

- Global routing and net-topology generation in VLSI placement and routing.
- Clock tree synthesis as a lower-bound estimate of interconnect length.
- Any network design problem that allows intermediate junctions to reduce total wire/path cost.

## Risks & Pitfalls

- Exact SMT is NP-hard; large nets must use heuristics.
- Heuristic gap depends on terminal distribution.
- Wire-length minimization alone may produce poor timing; weighted variants are often needed.

## Related Concepts

- [[concepts/graph-theory]]
- [[concepts/minimum-spanning-tree]]
- [[concepts/interconnect-routing]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-00-preface]]
- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
