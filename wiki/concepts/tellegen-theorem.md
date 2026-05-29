---
title: Tellegen's Theorem
type: claim
id: concepts/tellegen-theorem
tags:
- foundational
- graph
- analog
- sensitivity
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Tellegen's theorem states that for any two networks sharing the same topology (same incidence matrix A), the inner product sum_branches v_b^(1) * i_b^(2) = 0, where the voltages and currents come from different (or the same) network analyses. The theorem holds purely as a consequence of KCL and KVL — element constitutive equations play no role.

## How It Works

If A I^(2) = 0 (KCL on the second network) and v^(1) = A^T v_n^(1) (KVL on the first), then v^(1) . i^(2) = (A^T v_n^(1)) . i^(2) = v_n^(1) . (A i^(2)) = 0.

This decoupling of topology and constitution underlies the adjoint network method for sensitivity computation (Director & Rohrer, 1969). Constructing an "adjoint network" with the same topology but transposed admittances and an excitation at the output port produces sensitivities of the original output via Tellegen-like inner products.

## Key Parameters

- Incidence matrix A (shared by network and adjoint).
- Choice of branch orientations (must be consistent between networks).

## When To Use

- Foundational result for the adjoint method.
- Power conservation: setting both networks equal to the same network gives instantaneous power balance.
- Proof of orthogonality between cut-set and loop-set matrices.

## Risks & Pitfalls

- The theorem says nothing about element values; correctness of subsequent calculations requires consistent constitutive equations.
- In linear systems with controlled sources, the adjoint network's transposed Y matrix requires careful sign-handling.

## Related Concepts

- [[concepts/adjoint-method]]
- [[concepts/orthogonality-relations]]
- [[concepts/transpose-system-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
