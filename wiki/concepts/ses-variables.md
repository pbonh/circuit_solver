---
title: SES Variables
type: claim
id: concepts/ses-variables
tags:
- simulation
- modeling
- ses
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/07-5-specialization-and-pruning.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

SES Variables are typed values attached to SES entities that carry numeric or discrete data, optionally constrained per specialization choice. They participate in pruning and in the transformation from PES to executable simulation model.

## How It Works

Syntax: `An ENTITY has var1, var2, var3!` declares variables. `The range of an ENTITY's var1 is TYPE with values [LO, HI]!` constrains type and range. Specialization-conditional bindings like `Set short_boy's height to [20, 40]!` narrow ranges for a particular choice. During PES interpretation, values may be sampled randomly from the allowed range, enabling parameter sweeps and Monte Carlo experiments.

## Key Parameters

- Variable name and type
- Default value
- Range (double interval, discrete set, etc.)
- Specialization-conditional overrides

## When To Use

- Encoding numerical attributes of entities (height, weight)
- Driving design-space exploration with constrained variation
- Linking SES choices to downstream simulation-model parameters

## Risks & Pitfalls

- Forgetting to constrain ranges leads to unbounded random fills
- Specialization-conditional bindings can be inconsistent if a variable has multiple constraints
- Type mismatches between declared range and downstream Java code

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/ses-specialization]]
- [[concepts/pruned-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-07-5-specialization-and-pruning]]
