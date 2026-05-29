---
title: Amazon DynamoDB
type: entity
id: entity-dynamodb
tags:
- well-established
- distributed-systems
- nosql
- managed
- key-value
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
---

## Overview

Amazon DynamoDB is AWS's fully managed key-value and document database. Despite the name, it is architecturally different from the original Dynamo paper that inspired Cassandra and Riak: DynamoDB uses single-leader replication per partition rather than the leaderless Dynamo-style design. It is widely used for serverless backends and high-throughput workloads.

## Characteristics

- Per-partition single-leader replication with quorum-style reads/writes available as options.
- Global tables provide multi-region replication.
- Global secondary indexes (GSIs) are term-partitioned and updated asynchronously (with stated propagation delays).
- Local secondary indexes (LSIs) are document-partitioned and updated synchronously.
- ACID transactions (TransactWriteItems / TransactGetItems) since 2018.
- Charged by RCU/WCU or on-demand; auto-scaling and provisioned modes.

## Common Strategies

- Single-table design to colocate query patterns under a chosen partition key.
- Use GSIs sparingly; design partition keys to spread load.
- DynamoDB Streams + Lambda for change-data-capture.

## Related Entities

- [[entities/cassandra]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
