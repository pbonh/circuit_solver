---
title: Deadlock
type: claim
id: concepts/deadlock
tags:
- concurrency
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A deadlock is a state in which two or more threads (or processes) are each blocked forever, each holding a resource the other needs. The canonical illustration is the dining philosophers problem: each philosopher picks up the left chopstick first and waits for the right, creating a circular wait.

## How It Works

Coffman's four conditions for deadlock are: mutual exclusion, hold and wait, no preemption, and circular wait. Eliminating any one prevents deadlock. The usual remedy is imposing a global ordering on resource acquisition (e.g., always acquire chopstick[i] before chopstick[i+1]) so circular wait cannot form.

## Key Parameters

- Lock-acquisition ordering policy.
- Lock-acquisition timeouts.

## When To Use

Wherever multiple locks may be held concurrently. Always design lock ordering before writing the locking code.

## Risks & Pitfalls

- Subtle deadlocks emerge from unexpected lock paths (e.g., database row locks during a multi-table transaction).
- Timeouts that "fix" deadlocks merely convert them into livelocks unless the underlying ordering issue is resolved.

## Related Concepts

- [[concepts/concurrency]]
- [[concepts/thread]]
- [[concepts/race-condition]]
- [[concepts/two-phase-commit]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
