---
title: "Designing Data-Intensive Applications — Part I: Foundations of Data Systems (Chapters 1–4)"
type: summary
tags: [distributed-systems, storage, foundational, well-established, data-modeling]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Key Points

- A data-intensive application is built from standard components (databases, caches, search indexes, stream processors, batch processors), so the application developer becomes a data-system designer responsible for keeping them consistent.
- Three foundational concerns guide design: **reliability** (continuing to work correctly under hardware/software/human faults), **scalability** (coping with growth in load, data, or traffic), and **maintainability** (operability, simplicity, evolvability).
- Faults are distinct from failures; fault-tolerant systems can be built from unreliable parts by anticipating hardware faults, software bugs, and human error (Netflix Chaos Monkey deliberately exercises this).
- Load must be described quantitatively (Twitter fan-out as an example), and performance measured with **response-time distributions and percentiles** (p50/p95/p99/p999) rather than means, since tail latency is what users experience.
- Scaling out (horizontal, shared-nothing) versus scaling up (vertical) is application-specific; there is no generic "magic scaling sauce."
- The relational model (Codd, 1970) became dominant because it hid access paths behind a declarative interface; the document model (JSON, MongoDB) revisits hierarchical-model strengths and weaknesses; graph models (Neo4j/Cypher, RDF/SPARQL, Datalog) handle highly interconnected data.
- Declarative query languages (SQL, Cypher, SPARQL) let the optimizer choose execution and lend themselves to parallelization, unlike imperative APIs (CODASYL, raw DOM manipulation).
- Two main storage-engine families: **log-structured** (LSM-trees, SSTables, memtables, compaction — Bitcask, LevelDB, RocksDB, Cassandra, HBase, Lucene) and **page-oriented** (B-trees, write-ahead logs, in-place updates — used in essentially all relational databases).
- LSM-trees tend toward higher write throughput and better compression; B-trees give predictable read latency, in-place updates, and natural fit for range-locking transactional isolation.
- OLTP and OLAP have diverged: OLTP indexes a small number of records per query, OLAP scans many rows of a few columns. Data warehouses use star/snowflake schemas and **column-oriented storage** (Parquet, C-Store, Vertica, Dremel), often with bitmap-encoded columns, run-length encoding, and vectorized SIMD execution.
- Encodings differ in compactness, evolvability, and ecosystem: language-specific (Java Serializable, pickle) is hostile to interop; textual (JSON/XML/CSV) is universal but ambiguous; **schema-driven binary** (Protocol Buffers, Thrift, Avro) compact and supports controlled schema evolution.
- Backward and forward compatibility are required for rolling upgrades. Avro distinguishes writer's schema from reader's schema and is friendly to dynamically generated schemas; Protocol Buffers/Thrift use numbered field tags.
- Dataflow modes covered: through databases (data outlives code; preserve unknown fields), through services (REST vs SOAP, RPC pitfalls — network is not a function call), and asynchronous message passing (brokers, actor frameworks like Akka/Orleans/Erlang).

## Relevant Concepts

- [[concepts/reliability]] — System keeps working correctly under faults.
- [[concepts/scalability]] — Strategies for sustaining performance as load grows; described via load parameters and response-time percentiles.
- [[concepts/maintainability]] — Operability, simplicity, evolvability of long-lived software.
- [[concepts/fault-tolerance]] — Designing systems to tolerate hardware/software/human faults via redundancy, isolation, and rolling upgrades.
- [[concepts/response-time-percentiles]] — Tail-latency metrics (p95/p99/p999) and SLOs/SLAs.
- [[concepts/relational-model]] — Codd's tuples-in-relations model, hiding access paths via declarative SQL.
- [[concepts/document-model]] — JSON/MongoDB-style tree-of-records; good locality for self-contained docs, weak on many-to-many.
- [[concepts/graph-data-model]] — Property graphs and triple-stores for highly interconnected data.
- [[concepts/declarative-query-language]] — SQL, Cypher, SPARQL, Datalog; pattern-specification vs imperative code.
- [[concepts/mapreduce]] — Map+reduce pure-function pipelines for distributed reads (used by MongoDB/CouchDB historically); discussed more in Part III.
- [[concepts/b-tree]] — Page-oriented, in-place updated, balanced index structure with WAL for crash recovery.
- [[concepts/lsm-tree]] — Log-structured merge tree built from memtable + sorted SSTables, compacted in the background.
- [[concepts/sstable]] — Sorted String Table; immutable on-disk sorted segment used by LSM engines and Lucene.
- [[concepts/write-ahead-log]] — Append-only log written before in-place page updates to make B-trees crash-safe.
- [[concepts/bloom-filter]] — Memory-efficient probabilistic set membership used to skip nonexistent-key lookups in LSM engines.
- [[concepts/secondary-index]] — Non-primary index built atop key-value indexes; clustered vs covering vs heap-file variants.
- [[concepts/column-oriented-storage]] — Storing each column contiguously; foundational for OLAP throughput, compression, and vectorized execution.
- [[concepts/data-warehouse]] — Read-only analytics-optimized copy of OLTP data, populated via ETL, often using star schemas.
- [[concepts/star-schema]] — Dimensional model with a central fact table and surrounding dimension tables.
- [[concepts/oltp-vs-olap]] — Distinct access patterns and optimizations for transactional vs analytic workloads.
- [[concepts/materialized-view]] — Persisted query result; special case is the OLAP/data cube.
- [[concepts/schema-on-read-vs-schema-on-write]] — Implicit vs enforced data structure (dynamic vs static type-checking analogy).
- [[concepts/schema-evolution]] — Adding/removing/changing fields while preserving forward/backward compatibility.
- [[concepts/backward-and-forward-compatibility]] — Rolling-upgrade prerequisite: new code reads old data, old code reads new data.
- [[concepts/data-encoding]] — Binary or textual serialization formats for cross-process exchange.
- [[concepts/remote-procedure-call]] — Location-transparency abstraction that hides network differences; fundamentally flawed if pushed too far.
- [[concepts/message-broker]] — Asynchronous middleware (Kafka, RabbitMQ, ActiveMQ, NATS) decoupling producers and consumers.
- [[entities/sql]] — Declarative relational query language and its ecosystem.
- [[entities/postgresql]] — Open-source relational DB used as a running example for JSON support, recursive CTEs, etc.
- [[entities/mongodb]] — Document database used to motivate document-vs-relational discussion and MapReduce/aggregation-pipeline examples.
- [[entities/cassandra]] — Wide-column LSM-based store inspired by Bigtable/Dynamo.
- [[entities/apache-kafka]] — Durable log-structured message broker.
- [[entities/apache-avro]] — Schema-driven binary encoding with writer/reader schema resolution.
- [[entities/protocol-buffers]] — Google's tagged-field binary encoding and IDL.
- [[entities/apache-thrift]] — Facebook's IDL and binary protocols (BinaryProtocol, CompactProtocol).

## Source Metadata

- Source type: book chapter (compilation of Chapters 1–4)
- Book title: Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems
- Author: Martin Kleppmann
- Chapters covered: 1 (Reliable, Scalable, and Maintainable Applications), 2 (Data Models and Query Languages), 3 (Storage and Retrieval), 4 (Encoding and Evolution)
- File path: `raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt`
- Publisher: O'Reilly Media, 2017
