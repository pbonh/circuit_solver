---
title: 'Designing Data-Intensive Applications — Part III: Derived Data (Chapters 10–12)'
type: source
id: summaries/ddia-05-part-iii-derived-data
kind: publication
tags:
- batch
- streaming
- distributed-systems
- derived-data
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
---

## Key Points

- Real systems combine multiple datastores. Distinguish **systems of record** (authoritative source of truth, normalized) from **derived data systems** (caches, indexes, materialized views, ML models — recreatable from the source). Being explicit about which is which clarifies architecture.
- Three system categories by access pattern: **services** (online, response-time-oriented), **batch processing** (offline, throughput-oriented), and **stream processing** (near-real-time, low-latency on unbounded data).
- **Unix philosophy**: programs do one thing well; uniform interfaces (files, stdin/stdout); separation of logic and wiring; transparency for experimentation; immutable inputs allow safe retry. These principles map directly onto large-scale data systems.
- **MapReduce** (Hadoop) generalizes the Unix pattern across thousands of machines using HDFS as a distributed filesystem. A job: read input files → mapper extracts key-value pairs → framework sorts and shuffles → reducer aggregates → write outputs. Putting computation near the data via partition-aware scheduling minimizes network traffic.
- Workflows of multiple MapReduce stages are chained by directory naming; schedulers like Oozie, Azkaban, Luigi, Airflow, Pinball manage dependencies. Higher-level tools (Pig, Hive, Cascading, Crunch, FlumeJava) compile to MapReduce.
- **Reduce-side joins** (sort-merge, secondary-sort) and **map-side joins** (broadcast hash, partitioned hash, merge) implement equi-joins over batch datasets. Output is typically search indexes, ML training data, or recommendation feeds.
- **Dataflow engines** (Spark, Tez, Flink) generalize MapReduce by avoiding materialized intermediate state, supporting DAG topologies, and pipelining. Higher-level APIs (Spark DataFrame, Flink Table API, BigQuery, Drill) speak SQL-on-Hadoop.
- **Graph processing** (Pregel, GraphX, Giraph) uses the bulk synchronous parallel (BSP) model: vertex-centric "think-like-a-vertex" computation with message passing per superstep, ideal for iterative algorithms (PageRank, shortest paths).
- Stream processing handles **unbounded** datasets continuously. Direct messaging (UDP multicast, ZeroMQ, webhooks) is fragile; **message brokers** centralize durability; **log-based brokers** (Kafka, Kinesis, DistributedLog) combine database-like durability with low-latency notification by appending to partitioned logs that consumers read by offset.
- **Change Data Capture (CDC)** turns a database's write log into an event stream that other systems consume — search indexes, caches, warehouses stay consistent without dual writes (which cause race conditions). Tools: Bottled Water, Maxwell, Debezium, Mongoriver, GoldenGate, LinkedIn Databus, Yahoo Sherpa, Kafka Connect.
- **Event sourcing** stores user actions as immutable event log entries; current state is derived. Decouples writes (append-only, easy to make atomic) from reads (optimized views). Pairs with **CQRS** (Command Query Responsibility Segregation). Log compaction provides full snapshots from the event log.
- "The truth is the log. The database is a cache of the latest record values from the log" (Pat Helland). State and stream are duals: state is the integral of events, change-stream is the derivative of state.
- **Reasoning about time** in streams is hard: event time vs processing time, straggler events, windowing (tumbling, hopping, sliding, session). Use device + send + receive timestamps to estimate true event time when device clocks are unreliable. Watermarks signal when a window can be closed.
- **Stream joins**: stream-stream (windowed), stream-table (enrichment via local replica kept fresh by CDC), table-table (materialized view maintenance, e.g., Twitter timelines). All are time-dependent; slowly-changing-dimension versioning may be needed for deterministic reprocessing.
- **Stream fault tolerance**: microbatching (Spark Streaming) treats each batch like a mini job; **checkpointing** (Flink) periodically saves state. Exactly-once / effectively-once semantics combine atomic state-and-output commits with idempotent operations and end-to-end deduplication.
- **Lambda architecture** runs a batch pipeline for accuracy and a streaming pipeline for low latency in parallel; modern stream processors (Flink, Beam) **unify batch and stream** by replaying historical events through the same engine with event-time windowing and exactly-once semantics.
- **Unbundling databases**: a database is internally a composition of storage engine + index maintainer + replicator + query engine. Distributed dataflow systems are externalizing those components as separate services connected by event logs — a "Unix philosophy for distributed data." `CREATE INDEX` is conceptually identical to bootstrapping a derived view via CDC.
- **End-to-end argument**: low-level reliability (TCP checksums, transaction atomicity, broker exactly-once) is necessary but not sufficient for application correctness. Application-level request IDs propagated end-to-end provide robust idempotence and duplicate suppression — DDIA Example 12-2.
- **Timeliness vs integrity** are separable correctness concerns: timeliness violations are eventually consistent and self-healing; integrity violations are perpetual inconsistency and require explicit repair. In many applications integrity matters far more than linearizability.
- **Loosely interpreted constraints** with **compensating transactions** (apologies, refunds, overbooking adjustments) often replace strict synchronous uniqueness checks — yielding **coordination-avoiding data systems** that scale far better than 2PC-bound ones.
- **Trust but verify**: hardware faults (radiation bit-flips, SSD lies, rowhammer), software bugs (MySQL/PostgreSQL constraint and isolation bugs), and silent data corruption justify continual integrity auditing (HDFS/S3 scrubbing, Merkle trees, certificate transparency, distributed ledgers).
- The book closes with an ethical chapter: **predictive analytics** can encode bias and discrimination; **surveillance** is increasingly the default revenue model; **privacy** is a decision right being transferred from individuals to corporations. Kleppmann calls for engineers to treat users as humans, design for auditability, purge data when no longer needed, and self-regulate ahead of legislation.

