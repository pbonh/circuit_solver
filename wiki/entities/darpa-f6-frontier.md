---
title: DARPA F6 Frontier MSE
type: entity
id: entities/darpa-f6-frontier
tags:
- simulation
- modeling
- satellites
- darpa
- program
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt
---

## Overview

The DARPA System F6 program (Future, Fast, Flexible, Fractionated, Free-Flying Spacecraft United by Information Exchange) commissioned the Frontier Modeling Support Environment (MSE) — a service-oriented architecture for design and evaluation of fractionated satellite systems. The MSE includes Pre-Simulation, Development and Pruning of Alternatives, Simulation, Results Analysis, and Evaluation Services.

## Characteristics

- DARPA System F6 sponsorship (Technical Area 1: Design Tools for Adaptable Systems)
- Stakeholder taxonomy: Strategic/Tactical × Supply/Demand
- Two simulator types: Market Model (Strategic) and Physical Model (Tactical)
- Built on DEVS/SOA and ADEVS/SOA platforms
- Master SES encodes all cluster architectures
- Outputs ranked cluster instantiations vs. monolithic baseline

## Common Strategies

- Pruning master cluster architectures into ImageSat/RelaySat physical-model PESs
- Comparing fractionated vs. monolithic clusters under common demand profiles
- Iterative stakeholder-driven workflow with semantic-bus orchestration

## Related Entities

- [[entities/ms4-me]]
- [[entities/cosmos-devs-suite]]

## Sources

- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
