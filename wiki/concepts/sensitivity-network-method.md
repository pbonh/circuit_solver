---
title: Sensitivity Network Method
type: claim
id: claim-sensitivity-network-method
tags:
- sensitivity
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt
confidence:
  base: 0.85
---

## Definition

The sensitivity network method computes the sensitivities of all solution variables X (typically node voltages) to a single parameter h by differentiating the system equation TX = W with respect to h and solving T (dX/dh) = -(dT/dh) X + dW/dh. The same LU factorization of T is reused; only one extra forward/back substitution per parameter is required.

## How It Works

After solving TX = W by LU decomposition, the right-hand side -(dT/dh) X + dW/dh is assembled from the known solution X and the symbolic derivatives dT/dh, dW/dh. The factored T is then applied to recover dX/dh.

This method is preferred when many output variables but few parameters are of interest — the cost is one extra solve per parameter, regardless of how many outputs are extracted from dX/dh.

## Key Parameters

- Matrix size n.
- Number of parameters (each requires one extra forward/back substitution).
- Sparsity of dT/dh and dW/dh (typically very sparse — one or two nonzero entries).

## When To Use

- Sensitivity of the entire solution vector with respect to a single parameter.
- Few parameters but many outputs of interest.
- Educational illustration of the basic differentiation approach.

## Risks & Pitfalls

- Cost grows linearly with the number of parameters — for many parameters, the adjoint method is more efficient.
- Requires storage of dX/dh per parameter if all are kept.

## Related Concepts

- [[concepts/transpose-system-method]]
- [[concepts/adjoint-method]]
- [[concepts/sensitivity-analysis]]
- [[concepts/lu-decomposition]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
