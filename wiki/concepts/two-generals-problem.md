---
title: "Two Generals' Problem"
type: concept
tags: [distributed-systems, consistency, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

The Two Generals' Problem is a thought experiment proving that, on an unreliable channel where messages may be lost, two parties cannot guarantee to reach common knowledge of an agreed action. It is the classic illustration of why distributed consensus over lossy channels has no guaranteed-bounded-time solution.

## How It Works

Two armies on opposite sides of a city must attack simultaneously. They communicate by messengers who can be captured. Each general needs an acknowledgment of any plan, but each acknowledgment itself needs an acknowledgment, leading to an infinite regress. Pragmatic mitigations send many messengers to raise the probability of delivery, but no protocol guarantees agreement.

## Key Parameters

- Message-loss probability.
- Number of retries / duplicates sent.
- Acceptable failure-to-agree probability.

## When To Use

Any time you need to reason about the limits of agreement protocols across unreliable networks — e.g., distributed transactions, replica consistency, mobile-device synchronization.

## Risks & Pitfalls

- Tempting to invent "obvious" agreement protocols that turn out to be incorrect.
- Mistaking the theoretical impossibility for a practical impossibility (practical systems use timeouts and quorums).

## Related Concepts

- [[concepts/consensus]]
- [[concepts/partial-failure]]
- [[concepts/byzantine-faults]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
