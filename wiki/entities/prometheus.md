---
title: Prometheus
type: entity
id: entity-prometheus
tags:
- observability
- monitoring
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Overview

Prometheus is an open-source time-series metrics collection and alerting system, originally developed at SoundCloud and now a CNCF graduated project. It pulls metrics from instrumented services on a regular interval and exposes a query language (PromQL) for ad-hoc analysis and alerting rules.

## Characteristics

- Pull-based scraping model.
- Multi-dimensional data model: metric name plus labels.
- PromQL for queries and alerting expressions.
- Local time-series database; long-term storage typically delegated.
- Alertmanager for routing and deduplication of alerts.

## Common Strategies

- Pair with Grafana for dashboards.
- Use exporters to scrape third-party systems.
- Federate Prometheus servers for very large deployments.

## Related Entities

- [[entities/grafana]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
