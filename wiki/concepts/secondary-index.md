---
title: Secondary Index
type: claim
id: concepts/secondary-index
tags:
- storage
- well-established
- indexing
- query-performance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A secondary index is an additional access structure built on a non-primary-key column (or combination of columns), enabling efficient lookups, range scans, or joins on attributes other than the primary key. It can be implemented atop any underlying index data structure (B-tree, LSM-tree) and may either store the row inline (clustered) or reference it elsewhere (heap-file pointer).

## How It Works

- Secondary-index keys are not unique: multiple rows may share a value. Implementations either store a postings-list of row IDs per key or append the row ID to each key.
- A non-clustered (heap-file) index stores row data in a separate heap; multiple secondary indexes share that heap and each just keeps a pointer. Updating a row in place is cheap unless the new value enlarges the row.
- A clustered index stores the row inline within the primary index; secondary indexes then reference the primary key, not a heap location (InnoDB does this).
- A covering index (or "index with included columns") stores extra columns inside the index so that some queries can be answered without touching the heap; trades write overhead for read speed.
- Multi-column indexes concatenate fields in a defined order (e.g., (lastname, firstname)); they help queries that match the prefix.
- Multi-dimensional indexes (R-trees, space-filling curves) handle geospatial and similar range-on-multiple-axes queries.

## Key Parameters

- Indexed column(s) and order for concatenated keys.
- Inclusion of additional payload columns (covering).
- Whether index is clustered or heap-pointer style.

## When To Use

To accelerate queries on non-primary columns, join keys, sort orders, or range conditions. Heavily used in OLTP. Less central in OLAP, where column-oriented storage replaces row-pointer indexes.

## Risks & Pitfalls

- Each index slows down writes (must be updated on every mutation).
- Over-indexing wastes space and complicates query planning.
- Clustered / covering indexes risk inconsistency if not carefully transactional.
- Multi-dimensional queries don't work well with standard B-trees; spatial indexes are required.

## Related Concepts

- [[concepts/b-tree]]
- [[concepts/lsm-tree]]
- [[concepts/column-oriented-storage]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
