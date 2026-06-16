---
title: "Foundations of Scalable Systems"
type: source
slug: foundations-scalable-systems
created: 2026-06-16
updated: 2026-06-16
summary: O'Reilly practical guide to scalable distributed system design — covering concurrency, caching, messaging (Kafka), microservices, NoSQL databases (Redis/MongoDB/DynamoDB), and stream processing (Flink).
source_file: Books/Foundations of Scalable Systems
tags: [distributed-systems, scalability, kafka, microservices, nosql, consistency, stream-processing]
status: active
---

# Foundations of Scalable Systems

- **Source file:** `sources/Books/Foundations of Scalable Systems/`
- **Author / origin:** Ian Gorton; O'Reilly, 2022
- **Date:** 2022

## Summary

A practitioner's guide to scalable distributed system design, covering foundational concepts (scalability, reliability, availability), infrastructure (threads, caching, messaging), databases (NoSQL, consistency), and event/stream processing. Aimed at software engineers building production distributed systems.

### Part I: The Basics

**Scalability principles**: Scale-up (vertical) vs. scale-out (horizontal). Load (requests/sec, active users, data volume), throughput (requests/sec processed), latency (p50, p99, p999), availability (9s). Fundamental tradeoff: performance vs. availability vs. consistency.

**Distributed systems essentials**: TCP/UDP, RPC (stub + marshalling), partial failures (nodes fail independently), consensus (Paxos/Raft — needed when all nodes must agree), clock synchronization (NTP drift, monotonic clocks, logical clocks).

**Concurrency**: Threads, race conditions, deadlocks, thread pools, barrier synchronization, thread-safe collections. Java-centric examples but concepts are general.

### Part II: Scalable Systems

**Application services**: Stateless vs. stateful services; API design (REST, gRPC); horizontal scaling behind load balancer; health monitoring; session affinity; elasticity (auto-scaling).

**Distributed caching**: Read-through/write-through/write-behind caches; cache invalidation; Cache-Control headers; Redis as distributed cache (in-memory key-value with TTL).

**Asynchronous messaging (RabbitMQ)**: Message queues, exchanges (direct/topic/fanout), competing consumers, exactly-once processing challenges, dead-letter queues, durability vs. throughput trade-offs.

**Serverless**: Google App Engine autoscaling, AWS Lambda lifecycle, cost/performance trade-offs for bursty workloads.

**Microservices**: Decomposing monoliths; service boundaries; cascading failures (circuit breaker pattern); bulkhead pattern; resilience.

### Part III: Scalable Distributed Databases

**CAP theorem**: Consistency vs. availability under network partition. CP (consistent + partition tolerant) vs. AP (available + partition tolerant) systems.

**Eventual consistency**: Inconsistency window; read-your-own-writes; tunable consistency (Cassandra quorums); vector clocks for conflict detection; last-writer-wins.

**Strong consistency**: Two-phase commit (2PC); distributed consensus (Raft — leader election, log replication, fault tolerance); Google Spanner (TrueTime for global ordering), VoltDB (serializable isolation via single-threaded execution).

**NoSQL implementations**:
- Redis: in-memory key-value; pub/sub; persistence (RDB snapshots, AOF log); replication (master-replica)
- MongoDB: document store; sharding by shard key; replica sets; eventual vs. causal consistency
- DynamoDB: wide-column; partition key + sort key; consistent/eventually consistent reads; global tables

### Part IV: Event and Stream Processing

**Apache Kafka**: Distributed log for event streaming. Topics (partitioned, replicated), producers (key-based partitioning for ordering), consumers (consumer groups for parallelism), offset management, compacted logs. Guarantees: at-least-once, exactly-once (transactional API).

**Apache Flink stream processing**: DataStream API; stateful computations (keyed state, operator state); checkpointing for fault tolerance; windowing (tumbling, sliding, session); watermarks for event-time processing.

## Key takeaways

- Scalability = ability to add capacity proportionally to load; horizontal scaling + statelessness enables this
- Eventual consistency is the default in distributed systems; strong consistency requires explicit design choices and has performance costs
- Kafka's partitioned log enables both pub/sub messaging and persistent event replay — ideal for large-scale EDA result pipelines
- Flink's stateful stream processing enables complex event patterns on simulation result streams
- Raft consensus is the modern standard for distributed agreement (etcd, CockroachDB, TiKV all use it)
- Connection to circuit simulation: large-scale simulation farms produce event streams that Kafka/Flink can process in real-time

## Pages updated from this source

- [[scalable-distributed-systems]] - topic created
- [[data-analysis-tooling]] - Kafka/Flink as simulation result pipeline infrastructure
