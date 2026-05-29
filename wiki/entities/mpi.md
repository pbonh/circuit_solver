---
title: MPI (Message Passing Interface)
type: entity
id: entity-mpi
tags:
- distributed-systems
- parallel
- communication
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

MPI (Message Passing Interface) is the long-established standard for high-performance message-passing in distributed-memory parallel programs. Implementations include MPICH and OpenMPI. BigGraph@CUHK uses MPI for all inter-worker transport, layering its own object-level serialization on top of byte-level MPI primitives.

## Characteristics

- Each process has a rank (0..n-1); programs typically launch via `mpiexec`.
- Communication primitives include `MPI_Send`/`MPI_Recv` (point-to-point), `MPI_Alltoall`/`MPI_Alltoallv` (group), and broadcast/gather/scatter (`MPI_Bcast`, etc.).
- Data types include `MPI_INT`, `MPI_CHAR`, etc.; BigGraph@CUHK serializes everything to byte streams and transmits as `MPI_CHAR`.
- `MPI_Alltoall` is notoriously slow at scale; BigGraph@CUHK rolls its own all-to-all from point-to-point sends.

## Common Strategies

- Set up password-less SSH from master to all slaves for `mpiexec` distribution.
- Choose MPICH over OpenMPI for steadier per-superstep latency in BigGraph@CUHK.
- Hand-roll all-to-all when the built-in primitive does not scale.
- Treat MPI's point-to-point sends as the universal building block.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/pregel-plus]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
