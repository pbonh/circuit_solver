---
title: "Networks-on-Chip (NoC)"
type: concept
tags: [vlsi, graph, digital, well-established, architecture]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/04-1-introduction.txt"]
confidence: low
---

## Definition

A Network-on-Chip (NoC) is an on-chip communication subsystem that replaces ad-hoc point-to-point wiring or shared buses with a packet-switched (or circuit-switched) interconnect topology connecting cores, memories, and accelerators within a system-on-chip (SoC).

## How It Works

Routers and links are arranged in regular topologies (mesh, torus, ring, fat tree, hierarchical) modeled as graphs whose nodes are routers and edges are physical links. Packets traverse the graph under a routing algorithm (XY, deadlock-free adaptive routing). Graph-based optimization is used for topology selection, traffic mapping (assigning cores to routers), and physical layout.

## Key Parameters

- Topology (mesh, torus, tree).
- Number of nodes and link bandwidth.
- Routing algorithm (deterministic vs. adaptive).
- Buffer sizes and virtual channels.

## When To Use

- Many-core SoCs requiring scalable, high-bandwidth on-chip communication.
- Chiplet and 3D-integrated systems with heterogeneous components.

## Risks & Pitfalls

- Routing-induced deadlock without proper protocol design.
- Area and power overhead of routers.
- Topology-mapping NP-hardness requires heuristic solutions.

## Related Concepts

- [[concepts/graph-theory]]
- [[concepts/vlsi-design]]
- [[concepts/graph-partitioning]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
