---
title: 'Foundations of Scalable Systems — Part IV: Event and Stream Processing (Chapters
  14–16)'
type: source
id: source-foundations-scalable-systems-07-part-iv-event-and-stream-processing
kind: derived-summary
tags:
- distributed-systems
- scalability
- streaming
- messaging
- event-driven
- observability
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Key Points
- Event-driven architectures emit events when "something interesting happens" — package scans, license expirations, state changes. Events have no consumer expectation, creating loose coupling, and are commonly published to a messaging system. Persistent, immutable, append-only event logs differ from FIFO queues: they let new consumers replay history, allow event-processing logic to be revised against the full history, and back state-replication of derived data across microservices.
- Apache Kafka is the canonical "distributed persistent log" platform — a "dumb broker / smart clients" design. Consumers read non-destructively by specifying an offset; topics retain entries until a retention period elapses. Topic deletion uses TTL or "compacted topics" (only the latest value per key is retained; nulls function as tombstones). Cluster metadata historically lived in Apache ZooKeeper.
- Kafka producers batch events with `batch.size` and `linger.ms` to amortize network round trips; delivery guarantees are configurable via `acks` (0/1/all) and `enable.idempotence` (exactly-once). Consumers pull batches via `poll()`, with at-least-once (default, auto-commit), at-most-once (manual pre-process commit), or at-least-once-with-manual-commit semantics. The consumer API is not thread-safe; threading happens at the application layer.
- Kafka scales via topic partitions distributed across brokers; producers choose partitions via DefaultPartitioner (round-robin or key-hash for **semantic partitioning**). Order is guaranteed per-partition only; cross-partition ordering is not preserved. Increasing partition counts is allowed but post-deployment hashing can route the same key to different partitions, so consumers must be designed to not assume key affinity.
- Kafka consumer groups bind to topics with up to one consumer per partition. A group coordinator on the broker drives rebalances; a chosen group leader plans new partition assignments. The CooperativeStickyAssignor minimizes reassignment churn. Replication factor N produces N copies per partition with leader-follower replication, leader election via a custom algorithm constrained to the In-Sync Replica (ISR) list, and tunable safety via `acks=all` + `min.insync.replicas`.
- Stream processing systems process events in real time, computing partial results over the most-recent window. Use cases: credit-card fraud detection, network-intrusion detection, real-time route planning, trending topics. They complement (or replace) batch ETL pipelines. The **Lambda architecture** combines batch + speed + serving layers; the **Kappa architecture** treats the immutable log as the single source of truth and runs only stream processing.
- A streaming application is a **directed acyclic graph (DAG)** of processing nodes that ingest from sources (Kafka, S3, files), transform/aggregate, and write to sinks. Stateless streaming transforms individual events; stateful streaming maintains state across events (e.g., position windows, model parameters, hourly counts). Storm exposes the DAG explicitly via spouts and bolts with fieldsGrouping/globalGrouping; numTotalsBolts replicates a bolt across the cluster.
- **Apache Flink** offers higher-level functional APIs (DataStream, Table, SQL) and compiles user code into a dataflow DAG. Streams are typed (DataStream<T>). Operations include `map`, `keyBy`, `sum`, time windowing (sliding vs. tumbling — sliding emits results periodically over an overlapping window; tumbling has disjoint windows). Execution requires a StreamExecutionEnvironment; programs run lazily until `env.execute()`. Flink connects to Kafka, files, etc., with built-in connectors.
- Flink scales by specifying parallelism per operator (`setParallelism(n)`) or per environment, then mapping the logical DAG onto Task Managers (JVMs) with configurable task slots (`taskmanager.numberOfTaskSlots`, typically equal to CPU cores). A Job Manager coordinates the cluster, handles resource sharing, monitors node failures, and orchestrates recovery; HA configurations run multiple Job Managers as leader-follower. Operator chaining co-locates operators in a slot to minimize communication cost.
- Flink data safety uses persistent state storage (RocksDB by default via `state.backend`) plus periodic **stream barrier** checkpoints. The Job Manager injects barrier events into the source stream; stateful operators persist their state when barriers reach all inputs and propagate the barrier downstream. On failure Flink stops the entire app, restores state from the latest snapshot, and resumes the source from the barrier position. Checkpointing is disabled by default.
- The closing chapter ("Final Tips for Success") highlights four orthogonal essentials of scalable systems that the book does not deeply cover:
  - **Automation / DevOps**: continuous delivery and team ownership of deployment automation reduce the time between commit and production while preserving quality.
  - **Observability**: instrumented telemetry (OpenTelemetry, CloudWatch) feeds time-series data into Prometheus/Grafana/Graphite for dashboards, alerts, and forensic exploration.
  - **Deployment platforms**: containers (Docker) replace VMs as the deployment unit, orchestrated by Kubernetes or Apache Mesos; infrastructure-as-code (IaC) automates provisioning.
  - **Data lakes**: low-cost heterogeneous storage (S3, HDFS, ADLS) for historical and analytic data, with tiered storage classes to balance retrieval latency vs. cost.

