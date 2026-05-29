---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 15: Introduction
  to Optimization Theory'
type: source
id: summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory
kind: publication
tags:
- optimization
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt
---

## Key Points

- The optimization problem: minimize F(x) subject to equality e_j(x) = 0 and inequality g_j(x) >= 0 constraints. Unconstrained if no constraints. F is the objective function, x the design vector.
- Designer's responsibility: formulate F(x) and supply the gradient nabla F. CAD specialists provide gradients via the adjoint-method sensitivity computations of Chapter 6.
- Convex functions have a global minimum reachable by any downhill descent. Non-convex functions have local minima that can trap descent algorithms. In practice, multiple starts from different initial points reveal local minima.
- Inequality constraints define a feasible region. Active constraints touch the optimum; inactive ones don't. The boundary of the feasible region matters.
- Gradient: nabla F = [dF/dx_1, ..., dF/dx_n]^T points uphill; -nabla F points downhill. A direction s is descent if -s^T nabla F > 0.
- Classical minimization (Section 15.2): unconstrained — set nabla F = 0; for equality constraints, use Lagrange multipliers L(x, lambda) = F(x) - sum lambda_j e_j(x) and solve nabla_x L = 0, nabla_lambda L = 0. Mostly theoretical importance for circuit design.
- Modern iterative algorithms (Section 15.3): generate x^{k+1} = x^k + d_k s^k with descent direction s^k and step length d_k. The matrix S of search directions is updated each iteration. Successful when F(x^{k+1}) < F(x^k).
- Line search (Section 15.4): given direction s^k, find d_k = argmin F(x^k + d s^k). Sufficient algorithms include quadratic/cubic interpolation, golden section, or simple backtracking. The line search need not find the exact minimum; a sufficient-decrease condition (Wolfe condition) is often enough.
- Search direction (Section 15.5-15.6, no read but inferred): the most difficult part of optimization. Methods:
  - Steepest descent: s = -nabla F. Simple, but slow on ill-conditioned problems.
  - Newton: s = -H^{-1} nabla F where H is the Hessian. Quadratic convergence near minimum but expensive (Hessian + factorization per step).
  - Quasi-Newton (BFGS, DFP): build an approximation B to H^{-1} from successive gradient differences. Superlinear convergence without computing the Hessian explicitly.
  - Conjugate gradient: builds search directions that are mutually conjugate with respect to H. Linear memory, superlinear convergence on quadratic problems.
- Constrained minimization (Section 15.7): emphasis on Powell's algorithm (an early SQP variant) that has proved very effective in practice. Modern approaches include:
  - Penalty methods.
  - Augmented Lagrangian.
  - Sequential quadratic programming (SQP) — Vlach & Singhal highlighted SQP as one of the four CAD-enabling innovations in the Motivation chapter.

## Relevant Concepts

- [[concepts/optimization-theory]] — Overall framework.
- [[concepts/objective-function]] — Scalar function to minimize.
- [[concepts/gradient]] — n-vector of partial derivatives.
- [[concepts/lagrange-multipliers]] — For equality constraints.
- [[concepts/quasi-newton-method]] — BFGS, DFP families.
- [[concepts/line-search]] — Step-length determination.
- [[concepts/sequential-quadratic-programming]] — Already covered.
- [[concepts/steepest-descent]] — Simplest gradient method.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 15 — Introduction to Optimization Theory
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt`
- Authors: Jiri Vlach, Kishore Singhal
