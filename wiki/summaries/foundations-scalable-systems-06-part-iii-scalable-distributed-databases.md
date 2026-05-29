---
title: 'Foundations of Scalable Systems — Part III: Scalable Distributed Databases
  (Chapters 10–13)'
type: source
id: source-foundations-scalable-systems-06-part-iii-scalable-distributed-databases
kind: derived-summary
tags:
- distributed-systems
- replication
- consistency
- databases
- nosql
- newsql
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
---

## Key Points
- Relational databases scale up well (single-node deployment on increasingly large hardware) but face cost, availability, and growth ceilings; first-line scale-out tactics are **read replicas** (primary handles writes, asynchronous fan-out to read-only secondaries) and **data partitioning** (horizontal/sharding by row, or vertical/column splitting). SQL JOINs are intrinsically hard to distribute; common mitigations are reference-table replication, joining on partition keys, and selective filters.
- Shared-everything architectures like Oracle RAC up to 100 engines over a shared SAN illustrate one scale-out path but require expensive specialized hardware. The NoSQL movement instead embraces a shared-nothing architecture on commodity nodes.
- NoSQL databases share three core characteristics: simplified, evolvable data models; proprietary query languages with limited/no joins; native horizontal scaling. Modeling moves from problem-domain normalization to **solution-domain modeling** with denormalized, "table per use case" structures that prejoin data for read efficiency.
- Four main NoSQL data models: **key-value** (Redis, Oracle NoSQL), **document** (MongoDB, Couchbase — JSON/BSON with indexable fields), **wide column** (Cassandra, Bigtable — 2D hash map with named columns), and **graph** (Neo4j, Amazon Neptune — relationships are first-class). Graph databases resist sharding because partitioning a connected graph is hard; they typically scale up rather than out.
- Sharding strategies: hash-based (modulo or consistent hashing), value-based, and range-based. Replication on top of partitioning is needed for availability; leader-follower replication directs writes to a single leader, while leaderless replication accepts writes at any replica. Replica consistency is the central thorny issue.
- The **CAP theorem** says that during a network partition a system must choose between consistency (CP — reject writes that can't be propagated) or availability (AP — accept writes and reconcile later). In practice databases offer **tunable consistency** so this is a per-request, not per-system, decision.
- Eventual consistency permits an **inconsistency window** during which different replicas can return different values. Length depends on replica count, network load/distance, and operational glitches. **Read-Your-Own-Writes** (RYOWs) — guaranteeing a client sees its own update on subsequent reads — is a frequently needed guarantee weaker than full linearizability; MongoDB achieves it by default by reading from the primary, Neo4j via bookmark tokens.
- Tunable consistency uses parameters N (replicas), W (writes for success), R (reads). W=N favors consistency and read speed but slows writes and hurts availability. W=1 prioritizes write availability with longer inconsistency windows. **Quorum reads/writes** (W and R each greater than N/2) guarantee that read and write majorities overlap, so reads see the latest committed value — at the cost of failing when no majority is reachable.
- **Sloppy quorums and hinted handoff** (Dynamo, Cassandra, Riak) trade strict quorum for higher write availability: when home replicas are unreachable, a substitute node temporarily holds an update and later forwards it (hint) to the recovered replica. Increases write availability but allows stale reads.
- **Replica repair** combats entropy. Active/read repair runs on each read using digest hashes to detect divergence. Passive/anti-entropy repair runs periodically using **Merkle trees** (hash trees with object-level leaves and a compact root hash) to efficiently locate divergent ranges; implemented in Riak, Cassandra, ScyllaDB.
- Conflict resolution for concurrent writes: **last-writer-wins** via timestamps silently discards updates (clock-drift sensitive); **version vectors** generalize Lamport-clock-style logical clocks per replica to detect concurrent updates and surface "siblings" for application-level merge (Riak). **CRDTs** (conflict-free replicated data types — counters, sets, hashtables, lists, logs) enable automatic, mathematically guaranteed convergence; supported by Redis, Cosmos DB, Riak.
- Strong consistency = **serializability** (transactions equivalent to some sequential order — the C in ACID) + **linearizability** (single-object reads see the most recent committed write, ordered by wall-clock time). Strict serializability is the strongest model. Both require consensus algorithms.
- **Two-phase commit (2PC)** is the classic distributed-transaction consensus: prepare phase (each participant durably promises commit-or-abort) followed by resolve phase. 2PC is vulnerable to coordinator failure, where participants block holding locks until the coordinator recovers; this hurts availability and creates cascading-failure risk.
- Fault-tolerant consensus algorithms (**Paxos**, **Multi-Paxos**, **Raft**) make progress despite leader and follower failures. **Raft** is leader-based with a monotonically increasing term, AppendEntries heartbeats, log replication, and randomized election timers. New leaders must have all previously committed entries in their log. Used by Neo4j, YugabyteDB, etcd, Hazelcast.
- **VoltDB** is a strongly consistent NewSQL DB that partitions tables across single-threaded CPU cores with a per-partition Single Partition Initiator (SPI) serializing all access. Single-partition transactions commit without 2PC or locks; multi-partition transactions use a Multi-Partition Initiator running 2PC. Linearizability since v6.4 (Jepsen-validated). Persistence via command logs + periodic snapshots.
- **Google Cloud Spanner** provides external (strict serializability) consistency at global scale. Tables are sharded into splits, each a Paxos replica group with a long-lived leader. Multi-split transactions use 2PC where the coordinator is itself a Paxos group, removing the 2PC blocking weakness. Linearizability uses the **TrueTime** service (GPS + atomic clocks, ~7 ms bounded skew) plus a **commit wait** period so transactions see consistent timestamp ordering. Open-source descendants without TrueTime (CockroachDB, YugabyteDB) achieve weaker variants. Calvin/Fauna use deterministic transaction preprocessing as an alternative.
- **Redis** is an in-memory key-value/data-structure store (strings, lists, sets, sorted sets, hashes) with single-threaded event loop (multi-threaded networking in 6.0+). Persistence via periodic snapshots and append-only file (AOF). Redis Cluster shards across 16,384 hash slots (up to 1000 nodes), supports primary-replica with proprietary leader election, and tunable consistency via the WAIT command. Optimized for raw performance over data safety; multi-key "transactions" are not ACID.
- **MongoDB** is a schemaless JSON/BSON document database. WiredTiger storage engine (since v3.2) provides document-level locking, snapshot isolation, compression, and journaling. Shards via hash or range on a shard key; deployment uses mongod, mongos query routers (deployable per-app, per-shard, or dedicated), and config servers. Replica sets use Raft-based primary election, write concerns (default `majority`), read preferences, and (v4.0+) ACID multi-document transactions via 2PC + snapshot isolation. Achieves linearizable single-document reads with read concern `linearizable` + write concern `majority`. Storage chunks (64 MB default) are automatically migrated by a cluster balancer.
- **Amazon DynamoDB** is a fully managed NoSQL service that auto-shards, three-way replicates across availability zones, supports composite (partition + sort) primary keys, local and global secondary indexes, and on-demand or provisioned-capacity billing modes with autoscaling. APIs include classic CRUD, BatchGetItem/BatchWriteItem, and SQL-flavored PartiQL. ACID transactions consume 2x capacity per item. **Global tables** replicate multi-region with last-writer-wins; strongly consistent reads and transactions are scoped to a single region. SLA: 99.999% (global) / 99.99% (single region). Hot-key partitions cap throughput (3000 RCU / 1000 WCU).
- Database evaluation requires due diligence: read the docs, study the Jepsen analyses, and ideally build a proof-of-technology against your real workload. "All databases are scalable, but some are more scalable than others."

## Relevant Concepts
- [[concepts/distributed-database]] — umbrella term for partitioned/replicated data systems.
- [[concepts/read-replica]] — secondary that handles reads while a primary handles writes.
- [[concepts/sharding]] — partitioning of data across multiple nodes via a shard key.
- [[concepts/horizontal-partitioning]] — row-wise table split across nodes.
- [[concepts/vertical-partitioning]] — column-wise split (row splitting).
- [[concepts/shared-nothing-architecture]] — each node owns local storage; the NoSQL default.
- [[concepts/shared-everything-architecture]] — Oracle RAC-style cluster over a SAN.
- [[concepts/nosql]] — non-relational, horizontally scalable database family.
- [[concepts/key-value-store]] — primary NoSQL category; basis for Redis, DynamoDB.
- [[concepts/document-database]] — JSON-centric NoSQL (MongoDB, Couchbase).
- [[concepts/wide-column-store]] — Cassandra/Bigtable model.
- [[concepts/graph-database]] — relationship-first model (Neo4j); hard to partition.
- [[concepts/denormalization]] — solution-domain data modeling for NoSQL.
- [[concepts/cap-theorem]] — consistency vs. availability trade-off under partition.
- [[concepts/eventual-consistency]] — replicas converge eventually; default for most NoSQL.
- [[concepts/strong-consistency]] — single-node-style read-your-writes for all clients.
- [[concepts/linearizability]] — single-object real-time ordering of operations.
- [[concepts/serializability]] — transactional consistency; "C" in ACID.
- [[concepts/acid-transactions]] — atomicity, consistency, isolation, durability semantics.
- [[concepts/snapshot-isolation]] — common weaker transactional isolation level.
- [[concepts/read-your-own-writes]] — per-session consistency guarantee.
- [[concepts/tunable-consistency]] — N/W/R parameters per request.
- [[concepts/quorum]] — majority-based replica protocol.
- [[concepts/sloppy-quorum]] — Dynamo-style availability tweak using hinted handoff.
- [[concepts/hinted-handoff]] — temporary holder forwarding updates to recovered replicas.
- [[concepts/anti-entropy-repair]] — proactive replica reconciliation.
- [[concepts/merkle-tree]] — hash tree enabling efficient replica comparison.
- [[concepts/last-writer-wins]] — timestamp-based conflict resolution; lossy.
- [[concepts/version-vector]] — per-replica logical clocks for conflict detection.
- [[concepts/logical-clock]] — Lamport's happens-before counter.
- [[concepts/crdt]] — conflict-free replicated data types with automatic merge.
- [[concepts/consensus]] — distributed agreement protocols.
- [[concepts/two-phase-commit]] — classic distributed transaction protocol with blocking weakness.
- [[concepts/raft]] — leader-based fault-tolerant consensus algorithm.
- [[concepts/paxos]] — leaderless fault-tolerant consensus algorithm.
- [[concepts/leader-election]] — protocol step that picks a new coordinator after failure.
- [[concepts/truetime]] — Spanner's bounded-uncertainty time service.
- [[concepts/replication]] — duplication of state for availability and read scaling.
- [[concepts/leader-follower-replication]] — single-leader replication model.
- [[concepts/leaderless-replication]] — any-replica-accepts-writes model.
- [[entities/redis]] — in-memory KV/data-structure store.
- [[entities/mongodb]] — document database with WiredTiger and Raft-based replica sets.
- [[entities/dynamodb]] — AWS-managed NoSQL service.
- [[entities/cassandra]] — wide-column store cited as Netflix's user-content backbone.
- [[entities/voltdb]] — NewSQL in-memory database with single-threaded partitions.
- [[entities/cloud-spanner]] — Google's globally distributed strongly consistent SQL DB.
- [[entities/neo4j]] — graph database with Raft-based clustering.
- [[entities/cockroachdb]] — Spanner-inspired open source NewSQL DB.
- [[entities/yugabytedb]] — distributed SQL DB using Raft for consensus.
- [[concepts/newsql]] — class of distributed strongly consistent SQL databases.

## Source Metadata
- Source type: book chapter (concatenated Part III: Chapters 10-13)
- Book title: Foundations of Scalable Systems
- Author: Ian Gorton
- Part/Chapters: Part III, "Scalable Distributed Databases" — Chapter 10 Scalable Database Fundamentals; Chapter 11 Eventual Consistency; Chapter 12 Strong Consistency; Chapter 13 Distributed Database Implementations
- File path: raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
