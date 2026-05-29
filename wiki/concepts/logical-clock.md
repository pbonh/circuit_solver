---
title: Logical Clock
type: claim
id: concepts/logical-clock
tags:
- distributed-systems
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A logical clock is a counter-based mechanism (Lamport, 1978) that captures the happens-before relationship between events in a distributed system without requiring synchronized physical clocks. Lamport clocks define a partial order; vector clocks define enough information to detect concurrency.

## How It Works

Each process maintains a local counter. On a local event, the counter increments. On sending a message, the counter is included; on receiving, the local counter is set to max(local, received) + 1. Causally related events therefore satisfy clock(a) < clock(b).

## Key Parameters

- Counter width (32 or 64 bit).
- Vector-clock dimension (= number of replicas) for stricter ordering.

## When To Use

Wherever wall-clock timestamps are insufficient: distributed databases, version vectors, causal-consistency tracking, distributed snapshots.

## Risks & Pitfalls

- Lamport clocks cannot detect concurrency by themselves.
- Vector clocks scale with replica count.

## Related Concepts

- [[concepts/clock-drift]]
- [[concepts/version-vector]]
- [[concepts/consensus]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