## Relevant Concepts

- [[concepts/mapreduce]] — Batch programming model; already created in Part I summary.
- [[concepts/distributed-filesystem]] — HDFS, GFS, QFS; shared-nothing storage substrate for batch processing.
- [[concepts/dataflow-engine]] — Spark/Flink/Tez: DAG-based generalization of MapReduce with pipelining.
- [[concepts/bulk-synchronous-parallel]] — BSP / Pregel vertex-centric graph processing.
- [[concepts/stream-processing]] — Continuous processing of unbounded event streams.
- [[concepts/event-sourcing]] — Append-only immutable event log as system of record; state derived.
- [[concepts/cqrs]] — Command Query Responsibility Segregation; separate write and read models.
- [[concepts/change-data-capture]] — Extracting a DB's change log as a stream consumable by derived systems.
- [[concepts/log-compaction]] — Periodically keeping only the latest value per key in a log, yielding a full snapshot.
- [[concepts/log-based-message-broker]] — Kafka-style append-only partitioned logs combining DB durability and message-queue delivery.
- [[concepts/microbatching]] — Spark Streaming's approach: small batches for fault tolerance in streams.
- [[concepts/exactly-once-semantics]] — Effectively-once processing via idempotence and end-to-end IDs.
- [[concepts/idempotency]] — Operation invariance under repetition; already created.
- [[concepts/lambda-architecture]] — Parallel batch + streaming pipelines for accuracy + latency.
- [[concepts/end-to-end-argument]] — Saltzer/Reed/Clark 1984: correctness must be implemented at the endpoints; low-level mechanisms are optimizations.
- [[concepts/derived-data]] — Data whose contents are reproducible from a system of record.
- [[concepts/system-of-record]] — Authoritative, normalized source-of-truth dataset.
- [[concepts/materialized-view]] — Already created in Part I; central to derived-data thinking.
- [[concepts/total-order-broadcast]] — Already created; underpins log-based integration.
- [[concepts/causal-consistency]] — Already created; relevant for cross-stream causality.
- [[concepts/version-vector]] — Already created; tracks concurrent updates.
- [[concepts/two-phase-commit]] — Already created; contrasted with log-based loose coupling.
- [[concepts/coordination-avoiding-data-systems]] — Bailis et al. — systems preserving integrity without synchronous coordination.
- [[concepts/data-provenance]] — Tracking where derived data came from, for audit and debugging.
- [[entities/apache-spark]] — Dataflow engine with batch and streaming APIs.
- [[entities/apache-flink]] — Stream-first dataflow engine with batch as a subcase.
- [[entities/apache-kafka]] — Already created; canonical log-based broker.
- [[entities/hdfs]] — Already created.
- [[entities/google-dataflow-beam]] — Google's unified batch/stream model and Apache Beam SDK.
- [[entities/apache-storm]] — One of the first widely used stream processors.
- [[entities/samza]] — LinkedIn's Kafka-native stream processor.
- [[entities/elasticsearch]] — Distributed search engine, common derived data sink.
- [[entities/voltdb]] — Already created; deterministic stored procedures + log-based replication.

## Source Metadata

- Source type: book chapter (compilation of Chapters 10–12)
- Book title: Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems
- Author: Martin Kleppmann
- Chapters covered: 10 (Batch Processing), 11 (Stream Processing), 12 (The Future of Data Systems)
- File path: `raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt`
- Publisher: O'Reilly Media, 2017
