---
title: Monte Carlo Yield Analysis
type: claim
id: claim-monte-carlo-yield-analysis
tags:
- optimization
- statistics
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt
confidence:
  base: 0.65
---

## Definition

Monte Carlo yield analysis estimates the fraction of manufactured circuits that will meet specification by simulating many trial designs with element values randomly drawn from realistic statistical distributions (uniform, normal, lognormal). Used as the final verification step before mass production.

## How It Works

1. Define statistical distributions for each element (typical: normal with mean = nominal, std = tolerance / 3 to give 99.7% within +/- tolerance).
2. Sample N trial designs (N typically 1000-100000).
3. For each sample: run the simulation, check whether all specifications are met.
4. Yield = (samples meeting spec) / N.
5. Optionally, identify which spec is violated most often to guide redesign.

Monte Carlo complements sensitivity-based design: sensitivity tells you which components are critical; Monte Carlo gives a quantitative yield estimate accounting for higher-order effects and statistical correlations.

## Key Parameters

- Number of trials N (controls confidence interval).
- Statistical model (uniform vs. normal, correlations).
- Specifications to verify.
- Variance-reduction techniques (Latin hypercube, low-discrepancy sequences).

## When To Use

- Final design verification.
- Yield comparison between alternative designs.
- Setting tolerance specifications for components.

## Risks & Pitfalls

- Computational cost — each Monte Carlo trial is a full simulation; thousands required for accurate yield estimation.
- Statistical model assumptions (Gaussian independence) may not reflect real-world correlations.
- Cannot replace sensitivity analysis for understanding why yield is low.

## Related Concepts

- [[concepts/sensitivity-minimization]]
- [[concepts/multiparameter-sensitivity]]
- [[concepts/optimization-theory]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
