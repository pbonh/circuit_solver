---
title: "Extrapolation Method"
type: concept
tags: [ode, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: low
---

## Definition

Richardson/Aitken-Neville extrapolation of a base discretisation on a sequence of step sizes; yields adaptive-order methods like SEULEX and SODEX.

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

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
