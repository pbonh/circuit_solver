---
title: "Independent Set"
type: concept
tags: [graph, foundational, well-established, np-hard]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt", "raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

An independent set (or stable set) in a graph G is a nonempty set S ⊆ V such that no two vertices of S are adjacent. The independence number α(G) is the maximum cardinality of an independent set.

α(G) = ω(Ḡ) by complementation. Color classes of any proper coloring are independent sets, hence χ(G) ≥ n / α(G).

## How It Works

Maximum independent set is NP-complete in general but polynomial on many structured classes:
- Bipartite graphs via König's theorem (max matching).
- Claw-free graphs via Minty's algorithm, O(n^5) (improved to O(n^3) by Faenza et al.).
- AT-free graphs in O(n^4) using interval decompositions and dominating pairs.
- Perfect graphs in polynomial time via Lovász theta function.
- Planar graphs admit Baker-style PTAS.

## Key Parameters

- α(G).
- Independence ratio r(G) = α(G) / |V(G)|.
- Tensor capacity Θ(G) = lim r(G^k).

## When To Use

- Scheduling conflict-free tasks (each vertex = task, edges = conflicts).
- Code design (e.g. error-correcting codes via large independent sets in suitable confusion graphs).

## Risks & Pitfalls

- Maximum independent set is W[1]-hard (no FPT in k = |S| under standard assumptions).
- "Maximal" independent set is not the same as "maximum."

## Related Concepts

- [[concepts/clique]]
- [[concepts/complement]]
- [[concepts/chromatic-number]]
- [[concepts/perfect-graph]]
- [[concepts/dominating-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
