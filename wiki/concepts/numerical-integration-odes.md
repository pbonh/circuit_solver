---
title: "Numerical Integration of ODEs"
type: concept
tags: [transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"]
confidence: high
---

## Definition

Numerical integration of ordinary differential equations approximates the solution x(t) of x' = f(x, t) at discrete time points t_0, t_1, ..., t_n. The two main families are Linear Multistep (LMS) methods and Runge-Kutta (R-K) methods.

## How It Works

LMS methods use values and derivatives at several past time points to advance one step: sum_{i=0..k} alpha_i x_{n-i} = h sum_{i=0..k} beta_i f_{n-i}. Single-step LMS examples: forward Euler, backward Euler, trapezoidal. Multi-step LMS examples: Adams-Bashforth (explicit), Adams-Moulton (implicit), Gear/BDF (implicit, used for stiff systems).

R-K methods use multiple intermediate stages per step (e.g., RK4: four function evaluations). They are self-starting and have small memory requirements but require more function evaluations per step than LMS.

For stiff circuit simulation, implicit LMS methods (especially Gear/BDF) are universally preferred. SPICE uses Gear for stiff problems and trapezoidal for oscillatory ones.

## Key Parameters

- Step size h (variable for adaptive methods).
- Order p (typically 1-6 for production codes).
- Stability properties.
- Per-step cost (function evaluations and linear solves).

## When To Use

- Transient simulation of any circuit with reactive elements.
- DC continuation methods (pseudo-transient).
- Any time-domain ODE/DAE problem.

## Risks & Pitfalls

- Step size affects both accuracy and stability; adaptive control is essential.
- Stiff systems demand A-stable or stiffly-stable methods.
- Initial-condition errors propagate; consistent initial conditions are vital for DAEs.

## Related Concepts

- [[concepts/linear-multistep-methods]]
- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/a-stability]]
- [[concepts/stiff-systems]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
