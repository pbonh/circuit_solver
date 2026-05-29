---
title: Distributed Filesystem
type: claim
id: concepts/distributed-filesystem
tags:
- distributed-systems
- well-established
- storage
- batch
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A distributed filesystem stores files across multiple machines in a shared-nothing cluster, exposing one logical filesystem on top of commodity hardware. Examples include Hadoop's HDFS (an open-source GFS reimplementation), GlusterFS, and QFS. Object stores like Amazon S3, Azure Blob Storage, and OpenStack Swift solve a similar problem with different semantics.

## How It Works

- A daemon per node exposes its local disks; a central NameNode (HDFS) or metadata service tracks which blocks live where.
- Files are split into fixed-size blocks (e.g., 128 MB); each block is replicated (typically 3x) or erasure-coded across machines for durability.
- Reads can be served from any replica; writes flow through a pipeline of replicas.
- Compute scheduling (MapReduce, Spark) places tasks on machines that already hold the input block — "putting computation near the data."
- HDFS deployments scale to tens of thousands of machines and hundreds of petabytes of capacity.

## Key Parameters

- Block size (controls task granularity and metadata cost).
- Replication factor or erasure-coding scheme (durability vs storage cost).
- Rack/zone awareness for replica placement.

## When To Use

As the storage substrate for batch and stream processing in a data-lake architecture, or as a generic high-throughput shared filesystem in a Hadoop-style cluster. Object stores are now often preferred in cloud deployments.

## Risks & Pitfalls

- Centralized NameNode is a single point of failure (mitigated by HA setups).
- Small files (millions of <128 MB files) are inefficient; bundle them.
- Erasure coding sacrifices data-locality benefits.
- Permissioning and quotas are weaker than enterprise NAS.

## Related Concepts

- [[concepts/mapreduce]]
- [[concepts/dataflow-engine]]
- [[concepts/replication]]
- [[entities/hdfs]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
