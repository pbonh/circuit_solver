---
title: "BigGraph@CUHK"
type: entity
tags: [graph, distributed-systems, graph-processing, c++, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

BigGraph@CUHK is the C++ Pregel-family toolkit developed by Da Yan and colleagues at the Chinese University of Hong Kong. It comprises five systems — Pregel+, Blogel, Quegel, GraphD, and LWCP — built on a common foundation of MPI for transport and libhdfs for storage, shipped as a tree of C++ header files included into the user's program at compile time.

## Characteristics

- C++ header-only system design: each system is a folder of headers; users `#include` them and compile with `mpic++`.
- Shared utilities in `utils/`: data serialization (`serialization.h`), MPI-wrapped communication (`communication.h`), HDFS text interface (`ydhdfs.h` / `ydhdfs1.h` / `ydhdfs2.h`), global worker state (`global.h`).
- Pregel+ implements the basic Pregel model plus vertex mirroring (`ghost/`) and request-respond (`reqresp/`).
- Blogel adds block-centric APIs; Quegel adds query-centric APIs; GraphD adds out-of-core; LWCP adds lightweight checkpointing.
- Two-version compatibility: separate code branches for Hadoop 1.x and Hadoop 2.x (YARN) libhdfs.

## Common Strategies

- Use as a research or education vehicle: simpler than Giraph's heavy Java codebase, with all system internals exposed.
- Pick the right system: Pregel+ for general vertex-centric, Blogel for large-diameter, Quegel for online queries, GraphD for limited memory.
- Modify system headers freely (e.g., add tracing) since they compile in with user code.
- Combine with HDFS for durable storage and MPICH/OpenMPI for transport (MPICH preferred for stability).

## Related Entities

- [[entities/pregel-plus]]
- [[entities/blogel]]
- [[entities/quegel]]
- [[entities/graphd]]
- [[entities/g-thinker]]
- [[entities/mpi]]
- [[entities/hdfs]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
