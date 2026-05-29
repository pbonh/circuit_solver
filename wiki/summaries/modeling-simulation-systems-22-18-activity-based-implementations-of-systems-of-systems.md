---
title: 'Modeling and Simulation of Systems — Chapter 18: Activity-Based Implementations
  of Systems of Systems'
type: source
id: source-modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems
kind: derived-summary
tags:
- simulation
- modeling
- devs
- activity
- energy
- hardware
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/22-18-activity-based-implementations-of-systems-of-systems.txt
---

## Key Points

- "Activity" is the bridge concept linking information and energy: information processing takes energy; acquiring energy requires information processing. A sustainable SoS balances energy expenditure with energy capture.
- Activity is defined as the spatial-temporal distribution of state transitions among components of a system. More active components consume more energy. Activity-based design pulls energy considerations to the front of the SoS lifecycle, before implementation.
- A prototypical IT-driven SoS comprises Sensor System, Decision System, and Action System, augmented with a Communication System that mediates all message flow.
- An Experimental Frame (Generator/Acceptor/Transducer) is coupled to the SoS to test behavioral (correctness, timing latency) and non-functional (energy budget) requirements.
- Timing requirements are formulated as latency constraints between observable events (e.g., image arrival → action production within 1 second).
- Model Continuity (Hu & Zeigler 2005): the SoS conceptual DEVS model is unchanged when re-targeted to a real-time DEVS simulator; the simulator counts state transitions directly to measure activity. A Platform component represents the underlying hardware and emits Activity and Energy signals.
- The chapter develops DEVS-based hardware synthesis to FPGAs. Each atomic model becomes a synchronous logic block; the design uses a Globally Asynchronous, Locally Synchronous (GALS) pattern where each component has its own clock domain.
- Clock gating switches off clocks to components currently in passive phases — exploiting DEVS's explicit phase structure to reduce dynamic power consumption.
- Frequency scaling per Domain Clock Module (DCM) assigns lower clock frequencies to less-active components, since dynamic power scales linearly with frequency.
- An adaptive-quantizer sensor package experiment (Pifer 2012) reports:
  - Clock gating alone: ~80% reduction in dynamic power.
  - Single-frequency + clock gating + handshaking: additional ~30% reduction.
  - Three-frequency-domain assignment + clock gating: another ~30% reduction.
  - Tighter latency constraints yield greater savings because intrinsic activity disparity increases.
  - Combined: two orders of magnitude reduction in dynamic power for tight latency.
- Quantizer.dnl appendix demonstrates the FDDEVS model: waitForRawData with conditional internal transition (only emit when |new − last| > Quantum) and waitForQuantum for adaptive update of the quantization step.
- The methodology applies broadly beyond FPGAs to ASIC and SoC low-power design.

## Relevant Concepts

- [[concepts/activity-based-modeling]] — energy/information balancing in DEVS.
- [[concepts/activity-tracking]] — measuring per-component state-transition density.
- [[concepts/gals-design-pattern]] — Globally Asynchronous Locally Synchronous hardware synthesis.
- [[concepts/clock-gating]] — disabling clocks for passive components.
- [[concepts/devs-hardware-synthesis]] — generating FPGA hardware from DEVS models.
- [[concepts/experimental-frame]] — frames implementation models for energy/timing tests.
- [[concepts/model-continuity]] — preserves model code from simulation to real-time.
- [[concepts/discrete-event-system-specification]] — formal base.
- [[concepts/atomic-devs-model]] — synthesized to synchronous logic per component.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 18 — Activity-Based Implementations of Systems of Systems
- File path: `raw/ModelingAndSimulationOfSystems/_txt/22-18-activity-based-implementations-of-systems-of-systems.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
