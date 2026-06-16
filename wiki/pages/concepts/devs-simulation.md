---
title: DEVS Simulation
type: concept
slug: devs-simulation
created: 2026-06-16
updated: 2026-06-16
summary: Discrete Event System Specification — a formal, hierarchical, modular simulation framework where atomic models define timed state transitions and coupled models compose them with port-coupling.
tags: [devs, simulation, discrete-event, systems-of-systems, modeling]
sources: [modeling-simulation-systems]
status: active
---

# DEVS Simulation

DEVS (Discrete Event System Specification, Zeigler 1976) is a formal modeling and simulation framework. Unlike ODE-based simulation ([[spice-simulation]]), DEVS handles discrete-event behavior — state changes triggered by events, not continuous time integration.

## Atomic DEVS Model

An atomic DEVS model M = (X, Y, S, δ_int, δ_ext, λ, ta) where:
- X = input event set, Y = output event set, S = state set
- ta: S → ℝ⁺₀ — time advance function (how long in each state)
- δ_int: S → S — internal transition (fires after ta expires)
- δ_ext: Q × X → S — external transition (fires on input event, Q = elapsed time)
- λ: S → Y — output function (fires just before internal transition)

## Coupled DEVS Model

A coupled DEVS model N = (X, Y, D, {M_i}, EIC, EOC, IC) where D = set of component names, {M_i} = component DEVS models, EIC/EOC/IC = coupling relations (external input/output couplings, internal couplings). DEVS is **closed under coupling** — N is itself a valid DEVS model, enabling arbitrary hierarchical composition.

## System Entity Structure (SES)

SES is a tree-shaped ontology specifying: entities (system components), aspects (decompositions into subentities), specializations (variant choices). Pruning = selecting one variant per specialization → generates a specific coupled DEVS model. Enables automated experiment generation over a combinatorial configuration space.

## Connection to Circuit Simulation

| | SPICE/Spectre | DEVS |
|---|---|---|
| Domain | Continuous-time analog | Discrete-event behavioral |
| Time | Dense (real) | Discrete events |
| Models | Differential/algebraic equations | State machines with timing |
| Composition | Netlist connections | Port coupling |
| Simulation | NR + integration | Event queue + message passing |

DEVS is the right model for:
- Digital logic behavior in mixed-signal systems
- System-level behavioral models for PLLs, ADCs (when precise analog detail is not needed)
- Protocol modeling (e.g., test bench behavior)
- Distributed simulation of large SoS where subsystems have different model fidelity levels

Compare to [[verilog-ams]]: Verilog-AMS covers both continuous-time (analog block) and event-driven (always/initial blocks). DEVS provides a more formal, composable framework but requires more infrastructure.

## Related concepts and entities

- [[circuit-simulation]] - the continuous-time simulation counterpart
- [[verilog-ams]] - language-level analog/digital event-driven simulation
- [[modeling-simulation-systems]] - source book (via slug `modeling-simulation-systems`)
