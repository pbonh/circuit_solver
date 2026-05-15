---
title: "Star Schema"
type: concept
tags: [data-warehouse, well-established, olap, dimensional-modeling]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

The star schema (also known as dimensional modeling, popularized by Ralph Kimball) is a relational schema pattern used in data warehouses. A central **fact table** records individual events (sales, page views), with rows containing numeric attributes plus foreign keys to **dimension tables** describing the who/what/where/when/how/why of each event.

## How It Works

- Fact table rows correspond to atomic events; in retail, one row per item sold. Numeric columns (quantity, price, cost) are aggregated. Foreign keys reference dimensions: product, store, customer, date, promotion.
- Dimension tables hold descriptive attributes about each entity (e.g., dim_product with SKU, brand, category, fat content, package size). They can be very wide (tens of attributes).
- Date and time are typically modeled as dimensions to enable rich attribute filtering (holidays, weekdays, fiscal periods).
- Visualizing the relationships: the fact is the center of the star, dimensions are the rays.
- The **snowflake schema** is a variation where dimensions are further normalized into sub-dimensions; analysts often prefer denormalized stars for simplicity.

## Key Parameters

- Grain of the fact table (one row per transaction line vs aggregate).
- Number of dimensions and width of each.
- Slowly-changing-dimension (SCD) strategy (Types 1/2/3).

## When To Use

For analytics over event/transactional data where queries roll up facts along multiple dimensions. The default schema pattern for traditional data warehouses.

## Risks & Pitfalls

- Choosing the wrong grain forces painful redesigns later.
- Star schemas duplicate dimension attributes across many fact rows; this is intentional but increases storage.
- Slowly changing dimensions (renames, reorganizations) require careful versioning to keep historical reports correct.

## Related Concepts

- [[concepts/data-warehouse]]
- [[concepts/column-oriented-storage]]
- [[concepts/materialized-view]]
- [[concepts/oltp-vs-olap]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
