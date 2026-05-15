---
title: "Constrained DEVS"
type: concept
tags: [simulation, modeling, devs, verification, model-checking, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt"]
confidence: medium
---

## Definition

Constrained DEVS is a variant of the DEVS formalism in which state spaces, input/output port value sets, and time-advance functions are bounded to finite, computable ranges so that the resulting models can be model-checked by a verification engine such as the DEVS-Suite verifier (Gholami and Sarjoughian 2017).

## How It Works

Constraints applied:
- Every state variable has a finite typed value set, expressible via regular expressions (e.g., `queue: (String)^8 → ((Char)^4)^8`).
- Every input port has a finite value set plus a NULL value to admit pure-internal transitions.
- Time-advance functions are restricted to finite rational values, bounding the number of distinct internal/external transition firings.
- Data that is routed but not processed (e.g., NoC flits) can be excluded from state to further shrink the state space.

The verification engine performs reachability analysis, tracking visited (V), to-be-visited (W), and unsafe (U) state sets, terminating when W is empty or an unsafe state is reached.

## Key Parameters

- Per-state-variable value set
- Per-input-port value set plus NULL
- Time-advance range (finite rationals)
- Optional data-exclusion policy
- Unsafe-state predicate set

## When To Use

- Network-on-Chip and other discrete-event hardware model verification
- Safety analysis where unsafe-state reachability matters
- Liveness and deadlock detection

## Risks & Pitfalls

- Over-constraining can elide real-world behaviors of interest
- State-space size still explodes with high-cardinality variables
- Regex-style state specifications require care

## Related Concepts

- [[concepts/cosmos-framework]]
- [[concepts/model-verification]]
- [[concepts/atomic-devs-model]]
- [[concepts/finite-deterministic-devs]]

## Sources

- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
