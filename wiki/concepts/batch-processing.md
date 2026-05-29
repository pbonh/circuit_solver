---
title: Batch Processing
type: claim
id: claim-batch-processing
tags:
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
---

## Definition

Batch processing periodically processes a large accumulated body of data — typically millions to billions of records — in a single job. Contrast with stream processing, which handles events one at a time. ETL pipelines are the classic example.

## How It Works

Files of new data accumulate; on a schedule (hourly, daily), a job reads, transforms, and loads the data into a target store. Apache Hadoop MapReduce was the historical canonical engine; today Spark and managed cloud services (BigQuery, EMR, Glue) are common.

## Key Parameters

- Batch interval.
- Batch size / record count.
- Cluster size for the job.

## When To Use

Workloads tolerating minute-to-hour latency for fresh results; ETL pipelines feeding analytics warehouses.

## Risks & Pitfalls

- Time-lag between data arrival and queryable result.
- Reprocessing entire batches when business rules change is expensive.

## Related Concepts

- [[concepts/stream-processing]]
- [[concepts/lambda-architecture]]
- [[concepts/data-lake]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
