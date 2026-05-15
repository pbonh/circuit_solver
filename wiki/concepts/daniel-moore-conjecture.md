---
title: "Daniel-Moore Conjecture"
type: concept
tags: [ode, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: low
---

## Definition

Proved via order stars (Wanner-Hairer-Norsett 1978): A-stable s-stage RK or s-derivative multistep methods have order p <= 2s, equality requires diagonal Pade error constant.

## How It Works

Discussed and applied in the cited Hairer-Wanner chapter; see the source summary for the role this concept plays in the broader theory.

## Key Parameters

- See cited chapter for method-specific parameters, coefficients, and assumptions.

## When To Use

- When the cited Hairer-Wanner setting applies (stiff ODE, DAE, singular perturbation) and this concept is needed for stability, convergence, or order analysis.

## Risks & Pitfalls

- Subtleties beyond a low-confidence stub are not captured here; consult the cited chapter for proofs, counterexamples, and limitations.

## Related Concepts

- See the citing summary for the surrounding network of related concepts.

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
