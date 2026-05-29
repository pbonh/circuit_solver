---
title: SesPesPair
type: claim
id: concepts/ses-pes-pair
tags:
- simulation
- modeling
- ses
- xml
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/14-11-interest-based-information-exchange-mappings-and-models.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A SesPesPair is an MS4 Me implementation class that encapsulates, as instance variables, an SES together with a PES pruned from it. It also generates the XML Schema for the SES and the XML document for the PES, providing a complete chunk of context for data-engineering operations.

## How It Works

Constructors load existing SES/PES files or create new ones from natural-language scripts. The class exposes methods such as `setSesfile`, `setPesfile`, `setProjectNm`, `printXMLFile`, and `XMLPES` to read/write the underlying artifacts. Generator DEVS models hold a SesPesPair instance variable and emit its `XMLPES` on each output event.

## Key Parameters

- SES source file path
- PES (pruning script) file path
- Project name
- XML Schema and document accessors

## When To Use

- Generator DEVS models producing PES samples as XML
- Source-and-target packaging in SES-to-SES mappings
- Persisting both ontology and instance in a single object

## Risks & Pitfalls

- File-path coupling makes deployment fragile
- Re-pruning during a run requires seeding for reproducibility
- Schema regeneration overhead on each instantiation

## Related Concepts

- [[concepts/ses-to-ses-mapping]]
- [[concepts/ses-xml-mapping]]
- [[concepts/system-entity-structure]]
- [[concepts/pruned-entity-structure]]

## Sources

- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
