---
title: "Rapid Single Flux Quantum (RSFQ)"
type: concept
tags: [superconductive, digital, emerging, vlsi]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/14-11-qucts-single-flux-quantum-clock-tree-synthesis.txt"]
confidence: medium
---

## Definition

Per GraphsInVLSI Chapter 11: "Rapid single flux quantum (RSFQ) technology offers a range of advantages as compared to CMOS. Several orders of magnitude greater operating frequency and three orders of magnitude lower power are among the most prominent advantages of RSFQ." Foundational reference: Likharev and Semenov [587]. "Unlike traditional CMOS, where the information is represented with a high or low DC voltage level, short quantized voltage pulses are utilized in RSFQ. A logical high or low is represented by, respectively, the presence or absence of a single flux quantum (SFQ) pulse within a certain time interval."

## How It Works

GraphsInVLSI Chapter 11 captures the topology: "Most logic gates in RSFQ are therefore sequential, such as AND and OR gates that are combinatorial in CMOS. This structure drastically increases the pipeline depth as compared to CMOS, complicating the clock network design process." Interconnect choices: "RSFQ interconnect is either a passive transmission line (PTL) requiring a driver, receiver, and impedance matching [589, 590], or an active Josephson transmission line (JTL) requiring bias current for each Josephson junction. Finally, most RSFQ gates have a fanout of one. A splitter gate is used to generate two (or more) SFQ pulses from an input signal [588, 591]." The book cites "manufacturing technology... over 6,000 Josephson junctions (JJ) per mm² [582]" and "an 8-bit superconductive microprocessor operating at a frequency of 80 gigahertz" [583].

## Key Parameters

- Critical currents and inductances of Josephson loops.
- Operating temperature (~ 4 K).
- Pulse propagation speed on PTLs.
- Gate fanout.

## When To Use

- Ultra-high-frequency digital systems where cryogenic cooling is acceptable.
- Quantum computing control electronics.
- High-performance scientific instrumentation.

## Risks & Pitfalls

- Pulse-based logic mandates precise clock distribution.
- Reduced integration density vs CMOS.
- Limited fabrication ecosystem.

## Related Concepts

- [[concepts/single-flux-quantum]]
- [[concepts/josephson-junction]]
- [[concepts/passive-transmission-line]]
- [[concepts/josephson-transmission-line]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
