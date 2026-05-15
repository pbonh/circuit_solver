---
title: "Pregel+"
type: entity
tags: [graph, distributed-systems, graph-processing, pregel, c++, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

Pregel+ is the core Pregel-like system in the BigGraph@CUHK toolkit, written in C++ on top of MPI and libhdfs. It provides the basic Pregel API plus two message-reduction extensions: vertex mirroring (for combiner-applicable algorithms) and a request-respond API (for pointer-jumping algorithms like S-V). The system is structured as four libraries — `utils`, `basic`, `ghost`, `reqresp` — included as C++ headers.

## Characteristics

- C++ implementation; uses MPI for transport (mpic++/mpiexec) and libhdfs for storage.
- Vertex base class templated on KeyT, ValueT, MessageT, HashT; Worker base class with `toVertex` / `toline` UDFs.
- Serialization via overloaded `<<` / `>>` operators on `ibinstream` / `obinstream`; recursive (de)serialization for STL containers.
- Implements its own all-to-all communication on top of MPI point-to-point primitives (MPI_Alltoall is slow at scale).
- Vertex mirroring: high-degree vertices replicated on each machine that has a neighbor; degree threshold proven to minimize messages.
- Request-respond API: vertex v issues request to r; each machine combines local requests, r responds once per machine.
- Hadoop 1.x and 2.x supported via separate libhdfs wrappers (`ydhdfs1.h`, `ydhdfs2.h`).

## Common Strategies

- Use Pregel+ for C++ workloads needing better performance than Java-based Giraph.
- Apply vertex mirroring for high-degree-skewed graphs with combiner-applicable algorithms.
- Apply request-respond for pointer-jumping and arbitrary-vertex communication patterns.
- Treat the system header tree as part of your code: insert printf statements into headers when debugging.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/pregel]]
- [[entities/mpi]]
- [[entities/hdfs]]
- [[entities/blogel]]
- [[entities/graphd]]
- [[entities/quegel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
