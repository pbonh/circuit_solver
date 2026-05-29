---
title: 'Systems for Big Graph Analytics — Part I: Think Like a Vertex (Pregel-Like,
  Hands-On, Shared Memory)'
type: source
id: summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex
kind: publication
tags:
- graph
- distributed-systems
- big-data
- graph-processing
- pregel
- vertex-centric
- parallel
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Key Points

- Pregel models iterative graph computation as a sequence of supersteps (BSP); a user-defined `compute(msgs)` runs per vertex, sending messages, mutating local state, and voting to halt.
- Pregel introduced vertex-centric programming: write the computation logic of a single vertex like a state machine; the system runs many in parallel (SIMD-like).
- Optimizations sit on the basic Pregel model along several axes: message combiners and aggregators, communication mechanism, load balancing, out-of-core execution, fault recovery, and on-demand querying.
- Scalable Pregel algorithms are characterized as BPPA (balanced practical Pregel algorithm) or PPA — linear per-vertex space, computation, and communication, with O(log |V|) supersteps; pointer-jumping (path-doubling) techniques like list ranking and the Shiloach-Vishkin (S-V) algorithm achieve this.
- Communication-reduction techniques: vertex mirroring (Pregel+), Facebook Giraph improvements (multithreading, superstep splitting), message online computing (MOCgraph), request-respond API for pointer-jumping algorithms.
- Load balancing approaches: vertex migration (Mizan, WindCatch, GPS) — found to be mostly ineffective in practice — and dynamic concurrency control (PAGE) for intra-machine workload distribution.
- Out-of-core distributed systems exist (Pregelix, GraphD, Chaos) that hide disk-streaming inside message-transmission time; GraphD targets commodity Gigabit-Ethernet clusters.
- Fault recovery beyond plain checkpointing: lightweight checkpointing (skip messages, incremental edges), message-logging-based fast recovery, optimistic recovery for self-correcting algorithms, and replication (Imitator).
- Quegel introduces superstep-sharing for on-demand graph queries — many lightweight queries proceed one superstep each per super-round, sharing communication bandwidth.
- BigGraph@CUHK (CUHK) is a C++ toolkit (Pregel+, Blogel, Quegel, GraphD, LWCP) that exposes MPI- and HDFS-level concerns to users; its design avoids JVM GC overhead and provides a learning vehicle for Big Data system internals.
- Pregel+ system design: a `Vertex` base class (templated on KeyT, ValueT, MessageT, HashT), a `Worker` base class with `toVertex`/`toline` UDFs, plus optional Combiner and Aggregator; data (de)serialization via overloaded operators `<<` and `>>` on `ibinstream`/`obinstream`; MPI for transport; libhdfs for storage.
- Chapter 4 covers shared-memory abstractions (GraphLab, PowerGraph, Maiter) and single-PC disk-based systems (GraphChi, X-Stream, VENUS, GridGraph). GraphLab adopts full vertex scope and asynchronous execution with ghost replication; PowerGraph switches to the GAS (Gather-Apply-Scatter) model with edge partitioning and a vertex-cut objective.
- Maiter proposes delta-based accumulative iterative computation (DAIC): updates are computed on value changes, which permits prioritized execution while preserving exactness for ⊕-distributive update functions.
- Single-PC disk-based systems split a graph into shards (GraphChi), edge-centric scatter/gather streams (X-Stream), structure-vs-value separation with VSP (VENUS), or a 2-D grid of edge blocks (GridGraph); each represents a different way to stream a disk-resident big graph.

## Relevant Concepts

- [[concepts/vertex-centric-programming]] — the SIMD-like programming abstraction that Pregel pioneered.
- [[concepts/bulk-synchronous-parallel]] — the BSP execution model underpinning Pregel-like systems.
- [[concepts/message-combiner]] — combine messages targeted at the same destination vertex to cut network volume.
- [[concepts/aggregator]] — global reduction across vertices made available to all vertices in the next superstep.
- [[concepts/pointer-jumping]] — path-doubling technique used in BPPAs to reach O(log |V|) supersteps (list ranking, S-V).
- [[concepts/shared-memory-graph-abstraction]] — vertex-scope and edge-scope models where neighbors are read directly (GraphLab, PowerGraph, GraphChi, X-Stream, VENUS, GridGraph).
- [[concepts/gas-model]] — Gather-Apply-Scatter programming model used by PowerGraph and single-PC systems.
- [[concepts/vertex-cut-partitioning]] — PowerGraph's edge-partitioning that bounds vertex replicas.
- [[concepts/delta-accumulative-iterative-computation]] — Maiter's exact asynchronous prioritized model.
- [[concepts/out-of-core-graph-processing]] — distributed and single-PC techniques to spill graph state to disk.
- [[concepts/lightweight-checkpointing]] — Pregel checkpoint that omits messages and reuses static adjacency lists.
- [[concepts/message-logging-recovery]] — fast recovery via per-vertex local message/state logs.
- [[concepts/superstep-sharing]] — Quegel's mechanism for batching many concurrent graph queries.
- [[entities/pregel]] — Google's reference vertex-centric system.
- [[entities/apache-giraph]] — popular open-source Pregel-like system in Java.
- [[entities/graphx]] — Spark-based vertex-centric framework with wide-dependency tradeoffs.
- [[entities/graphlab]] — shared-memory abstraction system; commercialized as Turi.
- [[entities/powergraph]] — GAS-model successor to distributed GraphLab.
- [[entities/maiter]] — DAIC-based asynchronous system on MapReduce.
- [[entities/graphchi]] — single-PC shard-based out-of-core system with the GAS model.
- [[entities/x-stream]] — single-PC edge-centric streaming system.
- [[entities/venus]] — vertex-scope VSP system with structure/value separation.
- [[entities/gridgraph]] — single-PC 2-D grid partitioning with streaming-apply.
- [[entities/pregel-plus]] — BigGraph@CUHK's C++ Pregel-like system with vertex-mirroring and request-respond.
- [[entities/graphd]] — out-of-core distributed Pregel-like system in BigGraph@CUHK.
- [[entities/quegel]] — query-centric Pregel framework using superstep-sharing.
- [[entities/biggraph-cuhk]] — the C++ Pregel-family toolkit from CUHK.
- [[entities/mpi]] — Message Passing Interface used by BigGraph@CUHK for transport.
- [[entities/hdfs]] — Hadoop Distributed File System used as durable storage layer.
- [[entities/chaos]] — distributed X-Stream variant for SSD/40GbE clusters.

## Source Metadata

- Source type: book chapters
- Book title: Systems for Big Graph Analytics
- Chapters: 2 (Pregel-Like Systems), 3 (Hands-On Experiences with BigGraph@CUHK / Pregel+), 4 (Shared Memory Abstraction)
- File: raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
- Authors: Da Yan, Yingyi Bu, Yuanyuan Tian, Amol Deshpande (2017, Springer)
