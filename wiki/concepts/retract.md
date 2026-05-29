---
title: Retract
type: claim
id: concepts/retract
tags:
- graph
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph H is a retract of G if there are homomorphisms ρ : G → H (retraction) and γ : H → G (co-retraction) such that ρ ∘ γ = id_H.

When H is a retract of G, H is isomorphic to an induced subgraph of G; G and H share χ, ω, and odd girth.

## How It Works

For threshold graphs (Theorem 4.218): linear-time retract recognition via successive elimination of universal / isolated vertices. The algorithm matches universal vertices of H and G recursively.

For cographs (Theorem 4.225): retract decision is NP-complete. Reduction from 3-partition: encode the constraint that triples sum to B as a cotree structure for H, and the m subsets as cotrees for G. The matching of subsets to children of the join root corresponds to a valid 3-partition.

## Key Parameters

- Polynomial-time on threshold graphs.
- NP-complete on cographs.

## When To Use

- Graph homomorphism problems with closure under retract.
- Modeling "where can a graph be embedded with extra structure preserved."

## Risks & Pitfalls

- Retract checks fail under inappropriate complementation; preserve direction of homomorphisms.
- For perfect G and H, retract reduces to χ comparison (Corollary 4.220).

## Related Concepts

- [[concepts/homomorphism]]
- [[concepts/cograph]]
- [[concepts/threshold-graph]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
