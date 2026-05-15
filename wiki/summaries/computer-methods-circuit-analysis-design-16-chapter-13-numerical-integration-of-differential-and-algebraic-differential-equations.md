---
title: "Computer Methods for Circuit Analysis and Design — Chapter 13: Numerical Integration of Differential and Algebraic-Differential Equations"
type: summary
tags: [foundational, transient, numerical-integration, well-established, advanced, sparse-matrix]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt"]
confidence: high
---

## Key Points

- Higher-order linear multistep (LMS) formulae generalize the three formulae of Chapter 9 (forward Euler, backward Euler, trapezoidal). A unified derivation via polynomial interpolation matches past values x_{n+k-j} and derivatives x'_{n+k-j}.
- For a polynomial of degree m, m+1 matching conditions determine the coefficients d_i. Predictors do NOT use x_{n+k} (explicit), correctors DO (implicit). The transpose-system trick (analogous to Chapter 6's adjoint method) avoids solving the full Vandermonde-like system for each step — only the coefficient relating x_{n+k} to past data is needed.
- Example formulas derived this way:
  - Forward Euler (predictor, k=1, p=1): x_{n+k} = x_{n+k-1} + h x'_{n+k-1}.
  - Adams-Bashforth 2nd-order predictor: x_{n+k} = x_{n+k-1} + (h/2)(3 x'_{n+k-1} - x'_{n+k-2}).
  - Backward Euler (corrector, k=1, p=1).
  - Trapezoidal (corrector, k=1, p=2).
  - Adams-Moulton 2nd, 3rd, ... order correctors.
  - Gear backward differentiation formulas (BDF) of orders 1-6.
- Stability and order of integration are not independent: Dahlquist's theorem says no A-stable LMS formula has order > 2 (the trapezoidal rule is at the boundary). Gear's BDF formulas of orders up to 6 are stiffly-stable (a weaker condition adequate for stiff systems) — these are recommended for stiff circuits.
- Section 13.4 extends LMS to systems x' = f(x, t). For nonlinear systems, each implicit step requires Newton-Raphson iteration; the Jacobian is the small-signal admittance matrix at the current iterate.
- Section 13.5: step-size and order control. Modern variable-step, variable-order BDF codes (DASSL, Gear's original 1971 code) estimate local truncation error from the difference between predictor and corrector and adjust h and order to maintain a target error per step. Larger h, lower order, and re-factoring the Jacobian are all options.
- Algebraic-differential (DAE) systems: nodal/MNA/tableau formulations naturally produce DAEs (some equations have no derivative term). LMS methods extend to DAEs via the same predictor-corrector + Newton approach, but require attention to:
  - Index (number of differentiations needed to convert to ODEs).
  - Consistent initial conditions (the algebraic constraints must hold at t = 0).
  - Convergence rates (some BDF variants are more sensitive to high-index DAEs).
- Section 13.6: when nonlinear capacitors or inductors are present, introducing charges q and fluxes phi as additional state variables linearizes the differential equations and pushes all nonlinearities into algebraic relations q = f(v), phi = f(i). The combined system:
  - Algebraic: q_b = q(v_b), phi_b = phi(i_b), and resistive constitutive equations.
  - Differential: dq_b/dt = i_b (capacitor), dphi_b/dt = v_b (inductor).
- The combination of BDF integration + charge/flux state variables + Newton-Raphson at each step is the standard approach in modern simulators (SPICE-class). The matrices and stamps remain sparse and the same sparse solver of Chapter 2 applies.

## Relevant Concepts

- [[concepts/linear-multistep-methods]] — Already covered.
- [[concepts/adams-bashforth]] — Explicit LMS predictor family.
- [[concepts/adams-moulton]] — Implicit LMS corrector family.
- [[concepts/gear-bdf]] — Backward differentiation formulas for stiff systems.
- [[concepts/algebraic-differential-equations]] — DAEs arising from MNA/tableau.
- [[concepts/charge-flux-formulation]] — Linear differential / nonlinear algebraic split for nonlinear reactive elements.
- [[concepts/step-size-control]] — Adaptive h based on local truncation error estimate.
- [[concepts/order-control]] — Adaptive integration order in BDF/Adams codes.
- [[concepts/stiff-stability]] — Adequate for stiff problems (relaxation of A-stability).
- [[concepts/numerical-integration-odes]]
- [[concepts/newton-raphson-method]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 13 — Numerical Integration of Differential and Algebraic-Differential Equations
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt`
- Authors: Jiri Vlach, Kishore Singhal
