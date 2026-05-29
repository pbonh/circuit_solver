---
title: 'Modeling and Simulation of Systems — Chapter 16: Model Development and Execution
  Process with Repositories, Validation, and Verification'
type: source
id: summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification
kind: publication
tags:
- simulation
- modeling
- devs
- cosmos
- verification
- validation
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt
---

## Key Points

- CoSMo (Component-based System Modeler) and its extension CoSMoS introduce relational-database-backed persistence, unified logical/visual/persistent modeling, and partial code generation for the DEVS-Suite simulator.
- CoSMoS combines ER, SES, UML, and XML Schema concepts with DEVS to build hierarchical SoS model families with model management and complexity metrics.
- Three model abstraction layers used in CoSMoS:
  1. **Template Model** — at most two levels deep, captures composition and specialization relationships.
  2. **Instance Template Model** — any finite hierarchy, an instance of templates with concrete composition and specialization choices.
  3. **Instance Model** — fully instantiated executable model.
- Model types persist in relational databases for very-large-scale management; modifications are tracked across the families.
- Hybrid SW/HW co-design support in CoSMoS extends Chapter 15's SOC-DEVS into the persistence framework. Hardware system models include: Hardware System (HSM), Processor (PM), Network Interface (NIM), Link (LM), Router (RM), and corresponding Group models (PGM, NIGM, LGM, RGM, PNGM). H-Style configurations bundle related couplings.
- Software system models: Software System Model (SSM) composed of Software Application Models (SAMs). Composite SW/HW mapping connects SSM to HSM with constraints (mapping direction software→hardware, etc.).
- Model constraints enforce composition, coupling, and mapping rules (e.g., HSM can contain only group models; SSM must contain at least one SAM).
- Validation vs. Verification:
  - **Validation** answers "does the model behave as intended?" — driven by experimental frames (generators, transducers, acceptors); supports families of validation experiments.
  - **Verification** answers "is the structure/behavior correct?" — uses model-checking on a Constrained DEVS variant that bounds state, input, and transition counts to finite, computable sets.
- **Constrained DEVS** restrictions: state variables typed and bounded (e.g., regex-defined queue (String)^8); input ports include a NULL value to admit internal-only transitions; time-advance function constrained to finite rational values; external inputs constrained for reachability analysis.
- A verification algorithm explores reachable safe states W, tracks visited V and unsafe U sets, and reports the first state-input pair that enters an unsafe state.
- Data exclusion (e.g., Network-on-Chip flits that are routed but not processed) reduces the state-space size and makes verification tractable.
- Dynamic-property modeling (latency, throughput, NoC flit delay) is verified by embedding the experimental frame (generator, transducer, acceptor) inside the model under verification, so experimentation and measurement become parts of the model itself.
- The CoSMoS lifecycle from Template → Instance Template → Instance Model + V&V supports collaborative, incremental SoS modeling for software-only, hardware-only, and SW/HW co-design scenarios.

## Relevant Concepts

- [[concepts/cosmos-framework]] — unified logical/visual/persistent modeling.
- [[concepts/template-instance-template-instance]] — three-tier model abstraction layers.
- [[concepts/constrained-devs]] — verifiable DEVS subset for model checking.
- [[concepts/model-validation]] — behavior conformance under experimental frames.
- [[concepts/model-verification]] — structural correctness via reachability analysis.
- [[concepts/network-on-chip-model]] — verification example (data exclusion).
- [[concepts/software-hardware-co-design]] — CoSMoS extends co-design with persistence.
- [[concepts/system-entity-structure]] — companion modeling formalism.
- [[concepts/parallel-devs]] — execution target.
- [[entities/cosmos-devs-suite]] — environment hosting CoSMoS.
- [[entities/devs-suite]] — verification-engine host.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 16 — Model Development and Execution Process with Repositories, Validation, and Verification
- File path: `raw/ModelingAndSimulationOfSystems/_txt/20-16-model-development-and-execution-process-with-repositories-validation-and-verification.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
