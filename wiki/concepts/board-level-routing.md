---
title: Board-Level Routing
type: claim
id: concepts/board-level-routing
tags:
- vlsi
- power-integrity
- routing
- well-established
- board
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Per GraphsInVLSI Sect. 10.1: "A typical board-level layout consists of several metal layers, each separated by a dielectric layer. The connections between the layers are provided by vias. SPROUT uses layer information, design rules, and placement data to produce an initial layout. The objective of the algorithm is to generate a shape connecting the power management IC with the target ball grid array (BGA) balls and decoupling capacitors while complying with the design rules and minimizing the impedance."

## How It Works

Signal routing on PCBs is well-studied with mature commercial tools. Power-net routing is less automated; engineers manually shape large power polygons across multiple layers to deliver current from PMICs to BGA balls while honoring impedance, current-density, and EMI constraints. SPROUT introduces automation for the power-net case.

## Key Parameters

- Number of metal layers.
- BGA pitch and ball count.
- Component placement and blockages.
- Target impedance per net.

## When To Use

- PCB design from preliminary specification through layout sign-off.
- Power-integrity-driven optimization (e.g., SPROUT).

## Risks & Pitfalls

- Power-integrity issues often surface only after impedance extraction post-layout.
- Manual iteration is time- and labor-intensive.

## Related Concepts

- [[entities/sprout]]
- [[concepts/interconnect-routing]]
- [[concepts/multilayer-routing]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
- [[summaries/graphs-in-vlsi-18-c-multilayer-routing-algorithm]]
