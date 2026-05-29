---
title: 'Foundations of Scalable Systems — Part II: Scalable Systems (Chapters 5–9)'
type: source
id: source-foundations-scalable-systems-05-part-ii-scalable-systems
kind: derived-summary
tags:
- distributed-systems
- scalability
- microservices
- caching
- messaging
- serverless
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
---

## Key Points
- A scalable application's service tier is shaped around APIs (predominantly HTTP/CRUD using POST/GET/PUT/DELETE, optionally PATCH), stateless request handlers, and application-server runtimes (Tomcat/JEE, Express.js, Flask, Go net/http). API design pitfalls include "chatty APIs"; compression (gzip) on large payloads pays back its CPU cost.
- HTTP is technically stateless, but apps often layer conversational state on top. Stateful services don't scale: memory grows with sessions, timeout tuning is fragile, and load-imbalance is inevitable. Stateless services + an external session store (Redis/memcached, Spring Session) are the recommended pattern for scale.
- Tomcat-style application servers run a thread pool (default 25-200 in Tomcat), with a sockets backlog and a database connection pool. Tuning these pools is essential because systems degrade well before 100% utilization due to context switching and GC. JMX/MBeans enable observability via JConsole/JavaMelody.
- Horizontal scaling = stateless replicas + load balancer; capacity scales with replica count, and availability is enhanced because a failed replica's requests can simply retry elsewhere. Load balancers operate at L4 (network, TCP/UDP NAT) — fast — or L7 (application, HTTP-aware) — richer but slower; AWS testing showed ~20% throughput gap at low load that disappears once backend becomes the bottleneck.
- Load-balancer features: distribution policies (round-robin, least-connections, header/verb-based, weighted), health checks, **elasticity** (Auto Scaling groups with min/max bounds, schedule- or metric-driven), and session affinity / sticky sessions. Sticky sessions cause load imbalance at large scale.
- Application caching (cache-aside pattern with Redis/memcached) intercepts read traffic ahead of databases. Patterns include cache-aside, read-through, write-through, and write-behind (write-back). Twitter dedicates ~3% of infrastructure to application-level caches. Key design knobs: TTL, eviction policy (LRU, LFU), and hit-rate vs. update-rate trade-off; monitor get_hits/get_misses/evictions.
- Web caching uses HTTP infrastructure: browser/private caches, ISP proxy caches, organization caches, reverse-proxy caches, and CDN edge caches (e.g., Akamai delivers up to 30% of internet traffic from 2000+ POPs). HTTP directives — Cache-Control (public/private/no-store/no-cache/max-age), Expires, Last-Modified, and ETag with If-None-Match — drive freshness and revalidation (304 Not Modified).
- Asynchronous messaging decouples producers from consumers via brokers (queues, topics, exchanges). Producers and consumers can scale independently; consumer message retrieval can be pull (polling, inefficient) or push (broker invokes a callback). Persistent queues + manual ack + publisher confirms = full data safety at the cost of latency.
- RabbitMQ exposes connections (heavyweight, long-lived), multiplexed channels (created per-thread or pooled — channels are not thread-safe), and exchanges (direct, topic, fanout) that route via routing keys and bindings. Each queue runs on a single broker thread, so achieving multicore throughput requires multiple queues; broker throttles producers at ~40% memory. High availability via mirrored queues or quorum queues (RAFT-based; quorum queues are the future direction).
- Key messaging patterns: **competing consumers** (multi-consumer parallelism with built-in failover/load-balance), **exactly-once processing** (idempotency keys on both publisher and consumer sides via brokers like ActiveMQ Artemis), and **poison messages** (cap redelivery count via dead-letter queues, e.g., SQS maxReceiveCount).
- Serverless platforms (AWS Lambda, Google App Engine, Azure Functions, Apache OpenWhisk) eliminate explicit provisioning: code is uploaded, the platform scales it elastically on demand and bills per invocation/ms. Cold-start cost depends on language (Go/Node.js: hundreds of ms; Java/.NET: 1-3 s) and is mitigated via "provisioned concurrency" / minimum-instance settings.
- Lambda scales by instance count (per-region burst limit, e.g., 3,000 in us-west-2, 1,000 in eu-central-1; +500/min after that) and bills proportional to memory (128 MB-10 GB), where more memory also buys more CPU. GAE autoscaling has three interacting parameters (target_cpu_utilization, target_throughput_utilization, max_concurrent_requests) with non-obvious interactions — a parametric study on a Go+Firestore workload showed 96% of default throughput at 55% of default cost via {CPU70, max80}.
- Microservices decompose an application into fine-grained, independently deployable services aligned to bounded contexts (DDD). They tame monolithic code-base growth and enable per-service scale-out, but introduce distributed-system concerns. The "two-pizza" team rule frames team size, not service size.
- API gateways (Kong, NGINX Plus, AWS API Gateway) front microservices: routing, authn/z, throttling, caching, and observability. Workflows that span services are implemented via **orchestration** (centralized engine, e.g., Netflix Conductor) or **choreography** (peer-to-peer, often pub/sub).
- Microservice resilience patterns:
  - **Fail fast** via client read-timeouts and server-side throttling (HTTP 503) — preferred over open-ended waits because long-tail responses (P99 can be 15x median) starve thread pools.
  - **Circuit breaker** (CLOSED -> OPEN -> HALF_OPEN, e.g., Python `circuitbreaker`, Resilience4j) cuts off requests to an unhealthy dependency for a cooldown window, preventing cascading failures.
  - **Bulkhead** (Resilience4j, Spring Boot @Bulkhead) reserves thread-pool capacity per API so that one hot endpoint cannot starve others. Inspired by ship hull partitions.
