---
title: "Designing Data-Intensive Applications"
type: source
slug: designing-data-intensive-applications
created: 2026-06-16
updated: 2026-06-16
summary: Kleppmann's definitive reference on data system internals — storage engines (B-tree/LSM), replication, partitioning, transactions (ACID/isolation), distributed consistency (linearizability, Raft), batch (MapReduce) and stream (Kafka) processing.
source_file: Books/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann
tags: [distributed-systems, databases, replication, consistency, transactions, kafka, mapreduce, stream-processing]
status: active
---

# Designing Data-Intensive Applications

- **Source file:** `sources/Books/Designing Data-Intensive Applications.../`
- **Author / origin:** Martin Kleppmann; O'Reilly, 2017
- **Date:** 2017

## Summary

The definitive technical reference for understanding how data systems work under the hood. Covers storage engines, distributed replication and partitioning, transaction isolation models, distributed consistency theory, and derived data via batch/stream processing.

### Part I: Foundations of Data Systems

**Data models**: Relational (SQL, joins, many-to-many), document (MongoDB, JSON — good for 1:many), graph (property graphs, Cypher, RDF/SPARQL, Datalog). Graph models are naturally suited to VLSI netlist representation.

**Storage engines**: Log-structured (LSM-Tree + SSTables — fast writes, read-amplification); B-trees (balanced page-based — fast reads, write-amplification). Hash indexes (Bitcask). Secondary indexes, full-text search (Lucene). Column-oriented storage (Parquet, ORC) for analytics (compress per column, SIMD operations, vectorized processing).

**Encoding**: JSON (schema-less, slow), Thrift/Protocol Buffers (schema, field tags, efficient), Avro (schema evolution, no field IDs). Backward/forward compatibility. Dataflow: through databases, REST/RPC services, and message queues.

### Part II: Distributed Data

**Replication**: Single-leader (synchronous/async lag), multi-leader (conflict detection/resolution — last-writer-wins, CRDTs, version vectors), leaderless (Dynamo-style quorums, sloppy quorums). Replication lag anomalies: read-your-own-writes, monotonic reads, consistent prefix reads.

**Partitioning**: Key-range vs. hash partitioning; secondary index partitioning (document-local vs. term-based); rebalancing strategies; routing (client-side, centralized, gossip).

**Transactions**: ACID (atomicity, consistency, isolation, durability). Isolation levels: read committed, snapshot isolation (MVCC), serializable snapshot isolation (SSI — optimistic concurrency with conflict detection). Two-phase locking (pessimistic). Phantom reads and write skew.

**Distributed systems challenges**: Partial failures, unreliable networks (timeouts ≠ failure), clock skew (NTP inaccuracy, monotonic vs. wall clocks), process pauses (GC, VM migration). Truth by majority vote; Byzantine fault tolerance.

**Consistency and consensus**: Linearizability (strongest — appears as single-copy register), causal consistency, sequential consistency. Total order broadcast = distributed state machine replication. Raft consensus (leader election + log replication). ZooKeeper/etcd for distributed coordination (locks, service discovery).

### Part III: Derived Data

**Batch processing**: Unix philosophy (small composable tools). MapReduce on HDFS (map → shuffle → reduce). Joins: reduce-side (sort-merge), map-side (broadcast hash join). Beyond MapReduce: Spark/Flink dataflows, materialization vs. streaming. Graph iterative processing on MapReduce (Pregel emulation).

**Stream processing**: Event streams vs. polling. Kafka (partitioned log, consumer groups, exactly-once). Change data capture (CDC) from databases → event streams. Event sourcing (immutable event log as system of record). Stream-stream joins, stream-table joins, table-table joins. Windowing (tumbling, sliding, session). Fault tolerance via checkpointing.

**Future of data systems**: Unbundling databases (data integration via event streams + derived views). Dataflow applications (end-to-end correctness without distributed transactions). Integrity constraints without 2PC (idempotent writes + exactly-once). Timeliness vs. integrity trade-offs.

## Key takeaways

- B-trees and LSM-trees represent the fundamental read-write performance tradeoff in storage engines — relevant to simulation result databases
- Transactions with serializable isolation require either 2PL (pessimistic) or SSI (optimistic); most practical systems use weaker isolation
- Linearizability is expensive (requires consensus); causal consistency is often sufficient for simulation result storage
- Kafka's distributed log is the right foundation for large-scale simulation event pipelines: immutable, replayable, partitioned
- Graph data models (property graphs, RDF) are natural for VLSI netlist representation — can query netlist structure with Cypher or SPARQL
- Stream processing (Flink) enables real-time analysis of simulation result streams from parallel EDA farm runs

## Pages updated from this source

- [[scalable-distributed-systems]] - extended with storage engines, transactions, Kafka
