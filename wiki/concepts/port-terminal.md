---
title: Ports and Terminals
type: claim
id: concepts/port-terminal
tags:
- foundational
- analog
- well-established
- graph
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A terminal is a point in a network where external components can be connected. When terminals are arranged in ordered pairs, they form ports. An n-terminal network has voltages measured relative to a reference node; an n-port network has voltages and currents associated with each pair.

## How It Works

The terminal description is more general (does not require an even number of pins). A four-terminal network can be re-expressed as a two-port by introducing port voltages V1, V2 and port currents I1, I2 with appropriate sign and reference conventions (currents into each port). Vlach and Singhal use lettered subscripts for terminals (j, k, j', k') and numeric subscripts for ports.

## Key Parameters

- Number of terminals n.
- Number of ports (must be ≤ n/2).
- Reference (ground) node for terminal voltages.
- Sign convention: terminal currents flow away from the terminal; port currents into the top terminal of each port.

## When To Use

- Building macromodels of multi-pin devices (transistors, op-amps).
- Defining two-port matrices for transducers and elementary two-ports.
- Connecting black-box subnetworks at well-defined interfaces.

## Risks & Pitfalls

- Port assumption requires that the current entering one terminal of a port equals the current leaving the other; this may be violated when the supposed two-port is part of a larger interconnected network.
- Sign conventions vary across textbooks; careful adherence is required when stamping into MNA.

## Related Concepts

- [[concepts/dependent-source]]
- [[concepts/operational-amplifier]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
