---
title: Rule-Based Pruning
type: claim
id: concepts/rule-based-pruning
tags:
- simulation
- modeling
- ses
- pruning
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Rule-based pruning uses conditional `if-then` and `if-not-then` (unless) rules — placed in the `*.ses` file — to link the choice for one specialization to choices made for others. Rules constrain which selections automated pruning may make.

## How It Works

`if select X from spec1 for E1 then select Y from spec2 for E2!` causes selection of X to imply selection of Y wherever the contexts match. `if not select X ...` expands into explicit if-then rules covering every other choice in the specialization parent of X — useful for "default unless exception" semantics (e.g., patient activity → low sampling unless exercising). Conditions and actions can themselves be context-sensitive.

## Key Parameters

- `if select ... then select ...` form
- `if not select ... then select ...` (unless) form
- Context paths in conditions and actions
- Placement in `*.ses`

## When To Use

- Linking compute power to job size
- Forcing fastProcess aspect when timeConstrained is selected
- Default-with-exceptions modeling (sampling rates, wiring gauges)

## Risks & Pitfalls

- Rules can conflict with explicit pes selections — explicit overrides win
- Unless-expansion may create many implicit rules that are hard to debug
- Cyclic dependencies among rules can confuse the pruner

## Related Concepts

- [[concepts/automated-pruning]]
- [[concepts/context-sensitive-pruning]]
- [[concepts/ses-specialization]]

## Sources

- [[summaries/modeling-simulation-systems-10-8-automated-and-rule-based-pruning-and-experimental-execution]]
