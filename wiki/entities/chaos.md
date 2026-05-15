---
title: "Chaos"
type: entity
tags: [graph, distributed-systems, graph-processing, out-of-core, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: low
---

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
