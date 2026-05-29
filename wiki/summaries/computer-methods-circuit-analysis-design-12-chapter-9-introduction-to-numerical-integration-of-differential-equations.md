---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 9: Introduction
  to Numerical Integration of Differential Equations'
type: source
id: source-computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations
kind: derived-summary
tags:
- foundational
- transient
- numerical-integration
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt
---

## Key Points

- Two broad families of time-integration methods: Linear Multistep (LMS) and Runge-Kutta (R-K). This chapter focuses on three simplest LMS formulae as a foundation for Chapter 13. R-K methods are mentioned but not developed.
- Three foundational LMS formulae for x' = f(x,t):
  - Forward Euler (explicit): x_{n+1} = x_n + h x'_n.
  - Backward Euler (implicit): x_{n+1} = x_n + h x'_{n+1}.
  - Trapezoidal rule (implicit): x_{n+1} = x_n + (h/2)(x'_{n+1} + x'_n).
- Implicit methods (backward Euler, trapezoidal) require iteration since x'_{n+1} depends on the unknown x_{n+1}. A predictor (typically forward Euler) supplies the initial guess; the corrector is then applied iteratively until convergence.
- For linear ODEs x' = A x + w (state-variable normal form), the implicit step becomes a linear solve. Backward Euler: (I - h A) x_{n+1} = x_n + h w_{n+1}. Trapezoidal: (I - (h/2) A) x_{n+1} = (I + (h/2) A) x_n + (h/2)(w_{n+1} + w_n). For constant h, the matrix is factored once and reused.
- Order of integration p and truncation error c_{p+1}: by matching Taylor coefficients of the generic two-point formula a_1 x_1 + a_0 x_0 - h (b_1 x'_1 + b_0 x'_0) = 0:
  - Forward Euler: p=1, c_2 = -1/2.
  - Backward Euler: p=1, c_2 = 1/2.
  - Trapezoidal: p=2, c_3 = -1/12. (Higher p → smaller error at the same h.)
- Stability analysis on the test equation x' = lambda x. Each method's region of absolute stability in the q = lambda h complex plane:
  - Forward Euler: unit disk centered at -1 — small region; large step sizes diverge.
  - Backward Euler: outside the unit disk centered at +1 — stable for all Re lambda < 0 (A-stable) and large h.
  - Trapezoidal: stable in entire left half-plane (Re q < 0) — A-stable.
- A-stable formulae permit much larger time steps for stiff systems (widely separated time scales) — typical of circuits with both fast switching transients and slow envelope responses.
- For stiff systems, forward Euler must use a step h small enough to bring the fast pole into its tiny stability disk; backward Euler can use any h since it is unconditionally stable in the left half-plane.
- Time-domain solution of linear networks: instead of converting to state-variable form, work directly from G x + C x' = w (modified-nodal or tableau). Backward Euler gives (C + h G) x_{n+1} = C x_n + h w_{n+1}. Trapezoidal gives (C + (h/2) G) x_{n+1} = (C - (h/2) G) x_n + (h/2)(w_{n+1} + w_n). Both work even if C is singular (as in MNA with voltage sources and inductors) as long as C + h G is nonsingular.
- Companion models: each reactive element is replaced at each time step by a resistive companion plus a history-dependent source. Capacitor with backward Euler: i_{n+1} = (C/h)(v_{n+1} - v_n) — a conductance C/h in parallel with a current source (C/h) v_n. Inductor: a resistance L/h in series with a voltage source (L/h) i_n. Companion models reduce the transient problem to a sequence of resistive-network DC solves.

## Relevant Concepts

- [[concepts/forward-euler]] — Explicit one-step LMS formula.
- [[concepts/backward-euler]] — Implicit A-stable one-step formula.
- [[concepts/trapezoidal-rule]] — Implicit A-stable second-order formula.
- [[concepts/predictor-corrector]] — Explicit predictor + implicit corrector + fixed-point iteration.
- [[concepts/a-stability]] — Stability for all Re lambda < 0 regardless of h.
- [[concepts/stiff-systems]] — Multi-timescale ODEs that demand A-stable solvers.
- [[concepts/order-of-integration]] — Truncation order p; smaller error per step at same h.
- [[concepts/companion-model]] — Resistive-network equivalent of reactive elements at each time step.
- [[concepts/linear-multistep-methods]] — Already covered; this chapter introduces the simplest cases.
- [[concepts/numerical-integration-odes]] — Family of methods including LMS and R-K.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 9 — Introduction to Numerical Integration of Differential Equations
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt`
- Authors: Jiri Vlach, Kishore Singhal
