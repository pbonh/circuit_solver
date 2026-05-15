---
title: "Byzantine Faults"
type: concept
tags: [distributed-systems, fault-tolerance, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: medium
---

## Definition

Byzantine faults are failures in which a component does not just stop or omit messages but actively sends incorrect or contradictory information — including malicious behavior. They are the strongest failure model considered by distributed-system theory.

## How It Works

Algorithms that tolerate Byzantine faults (BFT consensus, PBFT, Bitcoin's proof-of-work) require strict majorities (typically more than 2/3 of nodes) of honest participants and use cryptographic message authentication. They are substantially more expensive than crash-tolerant algorithms.

## Key Parameters

- Fraction of nodes that may behave Byzantinely.
- Message-signing scheme.
- Consensus round complexity.

## When To Use

Cross-organization systems where some nodes are untrusted (blockchains, financial settlement networks). Generally excluded inside well-protected enterprise networks where crash-tolerance suffices.

## Risks & Pitfalls

- BFT protocols are much more expensive than crash-tolerant ones.
- Defining "Byzantine" precisely (vs. simple bugs) is rarely the right framing for internal systems.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/partial-failure]]
- [[concepts/two-generals-problem]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
