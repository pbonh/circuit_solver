---
title: "Data Provenance"
type: concept
tags: [well-established, derived-data, audit, debugging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt"]
confidence: medium
---

## Definition

Data provenance is metadata that records where a piece of data came from: which source inputs, which transformations, which version of code, at what time. In a dataflow architecture, provenance lets you trace any derived value back to the events that produced it, supporting auditing, debugging, reproducibility, and regulatory compliance.

## How It Works

- Each event carries a unique ID and source identifier.
- Derivation steps record the input event IDs and the version of the derivation code that produced each output.
- Stream processors and batch jobs emit lineage metadata alongside outputs.
- Tools like Apache Atlas, OpenLineage, and custom systems track lineage across pipelines.
- For debugging, you can replay events through the same derivation to reproduce a specific outcome (time-travel debugging).

## Key Parameters

- Granularity (per-event vs per-batch vs per-job).
- Retention period for provenance metadata.
- Storage and query interface.

## When To Use

For regulated industries (finance, healthcare), GDPR-style right-to-explanation requirements, ML reproducibility, debugging complex derived data, and auditing data flows across organizational boundaries.

## Risks & Pitfalls

- Provenance metadata can dwarf payload size if granularity is fine.
- Versioning derivation code is itself non-trivial.
- Schema evolution complicates replay-based reproduction.

## Related Concepts

- [[concepts/derived-data]]
- [[concepts/event-sourcing]]
- [[concepts/end-to-end-argument]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
