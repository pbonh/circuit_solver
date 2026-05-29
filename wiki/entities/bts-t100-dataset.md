---
title: BTS T100 Domestic Segment Dataset
type: entity
id: entity-bts-t100-dataset
tags:
- aviation
- dataset
- relational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables.txt
---

## Overview

The Bureau of Transportation Statistics (BTS) T100 Domestic Segment dataset (T100dm) is a monthly U.S. air traffic dataset reporting passenger, mail, and cargo volumes between airports. Unlike ATADS, which records per-airport totals, T100dm encodes segment-level relationships (origin + destination + carrier), enabling hub and segment analyses.

## Characteristics

- Monthly granularity, submitted by airlines.
- Columns covering origin and destination airport codes, carrier identifier, and P/C/M (passenger/cargo/mail) counts.
- Relational structure: not every airport pair has a route, so the connector-airport list depends on the selected hub.
- Supports analyses at multiple aggregation levels (hub-only, segment, carrier).

## Common Strategies

- Build a dashboard with three drop-downs (Hub, Connecting Airport, Carrier) whose contents cascade according to the selected analysis mode (H, S, SC, HC, C).
- Aggregate by month with `get_totals_by_month()` for histogram display.
- Expose the underlying filtered dataframe as a Dash `DataTable` with a built-in Export button for downstream Excel analysis.
- Apply a first-order trend line where useful.

## Related Entities

- [[entities/atads-dataset]]
- [[entities/dash]]
- [[entities/plotly]]
- [[entities/pandas]]

## Sources

- [[summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables]]
