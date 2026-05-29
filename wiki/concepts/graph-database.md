---
title: Graph Database
type: claim
id: claim-graph-database
tags:
- databases
- nosql
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.65
---

## Definition

A graph database represents data as nodes (entities) and edges (relationships), both of which can carry properties. Relationships are first-class citizens, enabling efficient traversal-based queries. Examples: Neo4j, Amazon Neptune, OrientDB.

## How It Works

Storage is optimized for traversal. Query languages include Cypher (Neo4j, openCypher), Gremlin (Apache TinkerPop), and SPARQL (RDF). Traversal queries explore paths through the graph without expensive joins.

## Key Parameters

- Index types on node properties.
- Traversal depth limits.
- Single-node vs. partitioned deployment.

## When To Use

Social networks, fraud detection, knowledge graphs, recommendation engines, routing problems.

## Risks & Pitfalls

- Partitioning highly connected graphs is theoretically hard; most graph databases scale up rather than out.
- Query performance varies dramatically with index quality.

## Related Concepts

- [[concepts/nosql]]
- [[concepts/document-database]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
