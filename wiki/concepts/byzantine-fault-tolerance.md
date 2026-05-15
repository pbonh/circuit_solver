---
title: "Byzantine Fault Tolerance"
type: concept
tags: [distributed-systems, advanced, consensus, security]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: medium
---

## Definition

Byzantine fault tolerance is the ability of a distributed system to operate correctly even when some nodes behave arbitrarily — sending contradictory, corrupted, or maliciously crafted messages — rather than merely crashing. The name comes from Lamport's "Byzantine Generals Problem" (1982). Byzantine-fault-tolerant consensus typically requires that fewer than one-third of nodes are faulty.

## How It Works

- Nodes exchange signed or echo-verified messages so that a single faulty node cannot get others to agree on contradictory values.
- Algorithms like PBFT (Castro & Liskov), Honey Badger BFT, Tendermint, and HotStuff achieve BFT consensus.
- Modern blockchains (Bitcoin proof-of-work, Ethereum proof-of-stake) provide BFT among mutually untrusting parties without a central authority.
- Aerospace systems use BFT to tolerate radiation-induced register corruption.

## Key Parameters

- Maximum tolerated Byzantine nodes (typically `(n-1)/3`).
- Cryptographic signature scheme.
- Network model (synchronous, partially synchronous, asynchronous).

## When To Use

When participants don't trust each other (cryptocurrencies, multi-organization ledgers) or when hardware/radiation-induced corruption is plausible (aerospace, satellite). Almost never in single-org datacenter deployments — the cost vastly outweighs the benefit.

## Risks & Pitfalls

- Performance is far below crash-fault-tolerant consensus (extra rounds, signature overhead).
- Same-software bugs are correlated faults — BFT doesn't help against systemic bugs unless you run multiple independent implementations.
- Most production data systems use crash-fault-tolerant consensus and rely on traditional auth/firewall/encryption against attackers.

## Related Concepts

- [[concepts/consensus]]
- [[concepts/fault-tolerance]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
