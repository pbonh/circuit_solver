---
title: Homomorphism
type: claim
id: concepts/homomorphism
tags:
- graph
- foundational
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

A homomorphism G → H between two graphs is a map h : V(G) → V(H) such that for every edge {x, y} ∈ E(G), {h(x), h(y)} ∈ E(H). The map preserves edges; vertices may be identified.

## How It Works

Homomorphism G → H exists iff χ(G) ≤ |V(H)| for some interpretations. Specifically:
- G → K_k iff χ(G) ≤ k.
- K_k → G iff ω(G) ≥ k.

The homomorphism quasi-order is reflexive and transitive but NOT antisymmetric (two non-isomorphic graphs can be homomorphism-equivalent). It is not well-quasi-ordered (odd cycles form an infinite antichain).

H is a retract of G if there are homomorphisms ρ: G → H and γ: H → G with ρ ∘ γ = id_H.

## Key Parameters

- For perfect G, H: homomorphism G → H exists iff χ(G) ≤ ω(H) (Lemma 4.219).
- Polynomial-time checkable when H is bipartite; NP-complete otherwise.

## When To Use

- Coloring problems (k-coloring = homomorphism to K_k).
- Constraint satisfaction (CSP).
- Modeling graph patterns and morphisms.

## Risks & Pitfalls

- Different from graph embedding (which requires injectivity).
- Different from graph isomorphism (which requires bijectivity preserving non-edges).

## Related Concepts

- [[concepts/retract]]
- [[concepts/chromatic-number]]
- [[concepts/clique]]
- [[concepts/cograph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
