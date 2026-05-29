---
title: NoSQL
type: claim
id: concepts/nosql
tags:
- databases
- distributed-systems
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

NoSQL is the loosely defined family of non-relational databases that emerged from internet-scale workloads: simplified, schemaless or schema-on-read data models; native horizontal scaling on commodity hardware; and proprietary query languages with limited or no joins. The term is best read as "Not Only SQL."

## How It Works

Four broad model categories: key-value, document, wide-column, and graph. Data is typically denormalized to suit query patterns. Sharding and replication are first-class. Most NoSQL systems offer tunable consistency rather than fully ACID.

## Key Parameters

- Data model.
- Consistency level per request.
- Sharding strategy.

## When To Use

Workloads with very large data sets, evolving schemas, or specific access patterns (key-value lookups, graph traversal) that relational databases handle poorly at scale.

## Risks & Pitfalls

- Loss of declarative joins shifts complexity to the application.
- Eventual consistency surprises engineers accustomed to ACID.
- Operational maturity varies widely between products.

## Related Concepts

- [[concepts/key-value-store]]
- [[concepts/document-database]]
- [[concepts/wide-column-store]]
- [[concepts/graph-database]]
- [[concepts/denormalization]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
