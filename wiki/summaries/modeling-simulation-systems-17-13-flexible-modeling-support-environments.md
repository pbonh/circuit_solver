---
title: 'Modeling and Simulation of Systems — Chapter 13: Flexible Modeling Support
  Environments'
type: source
id: summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments
kind: publication
tags:
- simulation
- modeling
- devs
- soa
- applications
- fractionated-satellites
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt
---

## Key Points

- The chapter contrasts rigid waterfall M&S workflows with flexible Modeling Support Environments (MSE) that route diverse stakeholders through different sequences of tools/services adapted to their objectives.
- Two defects of waterfall depictions: (1) real-world processes iterate and depart from idealized sequences, (2) arrows show control flow but hide the information artifacts produced and consumed.
- Tools are classified as progress tools (advance the workflow) and verification-and-validation (V&V) tools (check work and enable regression to earlier steps).
- Service-Oriented Architecture (SOA) approach encapsulates M&S tools as Web services around a common data store with semantic-bus orchestration — enabling flexibility, discoverability, reusability, and learning over time.
- Case study: the DARPA System F6 Frontier Modeling Support Environment for fractionated satellite system design. Fractionated architectures replace monolithic satellites with networked modular components.
- Stakeholder categorization along two axes: Strategic/Tactical (planning horizon: 20 years vs. 1 year) × Supply/Demand (architecture providers vs. service consumers), giving four user types.
- MSE Web services: Pre-Simulation (PSS), Development and Pruning of Alternatives (DPAS), Simulation (Market Model Simulator for Strategic, Physical Model Simulator for Tactical), Results Analysis (RAS), Evaluation (ES).
- Master System Entity Structure (GeneralClusterArch) encodes the full space of cluster configurations with multi-aspects for SatelliteModule and specializations for Sensors, Communication, etc. DPAS generates pruning scripts; the family of PESs constitutes the solution space.
- Pruning operations: `select X from spec for ENTITY!`, `don't select Y under ENTITY!` (structural restriction), `set count bounds for X as [LO,HI]!` (cardinality constraints, optionally per-parent).
- ADEVS/SOA (open-source C++ ADEVS extended with Apache Axis2C and Staff for Web service hosting) is contrasted with DEVS/SOA (Java-based): C++ lacks dynamic instantiation and reflection, limiting flexibility despite execution-performance advantages.
- A Master-SES-to-Physical-Model-SES mapping operates at the pruned XML level — e.g., "for each SatelliteModule, if both an intersatellite communication link and a Sensor are present, add an imageSat to the Physical Model cluster".
- Yield analysis: many master PESs do not yield valid physical-model configurations; conditioning pruning rules (e.g., require intersatellite communication on every satellite) increase yield by constraining the mapping's domain.
- Future work: semantic-based orchestration (Semantic Bus), automated mapping definition, broader range of stakeholder objectives, Java-based proxies for external Web services.

## Relevant Concepts

- [[concepts/modeling-support-environment]] — flexible workflow framework.
- [[concepts/service-oriented-architecture]] — SOA wrapping of M&S tools.
- [[concepts/devs-soa]] — DEVS Web-service platform.
- [[concepts/fractionated-satellite-systems]] — application domain.
- [[concepts/master-ses]] — overarching SoS architecture-space SES.
- [[concepts/v-and-v-tools]] — verification and validation companions to progress tools.
- [[concepts/system-entity-structure]] — central modeling language.
- [[concepts/automated-pruning]] — DPAS uses pruning scripts.
- [[concepts/ses-to-ses-mapping]] — Master SES → Physical Model SES mapping.
- [[entities/darpa-f6-frontier]] — sponsoring program and prototype environment.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 13 — Flexible Modeling Support Environments
- File path: `raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
