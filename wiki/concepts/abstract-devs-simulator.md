---
title: "Abstract DEVS Simulator"
type: concept
tags: [simulation, modeling, devs, foundational, theory, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt"]
confidence: high
---

## Definition

The Abstract DEVS Simulator is the technology-agnostic algorithm describing how a DEVS atomic model is executed. It specifies the interface that any DEVS atomic model must present (time-advance, output, internal/external/confluent transition functions and their associated sets) and the steps a conforming simulator implementation must perform.

## How It Works

A simulator instance is paired with a model and tracks last-event time `tL` and next-event time `tN = tL + ta(state)`. On each step it: (a) reports `tN`; (b) when the coordinator says current time `t == tN`, it invokes the model's output function; (c) on receiving input `m` with elapsed `e = t - tL`, it invokes external transition; (d) when imminent and no external input, it invokes internal transition; (e) for simultaneous internal and external events it invokes the confluent function.

## Key Parameters

- Atomic model state and event times
- Time-advance, output, internal/external/confluent functions
- Last-event time, next-event time

## When To Use

- Implementing any DEVS simulator that purports to be standard
- Wrapping non-DEVS engines so they participate in DEVS coordination
- Reasoning about the closure-under-coupling property at run time

## Risks & Pitfalls

- Incorrect handling of confluent events differs subtly from classic vs Parallel DEVS
- Floating-point time comparisons must be done with care
- Missing the elapsed-time bookkeeping breaks external-transition semantics

## Related Concepts

- [[concepts/atomic-devs-model]]
- [[concepts/devs-simulation-protocol]]
- [[concepts/coupled-devs-model]]
- [[concepts/closure-under-coupling]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
