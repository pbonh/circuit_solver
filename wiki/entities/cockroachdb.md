---
title: CockroachDB
type: entity
id: entity-cockroachdb
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

CockroachDB is an open-source, Spanner-inspired distributed SQL database that does not require custom TrueTime-style hardware, instead using NTP/HLC-bound clocks at the cost of weaker consistency guarantees compared to Spanner.

## Characteristics

- Postgres wire protocol compatible.
- Range-based sharding with Raft replication per range.
- Distributed transactions via a 2PC variant.
- Uses hybrid logical clocks (HLCs) instead of TrueTime.

## Common Strategies

- Deploy across multiple AZs/regions for survivability.
- Use locality-aware placement for low-latency reads.

## Related Entities

- [[entities/cloud-spanner]]
- [[entities/yugabytedb]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
