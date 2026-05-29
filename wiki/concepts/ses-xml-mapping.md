---
title: SES↔XML Mapping
type: claim
id: concepts/ses-xml-mapping
tags:
- simulation
- modeling
- ses
- xml
- data-engineering
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

The SES↔XML mapping is the systematic correspondence between an SES and an XML Schema (at the ontology vs. implementation level) and between a PES of that SES and an XML document instance of the Schema. It is the basis for transporting modeled data over the Internet in a platform-neutral form.

## How It Works

Each SES entity becomes an XML Schema complex type; aspects map to nested elements; specializations map to choice groups; variables map to typed attributes. Pruning at the SES level corresponds to choosing one branch of each choice group at the XML-instance level. Producers serialize PESs as XML documents; consumers parse them back into PES form for interpretation. Multi-aspects map to repeated XML elements indexed by id.

## Key Parameters

- Entity ↔ complex type
- Aspect ↔ nested element
- Specialization ↔ choice group
- Variable ↔ typed attribute
- Presence specialization ↔ optional element

## When To Use

- Sharing model data across heterogeneous platforms (Java, C++, etc.)
- Persisting design alternatives to disk in a portable format
- Integrating DEVS-based simulations with web services and DDS middleware

## Risks & Pitfalls

- XML verbosity adds overhead vs. binary encodings
- Schema evolution requires care to keep documents valid
- Mapping multi-aspect cardinality requires id-bearing index attributes

## Related Concepts

- [[concepts/system-entity-structure]]
- [[concepts/pruned-entity-structure]]
- [[concepts/ses-pes-pair]]
- [[concepts/ses-to-ses-mapping]]

## Sources

- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
