---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 17: Design by Minimization'
type: source
id: summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization
kind: publication
tags:
- optimization
- analog
- sensitivity
- well-established
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt
---

## Key Points

- Final chapter demonstrates CAD design by combining the methods of earlier chapters. The chapter is informal — its purpose is to motivate, not to provide exhaustive treatment.
- Strong advice: use other (deterministic) design tools to get a good initial estimate before optimization. Optimization on a poor initial point often finds local minima that are unacceptable.
- Realistic design flow: nominal design with ideal elements → add nonideal effects (losses, finite OPAMP bandwidth, parasitics) → sequence of minimizations with gradually-introduced nonidealities.
- Four minimization problem types presented:
  1. Mean-square amplitude matching (Section 17.1).
  2. Complex-value matching (Section 17.2) — e.g., for antenna lumped equivalents or impedance-matching network design.
  3. Minimax optimization (Section 17.3) — reduce the largest error peaks; difficult and expensive but provides the widest safety margin.
  4. Sensitivity minimization (Section 17.4) — minimize the sensitivity of an active network to component variations.
- Mean-square objective: E = (1/m) sum_i w_i (|phi(omega_i)| - A_i)^2 where omega_i are sample frequencies, A_i are target amplitudes, w_i are optional weights.
- Gradient via adjoint method (Chapter 6): for each parameter h:
  - dE/dh = Re((1/m) sum_i w_i (|phi_i| - A_i)|phi_i|/phi_i * (X^a)^T (dT/dh) X).
  - The adjoint RHS is -[|phi| - A]|phi|/phi times the d-vector for the original output.
  - One adjoint solve per frequency suffices to give the full gradient over all parameters.
- Transistor amplifier example (Fig. 17.1.1): an RC amplifier with R_L = 2k and L = 10 mH (initial) is optimized for gain = 40 with maximum bandwidth. After minimization: R_L = 2188.96 Ohm and L = 3.10417 mH. Gain 40 achieved with widened bandwidth.
- Complex-value matching (Section 17.2): network function phi(omega) is fit to a complex-valued target H(omega). Useful for antennas, transmission lines, and impedance-matching network synthesis. Mean-square error in complex space is differentiable; adjoint method gives gradient as in Section 17.1.
- Minimax (Section 17.3): minimize max_i |phi(omega_i) - H_i| via reformulation as a constrained problem with an auxiliary variable. Solvable by SQP or Powell-type algorithms.
- Sensitivity minimization (Section 17.4): the objective is the sum of squared multiparameter sensitivities. Useful for yield enhancement: a design with low sensitivities is robust to component tolerances.
- Monte Carlo verification (Section 17.5): the final design step samples element-value perturbations from realistic statistical distributions (uniform or normal) and counts how many trials meet specification. Estimates production yield. This complements but does not replace sensitivity-based design.

## Relevant Concepts

- [[concepts/objective-function]] — Already covered.
- [[concepts/mean-square-objective]] — L_2 matching of response to spec.
- [[concepts/minimax-optimization]] — L_infinity error minimization.
- [[concepts/sensitivity-minimization]] — Reducing yield-affecting sensitivities.
- [[concepts/monte-carlo-yield-analysis]] — Statistical-distribution verification of design.
- [[concepts/optimization-theory]]
- [[concepts/sensitivity-analysis]]
- [[concepts/adjoint-method]]
- [[concepts/sequential-quadratic-programming]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 17 — Design by Minimization
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt`
- Authors: Jiri Vlach, Kishore Singhal
