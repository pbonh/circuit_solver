---
title: "Designing Data-Intensive Applications — Part II: Distributed Data (Chapters 5–9)"
type: summary
tags: [distributed-systems, replication, partitioning, consistency, concurrency, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Key Points

- Three motivations for distributing data across machines: scalability beyond one machine, fault tolerance / high availability, and lower latency for geographically distributed users. Shared-memory and shared-disk architectures hit cost and contention limits; **shared-nothing** is the dominant approach.
- Data is distributed via **replication** (multiple copies of the same data) and **partitioning/sharding** (splitting into disjoint subsets); they are independent mechanisms typically combined.
- Three replication architectures: **single-leader** (only one node accepts writes; followers replicate via log, statement, or row-based shipping), **multi-leader** (each datacenter or device has a leader; conflicts must be resolved), and **leaderless / Dynamo-style** (clients write to many replicas, read from many, use quorums `w + r > n`).
- Synchronous vs asynchronous replication trades durability against availability; semi-synchronous (one synchronous follower) is common. Failover from a failed leader is fraught: split brain, lost writes, ambiguous timeouts, and discarded writes.
- Replication lag produces user-visible anomalies: **stale reads**, **non-monotonic reads** ("time goes backward"), **causality violations** ("answer before question"). Mitigations: read-your-writes consistency (route reads of recent writes to the leader), monotonic reads (sticky replica per user), consistent prefix reads (partition by causally related keys, or track dependencies).
- Multi-leader replication conflict resolution: avoid conflicts by routing each record to one home leader; converge using LWW (lossy), higher-replica-ID (lossy), value merge, or explicit conflict storage. **CRDTs**, mergeable persistent data structures, and operational transformation are research areas for automatic conflict resolution.
- Leaderless replication uses **quorum reads/writes** with read repair and anti-entropy. With `w + r > n`, reads usually see the latest value, but edge cases (sloppy quorums with hinted handoff, concurrent writes, partial write success, replica restore from stale copy) can still return stale data.
- **Version vectors** and **dotted version vectors** (Riak) track per-replica per-key version numbers to distinguish concurrent writes from causally ordered writes, preventing silent data loss; LWW with synchronized clocks is fundamentally unsafe.
- **Partitioning** strategies: **key range** (efficient range scans, risk of hot spots like timestamp-keyed sensor data — used by Bigtable/HBase/RethinkDB/MongoDB) vs **hash of key** (uniform distribution, no range queries — used by Cassandra, Riak, Voldemort, MongoDB hashed-shard mode). Cassandra's compound key gives both.
- **Hot spots** caused by celebrity users or single hot keys need application-level mitigations (random prefix splitting writes across keys, then merging on read).
- Secondary index partitioning: **document-partitioned/local** (scatter-gather reads, single-partition writes — MongoDB, Riak, Cassandra, ES, SolrCloud, VoltDB) vs **term-partitioned/global** (single-partition reads, multi-partition writes, often async — DynamoDB GSIs, Riak search, Oracle).
- **Rebalancing**: never use `hash mod N` (most keys move when N changes). Use fixed partition count (Riak, ES, Couchbase), dynamic partitioning by splitting/merging (HBase, RethinkDB), or fixed partitions per node (Cassandra, Ketama, consistent hashing). Manual confirmation often safer than automatic.
- **Request routing**: routing tier, partition-aware clients, or any-node forwarding; consistent metadata via ZooKeeper (Espresso/Helix, HBase, SolrCloud, Kafka) or gossip protocols (Cassandra, Riak).
- **Transactions** group reads and writes into a logical unit. ACID = Atomicity (abortability of a partial sequence), Consistency (application invariants — actually the application's responsibility), Isolation (concurrent transactions don't interfere), Durability (committed writes survive crashes).
- Isolation hierarchy: read uncommitted (dirty reads allowed), **read committed** (no dirty reads/writes — Oracle/PostgreSQL default), **snapshot isolation / repeatable read** (MVCC, readers don't block writers, prevents read skew), **serializable** (prevents write skew and phantoms).
- Lost-update prevention: atomic update operations, explicit locks (SELECT FOR UPDATE), automatic conflict detection (PostgreSQL/Oracle/SQL Server but not MySQL InnoDB), compare-and-set, CRDTs for replicated stores.
- **Write skew** and **phantoms**: two transactions read a set of rows, each decides based on that read, then writes — but their writes invalidate each other's premise (on-call doctor example, meeting room booking). Only true serializable isolation prevents this.
- Serializability via three approaches: **actual serial execution** on a single thread (VoltDB, Redis, Datomic — requires stored procedures and in-memory data), **two-phase locking 2PL** (predicate or index-range locks, decades-old MySQL/SQL Server/DB2 default but slow), **serializable snapshot isolation SSI** (optimistic, used by PostgreSQL ≥9.1 and FoundationDB — detects stale-read and read-write conflicts).
- Distributed systems suffer **partial failures** that are nondeterministic. Networks are unreliable (lost/delayed packets), clocks drift and skew (time-of-day vs monotonic, NTP limits), processes pause (GC, virtualization, swap, page faults). Treat real-time guarantees as expensive exceptions.
- **Spanner's TrueTime API** uses GPS/atomic clocks plus explicit uncertainty intervals plus "commit wait" to produce timestamps consistent with causality across datacenters.
- A node cannot trust its own judgment — distributed algorithms rely on **quorums** (typically majorities) to declare nodes dead, elect leaders, and acquire locks. **Fencing tokens** (monotonically increasing IDs from a lock service) prevent a stale leaseholder from corrupting shared storage.
- **Byzantine fault tolerance** matters in aerospace and blockchain settings but is impractical for most server-side systems. Most consensus protocols assume crash-stop or crash-recovery faults, partial synchrony, and non-Byzantine behavior.
- **Linearizability** (recency) means the system behaves as if there were a single copy with atomic operations; needed for leader election, uniqueness constraints, cross-channel timing dependencies. Single-leader replication can be linearizable; consensus algorithms are; multi-leader and most leaderless setups are not.
- **CAP theorem** (Consistent or Available when Partitioned) is historically influential but narrowly scoped and often confused; better superseded by latency/utilization trade-offs.
- **Causality** defines a partial order on operations (happens-before); linearizability is total order. Causal consistency is the strongest available-under-partition consistency model, achievable via vector clocks/version vectors.
- **Lamport timestamps** (counter, node-id pair, with max-counter propagation) give a total order consistent with causality but are not sufficient to enforce uniqueness in real time — need **total order broadcast**.
- **Total order broadcast = consensus**. Equivalent problems include linearizable compare-and-set, atomic transaction commit, leader election, locks/leases with fencing.
- **Two-phase commit (2PC)** achieves atomic distributed transactions via prepare → vote → commit-or-abort by a coordinator, but is **blocking** when the coordinator fails (participants stay in-doubt holding locks). XA implements 2PC across heterogeneous systems (DBs + message brokers) but has heavy operational issues.
- **Fault-tolerant consensus** algorithms (Paxos, Raft, Zab, Viewstamped Replication, Multi-Paxos) implement total order broadcast with safety (uniform agreement, integrity, validity) and termination as long as a majority of nodes function. They use **epoch numbers** and **quorum overlap** to handle leader changes safely.
- **ZooKeeper, etcd, Consul**: coordination services exposing linearizable atomic operations, totally ordered fencing tokens, failure detection via heartbeats, ephemeral nodes, and change notifications. Used by HBase, Kafka, YARN, Nova, and many others for leader election, membership, configuration, and service discovery.

## Relevant Concepts

- [[concepts/replication]] — Single-leader, multi-leader, leaderless approaches.
- [[concepts/partitioning]] — Splitting datasets across nodes for scalability.
- [[concepts/consensus]] — Getting nodes to agree; foundational for fault tolerance.
- [[concepts/linearizability]] — Strongest single-object consistency; recency guarantee.
- [[concepts/causal-consistency]] — Strongest consistency available under partitions; partial order.
- [[concepts/eventual-consistency]] — Replicas converge eventually; weakest commonly offered.
- [[concepts/quorum]] — Majority-overlap voting for fault-tolerant decisions.
- [[concepts/leader-election]] — Choosing one node to coordinate writes; requires consensus.
- [[concepts/two-phase-commit]] — Blocking atomic commit across heterogeneous nodes.
- [[concepts/total-order-broadcast]] — Reliable delivery + total ordering; equivalent to consensus.
- [[concepts/lamport-timestamp]] — (counter, node-id) producing total order consistent with causality.
- [[concepts/version-vector]] — Per-replica per-key version tracking for concurrent-write detection.
- [[concepts/transaction]] — Grouping reads/writes into an atomic unit with safety guarantees.
- [[concepts/acid]] — Atomicity, Consistency, Isolation, Durability.
- [[concepts/snapshot-isolation]] — MVCC-based isolation where readers see a frozen snapshot.
- [[concepts/serializability]] — Strongest isolation; transactions appear to run one-at-a-time.
- [[concepts/two-phase-locking]] — Pessimistic serializability via shared/exclusive locks held until commit.
- [[concepts/serializable-snapshot-isolation]] — Optimistic serializability detecting conflicts at commit time.
- [[concepts/lost-update]] — Read-modify-write race condition; needs CAS, locks, or atomic updates.
- [[concepts/write-skew]] — Two transactions read overlapping sets and write disjoint rows that invalidate each other's premises.
- [[concepts/fencing-token]] — Monotonic ID issued by a lock service to prevent stale leaseholders from corrupting storage.
- [[concepts/byzantine-fault-tolerance]] — Tolerating arbitrary/malicious node behavior; needed for blockchains, aerospace.
- [[concepts/clock-skew]] — Variability between nodes' clocks; why time-of-day timestamps are unsafe for ordering.
- [[concepts/network-partition]] — Network fault isolating subset of nodes; central to CAP discussion.
- [[concepts/cap-theorem]] — "Consistent or Available when Partitioned"; historically influential framing.
- [[concepts/crdt]] — Conflict-free replicated data types for automatic multi-leader merging.
- [[concepts/fault-tolerance]] — Already created; central to all of Part II.
- [[concepts/backward-and-forward-compatibility]] — Already created; needed for rolling upgrades.
- [[entities/zookeeper]] — Coordination service implementing consensus.
- [[entities/etcd]] — Raft-based key-value coordination service.
- [[entities/spanner]] — Google's globally distributed DB using TrueTime for linearizable transactions.
- [[entities/cassandra]] — Already created; leaderless Dynamo-style store.
- [[entities/dynamodb]] — Amazon's hosted KV store (single-leader, not the original Dynamo).
- [[entities/postgresql]] — Already created; supports SSI.
- [[entities/mongodb]] — Already created; single-leader replication.
- [[entities/apache-kafka]] — Already created; uses ZooKeeper for coordination.
- [[entities/voltdb]] — In-memory serial-execution database.
- [[entities/raft]] — Understandable consensus algorithm (Ongaro & Ousterhout).
- [[entities/paxos]] — Lamport's classic consensus algorithm and family.

## Source Metadata

- Source type: book chapter (compilation of Chapters 5–9)
- Book title: Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems
- Author: Martin Kleppmann
- Chapters covered: 5 (Replication), 6 (Partitioning), 7 (Transactions), 8 (The Trouble with Distributed Systems), 9 (Consistency and Consensus)
- File path: `raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt`
- Publisher: O'Reilly Media, 2017
