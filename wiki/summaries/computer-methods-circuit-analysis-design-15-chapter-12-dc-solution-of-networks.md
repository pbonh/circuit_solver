---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 12: DC Solution
  of Networks'
type: source
id: summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks
kind: publication
tags:
- foundational
- dc
- analog
- sensitivity
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt
---

## Key Points

- The DC solution (operating point) of a nonlinear network is found by solving the nonlinear algebraic system f(x) = 0. It is the first step in nonlinear circuit analysis, providing the bias point about which small-signal AC and large-signal transient analyses are conducted.
- Newton-Raphson (N-R) algorithm in n variables:
  1. Form the Jacobian M = [df_i/dx_j].
  2. Solve M Delta x = -f(x^k) for the update.
  3. Set x^{k+1} = x^k + Delta x.
  Quadratic convergence near the solution; requires evaluation of the Jacobian (and its LU factorization) each iteration.
- Damping factor: x^{k+1} = x^k + zeta^k Delta x with 0 < zeta^k <= 1 to ensure ||f(x^{k+1})|| < ||f(x^k)||. Source stepping and gmin stepping are practical variants.
- Nodal formulation for DC: all inductors short, all capacitors open. KCL: A g(A^T v_n) = j_n where g is the (nonlinear) branch-current function of branch voltages. The Jacobian is M = A G_b A^T with G_b = dg/dv_b — the same form as the linear nodal admittance matrix with conductances replaced by dynamic conductances.
- Tableau / MNA for DC: similar derivation, can handle nonlinear elements that don't have a current-controlled form (e.g., voltage-source-like devices in the model).
- Special handling for exponential functions (Section 12.4): the exp() in diode and BJT equations can overflow or cause Newton-Raphson to take far too large a step. Standard remedies: limit the step (clamp Delta v_BE per iteration), use logarithmic damping, or use limit-stepping schemes that keep V_BE within a few V_T per iteration. These are critical to convergence.
- Sensitivity of DC solution to network parameters (Section 12.5): after solving f(v_n, p) = 0 with parameter p, dv_n/dp = -M^{-1} (df/dp). M is already factored, so the sensitivity costs only one extra forward/back substitution per parameter. This reuses the Chapter 6 adjoint framework — for a scalar output, m parameters need only one adjoint solve.
- Piecewise-linear analysis (Section 12.6) — Katzenelson's algorithm [1]: nonlinear functions tabulated point-by-point are linearized between breakpoints. The DC solution proceeds by tracking which linear region each device is in, taking steps along the curve, and switching regions when a breakpoint is crossed. Avoids Newton-Raphson convergence issues at the cost of careful region-tracking bookkeeping.

## Relevant Concepts

- [[concepts/dc-analysis]] — Steady-state DC operating-point computation.
- [[concepts/newton-raphson-method]] — Already covered.
- [[concepts/jacobian-matrix]] — Linearization of f for Newton-Raphson.
- [[concepts/damped-newton]] — Step-size control for global convergence.
- [[concepts/source-stepping]] — Continuation method for hard DC problems.
- [[concepts/piecewise-linear-analysis]] — Katzenelson algorithm.
- [[concepts/dc-sensitivity]] — Derivative of operating point with respect to parameters.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 12 — DC Solution of Networks
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt`
- Authors: Jiri Vlach, Kishore Singhal
