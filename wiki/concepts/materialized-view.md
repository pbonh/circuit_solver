---
title: Materialized View
type: claim
id: claim-materialized-view
tags:
- data-warehouse
- well-established
- query-performance
- caching
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.65
---

## Definition

A materialized view is a precomputed query result stored to disk and refreshed when the underlying data changes, in contrast to a virtual view that is expanded inline at query time. In data warehouses, a common form is the **data cube** (OLAP cube) — a grid of aggregates over multiple dimensions.

## How It Works

- The view definition is a query (often a join + aggregation). The database executes it once and persists the result.
- When underlying data changes, the view must be updated — incrementally if possible, or fully re-materialized.
- Data cubes precompute aggregates (SUM, COUNT, AVG) along chosen dimensions, then "roll up" by summing along axes; queries that hit a precomputed cell are very fast.
- Materialized views are common in read-heavy data warehouses; rare in OLTP because the maintenance overhead penalizes writes.

## Key Parameters

- Refresh strategy: full vs incremental, synchronous vs scheduled.
- Set of dimensions/aggregates precomputed.
- Storage cost vs query speedup tradeoff.

## When To Use

For frequently-run heavy aggregate queries with stable shape. For dashboards that hit known summaries. As a stepping stone toward derived-data systems (DDIA Part III).

## Risks & Pitfalls

- Materialized views are denormalized copies — consistency lags real data.
- Data cubes have fixed dimensions; ad-hoc queries on non-precomputed dimensions get no benefit.
- Maintenance writes amplify storage I/O and can interfere with OLTP if shared.
- Stale views silently produce wrong answers if refresh fails.

## Related Concepts

- [[concepts/data-warehouse]]
- [[concepts/column-oriented-storage]]
- [[concepts/star-schema]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-05-part-iii-derived-data]]
