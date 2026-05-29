---
title: Globally Asynchronous, Locally Synchronous (GALS)
type: claim
id: concepts/galvanically-asynchronous-locally-synchronous
tags:
- vlsi
- digital
- synchronization
- well-established
- architecture
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

GraphsInVLSI Chapter 4 (Synchronization): "A less stringent, globally asynchronous, locally synchronous (GALS) clocking paradigm was introduced in 1984 [318]. By splitting an integrated circuit into separate clock regions, the delay from a clock source to each register is reduced, typically producing less clock skew. The transfer of data among the separate clock domains is established by an asynchronous communication protocol [319]." Later the chapter calls GALS "an effective method for controlling the size of a circuit ... By decomposing the circuits into separate clock domains, the permissible [clock-skew] range can be efficiently determined within each partition" [ref 339].

## How It Works

Each island has a separate clock distribution network and clock-skew schedule. Inter-island data transfers cross clock-domain boundaries through asynchronous FIFOs, mutual-exclusion (mutex) circuits, or handshake protocols. Local clock distribution is simpler because each domain is smaller, but cross-domain synchronizers introduce latency and metastability risk.

## Key Parameters

- Number of clock domains.
- Inter-domain communication bandwidth and latency.
- Metastability mean time between failures.

## When To Use

- Very large SoCs where a single global clock is impractical.
- Heterogeneous designs with disparate clock requirements.
- Power-managed designs with dynamic voltage and frequency scaling.

## Risks & Pitfalls

- Metastability of inter-domain synchronizers.
- Verification complexity (multi-clock-domain CDC analysis).
- Cross-domain latency degrades worst-case throughput.

## Related Concepts

- [[concepts/clock-distribution-network]]
- [[concepts/clock-skew-scheduling]]
- [[concepts/networks-on-chip]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
