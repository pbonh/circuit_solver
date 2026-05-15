---
title: "Partitioning (Sharding)"
type: concept
tags: [distributed-systems, well-established, foundational, partitioning]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Partitioning (also called sharding) is the practice of splitting a large dataset into disjoint subsets called partitions and placing each on a different node in a shared-nothing cluster. The goal is to scale storage and query throughput. Each record belongs to exactly one partition. Terminology varies: shard (MongoDB/ES/Solr), region (HBase), tablet (Bigtable), vnode (Cassandra/Riak), vBucket (Couchbase).

## How It Works

Two main partitioning schemes:

- **Key-range partitioning**: each partition owns a contiguous range of keys, like the volumes of an encyclopedia. Boundaries adapt to data distribution (manually or automatically). Enables efficient range scans but risks hot spots on monotonic keys like timestamps. Used by Bigtable, HBase, RethinkDB, pre-2.4 MongoDB.
- **Hash partitioning**: a hash function (MD5, Fowler-Noll-Vo) is applied to the key; partitions own hash ranges. Distributes load evenly but destroys key ordering — range scans require scatter-gather. Cassandra's compound key offers a hybrid (hash partition by first key column, sort within partition).

Secondary indexes complicate partitioning:
- **Document-partitioned (local) indexes** colocate index with its primary data; writes are single-partition, reads require scatter-gather across all partitions.
- **Term-partitioned (global) indexes** partition by indexed term; reads are single-partition, but writes must update multiple index partitions, often asynchronously (e.g., DynamoDB GSIs).

Rebalancing strategies: fixed-partition count with reassignment, dynamic split/merge (HBase, RethinkDB), or partitions-per-node (Cassandra, Ketama). Never use `hash mod N` because most keys move when N changes.

Request routing: routing tier, gossip protocol (Cassandra, Riak), or external coordination service (ZooKeeper for HBase, Kafka, SolrCloud, Espresso/Helix).

## Key Parameters

- Number of partitions (fixed vs dynamic).
- Partition key selection.
- Replication factor combined with partitioning.
- Rebalancing strategy.
- Routing tier / service discovery mechanism.

## When To Use

When data volume, write throughput, or query throughput exceeds a single machine's capacity; or when geographic locality demands partitioning data by region.

## Risks & Pitfalls

- Bad partition-key choice creates hot spots; celebrity users or timestamp keys are notorious.
- Cross-partition transactions are far slower than single-partition transactions (VoltDB reports ~1000/sec vs single-partition's much higher rate).
- Secondary indexes don't map cleanly to partitions; choose document vs term partitioning per workload.
- Rebalancing under load combined with automatic failure detection can cause cascading failures.

## Related Concepts

- [[concepts/replication]]
- [[concepts/secondary-index]]
- [[concepts/consensus]]
- [[concepts/transaction]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
