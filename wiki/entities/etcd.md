---
title: etcd
type: entity
id: entities/etcd
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

etcd is a distributed key-value store implementing linearizable storage on top of the Raft consensus algorithm. It originated at CoreOS and is now a CNCF project. It serves as the configuration and coordination backbone of Kubernetes and many other cloud-native systems.

## Characteristics

- Strong consistency via Raft: writes go through a leader, replicated to a majority before commit.
- Linearizable reads via a quorum read mode (optional, default is leader-local).
- Watch streams for change notifications.
- Lease-based key TTLs for leader election and distributed locks.
- HTTP/JSON and gRPC interfaces.

## Common Strategies

- Run a 3- or 5-node cluster for fault tolerance.
- Use leases for leader election (the lease holder is the leader; lease expiry triggers re-election).
- Keep data small (Kubernetes objects, configuration, secrets) — etcd is not a bulk store.

## Related Entities

- [[entities/zookeeper]]
- [[concepts/raft]]
- [[entities/kubernetes]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
