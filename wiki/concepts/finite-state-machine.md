---
title: Finite State Machine
type: claim
id: concepts/finite-state-machine
tags:
- graph
- digital
- foundational
- well-established
- synchronization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/04-1-introduction.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A finite state machine (FSM) is a mathematical model of computation in which a system can be in exactly one of a finite number of states at any given time and transitions between states in response to input events. The two canonical formulations are the Mealy machine (output depends on state and input, described by G. H. Mealy, 1955) and the Moore machine (output depends only on state, described by E. F. Moore, 1956).

## How It Works

An FSM is naturally represented as a directed graph: nodes are states, edges are transitions labeled with input conditions (and outputs in Mealy form). Synchronous digital circuits implement FSMs in hardware via registers holding state and combinational logic computing the next state and outputs. FSMs allow abstract reasoning about synchronous behavior while ignoring lower-level circuit details.

## Key Parameters

- Number of states.
- Input alphabet size.
- Output behavior (Mealy vs. Moore).
- Determinism (DFA vs. NFA) — usually deterministic in hardware.

## When To Use

- Modeling sequential digital control logic.
- Protocol design and verification.
- Behavioral synthesis as an intermediate representation.

## Risks & Pitfalls

- State explosion for complex systems; hierarchical FSMs or statecharts mitigate this.
- Race conditions and metastability in asynchronous implementations.
- One-hot vs. binary encoding tradeoffs in area, power, and timing.

## Related Concepts

- [[concepts/graph-theory]]
- [[concepts/vlsi-design]]
- [[concepts/timing-graph]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
