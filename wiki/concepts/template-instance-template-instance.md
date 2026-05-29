---
title: Template / Instance Template / Instance Model Hierarchy (CoSMoS)
type: claim
id: concepts/template-instance-template-instance
tags:
- simulation
- modeling
- cosmos
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

In CoSMoS, every logical, visual, and persistent model is structured in three layers: Template Model (composition and specialization with at most two-level hierarchy), Instance Template Model (any finite hierarchy referencing templates), and Instance Model (concrete simulatable instance derived from an Instance Template).

## How It Works

The modeler authors templates by declaring composition/specialization relationships. Instance Template Models combine templates into deeper hierarchies, choosing among specializations and composing reusable parts. Instance Models materialize concrete simulation runs. Two Instance Template Models may share templates while differing in composition; Instance Models inherit from their Instance Template's structure but can specialize behavior parameters. The layering supports systematic, incremental model development.

## Key Parameters

- Template depth (≤ 2 levels with self-composition allowed)
- Instance Template depth (arbitrary finite)
- Reuse of templates across Instance Templates
- Instance-level behavior parameters

## When To Use

- Library-style management of reusable SoS components
- Systematic model-family generation
- Versioning and reuse across collaborative teams

## Risks & Pitfalls

- Naming and identity must remain consistent across levels
- Template editing can ripple through many instances
- Database-backed persistence requires migration discipline

## Related Concepts

- [[concepts/cosmos-framework]]
- [[concepts/system-entity-structure]]
- [[concepts/parallel-devs]]

## Sources

- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
