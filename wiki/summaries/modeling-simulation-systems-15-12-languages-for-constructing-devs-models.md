---
title: 'Modeling and Simulation of Systems — Chapter 12: Languages for Constructing
  DEVS Models'
type: source
id: source-modeling-simulation-systems-15-12-languages-for-constructing-devs-models
kind: derived-summary
tags:
- simulation
- modeling
- devs
- dsl
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt
---

## Key Points

- Three levels of DEVS model authoring support: constrained natural languages (top), DEVS Specification Languages (middle, e.g., FDDEVS), and DEVS Simulators in general-purpose languages like Java/C++ (bottom).
- DEVS Simulators (MS4 Me Java, ADEVS C++) mirror DEVS math constructs and provide full expressive power, but mix DEVS constructs with host-language syntax and require user mastery of both.
- DEVS Specification Languages provide DEVS-specific constructs that can be syntactically/semantically validated and analyzed (e.g., for liveness, safety) before transformation into a simulatable model. FDDEVS restricts to finite, deterministic structures, enabling analysis tools like XSY.
- Constrained natural languages (a small set of English sentence forms) hide computational complexity, support content-assisted authoring, then map down to FDDEVS or directly to a DEVS Simulator.
- MS4 Me is built on Xtext/EBNF in Eclipse, supporting both constrained NL for FDDEVS and SES authoring.
- FDDEVS formal definition (7-tuple): incomingMessageSet, outgoingMessageSet, StateSet, TimeAdvanceTable, InternalTransitionTable, ExternalTransitionTable, OutputTable. The mapping into a DEVS atomic model is detailed in a table; the confluent function is not specified by FDDEVS — modelers must supply it.
- FDDEVS limitations addressed by the elaboration facility: ports limited to strings (extend with typed messages), single phase state (extend with instance variables), no use of elapsed time (extend with external-event tagged blocks), no transition/output side effects (extend with tagged code).
- The SES→PES→Java path: the pruner produces a `*.pes` file that, on transformation, walks the SES and instantiates atomic model components from the model repository by name. Underscore names trigger the inheritance rules from Chapter 7.
- Two transformation outcomes from an SES+PES: (a) a hierarchical coupled model with real atomic components from the repository, or (b) an animation with auto-generated components that emit all possible outputs on any input.
- UML can serve both as a source framework (UML statecharts mapping into DEVS atomic models via tools like eUDEVS) and as a target framework (DEVS models exported to UML class and statechart diagrams).
- EMF-DEVS uses the Eclipse Modeling Framework to define meta-level Parallel DEVS atomic/coupled models with structural constraints for domain-specific languages.

## Relevant Concepts

- [[concepts/finite-deterministic-devs]] — DEVS specification language layer.
- [[concepts/constrained-natural-language]] — top authoring layer.
- [[concepts/discrete-event-system-specification]] — formalism at the heart of the stack.
- [[concepts/atomic-devs-model]] — Java target.
- [[concepts/coupled-devs-model]] — generated from SES+PES.
- [[concepts/dnl-elaboration]] — facility to overcome FDDEVS limitations.
- [[concepts/system-entity-structure]] — coupled-model specification language.
- [[concepts/pruned-entity-structure]] — script-driven concrete instance.
- [[concepts/devs-uml-mapping]] — interop with Unified Modeling Language.
- [[concepts/emf-devs]] — Eclipse-Modeling-Framework-based meta-DEVS.
- [[entities/eclipse-xtext]] — language workbench underpinning MS4 Me.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 12 — Languages for Constructing DEVS Models
- File path: `raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
