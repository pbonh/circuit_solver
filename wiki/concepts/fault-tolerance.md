---
title: Fault Tolerance
type: claim
id: concepts/fault-tolerance
tags:
- distributed-systems
- foundational
- well-established
- reliability
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Fault tolerance is the property of a system that allows it to continue providing its required service to users even when individual components deviate from their specification. A fault (one component failing) need not become a failure (the whole system stopping) if the system anticipates the fault class.

## How It Works

- Classify faults: hardware (random, mostly uncorrelated), software (systematic, often correlated), human (configuration mistakes, dominant cause of outages in many studies).
- Apply redundancy at the appropriate layer: RAID, replication, multi-machine clusters, multiple datacenters.
- Build "reliable systems from unreliable parts" — software fault-tolerance lets you tolerate whole-machine loss rather than relying only on hardware redundancy.
- Exercise fault paths continuously (e.g., chaos engineering, kill-9 testing); use rolling upgrades to avoid planned downtime.
- For human error: design APIs to make right action easy; provide sandboxes; rapid rollback; gradual rollout.

## Key Parameters

- Fault classes the system explicitly tolerates (single node, AZ, region).
- Replication factor and quorum settings (developed in Part II of DDIA).
- Recovery time objective (RTO) and recovery point objective (RPO).
- Fault-injection cadence and coverage.

## When To Use

For any service whose downtime cost — financial, reputational, safety — exceeds the engineering cost of fault tolerance. Modern cloud-native systems essentially require fault tolerance because VMs disappear without warning.

## Risks & Pitfalls

- Cannot tolerate every possible fault (e.g., planet swallowed by black hole). Pick a fault model.
- Hardware redundancy alone is insufficient at scale.
- Cascading failures can propagate faults across components designed independently.
- Security faults often cannot be recovered from after the fact — prevention may be required.

## Related Concepts

- [[concepts/reliability]]
- [[concepts/scalability]]
- [[concepts/write-ahead-log]]
- [[concepts/backward-and-forward-compatibility]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
