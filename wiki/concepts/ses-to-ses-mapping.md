---
title: SES-to-SES Mapping
type: claim
id: concepts/ses-to-ses-mapping
tags:
- simulation
- modeling
- ses
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

An SES-to-SES Mapping is a declarative set of transfer operations that projects values, specialization choices, and structural correspondences from a source SesPesPair to a target SesPesPair, producing an interest-tailored PES at the target side.

## How It Works

Operations include:
- `transfer SOURCE's ATTRIBUTE value to TARGET's ATTRIBUTE` — direct value copy.
- `transfer SOURCE's SPECIALIZATION choice to TARGET's SPECIALIZATION` — choice transfer.
- Context-qualified references (`AddressSection's IPAddress under DestHost`) when the same element name appears at multiple paths.
- Indexed multi-aspect references (`Port.0's portNumber under Ports`) for multi-entity occurrences.
- `pairUp`, `match`, and per-pair `transfer` for multi-aspect-to-multi-aspect mappings.

The `MappingSesPesPairToSesPesPair` class encapsulates source and target SesPesPair instances and provides the transfer methods. After all transfers, `finishPruning` fills any remaining choices and `printXMLFile` serializes the resulting PES.

## Key Parameters

- Source SesPesPair
- Target SesPesPair
- Ordered list of transfer operations
- Optional multi-aspect pairing rules

## When To Use

- Producing tailored consumer views of a producer's master PES
- Implementing the protocol/throughput/LAND/POD analysis SES projections
- Bridging incompatible SES vocabularies through a translation layer

## Risks & Pitfalls

- Forgetting `finishPruning` leaves the target PES under-specified
- Context-qualified references silently miss occurrences if paths drift
- Multi-aspect pairing imbalance discards data when source > target cardinality

## Related Concepts

- [[concepts/ses-pes-pair]]
- [[concepts/ses-xml-mapping]]
- [[concepts/interest-based-data-distribution]]

## Sources

- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
