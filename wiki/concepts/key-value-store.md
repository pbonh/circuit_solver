---
title: Key-Value Store
type: claim
id: claim-key-value-store
tags:
- databases
- nosql
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

A key-value (KV) store is the simplest NoSQL data model: each object has a unique key and an opaque or simply typed value. The store provides get/put/delete operations, typically constant-time on the key.

## How It Works

Internally a KV store is usually a distributed hash table that maps keys to values across many nodes. Examples include Redis, Oracle NoSQL, Amazon DynamoDB (which extends to nested attributes), and memcached.

## Key Parameters

- Maximum value size.
- TTL/eviction policy.
- Persistence model.

## When To Use

Session stores, caching, configuration data, simple object lookups.

## Risks & Pitfalls

- Limited query capabilities (no secondary indexes in pure KV stores).
- Application must handle relationships explicitly.

## Related Concepts

- [[concepts/distributed-cache]]
- [[concepts/nosql]]
- [[concepts/document-database]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
