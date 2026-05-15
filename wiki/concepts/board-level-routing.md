---
title: "Board-Level Routing"
type: concept
tags: [vlsi, power-integrity, routing, well-established, board]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: low
---

## Definition

Board-level routing synthesizes interconnections between components on a printed circuit board (PCB), including signal nets and power/ground nets. Multiple metal layers separated by dielectric layers are connected by vias; design rules govern wire widths, spacings, and via constraints.

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
