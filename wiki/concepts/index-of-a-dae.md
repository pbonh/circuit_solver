---
title: "Index of a DAE"
type: concept
tags: [dae, classification, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The index of a [[concepts/differential-algebraic-equation]] is an integer classification measuring how far the system is from being an ordinary ODE. Several non-equivalent definitions exist: the [[concepts/differentiation-index]] di (Gear–Petzold 1983/84), the [[concepts/perturbation-index]] pi (Hairer–Lubich–Roche 1989), the [[concepts/index-of-nilpotency]] (for linear constant-coefficient pencils), and Campbell's geometric index.

## How It Works

For linear constant-coefficient B u' + A u = d, the index equals the size of the largest nilpotent block in the [[concepts/weierstrass-kronecker-form]]. For nonlinear semi-explicit DAEs, the *differentiation* index is the number of times one must differentiate the algebraic constraints to extract an underlying ODE: index 1 differentiates g(y, z) once via g_y f + g_z z' = 0 (assuming g_z invertible); index 2 requires differentiating a constraint g(y) once (giving the [[concepts/hidden-constraint]] g_y f(y, z) = 0); index 3 requires two differentiations (typical of constrained mechanical systems). The *perturbation* index measures sensitivity: pi = m if perturbations of size δ produce errors bounded in terms of ‖δ‖, …, ‖δ^{(m−1)}‖. The two can differ arbitrarily (Lubich's M(y)y' = f(y) example; Campbell–Gear's nilpotent Jordan example).

## Key Parameters

- Differentiation index di.
- Perturbation index pi.
- Geometric / nilpotency index for special forms.

## When To Use

- Selecting a numerical method appropriate to the index.
- Diagnosing convergence failures of an integrator on a poorly-posed DAE.
- Theoretical classification of model categories.

## Risks & Pitfalls

- Different index definitions can disagree; always state which one is used.
- The index can change along the solution if a Jacobian loses rank — these are "structural singularities."
- High-index DAEs (index ≥ 3) require special handling: [[concepts/index-reduction]], [[concepts/projection-method-dae]], or [[concepts/half-explicit-method]].

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/differentiation-index]]
- [[concepts/perturbation-index]]
- [[concepts/index-of-nilpotency]]
- [[concepts/index-1-dae]]
- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]
- [[concepts/weierstrass-kronecker-form]]
- [[concepts/index-reduction]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
