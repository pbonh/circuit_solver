---
title: "Radau IIA as the primary stiff ODE/DAE integrator"
status: proposed
date: 2026-06-16
decision-makers:
  - circuit-solver-team
consulted: []
informed: []
---
# Radau IIA as the primary stiff ODE/DAE integrator


## Context and Problem Statement

Transient circuit simulation requires numerical integration of stiff systems of differential-algebraic equations (DAEs) arising from non-linear elements (diodes, transistors) and reactive components (capacitors, inductors).

Traditional SPICE simulators use Backward Differentiation Formula (BDF) methods, typically Gear's method (BDF2), which trade computational cost for robustness on stiff problems. However, recent research in numerical methods for DAEs (Hairer & Wanner, *Solving Ordinary Differential Equations II: Stiff and Differential-Algebraic Problems*) recommends Radau IIA as the gold standard for index-1 DAEs.

The decision impacts:
- Accuracy and stability on stiff circuits (deep submicron RC ladders, oscillator subcircuits)
- Computational cost per integration step (Radau requires multiple Newton iterations per step)
- Compatibility with SPICE reference behavior
- Quality of automatic error control
- Ability to handle discontinuities and mixed-signal transitions

## Decision Drivers

1. **L-stability and accuracy on stiff systems**: Radau IIA is A-stable and L-stable, avoiding TR ringing and spurious oscillations on very stiff problems.
2. **Error control quality**: Radau IIA's embedded method provides reliable local error estimation without Richardson extrapolation.
3. **Self-starting**: Radau IIA requires no history; each step is independent (unlike BDF which requires warm-up).
4. **Index-1 DAE handling**: The formulation naturally handles voltage source and inductor constraints without index reduction tricks.
5. **Mixed-signal circuits**: Circuits with oscillators, phase-locked loops, and switching events demand high-order accuracy on long transients.
6. **Academic consensus**: Hairer & Wanner recommend Radau IIA for index-1 DAEs with confidence intervals; RADAU5 is the reference implementation.

## Considered Options

### Option 1: Radau IIA (Order 5)
- **Pros**:
  - A-stable and L-stable; no spurious oscillations on stiff circuits.
  - 5th-order accuracy; large stable timesteps on stiff problems.
  - Self-starting; no history buffer or warm-up required.
  - Hairer & Wanner recommend Radau IIA as gold standard for index-1 DAEs.
  - Embedded error estimator (4th order) provides reliable local error control.
  - RADAU5 is the reference implementation; extensive published comparisons.
- **Cons**:
  - Each step requires ~3 Newton-Raphson solves (3 stages); more expensive per step than BDF2.
  - Jacobian evaluation 3 times per step (though may be amortized with Jacobian caching).
  - Initial implementation effort larger than BDF (implicit Runge-Kutta staging).
  - Less familiar to circuit designers accustomed to SPICE BDF.

### Option 2: BDF1 / BDF2 (SPICE Standard)
- **Pros**:
  - Mature, well-understood; all SPICE simulators use BDF2.
  - Lower cost per step; fewer Jacobian evaluations.
  - Existing SPICE device models assume BDF timestep control.
  - Circuit designers intuitive with BDF behavior.
- **Cons**:
  - BDF2 is only A-stable, not L-stable; can exhibit ringing on very stiff problems.
  - Requires warm-up history (2 prior points); not self-starting.
  - Lower order (2 vs 5); requires smaller timesteps on smooth problems.
  - Error estimation via Richardson extrapolation is less reliable.
  - No advantage on modern hardware; the cost of extra Newton iterations is negligible.

### Option 3: Rosenbrock Methods
- **Pros**:
  - A-stable and L-stable.
  - Requires only 1 Jacobian LU factorization per step (vs repeated evaluation in Runge-Kutta).
  - Lower cost than Radau on problems where Jacobian evaluation dominates.
- **Cons**:
  - Lower accuracy (order 3–4 typically).
  - Requires explicit Jacobian matrix (not just Jacobian-vector products).
  - Less suitable for index-1 DAEs without additional reduction.

### Option 4: SDIRK (Singly Diagonally Implicit Runge-Kutta)
- **Pros**:
  - A-stable and L-stable.
  - Similar cost to Radau (requires multiple NR solves).
  - Suitable for index-1 DAEs.
