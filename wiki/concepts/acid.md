---
title: "ACID"
type: concept
tags: [foundational, well-established, transactions]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

ACID stands for Atomicity, Consistency, Isolation, Durability — four safety properties of database transactions coined by Härder and Reuter in 1983 to formalize fault-tolerance guarantees. The acronym has become marketing shorthand; actual implementations vary widely, especially on the I.

## How It Works

- **Atomicity**: a transaction either commits all its writes or aborts and discards them. Implemented via undo logs / WAL.
- **Consistency**: application invariants (balances, uniqueness, foreign keys) hold before and after each transaction. This depends on the application defining its transactions correctly; the DB only enforces declared constraints.
- **Isolation**: concurrent transactions don't see each other's intermediate state. Classical definition is serializability; in practice weaker levels (read committed, snapshot isolation) are used for performance.
- **Durability**: once committed, the data survives crashes — via WAL, replication, archiving. Perfect durability is unattainable; combine techniques.

BASE (Basically Available, Soft state, Eventual consistency) is the marketing counter-acronym, even vaguer than ACID.

## Key Parameters

- Isolation level chosen.
- WAL fsync policy.
- Replication mode for durability.

## When To Use

For any system whose correctness depends on safe handling of partial failures and concurrency. Single-record CRUD apps may not need explicit transactions; complex business workflows almost always do.

## Risks & Pitfalls

- "ACID-compliant" claims are often misleading — verify which isolation level is actually default and what guarantees are provided.
- The C in ACID is the application's responsibility; the DB cannot prevent semantically inconsistent writes.
- ACID does not imply linearizability or distributed correctness; those are separate concerns.

## Related Concepts

- [[concepts/transaction]]
- [[concepts/serializability]]
- [[concepts/snapshot-isolation]]
- [[concepts/two-phase-commit]]
- [[concepts/write-ahead-log]]
- [[concepts/linearizability]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
