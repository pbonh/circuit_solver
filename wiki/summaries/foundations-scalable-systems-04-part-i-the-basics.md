---
title: 'Foundations of Scalable Systems — Part I: The Basics (Chapters 1–4)'
type: source
id: source-foundations-scalable-systems-04-part-i-the-basics
kind: derived-summary
tags:
- distributed-systems
- scalability
- concurrency
- networking
- replication
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
---

## Key Points
- Scalability is defined as a software system's capability to handle growth in some operational dimension (request volume, data, response-time stability, derived value), and is achieved primarily via two complementary principles: **replication** (more parallel processing paths) and **optimization** (better use of existing resources).
- "Hyperscale" systems exhibit exponential growth in capability while costs grow only linearly; achieving this requires designing for scale from the start, because retrofitting can be ruinously expensive (HealthCare.gov vs. Oregon's failed exchange).
- Scalability trades off against other quality attributes: performance, availability, security (TLS, encryption-at-rest add 5-10% overhead), and manageability (observability, DevOps). Performance helps scalability, but in-memory optimizations can hurt it; security is generally an opposing force.
- A canonical scalable web architecture evolves through stages: monolith → scale-up → horizontal scale-out with a load balancer and stateless services → distributed cache (Redis/memcached) → distributed database (SQL or NoSQL) → multiple tiers / Backend-for-Frontend → asynchronous queues for write responsiveness.
- Stateless services are the prerequisite for load-balanced horizontal scaling; session state must live in an external store so any replica can serve any request.
- Amdahl's law caps achievable speedup with parallelism: with 5% serial code, ~2,048 cores is the practical ceiling; with 50% serial, ~8 cores. Efficient multithreading is a hard prerequisite for scalability.
- The internet's communications fabric is heterogeneous (LAN/WAN/WiFi/cellular) with very different bandwidths and latencies; light-speed limits and routers introduce significant cross-continent latency (e.g., NYC→Sydney ~80 ms fiber). DNS, IP, TCP, UDP each contribute distinct semantics; TCP is reliable, connection-oriented, stream-oriented but heavyweight, while UDP is fast and unreliable.
- RPC/RMI (DCE, CORBA, Java RMI, XML web services, gRPC) provides location-transparent client/server invocation via stub/skeleton marshalling, but modern systems usually prefer HTTP+JSON/REST; the underlying network failure modes leak through all of them.
- Partial failures (crash faults) are unavoidable: clients cannot distinguish lost requests, lost responses, slow servers, or dead servers. The mitigation is **idempotency**: clients attach a unique idempotency key, the server stores keys and rejects duplicates so retries are safe. Exactly-once semantics in APIs is achieved by combining at-least-once TCP delivery with idempotent handlers, typically via transactional updates of both state and idempotency key.
- The FLP impossibility theorem states that consensus on an asynchronous network with crash faults cannot be guaranteed in bounded time; in practice, sensible timeouts and retries make consensus achievable. Byzantine faults (malicious participants) are usually excluded inside trusted enterprise boundaries.
- Clocks on different nodes drift (10-20 s/day is typical) and even with NTP synchronization (millisecond accuracy on a LAN) cross-node timestamps cannot be used to determine event order; this motivates logical-clock and consensus mechanisms covered later. Two clocks matter per node: time-of-day (can jump backward after NTP sync) and monotonic (only forward).
- Threads on a single node fully exploit multicore CPUs but introduce **race conditions** (lost updates because increments aren't atomic at the machine level) and **deadlocks** (circular waits, as in dining philosophers). Java solves these with synchronized blocks/monitor locks, java.util.concurrent primitives (BlockingQueue, CountDownLatch, ConcurrentHashMap), thread pools (ExecutorService), and barrier synchronization (CountDownLatch, CyclicBarrier, Phaser).
- Different languages adopt different concurrency models — Go's CSP/channels, Erlang's actors, Node.js's single-threaded event loop — but the underlying problems (correctness under nondeterministic interleaving, avoiding deadlock) are universal.

## Relevant Concepts
- [[concepts/scalability]] — central quality attribute; replication and optimization are its two pillars.
- [[concepts/horizontal-scaling]] — adding service replicas behind a load balancer; the default scale-out tactic.
- [[concepts/vertical-scaling]] — scaling up to bigger hardware; simple, limited, and expensive.
- [[concepts/load-balancing]] — reverse-proxy that distributes requests across stateless replicas.
- [[concepts/stateless-service]] — service-design constraint that makes horizontal scaling tractable.
- [[concepts/distributed-cache]] — in-memory layer (Redis, memcached) for offloading read-heavy DB work.
- [[concepts/replication]] — duplication of services and data for capacity and availability.
- [[concepts/availability]] — quality attribute that scales naturally with replication when state is read-only.
- [[concepts/amdahls-law]] — bounds achievable speedup by the serial fraction of computation.
- [[concepts/partial-failure]] — the defining characteristic of distributed systems' failure modes.
- [[concepts/idempotency]] — mandatory property for mutating APIs subject to retries.
- [[concepts/consensus]] — agreement protocol problem that is impossible in bounded time on asynchronous networks (FLP).
- [[concepts/two-generals-problem]] — illustrative impossibility result for agreement over lossy channels.
- [[concepts/byzantine-faults]] — malicious-actor failure model; usually excluded for enterprise systems.
- [[concepts/clock-drift]] — the reason cross-node timestamps cannot order events reliably.
- [[concepts/ntp]] — practical time-synchronization protocol; bounds but does not eliminate drift.
- [[concepts/rpc]] — remote procedure call paradigm and its evolution (DCE, CORBA, RMI, gRPC).
- [[concepts/rest]] — modern HTTP/JSON alternative to RPC.
- [[concepts/tcp]] — reliable connection-oriented transport.
- [[concepts/udp]] — lightweight connectionless transport with at-most-once semantics.
- [[concepts/concurrency]] — single-node parallelism enabling overlap with I/O and multicore use.
- [[concepts/thread]] — basic concurrency primitive in mainstream languages.
- [[concepts/race-condition]] — lost-update bug caused by nondeterministic interleavings on shared state.
- [[concepts/deadlock]] — circular-wait blocking; dining philosophers is the canonical illustration.
- [[concepts/thread-pool]] — bounded executor managing reusable worker threads.
- [[concepts/barrier-synchronization]] — wait-for-all coordination via CountDownLatch / CyclicBarrier.
- [[concepts/monolithic-architecture]] — starting point most scalable systems evolve away from.
- [[concepts/backend-for-frontend]] — pattern of dedicated services per client channel.
- [[concepts/asynchronous-messaging]] — write-side decoupling for responsiveness.
- [[concepts/hyperscale]] — exponential capability growth on linear cost.
- [[entities/aws]] — exemplar cloud provider used throughout the book.

## Source Metadata
- Source type: book chapter (concatenated Part I: Chapters 1-4)
- Book title: Foundations of Scalable Systems
- Author: Ian Gorton
- Part/Chapters: Part I, "The Basics" — Chapter 1 Introduction to Scalable Systems; Chapter 2 Distributed Systems Architectures: An Introduction; Chapter 3 Distributed Systems Essentials; Chapter 4 An Overview of Concurrent Systems
- File path: raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
