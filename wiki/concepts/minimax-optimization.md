---
title: Minimax Optimization
type: claim
id: concepts/minimax-optimization
tags:
- optimization
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Minimax optimization minimizes the worst-case (L_infinity) error: minimize max_i |phi(omega_i) - H_i|. Compared to mean-square optimization, minimax reduces the largest error peaks while allowing smaller errors to grow. Provides the widest safety margin against specification violations.

## How It Works

Reformulate as a constrained problem: minimize gamma subject to |phi(omega_i) - H_i| <= gamma for all i. The new variable gamma is to be minimized; the inequality constraints can be handled by SQP, interior-point, or Powell-type algorithms.

Vlach & Singhal note that minimax is generally more difficult and expensive than mean-square but produces better worst-case performance, important for production designs with tight specifications.

## Key Parameters

- Frequency sample set {omega_i}.
- Target values H_i.
- Algorithm (SQP, semi-infinite programming).

## When To Use

- Tight-spec filter design (equiripple Chebyshev/elliptic filters).
- Robust-control design.
- Critical RF/microwave matching.

## Risks & Pitfalls

- More iterations and inner-problem solves than mean-square.
- Non-smooth objective (the max is not differentiable).
- Local minima can also occur.

## Related Concepts

- [[concepts/objective-function]]
- [[concepts/mean-square-objective]]
- [[concepts/optimization-theory]]
- [[concepts/sequential-quadratic-programming]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
