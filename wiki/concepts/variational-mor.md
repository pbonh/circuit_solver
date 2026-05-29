---
title: Variational / Parameterized MOR
type: claim
id: claim-variational-mor
tags:
- mor
- process-variation
- statistical
- emerging
- vlsi
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/05-2-symbolic-analysis-techniques-in-a-nutshell.txt
confidence:
  base: 0.65
---

## Definition

Variational (or parameterized) model order reduction retains one or more process-variation parameters as symbols in the reduced model, so the reduced model can be re-evaluated for any sample of the parameter distribution without redoing the full reduction.

## How It Works

Approaches surveyed in the chapter include: (1) perturbation-based MOR — valid only for small variations; (2) multi-dimensional moment matching — Taylor-expand in both `s` and process variables, generating multi-dim moments; (3) interval / affine arithmetic MOR — propagate interval-valued matrices through reduction; (4) variational-subspace methods such as varPMTBR — sample in `(s, parameter)` space and build a joint subspace; (5) stochastic-FEM / Galerkin-orthogonal-polynomial methods for Gaussian/lognormal/uniform distributions.

## Key Parameters

- Number of variational parameters preserved.
- Sampling strategy in parameter space.
- Choice of orthogonal polynomial basis (Hermite, Legendre, etc.).
- Truncation order for moment-matching variants.

## When To Use

- Statistical timing / crosstalk analysis on interconnects.
- Variational analog/RF macromodels for yield/centering.
- Pre-stage for symbolic Monte Carlo (DDD-based statistical analysis).

## Risks & Pitfalls

- Multi-dim moment matching suffers exponential growth in parameter count.
- Interval/affine methods can over-estimate ranges.
- varPMTBR's optimal sample selection remains open.

## Related Concepts

- [[concepts/model-order-reduction]]
- [[concepts/process-variation]]
- [[concepts/monte-carlo-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
