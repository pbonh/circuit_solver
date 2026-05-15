---
title: "Sequential Quadratic Programming (SQP)"
type: concept
tags: [optimization, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt"]
confidence: medium
---

## Definition

Sequential quadratic programming (SQP) is one of the four major numerical innovations highlighted by Vlach and Singhal for modern CAD: a class of iterative methods for solving constrained nonlinear optimization problems by repeatedly solving a quadratic-programming (QP) subproblem that approximates the original problem.

## How It Works

At each iterate *x_k*, SQP forms a QP subproblem whose objective is a quadratic approximation of the Lagrangian and whose constraints are linearizations of the original constraints. The QP solution gives a search direction; a line search or trust region updates *x_k*. Hessian information is typically updated by quasi-Newton (e.g., BFGS) approximations.

## Key Parameters

- Hessian approximation strategy (quasi-Newton update, damped BFGS, full Newton).
- Globalization technique (line search or trust region).
- Merit function or filter for accepting steps.
- Tolerance on KKT conditions.

## When To Use

- Constrained nonlinear optimization in design — e.g., choosing element values to meet specifications subject to inequality and equality constraints.
- Whenever derivatives (gradients) are available, especially cheaply (e.g., from adjoint sensitivities).

## Risks & Pitfalls

- Convergence requires good initial estimates for difficult nonconvex problems.
- The QP subproblem can be infeasible or unbounded if constraints are inconsistent.
- Quasi-Newton updates may degrade on highly nonconvex Hessians.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/adjoint-method]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