- **Cons**:
  - Lower accuracy than Radau IIA (typical order 3–4).
  - No clear advantage over Radau IIA for circuit problems.

## Decision Outcome

**Decision**: Adopt **Radau IIA (Order 5)** as the default primary integrator, with **BDF2 exposed as a selectable fallback** for SPICE compatibility mode.

**Rationale**:
1. Hairer & Wanner's extensive numerical analysis establishes Radau IIA as the gold standard for index-1 DAEs; their recommendation is backed by decades of experience.
2. L-stability eliminates TR ringing and spurious oscillations on stiff circuits, improving accuracy without ad-hoc damping.
3. Self-starting and high order (5) allow large timesteps on smooth transients, reducing total simulation time despite higher cost per step.
4. The embedded error estimator provides reliable automatic step control, critical for mixed-signal transients with varying stiffness.
5. On modern hardware, the cost of 3 NR solves per Radau step is negligible compared to sparse matrix factorization and device evaluation.

**Fallback**: BDF2 remains available as an option for circuits where Radau proves too aggressive (e.g., circuits with frequent discontinuities forcing tiny steps). Users can select via `integrator: "bdf2"` in configuration.

## Consequences

1. **Computational cost**: Each Radau step costs approximately 3× the NR solves of a single BDF2 step. However, 5th-order accuracy may permit 10× larger timesteps, yielding net speedup on smooth transients. Mixed-signal circuits with discontinuities show smaller net gains.
2. **Jacobian re-evaluation**: Three stages per step require three Jacobian evaluations or one LU factorization + backsolves. Cache Jacobians aggressively; use low-rank updates if feasible.
3. **Numerical sensitivity**: Radau's high order can amplify model discontinuities (diode switch, relay snap). Circuit models must be smooth; discontinuity handling deferred to event detection layer.
4. **Reference compatibility**: Circuits simulated with Radau will differ numerically from SPICE (BDF2) even with identical timesteps. Tolerance tuning may be required.

## Confirmation

1. **Accuracy benchmark**: Simulate a stiff RC ladder (100 stages, τ = 1 µs to 10 ns) and LC oscillator (resonance Q = 100):
   - Radau with h = 100 ns must give correct amplitude within 1% of reference solution at h = 1 ns.
   - BDF2 with same h = 100 ns must give amplitude error ≥ 5% on the oscillator (demonstrating L-stability advantage).
2. **Self-starting verification**: Transient starting from t=0 (no history) must converge without bootstrapping.
3. **Error estimator test**: Embedded error estimate must correlate with true error (correlation > 0.95) across test suite.
4. **Device discontinuity handling**: Circuits with ideal diode switches must not cause Radau to fail; step size control must catch slope discontinuities.

## Pros and Cons of the Options

| Criterion | Radau IIA | BDF2 | Rosenbrock | SDIRK |
|-----------|-----------|------|-----------|-------|
| L-stability | ✓ yes | ✗ no | ✓ yes | ✓ yes |
| Accuracy (order) | ✓ 5 | ~ 2 | ~ 3-4 | ~ 3-4 |
| Cost per step | ✗ 3× NR | ✓ 1× NR | ✗ 2× NR | ✗ 2-3× NR |
| Self-starting | ✓ yes | ✗ needs history | ✓ yes | ✓ yes |
| Embedded error | ✓ robust | ~ Richardson | ~ medium | ~ medium |
| Index-1 DAE | ✓ native | ~ needs care | ~ medium | ✓ native |
| SPICE compatibility | ✗ no | ✓ yes | ✗ no | ✗ no |
| Timestep on smooth | ✓ 10× larger | ~ small | ~ medium | ~ medium |
| Implementation effort | ✗ high | ✓ low | ~ medium | ~ medium |

## Evidence

This decision is grounded in the following wiki evidence:
- [[stiff-ode-methods]] — Stiffness, L-stability, A-stability, and comparative analysis of stiff ODE integrators
- [[bdf-methods]] — Backward Differentiation Formulas, Gear's method, warm-up requirements, and SPICE implementation
- [[solving-ode-ii-stiff-dae]] — Hairer & Wanner's authoritative treatment of DAE solvers and Radau IIA recommendations
- [[integration-methods]] — General framework for ODE/DAE integration methods, convergence, error control
- [[runge-kutta-methods]] — Implicit Runge-Kutta methods, staging, and L-stability theory