- Cascading failures are insidious because slow (not failed) downstream services tie up upstream threads; immediate retries make it worse, so use exponential backoff and combine with circuit breakers. Default fallback responses can hide transient failures (e.g., "shows you might like" instead of a personal watchlist).

## Relevant Concepts
- [[concepts/rest]] — predominant API style; HTTP+JSON CRUD.
- [[concepts/openapi]] — formal HTTP-API specification (YAML, SwaggerHub).
- [[concepts/stateless-service]] — necessary for effective horizontal scaling.
- [[concepts/session-state]] — what stateless services must externalize to a store.
- [[concepts/load-balancing]] — covering L4 vs. L7, distribution policies, health checks, elasticity, sticky sessions.
- [[concepts/horizontal-scaling]] — stateless replicas behind a load balancer.
- [[concepts/elastic-scaling]] — dynamic capacity adjustment based on metrics or schedule.
- [[concepts/auto-scaling-group]] — AWS abstraction for elastic capacity with min/max.
- [[concepts/distributed-cache]] — Redis/memcached layer between services and databases.
- [[concepts/cache-aside]] — primary application-caching pattern.
- [[concepts/read-through-cache]] — application reads via cache, which loads on miss.
- [[concepts/write-through-cache]] — application writes via cache, which synchronously persists.
- [[concepts/write-behind-cache]] — async-persisting cache; faster writes but risk lost updates.
- [[concepts/cdn]] — geographically distributed edge caches for media-heavy traffic.
- [[concepts/http-caching]] — Cache-Control/Expires/Last-Modified/ETag mechanisms.
- [[concepts/etag]] — opaque revalidation token enabling 304 responses.
- [[concepts/asynchronous-messaging]] — producer/consumer decoupling via a broker.
- [[concepts/message-broker]] — service that manages queues, topics, and routing.
- [[concepts/message-queue]] — FIFO buffer with point-to-point delivery.
- [[concepts/publish-subscribe]] — topic-based one-to-many event distribution.
- [[concepts/message-persistence]] — disk-backed queues for data safety.
- [[entities/rabbitmq]] — open-source broker built on AMQP/Erlang.
- [[concepts/competing-consumers]] — parallelism pattern over a single queue.
- [[concepts/exactly-once-processing]] — combination of dedupe at producer and consumer.
- [[concepts/idempotency]] — required for safe retries; uses idempotency keys.
- [[concepts/poison-message]] — message that cannot be processed and must be sidelined.
- [[concepts/dead-letter-queue]] — destination for messages that exceed redelivery limits.
- [[concepts/serverless]] — function-as-a-service execution model.
- [[concepts/cold-start]] — initial-invocation latency on serverless platforms.
- [[concepts/provisioned-concurrency]] — preallocated runtime instances to mitigate cold starts.
- [[entities/aws-lambda]] — Amazon's FaaS platform.
- [[entities/google-app-engine]] — Google's PaaS/FaaS platform with autoscaling.
- [[concepts/microservices]] — fine-grained, independently deployable services.
- [[concepts/monolithic-architecture]] — the alternative pattern microservices replace.
- [[concepts/domain-driven-design]] — bounded contexts inform microservice boundaries.
- [[concepts/api-gateway]] — front-door for microservice ecosystems.
- [[concepts/orchestration]] — centralized workflow control across services.
- [[concepts/choreography]] — peer-to-peer event-driven workflow style.
- [[concepts/cascading-failure]] — chain reaction of overload across coupled services.
- [[concepts/fail-fast]] — bound waits via timeouts and throttling.
- [[concepts/circuit-breaker]] — client-side guard that trips on dependency errors.
- [[concepts/bulkhead-pattern]] — resource isolation via per-API thread reservation.
- [[concepts/long-tail-latency]] — tail of slow requests captured by P95/P99 percentiles.
- [[concepts/throttling]] — rate limiting to protect downstream services.
- [[concepts/backpressure]] — propagation of slow consumption upstream through a system.

## Source Metadata
- Source type: book chapter (concatenated Part II: Chapters 5-9)
- Book title: Foundations of Scalable Systems
- Author: Ian Gorton
- Part/Chapters: Part II, "Scalable Systems" — Chapter 5 Application Services; Chapter 6 Distributed Caching; Chapter 7 Asynchronous Messaging; Chapter 8 Serverless Processing Systems; Chapter 9 Microservices
- File path: raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
