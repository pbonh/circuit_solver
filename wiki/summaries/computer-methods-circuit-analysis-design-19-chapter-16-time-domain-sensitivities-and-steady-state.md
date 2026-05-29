---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 16: Time-Domain
  Sensitivities and Steady State'
type: source
id: source-computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state
kind: derived-summary
tags:
- advanced
- transient
- sensitivity
- analog
- well-established
- optimization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt
---

## Key Points

- Time-domain sensitivity computation for nonlinear networks is substantially more complex than frequency-domain sensitivity. Main applications: gradients for time-domain optimization problems and acceleration of steady-state computation.
- Sensitivity network method (Section 16.1): differentiate the algebraic-differential system equations with respect to a parameter h, producing a linear time-varying system (the "sensitivity network") with the same system matrix as the linearized original. The sensitivity network is integrated simultaneously with the original system at each BDF step, costing only one extra forward/back substitution per parameter per step.
- Initial conditions are an additional parameter: sensitivities with respect to initial conditions (q_0) describe how the time-domain solution changes when initial charges/fluxes vary. Critical for steady-state computation.
- Charge/flux formulation (from Chapter 13) makes the analysis clean: differential equations are linear; nonlinearities are confined to algebraic constraints. The sensitivity equations then have a regular structure.
- Adjoint/transpose method for time-domain (Section 16.2): when sensitivities of an objective function (integral over time of some response measure) with respect to many parameters are needed, integrate an adjoint system backward in time. Requires storage of the forward-time solution trajectory.
- Steady-state computation: finding the periodic solution X(t + T) = X(t) of a periodically excited nonlinear network without integrating through the initial transient.
  - Method 1 (Section 16.3, classical): Newton-Raphson on initial conditions q_0 such that q(T; q_0) = q_0. The Jacobian is the sensitivity matrix dq(T)/dq_0, computed by integrating the sensitivity network from t=0 to t=T.
  - Method 2 (Section 16.4): formulate as a 2-point boundary-value problem with the periodicity condition; use a shooting method that combines Newton-Raphson with sensitivity-network integration.
  - Method 3 (Section 16.5, the chapter's novel contribution): extrapolation-based steady-state finder. Does not require sensitivities or derivatives — uses the sequence of states at successive periods and accelerates convergence to the limit. Simple to program and gives excellent results.
- Steady-state methods are most useful for analyzing oscillators, mixers, and class-C amplifiers where the steady-state behavior (after transients die out) is the engineering interest. Direct transient integration would require many cycles of waste before reaching steady state.
- Powell's algorithm (referenced from Chapter 15) is the recommended optimization driver for steady-state-based design objectives.

## Relevant Concepts

- [[concepts/time-domain-sensitivity]] — Already covered; this chapter extends to nonlinear time-varying.
- [[concepts/sensitivity-network]] — Linear time-varying companion system for nonlinear sensitivity.
- [[concepts/steady-state-analysis]] — Finding periodic solutions efficiently.
- [[concepts/shooting-method]] — Newton-Raphson on initial conditions for boundary-value periodicity.
- [[concepts/extrapolation-steady-state]] — Derivative-free accelerated convergence.
- [[concepts/charge-flux-formulation]]
- [[concepts/gear-bdf]]
- [[concepts/algebraic-differential-equations]]
- [[concepts/optimization-theory]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 16 — Time-Domain Sensitivities and Steady State
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt`
- Authors: Jiri Vlach, Kishore Singhal
