---
title: 'Modeling and Simulation of Systems — Chapter 17: Modeling and Simulation of
  Living Systems as Systems of Systems'
type: source
id: summaries/modeling-simulation-systems-21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems
kind: publication
tags:
- simulation
- modeling
- devs
- living-systems
- agriculture
- epidemiology
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt
---

## Key Points

- Contributed by Raphaël Duboz and Jean-Christophe Soulié, this chapter applies DEVS and the Virtual Laboratory Environment (VLE) to modeling and simulation of living systems (biological, ecological, sociological).
- Four practical/theoretical limitations of living-system modeling: incomplete knowledge of components, ongoing evolution and structural change, interdependent organization scales (cells/organisms/populations/societies), and chaotic sensitivity to initial conditions.
- DEVS suits living systems because of:
  - Modular and hierarchical composition (matches the biology "Russian dolls" view of nested organization).
  - Multi-formalism integration (continuous ODE, cellular automata, statecharts, Petri nets, agent-based models can all be wrapped in DEVS).
  - Dynamic Structure DEVS (DSDEVS) supports the growth/birth/death/mutation of components characteristic of living systems.
- Agent-Based Simulation (ABS, also IBM) is widely used in life sciences; DEVS formalization of ABS (Duboz et al. 2006) addresses ABS's lack of common formalization while preserving emergence.
- Scale transfer modeling: compute emerging properties from a lower scale that become parameters at the upper scale, and compute environmental constraints (e.g., population size, temperature) from upper to lower — formalizable in DEVS.
- The RECORD project at INRA (Bergez et al. 2012) is the success case for DEVS-based agro-ecosystem modeling, leveraging coupling and heterogeneous formalism integration.
- Case study 1 — animal epidemiology: surveillance and control of livestock epidemic outbreaks, with virtual build and test of response policies impossible to test in reality.
- Case study 2 — plant growth: dynamic structure DEVS captures the changing morphology of plants as the growth process proceeds. EcoMeristem model of rice morphogenesis and sink competition is integrated with 3D plant and energy-balance tools in OpenAlea via VLE.
- Model continuity: research models can carry through to real-time decision-support systems by leveraging the DEVS Protocol's separation of model and simulation engine.
- Aumann (2007) methodology defines a focal level for modeling, with models and frames at the immediate scales above and below relating to the focal level.
- Intelligence amplification (IA) framing: virtual experiments increase the information-processing capability of the human decision maker rather than fully automating decisions.

## Relevant Concepts

- [[concepts/living-systems-modeling]] — DEVS-based modeling of biological/ecological/sociological systems.
- [[concepts/agent-based-simulation]] — formalized using DEVS to address ABS's lack of standardization.
- [[concepts/scale-transfer-modeling]] — bidirectional cross-scale parameter and constraint exchange.
- [[concepts/dynamic-structure-devs]] — supports growth, birth/death, mutation.
- [[concepts/emergence]] — observable in simulation outputs across scales.
- [[concepts/multi-formalism-modeling]] — heterogeneous formalisms in one DEVS model.
- [[concepts/discrete-event-system-specification]] — formal foundation.
- [[concepts/model-continuity]] — research-model carryover to real-time decision support.
- [[entities/vle]] — Virtual Laboratory Environment used throughout.
- [[entities/inra]] — RECORD project sponsor.
- [[entities/openalea]] — 3D plant and energy-balance tooling integrated with VLE.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 17 — Modeling and Simulation of Living Systems as Systems of Systems
- File path: `raw/ModelingAndSimulationOfSystems/_txt/21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian; chapter contributed by Raphaël Duboz, Jean-Christophe Soulié (Springer, 2nd ed. 2017)
