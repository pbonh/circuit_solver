---
title: "Modeling and Simulation of Systems of Systems"
type: source
slug: modeling-simulation-systems
created: 2026-06-16
updated: 2026-06-16
summary: Comprehensive guide to DEVS (Discrete Event System Specification) and SES (System Entity Structure) for hierarchical modeling and simulation of complex systems of systems.
source_file: Books/ModelingAndSimulationOfSystems
tags: [devs, simulation, systems-of-systems, modeling, discrete-event, distributed-simulation]
status: active
---

# Modeling and Simulation of Systems of Systems

- **Source file:** `sources/Books/ModelingAndSimulationOfSystems/`
- **Author / origin:** [Bernard Zeigler et al.; Springer]
- **Date:** ~2019

## Summary

A monograph on DEVS (Discrete Event System Specification) — a formal, hierarchical, modular framework for constructing and simulating models of complex systems. Built around the MS4 Me DEVS IDE and applications to systems of systems (SoS).

### Core Framework

**DEVS (Discrete Event System Specification)**: Atomic models define state, time advance function (how long in a state), internal/external transitions, and output function. Coupled models compose atomic models by defining coupling (output → input port mappings). DEVS is closed under coupling — a coupled model is itself a DEVS model.

**FDDEVS (Finite Deterministic DEVS)**: Simplified DEVS variant for the MS4 Me IDE; natural language specification (constrained English sentences describing transitions); elaborated to Java.

**System Entity Structure (SES)**: A hierarchical ontology tree for specifying system configurations. Nodes: entities, aspects (decompositions), specializations (variants). Pruning = selecting specific configuration from SES tree → generates specific coupled DEVS model. Supports automated and rule-based pruning for experiment design.

**DEVS Simulation Protocol**: Abstract simulator pattern — coordinator (coupled) and simulator (atomic) objects exchange timed messages (next event time, output, transition). Standard protocol enables simulation interoperability across different DEVS tools.

### Advanced Topics

**Dynamic structure**: Dynamic agent modeling — models can add/remove ports and couplings at runtime; publish/subscribe data distribution for agent communication.

**Distributed simulation**: Peer message exchange protocol; real-time DEVS; integration with DDS (Data Distribution Service); DEVS as simulation interoperability standard.

**SOA-DEVS**: Model service-oriented architectures (SOA) using DEVS — primitive/composite service models; broker-executive dynamic structure model; cloud system simulation.

**Model repositories and verification**: CoSMoS process lifecycle; hybrid software/hardware modeling; guided model validation; constrained DEVS model verification.

**Markov modeling support**: Stochastic DEVS extensions for Markov chain simulation.

**Living system modeling**: Application to animal epidemiology (surveillance/control of cattle disease) and plant growth (Ecomeristem model) using heterogeneous formalisms.

### Connection to Circuit Simulation

- DEVS is a candidate behavioral simulation framework for circuit systems — each behavioral block (opamp, PLL, ADC) can be an atomic DEVS model; the system is a coupled DEVS model
- Compare to [[verilog-ams]]: Verilog-AMS covers continuous-time analog; DEVS covers discrete-event behavior; DEVS could model digital controller behavior in a mixed-signal system
- DEVS simulation protocol enables distributed simulation — parallel blocks can run on separate processors
- SES pruning enables automatic generation of simulation configurations for corner analysis

## Key takeaways

- DEVS is closed under composition — hierarchical modular simulation without loss of generality
- SES provides a formal combinatorial model exploration structure (pruning = experiment design)
- The DEVS simulation protocol is a standard enabling interoperability between simulation frameworks
- Dynamic structure DEVS can model adaptive/reconfigurable systems (agent-based)
- SOA-DEVS bridges simulation and cloud service architecture modeling

## Pages updated from this source

- [[devs-simulation]] - concept created
- [[circuit-simulation]] - DEVS behavioral simulation complement noted
