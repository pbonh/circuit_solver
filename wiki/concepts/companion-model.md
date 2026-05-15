---
title: "Companion Model"
type: concept
tags: [vlsi, circuit, analysis, transient, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt"]
confidence: high
---

## Definition

A companion model is the discrete-time equivalent circuit obtained by numerical integration (e.g., Backward Euler, Trapezoidal) of a transient element such as a capacitor or inductor. Within a time step, the dynamic element is replaced by an equivalent linear resistor in parallel (or series) with an independent current (or voltage) source whose value depends on the previous time-step state.

## How It Works

For a capacitor under Backward Euler with step h: i_C(t^k) = g_eq v_C(t^k) + i_eq with g_eq = C/h and i_eq = -C/h · v_C(t^{k-1}). The companion model converts a dynamic system at each time step into a resistive system that can be solved via MNA. Different integration rules (Trapezoidal, Gear-2) produce different companion models with different stability and accuracy.

## Key Parameters

- Integration rule (Backward Euler, Trapezoidal, Gear).
- Time step h.
- Capacitance / inductance value.

## When To Use

- Transient circuit simulation in SPICE-class tools.
- High-level synthesis of analog/mixed-signal blocks.

## Risks & Pitfalls

- Stability depends on the chosen integration rule.
- Trapezoidal rule can introduce numerical ringing.
- Adaptive time-stepping is usually required for efficiency.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/laplacian-matrix]]
- [[entities/spice]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
