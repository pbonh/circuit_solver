---
title: Lagrange Multipliers
type: claim
id: claim-lagrange-multipliers
tags:
- optimization
- foundational
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt
confidence:
  base: 0.85
---

## Definition

Lagrange multipliers convert equality-constrained optimization "minimize F(x) subject to e_j(x) = 0" into an unconstrained problem on the Lagrangian L(x, lambda) = F(x) - sum_j lambda_j e_j(x). At the optimum: nabla_x L = 0 (stationary in x) and nabla_lambda L = 0 (constraints satisfied).

## How It Works

The system nabla_x L = 0, e_j(x) = 0 is n + k equations in n + k unknowns (x and lambda). Linear constraints with a linear objective give a linear system; nonlinear constraints require Newton-Raphson.

Each lambda_j has physical meaning: it equals dF*/de_j, the rate of change of the optimal F when constraint j is loosened. This is a sensitivity interpretation that extends to KKT conditions for inequality-constrained problems.

## Key Parameters

- Number of equality constraints k.
- Sign convention (Vlach & Singhal use L = F - sum lambda e).

## When To Use

- Equality-constrained classical optimization.
- Derivation of optimality conditions.
- Building blocks of sequential quadratic programming (SQP).

## Risks & Pitfalls

- Constraint linear-independence required for solution existence.
- Inequality constraints need KKT extension; pure Lagrange does not directly handle them.
- Mostly of theoretical interest; modern algorithms (SQP, interior point) embed Lagrangian ideas indirectly.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/sequential-quadratic-programming]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
