---
title: "Google Cloud Spanner"
type: entity
tags: [database, newsql, relational, cloud]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Overview

Cloud Spanner is Google's globally distributed, strongly consistent (external consistency / strict serializable) SQL database, exposed as a managed service on Google Cloud Platform. It originated from the internal Spanner system described in the 2013 Google paper.

## Characteristics

- Sharded into "splits" — contiguous key ranges that are independently scheduled.
- Multi-Paxos replication per split with long-lived leaders.
- Multi-split transactions use 2PC where the coordinator is itself a Paxos group (no blocking on coordinator failure).
- Uses the TrueTime service (GPS + atomic clocks, ~7 ms bounded skew) plus commit-wait to provide linearizability.
- Strongly consistent reads contact the Paxos leader.
- Customer base spans financial services, retail, and gaming.

## Common Strategies

- Choose primary keys carefully to avoid hotspot splits.
- Use stale reads to reduce latency when freshness is not required.
- Co-locate frequently joined rows by primary key prefix.

## Related Entities

- [[entities/cockroachdb]]
- [[entities/yugabytedb]]
- [[entities/voltdb]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
