---
title: "Multibody System"
type: concept
tags: [mechanical, dae, simulation, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A multibody system is a mechanical model composed of rigid (or flexible) bodies connected by joints (revolute, prismatic, spherical) and subject to applied forces (gravity, springs, dampers, contact). The dynamics are typically expressed as a constrained Lagrangian system M(q) q̈ = f(q, q̇) − G(q)^T λ, g(q) = 0 — an [[concepts/index-3-dae]] [[concepts/constrained-mechanical-system]].

## How It Works

Multibody formulations use *generalised coordinates* (joint angles, body positions) plus *kinematic constraints* (loop closures, joint axes). The number of constraints equals (degrees of joints constrained) per joint; for a system with l independent loops, l × 6 (in 3D) closure constraints typically arise. The benchmark Andrews [[concepts/squeezer-mechanism]] has 7 bodies, 3 loops, and 6 algebraic constraints; the Hairer–Wanner treatment provides a complete Fortran reference implementation. Multibody simulation tools (Modelica, Simpack, ADAMS, MBDyn) rely on DAE solvers: explicit half-explicit for nonstiff, BDF (DASSL) or RODAS / RADAU5 for stiff. References: Schiehlen 1990 ([[entities/werner-schiehlen]]).

## Key Parameters

- Number of bodies and joints.
- Constraint count (loop closures).
- Stiffness ratio (rigid vs. flexible bodies).
- DAE index (3 / 2 / 1 depending on formulation level).

## When To Use

- Vehicle dynamics, robot manipulators, biomechanics, machinery design.
- Real-time interactive simulations.
- Validation problems for DAE solvers.

## Risks & Pitfalls

- Constraint drift over long simulations; pair with [[concepts/baumgarte-stabilization]] or projection.
- Constraint redundancy (over-determined loops) needs special linear-algebra handling.
- Singular configurations (gimbal lock, lockup) require numerical detection.

## Related Concepts

- [[concepts/constrained-mechanical-system]]
- [[concepts/index-3-dae]]
- [[concepts/half-explicit-method]]
- [[concepts/projected-runge-kutta]]
- [[concepts/squeezer-mechanism]]
- [[concepts/pendulum-equation]]
- [[concepts/lagrange-multiplier]]
- [[concepts/baumgarte-stabilization]]
- [[entities/phem56]]
- [[entities/dassl]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
