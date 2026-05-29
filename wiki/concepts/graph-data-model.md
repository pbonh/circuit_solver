---
title: Graph Data Model
type: claim
id: concepts/graph-data-model
tags:
- data-modeling
- graph
- well-established
- query-languages
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph data model represents data as vertices (nodes/entities) and edges (relationships/arcs). Two principal flavors exist: the property graph model (Neo4j, Titan, InfiniteGraph), in which vertices and edges carry labels and key-value properties, and the triple-store model (Datomic, AllegroGraph), in which all information is encoded as (subject, predicate, object) statements (RDF). Graphs naturally model many-to-many relationships and heterogeneous entity types in a single store.

## How It Works

- A property-graph vertex has a unique ID, sets of incoming and outgoing edges, and a property map. An edge has an ID, head and tail vertices, a label, and properties.
- The model permits arbitrary edges between any two vertices; no schema restricts pairings, giving evolvability for new relationship types.
- Traversal is supported by indexes on edge endpoints, allowing efficient navigation in both directions.
- Declarative graph query languages (Cypher, SPARQL, Datalog) express pattern matching with variable-length paths (e.g., `:WITHIN*0..` in Cypher) much more concisely than SQL recursive CTEs.
- Triple-stores associate naturally with the semantic web and RDF/Turtle serialization; in practice the data model is independent of the semantic-web vision.
- Datalog (subset of Prolog) defines rules over predicate facts; rules can call other rules, enabling composable queries.

## Key Parameters

- Choice of graph database (property vs triple-store) and query language.
- Edge indexing strategy (which directions need fast traversal).
- Use of labels/properties vs separate vertex types.

## When To Use

When relationships dominate the data: social networks, recommendation engines, knowledge graphs, biological pathways, fraud detection. When schema must evolve to admit new relationship types without migrations. When variable-length path queries are common.

## Risks & Pitfalls

- Scaling and partitioning highly connected graphs is intrinsically difficult; many graph DBs are single-node or specialized.
- Query optimizer maturity varies widely; bad plans can be catastrophic on dense graphs.
- Triple-store with URIs everywhere is verbose; semantic-web tooling is often overkill.
- Graph schema flexibility can become as much of a liability as document schemaless-ness if not disciplined.

## Related Concepts

- [[concepts/relational-model]]
- [[concepts/document-model]]
- [[concepts/declarative-query-language]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
