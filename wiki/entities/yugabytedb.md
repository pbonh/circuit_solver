---
title: YugabyteDB
type: entity
id: entities/yugabytedb
tags:
- database
- newsql
- relational
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
---

## Overview

YugabyteDB is an open-source distributed SQL database (Postgres-compatible) that uses Raft for replication consensus and supports multi-region deployments. Like CockroachDB, it is Spanner-inspired but uses standard time sources.

## Characteristics

- Postgres-compatible SQL with YSQL; also supports a Cassandra-compatible API (YCQL).
- Raft per shard for consensus.
- ACID transactions across shards.

## Common Strategies

- Cluster placement policies to control replica geography.
- Hot-shard splitting and rebalancing.

## Related Entities

- [[entities/cloud-spanner]]
- [[entities/cockroachdb]]
- [[entities/voltdb]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
