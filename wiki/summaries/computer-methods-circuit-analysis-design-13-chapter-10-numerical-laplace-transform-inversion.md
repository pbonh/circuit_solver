---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 10: Numerical Laplace
  Transform Inversion'
type: source
id: source-computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion
kind: derived-summary
tags:
- advanced
- transient
- analog
- well-established
- numerical-integration
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/13-chapter-10-numerical-laplace-transform-inversion.txt
---

## Key Points

- Numerical Laplace transform inversion (NILT) computes the time-domain response of a linear network without finding poles and residues. A program for frequency-domain analysis can be repurposed for time-domain solution with only minor additions.
- The Vlach NILT method (Vlach 1969, [1], [2] of the chapter): substitute z = s t into the Laplace inversion integral, then approximate e^z by a Padé rational function R_{N,M}(z) = P_N(z)/Q_M(z). The first M+N+1 Taylor coefficients of R_{N,M} match those of e^z (Eq. 10.1.6 gives a closed form).
- For M sufficiently larger than N, all poles z_i of R_{N,M} lie in the right half-plane. Closing the contour around these poles gives the basic inversion formula v(t) ≈ -(1/t) sum K_i V(z_i / t), with K_i the residues. Complex-conjugate poles combine to halve evaluations (Eq. 10.1.12).
- Computation: for each desired time t, divide each precomputed z_i by t, evaluate V(s) at that complex frequency, multiply by precomputed K_i, sum, take real part, divide by -t. All z_i and K_i are tabulated (Table 10.1.2 in the book, more in [3]).
- Properties:
  - Handles stiff systems naturally.
  - Handles multiple poles without special treatment.
  - Handles distributed-parameter networks (transcendental network functions in s).
  - Handles Dirac impulses and their derivatives easily.
  - Inversion at t = 0 fails by division-by-zero; use initial-value theorem or substitute a tiny t (e.g., 10^{-10}).
- Accuracy degrades as t grows large; the basic formula is most accurate for small t. To extend accuracy to large t, "step" the method: at each new step, reset the time origin and use the previous state as initial condition.
- Stepping algorithm equivalence: when applied repeatedly, the stepped NILT method is equivalent to a very high-order A-stable integration method. The chapter establishes its stability properties.
- Time-domain sensitivity: by differentiating V(s) symbolically (using the adjoint method of Chapter 6 in the frequency domain), NILT gives time-domain sensitivities essentially "for free" — a key advantage over direct time-stepping integration.
- Distributed-element example: an exponentially tapered RC line has transfer function involving cosh and sinh of (s + 0.5)^{1/2}. NILT computes its step response directly from this transcendental closed form; direct time-stepping ODE methods cannot.
- RC line accuracy demonstration: with M = 10, N = 8, the relative error in v(t) is ~10^{-6} for t = 0.1 to 2.2, comparing against the exact infinite-series solution.
- The stepping algorithm is coded in the Appendix D analysis program.

## Relevant Concepts

- [[concepts/numerical-laplace-transform-inversion]] — Already covered; this chapter gives the Vlach method details.
- [[concepts/pade-approximation]] — Rational function matching Taylor expansion of e^z.
- [[concepts/nilt-stepping]] — Time-origin reset for extended-time accuracy.
- [[concepts/distributed-element-analysis]] — Transcendental-s transfer functions in NILT.
- [[concepts/time-domain-sensitivity]] — Adjoint-based sensitivity in time domain via NILT.
- [[concepts/laplace-transform]]
- [[concepts/adjoint-method]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 10 — Numerical Laplace Transform Inversion
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/13-chapter-10-numerical-laplace-transform-inversion.txt`
- Authors: Jiri Vlach, Kishore Singhal
