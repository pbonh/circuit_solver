---
title: "Floorplanning"
type: concept
tags: [vlsi, physical-design, graph, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

Floorplanning is the VLSI physical-design step that determines the shape and arrangement of macro-blocks within a chip layout. It is fundamentally a rectangular-packing optimization with multiple quality metrics including area efficiency and estimated wirelength.

## How It Works

Each circuit partition is represented as a rectangular block. Two common metrics are area efficiency η_A(F) = total-block-area / enclosing-rectangle-area and total Half-Perimeter Wire Length (HPWL) L(F) = Σ c_ij d_M(m_i, m_j). Floorplans can be encoded by horizontal/vertical constraint graphs (HCG/VCG), O-trees (DFS traversal of VCG), and B*-trees (binary trees with right/top child relationships).

## Key Parameters

- Number of blocks.
- Aspect-ratio constraints.
- Weight w in objective Q(F) = w·η_A(F) + (1−w)·L(F)/L*.
- Block shape variability.

## When To Use

- Early-stage physical design after partitioning, before placement.
- Hierarchical SoC integration of macro blocks.

## Risks & Pitfalls

- NP-hard in general — heuristic and metaheuristic methods (simulated annealing, evolutionary) dominate.
- Poor floorplans propagate into intractable placement/routing problems.

## Related Concepts

- [[concepts/graph-partitioning]]
- [[concepts/placement]]
- [[concepts/interconnect-routing]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
