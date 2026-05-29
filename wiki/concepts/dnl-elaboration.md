---
title: DNL Elaboration
type: claim
id: claim-dnl-elaboration
tags:
- simulation
- modeling
- fddevs
- java
- devs
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt
confidence:
  base: 0.65
---

## Definition

DNL elaboration is the mechanism in MS4 Me by which FDDEVS natural-language `*.dnl` files are extended with tagged Java code blocks that are inserted into the generated atomic-model Java source at well-defined points. Elaboration converts a base FDDEVS skeleton into a fully expressive DEVS atomic model without losing round-trip traceability.

## How It Works

The dnl file uses `<% ... %>!` delimiters around Java fragments under tag headers like `Initialize variables`, `internal event for STATE`, `external event for STATE with PORT`, `output event for STATE`, and `add Library`. Companion declarations introduce instance variables (`use NAME with type TYPE and default "EXPR"!`), port types (`accepts input on PORT with type CLASS!`, `generates output on PORT with type CLASS!`), and helper classes (`A WorkToDo has id, processingTime, startTime!`). On save, MS4 Me regenerates the Java class with the fragments inserted at the right hooks, preserving FDDEVS as the single source of truth.

## Key Parameters

- Tagged block kinds: Initialize, internal/external/output events, add Library
- Instance-variable declarations with default expressions
- Port-type bindings
- User-defined message classes with ranges and defaults

## When To Use

- Adding state beyond FDDEVS phases
- Overriding default next-state transitions for non-deterministic behavior
- Handling multiple simultaneous inputs/outputs in a single message bag
- Integrating Java libraries (collections, random number generators)

## Risks & Pitfalls

- Errors in tagged blocks are deferred to Java compilation
- Forgetting to declare ports via "accepts/generates" before using them in code
- Inserted code can shadow FDDEVS defaults in confusing ways

## Related Concepts

- [[concepts/finite-deterministic-devs]]
- [[concepts/atomic-devs-model]]
- [[concepts/object-oriented-simulation]]

## Sources

- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
- [[summaries/modeling-simulation-systems-09-7-managing-inheritance-in-pruning]]
- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
