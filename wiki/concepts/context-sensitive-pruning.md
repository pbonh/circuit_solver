---
title: "Context-Sensitive Pruning"
type: concept
tags: [simulation, modeling, ses, pruning, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt"]
confidence: medium
---

## Definition

Context-sensitive pruning lets a selection rule specify a partial path from an entity occurrence up toward the root (e.g., `for CPU under HP under Computer under JobContext`), constraining the rule to apply only to occurrences matching that path. A context-free rule is the special case where the path is empty.

## How It Works

The pruning algorithm collects, for each entity occurrence, all rules whose partial-context path is an initial segment of the occurrence's actual path to the root. If multiple rules match, the longest (most specific) wins; ties are broken by lexical order. If no rule matches, a context-free fallback applies, or — if none — random selection fills the choice.

## Key Parameters

- Partial-context path
- Conflict-resolution order (longest path wins)
- Context-free fallback rules

## When To Use

- Same specialization appearing under multiple parents with different desired defaults
- Per-brand exceptions to a context-free default
- Encoding business rules that depend on the parent path

## Risks & Pitfalls

- Long context paths are brittle if the SES is later refactored
- Ambiguous tie-breaking may produce surprising selections
- Easy to forget that an empty context is a global default

## Related Concepts

- [[concepts/automated-pruning]]
- [[concepts/rule-based-pruning]]
- [[concepts/ses-pruning]]

## Sources

- [[summaries/modeling-simulation-systems-10-8-automated-and-rule-based-pruning-and-experimental-execution]]
