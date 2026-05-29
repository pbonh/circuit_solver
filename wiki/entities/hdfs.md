---
title: HDFS (Hadoop Distributed File System)
type: entity
id: entity-hdfs
tags:
- distributed-systems
- big-data
- storage
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

HDFS is the distributed file system of the Apache Hadoop ecosystem. It abstracts a pool of machine disks as a single logical file system; each file is broken into 64 MB blocks (configurable) and each block is replicated to multiple machines (default replication factor 3) for fault tolerance. Cluster-wide aggregate read/write bandwidth scales with the number of nodes.

## Characteristics

- libhdfs provides a JNI-based C API (`hdfs.h`) so non-Java systems like BigGraph@CUHK can read/write HDFS.
- API differences between Hadoop 1.x and 2.x (YARN) require separate library bindings (e.g., the new `recursive` argument in `hdfsDelete`, and `hdfsBuilderConnect` replacing `hdfsConnect`).
- Provides only a binary stream interface; text record delimitation must be layered on top (BigGraph@CUHK's `LineReader` and `LineWriter`).
- Big-file uploads must respect block alignment (a line cannot span two files for parallel reads), motivating the `put` helper that breaks files into ~8 MB chunks.

## Common Strategies

- Store both input and output of distributed Big Data jobs on HDFS to enjoy fault tolerance and aggregate disk bandwidth.
- Set `LD_LIBRARY_PATH` and `CLASSPATH` to point at libhdfs binaries and Hadoop jars.
- Choose between Hadoop 1.x and 2.x (YARN) libhdfs based on cluster availability.
- For pseudo-distributed local development, run HDFS in single-node mode.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/apache-giraph]]
- [[concepts/mapreduce]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
