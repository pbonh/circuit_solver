---
title: "Google Spanner"
type: entity
tags: [well-established, distributed-systems, relational, globally-distributed]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: medium
---

## Overview

Google Spanner is a globally distributed, relational, strongly consistent database. It is famous for the **TrueTime API**, which exposes the uncertainty of clock readings as a `[earliest, latest]` interval and uses commit-wait to produce timestamps consistent with causality across datacenters. Spanner uses Paxos for replication within tablet groups and two-phase commit across groups.

## Characteristics

- SQL interface with externally consistent transactions across globally distributed shards.
- TrueTime backed by GPS receivers and atomic clocks in each datacenter, achieving ~7 ms clock uncertainty.
- Tablet-level Paxos replication; cross-tablet 2PC for distributed transactions.
- Schema and secondary-index support.
- Foundation for Cloud Spanner (managed offering) and inspired CockroachDB and YugabyteDB.

## Common Strategies

- Use interleaved tables to colocate hierarchically related data, minimizing cross-partition transactions.
- Choose primary keys to spread load and avoid hot partitions.
- Pair with read-only replicas for low-latency reads at the cost of staleness.

## Related Entities

- [[entities/postgresql]]
- [[entities/paxos]]
- [[entities/raft]]
- [[entities/martin-kleppmann]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
