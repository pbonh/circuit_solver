---
title: "A-Stability"
type: concept
tags: [transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"]
confidence: high
---

## Definition

A linear multistep formula is A-stable if it produces a bounded solution to the test equation x' = lambda x for any step size h and any number of steps, whenever Re lambda < 0. The region of absolute stability includes the entire left half of the q = h lambda plane.

## How It Works

A-stability is the strongest practical stability requirement. The Dahlquist barrier theorem proves that no explicit LMS method is A-stable and no A-stable LMS method has order greater than 2. The trapezoidal rule (p=2) and backward Euler (p=1) are A-stable; higher-order BDF methods (Gear) are stiffly-stable (a weaker condition that still works for stiff problems) but not strictly A-stable for orders >2.

## Key Parameters

- Region of absolute stability (subset of q = h lambda plane).
- Method order p.
- Whether the formula is implicit (necessary for A-stability beyond order 1).

## When To Use

- Stiff systems requiring large time steps.
- Long-time transient simulation where step-size constraints from stability would be prohibitive.

## Risks & Pitfalls

- A-stability does not imply accuracy — large h still gives large truncation error.
- A-stable methods may have numerical damping (backward Euler) or non-damping (trapezoidal); choose accordingly.
- Stiff stability is preferable for stiff systems with widely separated time constants.

## Related Concepts

- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/stiff-systems]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
