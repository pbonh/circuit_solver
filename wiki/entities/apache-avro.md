---
title: Apache Avro
type: entity
id: entity-apache-avro
tags:
- well-established
- encoding
- schema-evolution
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
---

## Overview

Apache Avro is a binary encoding format and schema language that originated as a Hadoop subproject (2009). It was created in response to Thrift not fitting Hadoop's use cases. Avro's distinguishing feature is its writer-schema / reader-schema separation: data on the wire contains no field tags, and resolution between schemas happens by field name at decode time, enabling dynamically generated schemas (e.g., dumping a relational table).

## Characteristics

- Two schema languages: Avro IDL (human-friendly) and JSON-based (machine-friendly).
- Binary encoding is extremely compact (the DDIA example record is 32 bytes — smallest of all formats compared).
- No field tag numbers; instead, fields are matched by name during schema resolution.
- Schema evolution rules: every added or removed field must have a default value (or be null via union types); aliases allow field renames.
- Object container files prepend the writer's schema once for the whole file; databases and streaming systems use schema registries or in-band version numbers.
- Optional code generation; Avro can be used dynamically without it (well-suited for Pig and similar dynamic environments).

## Common Strategies

- Use object container files for large batch outputs (Hadoop, Spark).
- Pair with a schema registry (e.g., Confluent Schema Registry) when streaming through Kafka.
- Auto-generate Avro schemas from relational table definitions.
- Encode versioned application messages with explicit schema versions.

## Related Entities

- [[entities/protocol-buffers]]
- [[entities/apache-thrift]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
