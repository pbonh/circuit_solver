---
title: "Chaos"
type: entity
tags: [graph, distributed-systems, graph-processing, out-of-core, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: medium
---

> Source citations: Systems For Big Graph Analytics Chapter 2 names Chaos repeatedly. "Chaos [18] extends X-Stream to work in a distributed environment, but it is only efficient when network bandwidth far outstrips storage bandwidth (which is also an assumption in its system design)." Later (Sect. on Scaling-Out): "In Chaos, a master keeps track of the vertices and edges of every partition, and the generated updates towards every partition; while a computing thread sends requests to the master for the necessary data for processing a partition. ... Roy et al. [11] reported that Chaos only achieves good performance by using large-SSD machines connected by 40 Gigabit Ethernet, and the performance is undesirable when Gigabit Ethernet is used." "Chaos also supports work stealing for load balancing." Reference: A. Roy, L. Bindschaedler, J. Malicevic, and W. Zwaenepoel. Chaos: scale-out graph processing from secondary storage. In SOSP, pages 410–424, 2015.

## Overview

Chaos (Roy et al., SOSP 2015) extends X-Stream to a distributed environment by distributing edge partitions across machines. A master tracks partition data and updates, and worker threads pull required data from the master. Effective only when network bandwidth far exceeds storage bandwidth.

## Characteristics

- Inherits X-Stream's edge-centric streaming model.
- Master coordinates partition assignment and update routing.
- Supports work stealing: idle workers request unfinished partitions from busy peers.
- Requires large-SSD machines connected by 40 Gigabit Ethernet for good performance; reportedly underperforms on Gigabit Ethernet.

## Common Strategies

- Use when both network (40GbE+) and per-node SSD bandwidth are very high.
- Avoid on commodity Gigabit clusters; prefer GraphD instead.
- Enable work stealing in long-tail stragglers, accepting the master-side coordination cost.

## Related Entities

- [[entities/x-stream]]
- [[entities/graphd]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
