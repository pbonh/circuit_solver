---
title: "Modeling and Simulation of Systems — Chapter 11: Interest-Based Information Exchange: Mappings and Models"
type: summary
tags: [simulation, modeling, ses, xml, data-engineering, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/14-11-interest-based-information-exchange-mappings-and-models.txt"]
confidence: high
---

## Key Points

- Two-level framework: at the ontology level, an SES generates PESs; at the implementation level, the SES becomes an XML Schema and each PES becomes an XML document instance.
- Information exchange is recast in this framework: an event changes world state → producer encodes the resulting PES as an XML document → consumer receives and interprets per the same Schema.
- Different consumers can have different "pragmatic frames" — interests in different subsets of the producer's data. Example: car purchase event seen by DMV (full buyer identity) vs. manufacturer (aggregate by model).
- Network packet capture example: a master PacketInfo SES with SrcHost/DestHost/Protocol/Payload is mapped to interest-specific SESs (ThruPutAnalyze, ProtocolAnalyze, LANDAttack, PODAttack) to feed analysis and intrusion-detection consumers without sending full traces.
- Two implementation classes: `SesPesPair` packages an SES with one of its PES (and its XML representation); `MappingSesPesPairToSesPesPair` encapsulates source and target pairs with mapping operations.
- Mapping operations include: `transfer SOURCE's ATTRIBUTE value to TARGET's ATTRIBUTE`, `transfer SOURCE's SPEC choice to TARGET's SPEC`, context-qualified attribute references (`AddressSection's IPAddress under DestHost`), and dot-indexed access on multi-aspects (`Port.0's portNumber under Ports`).
- Multi-aspect to multi-aspect mapping uses a three-step pattern: `pairUp SOURCE's ENTITY with TARGET's ENTITY!`, `match SOURCE's ELEMENT with TARGET's ELEMENT!`, then transfer attribute values across matched pairs.
- DEVS implementations: GeneratorXML produces new XML documents periodically by re-pruning the SES; MapXML consumes XML, runs the mapping, and emits a target XML document; MapNDistribute generates per-consumer XML from a single source PES.
- A shared-document pattern (SimonSays example) lets multiple DEVS models fill different sections of the same XML document — CommandSection (Simon), ResponseSection (Player), YouAreOut (evaluation) — using presence specializations to mark which sections are present.
- The chapter is grounded in Zeigler & Hammonds, "Modeling & Simulation-Based Data Engineering" (2007).

## Relevant Concepts

- [[concepts/ses-xml-mapping]] — SES↔XML Schema, PES↔XML instance correspondence.
- [[concepts/interest-based-data-distribution]] — pragmatic-frame-driven information exchange.
- [[concepts/ses-pes-pair]] — implementation class packaging an SES with one of its PES.
- [[concepts/ses-to-ses-mapping]] — declarative transfer operations between SesPesPair instances.
- [[concepts/system-entity-structure]] — ontology base.
- [[concepts/pruned-entity-structure]] — XML-encoded data instance.
- [[concepts/discrete-event-system-specification]] — execution layer for the generator/mapper/distributor models.
- [[concepts/data-distribution-service]] — middleware that can carry the XML documents.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 11 — Interest-Based Information Exchange: Mappings and Models
- File path: `raw/ModelingAndSimulationOfSystems/_txt/14-11-interest-based-information-exchange-mappings-and-models.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
