---
title: "ADR-0006: Dual Convergence Criterion for Newton-Raphson"
adr_id: ADR-0006
status: accepted
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
supersedes: []
superseded_by: null
asr:
  - "Newton-Raphson iteration must reliably distinguish true convergence from false convergence caused by iteration stall (small Δv/ΔI but large residue)."
tags: [numeric-solver, newton-raphson, convergence, dc-analysis, transient-analysis]
created: 2025-07-18
---

# ADR-0006: Dual Convergence Criterion for Newton-Raphson

## Y-Statement

**In the context of** the numeric solver's Newton-Raphson iteration loop,
**facing** the risk of false convergence where both Δv and ΔI appear small while the residue norm remains large (a stall condition caused by device-model derivative errors or near-singular Jacobians),
**we decided for** a dual convergence criterion requiring both the update norm ‖Δx‖ and the residue norm ‖F(x)‖ to fall below their respective tolerances,
**and against** the single-criterion alternatives (update-only or residue-only),
**to achieve** reliable convergence detection that catches stalls and mis-reported convergence,
**accepting** that the dual check adds one extra norm computation per iteration and may reject some iterations that update-only would have accepted (increasing iteration count on marginally convergent problems).

## Architecturally Significant Requirement

Newton-Raphson convergence detection is architecturally significant because it gates every DC operating-point and transient timestep solve. A false-positive convergence report produces silently wrong results (voltages and currents that do not satisfy KCL/KVL) that propagate through subsequent analyses (AC, noise, transient continuation). This risk is called out explicitly in [[concepts/newton-raphson-method]]'s pitfalls: "False convergence under the ΔI check: if NR stalls, Δv and ΔI both look small even though the residue is large."

## Options Considered

### Option A — Update-only criterion
Convergence is declared when ‖Δx‖ < ε_update.

- **Pros:** Cheapest to compute (one norm); matches the behavior of several legacy SPICE implementations; most iterations that satisfy update-only also satisfy residue.
- **Cons:** Cannot detect stall conditions where the Jacobian is nearly singular or device-model derivatives are wrong; may report "converged" with a large residue, producing silently incorrect OperatingPoint results.

### Option B — Residue-only criterion
Convergence is declared when ‖F(x)‖ < ε_residue.

- **Pros:** Directly checks that KCL/KVL is satisfied; catches stalls that update-only misses.
- **Cons:** Residue norms can be misleading for circuits with widely varying current magnitudes (e.g., pA leakage vs. A supply currents); normalization is non-trivial; does not detect oscillation between two nearly-correct iterates where residue is small but the iterate keeps bouncing.

### Option C — Dual criterion (chosen)
Convergence is declared when **both** ‖Δx‖ < ε_update **and** ‖F(x)‖ < ε_residue.

- **Pros:** Catches both stall (large residue) and oscillation (small residue but large update); provides the strongest guarantee that the reported solution is a genuine root of F(x) = 0; aligns with the [[concepts/newton-raphson-method]] pitfall mitigation.
- **Cons:** One additional norm computation per iteration (marginal cost relative to sparse-LU factorization); may increase iteration count on circuits where update-only would have stopped earlier; requires the user (or defaults) to configure two tolerance parameters.

## Consequences

- **Positive:** Every converged OperatingPoint and transient timestep satisfies both KCL/KVL (residue) and update stability, eliminating the dominant source of silently wrong SPICE results.
- **Positive:** The dual criterion naturally supports homotopy methods: during Gmin-stepping, both criteria must be met at each homotopy step, ensuring that intermediate solutions are genuine before reducing the shunt conductance.
- **Negative:** Two tolerance parameters (`reltol` for updates, `abstol` for residues) must be exposed in the `AnalysisRequest` API and documented; default values must be chosen conservatively to avoid unnecessary convergence failures.
- **Negative:** Circuits with pathological scaling (very large and very small currents simultaneously) may need per-node or per-branch absolute tolerances rather than a single global residue norm; this is a tuning concern, not a correctness concern.
- **Follow-up:** The `NewtonRaphsonDriver` component in `numeric-solver` must compute and report both norms in its `ConvergenceStatus` return value, so that diagnostics can distinguish "update converged but residue did not" from "residue converged but update did not."

## Supersession

This ADR does not supersede any prior ADR. It complements ADR-0002 (sparse-direct solver backend) and ADR-0005 (closed-enum device model dispatch) by ensuring that the solver's convergence detection is as reliable as its linear algebra and model evaluation.
