---
title: "Response-Time Percentiles"
type: concept
tags: [performance, well-established, distributed-systems, monitoring]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

Response time is what a client observes: service time plus network and queueing delays. Because response time varies request to request, it must be characterized as a distribution. Percentiles (p50/median, p95, p99, p999) describe the threshold below which that fraction of requests fall, while latency strictly refers to the time a request spends awaiting service.

## How It Works

- Sort observed response times and pick the value at the desired fraction. p99 = 1.5 s means 99% of requests are faster than 1.5 s.
- Higher percentiles describe tail latency, which disproportionately affects the most valuable users (e.g., Amazon found p99.9 customers had the largest accounts).
- For backend services that fan out, slow requests in any child slow the parent (tail latency amplification).
- Always measure on the client side, not the server, to capture queueing and head-of-line blocking.
- Load generators must keep sending requests without waiting for responses, or queues stay artificially short.
- Compute percentiles efficiently with forward decay, t-digest, or HdrHistogram; never average percentiles across machines or time windows — aggregate histograms instead.

## Key Parameters

- Reporting window length (e.g., rolling 10-minute window).
- Set of percentiles tracked (commonly p50, p95, p99, p999).
- SLO/SLA thresholds tied to percentile values.

## When To Use

Whenever performance affects user experience. Use percentiles instead of means for SLOs, alerts, monitoring dashboards, and capacity planning.

## Risks & Pitfalls

- Reporting mean response time hides outliers and gives a falsely optimistic picture.
- Averaging percentiles is mathematically meaningless — aggregate the underlying histograms.
- Optimizing very-high percentiles (p99.99) can be cost-prohibitive with diminishing returns.
- Synthetic load tests that wait for each response underestimate real tail latency.

## Related Concepts

- [[concepts/scalability]]
- [[concepts/reliability]]
- [[concepts/lsm-tree]]
- [[concepts/b-tree]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
