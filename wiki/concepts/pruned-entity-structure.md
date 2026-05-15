---
title: "Pruned Entity Structure (PES)"
type: concept
tags: [simulation, modeling, ses, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/07-5-specialization-and-pruning.txt"]
confidence: high
---

## Definition

A Pruned Entity Structure (PES) is the result of resolving every specialization and multi-aspect in an SES to a single choice and cardinality. It carries all the information needed to automatically construct a specific hierarchical simulation model.

## How It Works

The pruning interface walks the SES tree, presenting choices at each specialization. Selections are saved in a pruning script file that can be re-run, edited, and applied to revised SES versions. Any unmade selections may be filled in randomly during transformation. Once complete, the PES is transformed into a DEVS coupled model whose components are populated from the atomic-model repository.

## Key Parameters

- Selected choice per specialization
- Selected cardinality per multi-aspect
- Variable bindings per entity
- Random-fill policy for unspecified slots

## When To Use

- Materializing a specific SoS architecture for simulation
- Generating model variants for parameter-sweep experiments
- Storing reusable pruning scripts as design assets

## Risks & Pitfalls

- Inconsistency between a stored pruning script and a revised parent SES
- Random fills can mask intentional gaps in selections
- Many PES per SES requires disciplined naming/versioning

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/ses-pruning]]
- [[concepts/ses-specialization]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-07-5-specialization-and-pruning]]
- [[summaries/modeling-simulation-systems-08-6-aspects-and-multi-aspects]]
- [[summaries/modeling-simulation-systems-10-8-automated-and-rule-based-pruning-and-experimental-execution]]
- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
