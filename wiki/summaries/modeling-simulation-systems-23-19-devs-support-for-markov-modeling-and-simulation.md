---
title: 'Modeling and Simulation of Systems — Chapter 19: DEVS Support for Markov Modeling
  and Simulation'
type: source
id: source-modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation
kind: derived-summary
tags:
- simulation
- modeling
- devs
- markov
- stochastic
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt
---

## Key Points

- MS4 Me's Markov Modeling facility produces full-fledged DEVS models that integrate with other DEVS models. Markov stochastic modeling is implicitly at the heart of most discrete-event simulation.
- Three Markov model classes are supported via Finite Probability DEVS (FP-DEVS):
  - **Continuous Time Markov (CTM)** — atomic stochastic DEVS sampling exponential inter-event times.
  - **Discrete-Time Markov (DTM)** — time-sliced approximation with events at fixed steps.
  - **Markov Matrix (MM)** — deterministic computation of probabilities (frequencies of state occupation).
- The same state diagram with labeled arrows is reinterpreted by class: MM treats labels as transition probabilities; CTM treats them as transition rates (and 1/rate as average time to transition).
- Stock Market example: Bear/Bull/Stagnant states with six nonzero transitions, columns of the 3×3 transition matrix sum to 1 (self-loops are computed automatically). Steady-state probabilities: Bear=0.31, Bull=0.63, Stagnant=0.06.
- CTM transition selection algorithm: for each phase, sample `σ' = (1/p') × (−ln r)` for each outgoing transition (r uniform [0,1]); pick the phase with the minimum σ' and advance by that time. Implements Markov assumption that transitions are independent Poisson events with rate p'.
- DTM is a coarser approximation that updates probability vectors at fixed time steps.
- MM provides:
  - Steady-state probabilities for ergodic CTMs.
  - Absorption probabilities for CTMs that reach absorbing states.
  - State-to-state traversal times.
  - Compositional approximations to coupled CTMs.
- For a state with multiple outgoing transitions of rates r1, r2, ..., the total exit rate is the sum; the average residence time is 1/Σri. Transition selection frequencies are proportional to the rates.
- A node can encode both residence time and probability by setting rates = probabilities / residence_time (rescale if rates sum > 1).
- DEVS simulator provides a Monte Carlo layer that generates stochastic sample paths; experimental frames provide queuing-style performance metrics (queue sizes, waiting times, throughput, losses).
- Both transient and steady-state behavior can be observed.
- Applications: Stock market dynamics, healthcare cost-effectiveness analysis (Soares & Castro 2012), compartmental epidemiological models (Özmen et al. 2016), agent-directed systems, Internet of Things modeling.

## Relevant Concepts

- [[concepts/continuous-time-markov]] — stochastic DEVS sampling exponential inter-event times.
- [[concepts/discrete-time-markov]] — time-step approximation.
- [[concepts/markov-matrix-model]] — deterministic equilibrium computation.
- [[concepts/finite-probability-devs]] — the FP-DEVS base capability.
- [[concepts/exponential-distribution-sampling]] — core sampling mechanism for CTM transitions.
- [[concepts/monte-carlo-analysis]] — DEVS simulator layer.
- [[concepts/discrete-event-system-specification]] — formal base.
- [[concepts/atomic-devs-model]] — CTM as an atomic stochastic DEVS.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 19 — DEVS Support for Markov Modeling and Simulation
- File path: `raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
