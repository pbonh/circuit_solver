---
title: Neo4j
type: entity
id: entity-neo4j
tags:
- database
- graph
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
---

## Overview

Neo4j is the leading open-source native graph database, optimized for storing and traversing highly connected data. It uses the Cypher query language (also open-sourced as openCypher) for declarative graph queries.

## Characteristics

- Native graph storage with index-free adjacency.
- Cypher query language.
- Cluster mode uses Raft-based replication; all writes route to the leader.
- Bookmark tokens enable read-your-own-writes consistency for follower reads.
- Fabric extension for manual graph partitioning.

## Common Strategies

- Scale up first; partitioning requires manual sharding via Fabric.
- Use indexes on key node properties to accelerate traversal entry points.

## Related Entities

- [[entities/mongodb]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
