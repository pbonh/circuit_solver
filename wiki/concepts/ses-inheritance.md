---
title: "SES Inheritance"
type: concept
tags: [simulation, modeling, ses, inheritance, java, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/09-7-managing-inheritance-in-pruning.txt"]
confidence: medium
---

## Definition

SES Inheritance is the mechanism by which underscores embedded in SES entity names (e.g., `Slow_GeneratorOfJobs`) determine the Java class subclass/superclass relationship of the generated atomic model, and serve as a controllable hook for selecting which side of the underscore drives inheritance.

## How It Works

In a hyphenated underscore name `A_B`, by default `B` (the parent in the SES, last token in the concatenation) is the superclass; the generated `A_B` class extends `B` and forwards its constructor arguments. The parent's constructor stores `modelName`, which can be parsed in `initialize()` to configure behavior per the prefix. The pruning script can override the default with `inherit from A!` to make `A` the superclass instead — preferred when the child is a fully implemented behavior and the parent is a placeholder.

## Key Parameters

- Underscore-separated name tokens
- Default-rightmost-token superclass rule
- `inherit from NAME!` pruning-script override
- Configuration via parsed-name in parent's `initialize()`

## When To Use

- Reusing a parent class with multiple specialization-driven configurations (Slow/Fast)
- Composing fully implemented child behaviors into named instances (Reactive/Proactive players)
- Creating multiple instances of the same class via name prefixing (First_/Second_)

## Risks & Pitfalls

- Multi-token underscore names can confuse the default-superclass rule
- Forgetting `inherit from X!` results in unintended parent-as-superclass generation
- Overriding `initialize()` in the parent class to parse name strings is fragile

## Related Concepts

- [[concepts/ses-specialization]]
- [[concepts/ses-pruning]]
- [[concepts/atomic-devs-model]]
- [[concepts/object-oriented-simulation]]

## Sources

- [[summaries/modeling-simulation-systems-09-7-managing-inheritance-in-pruning]]
