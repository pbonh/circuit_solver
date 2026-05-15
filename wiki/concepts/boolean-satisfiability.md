---
title: "Boolean Satisfiability (SAT)"
type: concept
tags: [algorithm, digital, boolean, eda, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

The Boolean satisfiability problem (SAT) asks whether there exists an assignment of truth values to a set of Boolean variables that makes a given Boolean formula evaluate to true. SAT was the first problem proven NP-complete (Cook-Levin theorem, 1971).

## How It Works

Modern SAT solvers (MiniSat, Glucose, Kissat) use Conflict-Driven Clause Learning (CDCL), unit propagation, and sophisticated heuristics on conjunctive-normal-form (CNF) inputs. AIG- and BDD-based engines provide alternate routes. SAT underlies model checking, equivalence checking (via the miter construction: connect outputs of two circuits to an XOR gate; the SAT instance asks whether the XOR can be 1), and bounded reachability.

## Key Parameters

- Number of variables and clauses.
- Clause-to-variable ratio (phase-transition behavior).
- Restart and heuristic policy.

## When To Use

- Hardware verification (equivalence and model checking).
- Software verification, planning, scheduling.
- Solving combinatorial problems that admit a SAT encoding.

## Risks & Pitfalls

- Worst-case exponential time (NP-complete).
- Encoding choices dramatically affect solver runtime.

## Related Concepts

- [[concepts/and-inverter-graph]]
- [[concepts/ordered-binary-decision-diagram]]
- [[concepts/electronic-design-automation]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
