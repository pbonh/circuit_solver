---
title: Apache Storm
type: entity
id: entities/apache-storm
tags:
- streaming
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Overview

Apache Storm is an open-source distributed real-time computation system. Storm topologies are DAGs of spouts (data sources) and bolts (processing nodes) connected by streams. Storm was a pioneering streaming platform but has been largely superseded by Flink and Kafka Streams for new workloads.

## Characteristics

- Explicit topology construction in Java.
- Spouts connect to data sources (queues); bolts implement processing logic.
- fieldsGrouping, shuffleGrouping, and globalGrouping control inter-bolt routing.
- Multiple parallel instances per bolt via `setBolt(numTasks)`.

## Common Strategies

- Field-based grouping to ensure related events go to the same bolt instance.
- Trident API for higher-level abstractions and exactly-once semantics.

## Related Entities

- [[entities/apache-flink]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
