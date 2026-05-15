---
title: "Observability"
type: concept
tags: [foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: high
---

## Definition

Observability is the ability to infer a system's internal state from its external outputs — metrics, logs, and traces. Strong observability lets operators diagnose unexpected failures and performance issues that pre-built dashboards cannot anticipate.

## How It Works

Three telemetry "pillars": metrics (time-series counters/gauges/histograms), logs (event records), and traces (distributed request flows across services). Instrumentation libraries (OpenTelemetry, AWS CloudWatch) emit telemetry; backends (Prometheus, Grafana, Graphite, Honeycomb) aggregate and query. Alerts fire on threshold breaches.

## Key Parameters

- Cardinality budget for metric labels.
- Sample rate for traces.
- Log retention.

## When To Use

Required for any production-grade distributed system.

## Risks & Pitfalls

- Cardinality explosions kill metric backends.
- "Dashboards everywhere" without actionable alerts.

## Related Concepts

- [[concepts/devops]]
- [[concepts/long-tail-latency]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
