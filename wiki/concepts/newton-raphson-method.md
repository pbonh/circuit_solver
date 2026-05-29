---
title: Newton-Raphson Method
type: claim
id: claim-newton-raphson-method
tags:
- analog
- foundational
- well-established
- dc
- transient
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

The Newton-Raphson (NR) method, also called Newton's method, is the iterative root-finding algorithm at the heart of every SPICE-class circuit simulator. Given a nonlinear system f(x) = 0 and an initial guess x₀, NR constructs a linear approximation at the current iterate (using f and its Jacobian), solves the linear system for an update, and repeats until the update and residue are below tolerance.

## How It Works

At iteration k, NR linearizes the nonlinear system around the current iterate xₖ: f(x) ≈ f(xₖ) + J(xₖ)(x − xₖ) where J is the Jacobian. Setting the linearization to zero yields the update xₖ₊₁ = xₖ − J(xₖ)⁻¹ f(xₖ), which in [[concepts/modified-nodal-analysis]] corresponds to building and solving a sparse linear system one iteration at a time. The procedure repeats until simultaneous update and residue criteria are met. In [[concepts/transient-analysis]], NR is run inside every timestep on the difference equation produced by the chosen [[concepts/integration-method]]; in [[concepts/dc-analysis]], it solves the algebraic system obtained by discarding time derivatives.

## Key Parameters

- Convergence tolerances: an update check `|Δv| < ε` (important at high-impedance nodes) plus a residue check
- Residue check variants: SPICE's `ΔI` check (susceptible to false convergence) vs. Spectre's KCL check `|ΣI| < δ` (robust but more expensive)
- `reltol`, `abstol`, `vntol` — the user-facing knobs that scale the tolerances
- Initial guess source: previous timepoint in transient, the DC operating point in AC, user-supplied [[concepts/nodeset]] or zero in DC

## When To Use

NR is used everywhere a nonlinear algebraic system must be solved inside a circuit simulator — DC operating-point calculation, every Newton iteration of every transient timestep, the inner solve of harmonic balance and shooting Newton, and the consistency solve in [[concepts/pseudo-transient-analysis]].

## Risks & Pitfalls

- Convergence is guaranteed only when (1) the model equations are sufficiently smooth, (2) the solution is isolated, and (3) the initial guess is close enough. The third is hard in practice, motivating [[concepts/homotopy-method]] continuation aids.
- False convergence under the ΔI check: if NR stalls (often due to an error in the device-model derivative), Δv and ΔI both look small even though the residue is large.
- Non-isolated solutions (floating nodes, loops of shorts) cannot be reached by standard NR — `Gmin` and topology checkers exist to flag/avoid them.
- Singular or ill-conditioned Jacobians degrade convergence; pivoting and regularization help but cannot fix structurally degenerate problems.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/homotopy-method]]
- [[concepts/dc-analysis]]
- [[concepts/transient-analysis]]
- [[concepts/integration-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
