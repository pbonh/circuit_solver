---
title: "Backward and Forward Compatibility"
type: concept
tags: [well-established, distributed-systems, encoding, deployment]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

**Backward compatibility**: newer code can read data written by older code. **Forward compatibility**: older code can read data written by newer code. Both are required during rolling upgrades because both old and new versions of code (and of data) coexist in a running system, and either can be the reader or writer.

## How It Works

- Backward compatibility is normally straightforward: the new code author knows the old format and can add fallbacks for legacy fields.
- Forward compatibility is harder: old code must gracefully ignore unknown fields rather than crashing or losing data when round-tripping through unfamiliar structures.
- Encoding formats provide different mechanisms:
  - **Protobuf/Thrift**: forward compatibility comes from skipping unknown tag numbers using datatype annotations.
  - **Avro**: writer's schema and reader's schema are reconciled by name; missing fields use defaults; unknown fields are ignored.
  - **JSON/XML**: depends entirely on consumer code; "unknown fields" must be deliberately preserved on round-trips.
- For database storage, an important hazard: an older version reads a record, mutates it, and writes it back. If the older version doesn't preserve unknown fields, data added by the newer version is silently lost (DDIA Figure 4-7).

## Key Parameters

- Encoding format (controls compatibility mechanics).
- Deployment cadence and overlap window between versions.
- Whether unknown-field preservation is enabled in the encoding library.

## When To Use

Always, for any data crossing a process boundary in a system that does rolling upgrades or has clients of unknown version.

## Risks & Pitfalls

- Default Java serialization, Erlang record changes, and similar language-specific formats often fail to provide compatibility.
- Required-field migrations in Protobuf/Thrift can break forward compatibility.
- API versioning across organizational boundaries (RESTful services) often forces maintaining multiple versions indefinitely.

## Related Concepts

- [[concepts/schema-evolution]]
- [[concepts/data-encoding]]
- [[concepts/maintainability]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
