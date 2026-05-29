---
title: Schema Evolution
type: claim
id: claim-schema-evolution
tags:
- well-established
- data-modeling
- encoding
- schema-evolution
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

Schema evolution is the discipline of changing the structure of stored or transmitted data (adding/removing/renaming fields, changing types) while keeping forward and backward compatibility so that old and new code can coexist, especially during rolling upgrades. Different encoding formats provide different evolution guarantees.

## How It Works

- **Protocol Buffers / Thrift**: each field has a numeric tag plus required/optional/repeated marker. Adding a new optional/repeated field with a fresh tag is backward- and forward-compatible. Removing a field is allowed only if it was optional. Tag numbers must never be reused. Renaming fields is free (tags are the identifier).
- **Avro**: there are no tag numbers. The writer's schema and reader's schema are both known to the resolver, which matches fields by name. To stay compatible, every added or removed field must have a default value. Aliases support field renames (backward only). Avro is friendly to dynamically generated schemas (e.g., dumping a relational DB).
- **JSON/XML/CSV**: no explicit schema, so evolution is purely a matter of how consuming code treats missing or extra fields.

## Key Parameters

- Encoding format and its evolution rules.
- Field-tag assignment policy (Protobuf/Thrift) or default-value policy (Avro).
- Schema registry / version-store strategy for Avro.

## When To Use

In every multi-process system where data outlives any single deployment — databases, RPC services, message queues. Required for rolling upgrades and zero-downtime deployments.

## Risks & Pitfalls

- Reusing a removed tag number in Protobuf/Thrift silently corrupts old data.
- Adding a required field breaks readers of old data (Protobuf/Thrift).
- Avro readers and writers must somehow agree on the writer's schema (object container files, schema registry, or in-band version number).
- Older code reading newer data and writing it back can lose unknown fields if not preserved (DDIA Figure 4-7).

## Related Concepts

- [[concepts/backward-and-forward-compatibility]]
- [[concepts/data-encoding]]
- [[concepts/schema-on-read-vs-schema-on-write]]
- [[entities/apache-avro]]
- [[entities/protocol-buffers]]
- [[entities/apache-thrift]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