## Relevant Concepts
- [[concepts/event-driven-architecture]] — loosely coupled architecture pattern around event emission.
- [[concepts/event-log]] — append-only, immutable record of events; foundational to Kafka/Kappa.
- [[concepts/log-compaction]] — Kafka mechanism for retaining only latest value per key.
- [[entities/apache-kafka]] — leading distributed persistent-log platform.
- [[entities/zookeeper]] — coordination service historically used for Kafka metadata.
- [[concepts/topic-partition]] — unit of Kafka parallelism within a topic.
- [[concepts/consumer-group]] — set of cooperating consumers sharing a topic's partitions.
- [[concepts/exactly-once-processing]] — delivery guarantee via idempotent producers + transactional consumers.
- [[concepts/in-sync-replica]] — Kafka's set of replicas eligible for leader election.
- [[concepts/leader-follower-replication]] — Kafka's per-partition replication model.
- [[concepts/stream-processing]] — real-time analytics on unbounded data streams.
- [[concepts/batch-processing]] — periodic processing of accumulated data.
- [[concepts/lambda-architecture]] — batch + speed + serving hybrid pattern.
- [[concepts/kappa-architecture]] — log-only streaming-first design pattern.
- [[concepts/dataflow]] — DAG-of-operators streaming computation model.
- [[concepts/stateful-stream]] — streaming app that maintains cross-event state.
- [[concepts/stateless-stream]] — streaming app that transforms events independently.
- [[concepts/sliding-window]] — overlapping time-windowed stream computation.
- [[concepts/tumbling-window]] — disjoint time-windowed stream computation.
- [[entities/apache-flink]] — high-throughput, low-latency open-source streaming engine.
- [[entities/apache-storm]] — earlier streaming platform with explicit spout/bolt topologies.
- [[concepts/stream-barrier]] — Flink's mechanism for consistent distributed checkpoints.
- [[concepts/checkpoint]] — periodic durable snapshot of streaming state for fault tolerance.
- [[concepts/devops]] — practices linking development and operations for fast safe releases.
- [[concepts/continuous-delivery]] — automated build/test/deploy pipeline practices.
- [[concepts/observability]] — instrumentation + analysis + alerting on system telemetry.
- [[concepts/infrastructure-as-code]] — programmatic provisioning of compute resources.
- [[concepts/container]] — lightweight isolated runtime unit (Docker).
- [[entities/kubernetes]] — container orchestration platform.
- [[concepts/data-lake]] — heterogeneous low-cost historical data repository.
- [[entities/prometheus]] — popular metrics collection and query system.
- [[entities/grafana]] — observability dashboarding tool.

## Source Metadata
- Source type: book chapter (concatenated Part IV: Chapters 14-16)
- Book title: Foundations of Scalable Systems
- Author: Ian Gorton
- Part/Chapters: Part IV, "Event and Stream Processing" — Chapter 14 Scalable Event-Driven Processing; Chapter 15 Stream Processing Systems; Chapter 16 Final Tips for Success
- File path: raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
