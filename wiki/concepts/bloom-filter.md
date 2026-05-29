---
title: Bloom Filter
type: claim
id: concepts/bloom-filter
tags:
- storage
- well-established
- indexing
- probabilistic-data-structures
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A Bloom filter is a memory-efficient probabilistic data structure that approximates set membership: it can tell you with certainty when an element is *not* in the set, and with bounded false-positive probability when it *might* be. It was introduced by Burton Bloom in 1970.

## How It Works

- A fixed-size bit array plus k hash functions.
- To insert: hash the key k times, set the k bits at those positions.
- To query: hash the key k times and check the k bits; if any is 0, the key is definitely absent; if all are 1, the key is probably present (false positives possible, false negatives impossible).
- Storage engines like LevelDB, RocksDB, and Cassandra attach a Bloom filter to each SSTable so that lookups for absent keys can skip the file without performing a disk read.

## Key Parameters

- Bit-array size m and number of hash functions k chosen for expected element count n and target false-positive rate.
- Typically 10 bits per key for ~1% false-positive rate.

## When To Use

When fast negative answers are valuable — e.g., LSM-tree absent-key lookups, web cache "have I seen this URL", duplicate suppression in distributed systems.

## Risks & Pitfalls

- Cannot delete elements (standard Bloom filter). Variants (counting Bloom, Cuckoo filter) support deletion.
- False positives compound across many filters; tune for the workload.
- Hash quality matters; cryptographic hashes are wasteful.

## Related Concepts

- [[concepts/lsm-tree]]
- [[concepts/sstable]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
