---
title: Timing Simulation
type: claim
id: claim-timing-simulation
tags:
- digital
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

Timing simulation (also called fast or reduced-accuracy circuit simulation) is a class of MOS-digital simulators that trade accuracy for speed against full SPICE-level simulation, typically achieving 10–100× speedup on large MOS digital designs. The approach partitions the circuit into small subcircuits, uses explicit integration ([[concepts/forward-euler]]), and applies simplified device models.

## How It Works

- **Partition** the circuit into single-node or small subcircuits at points of weak coupling. Partitioning requires loosely-coupled, largely-unidirectional subcircuits with capacitors to ground at every node, and a scheduling order along signal-propagation paths. Tight feedback loops cannot be partitioned and must be handled monolithically.
- **Explicit integration** ([[concepts/forward-euler]]) eliminates the per-step matrix solve and Newton iteration (given linear capacitors), which is the main source of speedup.
- **Simplified models** linearize capacitors, discard floating caps (approximating with grounded Miller caps), and drop the subtleties of full device models — back-gate bias, subthreshold, nonlinear capacitance.

## Key Parameters

- Partition granularity and tolerance for residual feedback
- Model fidelity setting (which device subtleties are dropped)
- Internal timestep (bounded by stability since FE is being used)

## When To Use

- **Pure MOS digital** circuits where the speedup is realized and the simplifications are acceptable.
- **Mixed-signal timing simulation** with analog/bipolar partitions handled by full circuit simulation embedded inside the timing simulator (subcircuit-based partitioning, implicit integration for analog blocks, two sets of models).

## Risks & Pitfalls

- Limited applicability: **not** suitable for memories (sense amps, cell internals), busses, analog circuits, or bipolar circuits.
- Strong assumptions on circuit structure mean violated assumptions produce *plausible but incorrect* answers — riskier than a slower SPICE-class run.
- Mixed-signal speedup is bounded by the analog fraction; typical real-world gain is 2-5× when analog content is non-trivial.
- Often requires significant manual tweaking to achieve correct or performant results.

## Related Concepts

- [[concepts/forward-euler]]
- [[concepts/transient-analysis]]
- [[concepts/stiff-circuit]]
- [[concepts/mixed-level-simulation]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
