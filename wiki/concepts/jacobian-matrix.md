---
title: Jacobian Matrix (in Nonlinear DC Analysis)
type: claim
id: concepts/jacobian-matrix
tags:
- foundational
- dc
- numerical
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/15-chapter-12-dc-solution-of-networks.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Jacobian matrix M of a vector function f(x) is the matrix of all first partial derivatives M_ij = df_i/dx_j. In nonlinear circuit analysis, the Jacobian is the system matrix of the Newton-Raphson iteration, formed by linearizing the network around the current iterate.

## How It Works

For nodal formulation of a nonlinear circuit: M = A G_b A^T where G_b is the diagonal matrix of dynamic conductances dg/dv_b. The Jacobian has the same sparsity pattern as the linear-network admittance matrix — linear elements contribute constant G; nonlinear elements (diodes, transistors) contribute V-dependent dynamic conductance.

Each Newton iteration:
- Evaluate the Jacobian M(x^k) and the residual f(x^k).
- LU-factor M.
- Solve M Delta x = -f(x^k).
- Update x^{k+1} = x^k + Delta x.

## Key Parameters

- Sparsity pattern (same as linear-network admittance).
- Conditioning at the current iterate (poor when devices are in steep regions).

## When To Use

- Newton-Raphson DC operating-point analysis.
- Each implicit time step of transient analysis (linearization of the algebraic constraint).
- Continuation methods (Gmin stepping, source stepping).

## Risks & Pitfalls

- Singularity when no current flows (e.g., capacitor at DC); gmin stepping adds tiny shunt conductances to prevent singular Jacobians.
- Large dynamic conductances in steep regions of diode/transistor curves can cause ill-conditioning.
- Recomputing and refactoring per iteration is expensive; quasi-Newton variants (Broyden) trade off accuracy for speed.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/dc-analysis]]
- [[concepts/damped-newton]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
