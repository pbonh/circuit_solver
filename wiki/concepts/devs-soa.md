---
title: "DEVS/SOA"
type: concept
tags: [simulation, modeling, devs, soa, distributed, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/17-13-flexible-modeling-support-environments.txt"]
confidence: medium
---

## Definition

DEVS/SOA is a service-oriented architecture environment in which DEVS coupled-model simulations execute on Web-services platforms. Both Java-based (DEVSJava-derived DEVS/SOA) and C++-based (ADEVS/SOA built on Apache Axis2C and Staff) variants exist.

## How It Works

DEVS simulators and coordinators are exposed as Web services on a Tomcat-style platform. Coupled-model pruned entity structures (PESs) are uploaded as XML files; the MainService coordinates Simulator Services, exchanging DEVS control messages (GetTN, GetOutput, etc.) and XML-encoded DEVS payload messages. Aggregated simulation logs are sent back to the client.

## Key Parameters

- Java vs. C++ runtime (DEVS/SOA vs. ADEVS/SOA)
- PES XML payload
- Web-service container (Tomcat)
- Per-service tool interface

## When To Use

- Distributed DEVS simulation across enterprise/cloud platforms
- Federations of DEVS engines from heterogeneous sources
- DARPA F6 / Frontier-style flexible MSE deployments

## Risks & Pitfalls

- C++ variant lacks dynamic instantiation and reflection
- Per-server simulator provisioning is manual for ADEVS/SOA
- XML payload overhead

## Related Concepts

- [[concepts/service-oriented-architecture]]
- [[concepts/devs-simulation-protocol]]
- [[concepts/devs-dds-architecture]]

## Sources

- [[summaries/modeling-simulation-systems-17-13-flexible-modeling-support-environments]]
