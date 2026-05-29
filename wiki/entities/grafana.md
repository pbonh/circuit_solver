---
title: Grafana
type: entity
id: entities/grafana
tags:
- observability
- dashboarding
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Overview

Grafana is an open-source dashboarding and observability platform that visualizes metrics, logs, and traces from many backends (Prometheus, Graphite, InfluxDB, Elasticsearch, Loki, Tempo, CloudWatch, etc.). It is the de facto front-end for time-series monitoring stacks.

## Characteristics

- Pluggable data sources.
- Templated dashboards with variables and templating.
- Alerting on panel queries with multi-channel notifications.
- Grafana Cloud and Grafana Enterprise commercial offerings.

## Common Strategies

- Paired with Prometheus for metrics.
- Drill-down workflows between metrics dashboards and trace exploration.

## Related Entities

- [[entities/prometheus]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
