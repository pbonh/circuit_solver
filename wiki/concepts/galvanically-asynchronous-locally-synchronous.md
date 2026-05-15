---
title: "Globally Asynchronous, Locally Synchronous (GALS)"
type: concept
tags: [vlsi, digital, synchronization, well-established, architecture]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: low
---

## Definition

Globally Asynchronous, Locally Synchronous (GALS) is a clocking paradigm (introduced ca. 1984) in which an IC is partitioned into multiple synchronous islands, each driven by its own clock; communication between islands uses asynchronous handshaking protocols, avoiding the need for a single global clock.

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
