---
title: Apache ZooKeeper
type: entity
id: entities/zookeeper
tags:
- well-established
- distributed-systems
- consensus
- coordination
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
---

## Overview

Apache ZooKeeper is a distributed coordination service modeled after Google's Chubby. It provides linearizable atomic operations, totally ordered fencing tokens, failure detection via heartbeats and ephemeral nodes, and change notifications. ZooKeeper holds small amounts of data (everything fits in memory, persisted to disk for durability) replicated via the Zab consensus protocol — a variant of total order broadcast.

## Characteristics

- Hierarchical key-value namespace (znodes) with watches for change notifications.
- Ephemeral znodes auto-delete when the creating client's session times out — basis for distributed locks and leader election.
- Each operation gets a monotonically increasing transaction ID (zxid) usable as a fencing token.
- Reads can be served from any replica (potentially stale); `sync()` then read gives linearizable semantics.
- Used by HBase, Apache Kafka, Hadoop YARN, OpenStack Nova, SolrCloud, LinkedIn's Espresso/Helix, and many other distributed systems for leader election, membership, configuration, and service discovery.

## Common Strategies

- Use Apache Curator on top of ZooKeeper for higher-level recipes (locks, leader latches, distributed queues).
- Keep data small and slow-changing; not a general-purpose database.
- Replicate writes via Zab; deploy 3 or 5 nodes for fault tolerance.

## Related Entities

- [[entities/etcd]]
- [[concepts/raft]]
- [[concepts/paxos]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
