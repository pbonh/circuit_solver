---
title: "Merkle Tree"
type: concept
tags: [distributed-systems, data-structure, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

A Merkle tree is a binary hash tree where each leaf stores a hash of a data object and each internal node stores the hash of its children. The root provides a compact fingerprint of the entire data set, making divergence detection efficient.

## How It Works

To compare two replicas, peers exchange root hashes; if equal, the data sets match. Otherwise, they recurse into the differing subtrees and exchange child hashes until divergent leaves are located. Used in anti-entropy repair (Cassandra, Riak), Git, Bitcoin, IPFS, and certificate transparency.

## Key Parameters

- Hash function (SHA-256, BLAKE2, etc.).
- Tree depth / chunk size.

## When To Use

Comparing large data sets across nodes with minimum bandwidth, signing collections of items, content-addressable storage.

## Risks & Pitfalls

- Construction is CPU- and memory-intensive at very large data sizes.
- Hash collisions are theoretically possible but cryptographically negligible.

## Related Concepts

- [[concepts/anti-entropy-repair]]
- [[concepts/replication]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
