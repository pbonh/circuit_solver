---
title: Network Partition
type: claim
id: claim-network-partition
tags:
- distributed-systems
- well-established
- fault-tolerance
- networking
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
confidence:
  base: 0.85
---

## Definition

A network partition (or netsplit) is a network fault in which one part of the cluster becomes unable to communicate with another, even though each subset internally remains healthy. Partitions are the canonical fault that the CAP theorem addresses: under a partition, a system must choose between linearizable consistency and availability.

## How It Works

- Caused by switch failures, BGP misconfiguration, undersea cable damage, datacenter power outages, asymmetric link failures (packets flow one way only), or virtualization issues.
- Network partitions are common even in well-managed datacenters; studies report 10+ per month in medium-sized DCs.
- Detection is inherently ambiguous: a timeout cannot distinguish a dead node from a slow node from a partitioned link.
- Systems respond with one of: pause writes (CP), continue with reduced consistency (AP), or attempt healing via quorums and fencing.

## Key Parameters

- Heartbeat / timeout configuration.
- Quorum size (must work in the majority partition).
- Recovery procedure for re-merging partitions (especially in multi-leader / leaderless setups).

## When To Use

Awareness rather than choice — partitions will happen. Plan for them: use fencing tokens, choose CP or AP intentionally per service, run chaos engineering, monitor cluster topology.

## Risks & Pitfalls

- Asymmetric partitions (one direction works, the other doesn't) are especially confusing — a node may receive messages but its replies never get through.
- Sloppy quorums during partitions can give the appearance of progress but break durability/linearizability.
- Leader election during partitions can produce split-brain if fencing isn't enforced.

## Related Concepts

- [[concepts/cap-theorem]]
- [[concepts/quorum]]
- [[concepts/leader-election]]
- [[concepts/fencing-token]]
- [[concepts/fault-tolerance]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
