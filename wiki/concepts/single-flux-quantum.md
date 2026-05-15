---
title: "Single Flux Quantum (SFQ)"
type: concept
tags: [superconductive, digital, emerging, vlsi, clock]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/01-acknowledgments.txt"]
confidence: low
---

## Definition

Single Flux Quantum (SFQ) is a family of superconductive digital logic technologies in which information is encoded as discrete magnetic flux quanta (typically Φ₀ = h/2e) propagated as picosecond voltage pulses between Josephson junctions.

## How It Works

SFQ circuits use superconductive loops containing Josephson junctions. A bit "1" corresponds to a quantized flux pulse traversing a junction; arrival/absence of such pulses within a clocked window represents data. SFQ logic operates at cryogenic temperatures (typically 4 K), enabling very high clock rates (tens to hundreds of GHz) with minimal switching energy. However, distribution of clock pulses to gates demands tight skew control and aware synthesis tools (e.g., QuCTS in this book).

## Key Parameters

- Critical current of Josephson junctions.
- Inductance of superconductive loops.
- Operating temperature (cryogenic).
- Clock pulse rise time and timing margins.

## When To Use

- High-frequency, low-energy digital systems where cryogenic cooling is acceptable (quantum computing control, signal processing for scientific instruments, future exascale computing).

## Risks & Pitfalls

- Requires cryogenic infrastructure.
- Limited fabrication ecosystem compared with CMOS.
- Pulse-based timing makes clock distribution and skew control much harder than in CMOS.

## Related Concepts

- [[entities/qucts]]
- [[concepts/clock-distribution-network]]
- [[concepts/clock-skew-scheduling]]

## Sources

- [[summaries/graphs-in-vlsi-01-acknowledgments]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
