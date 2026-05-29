---
title: Protocol Buffers
type: entity
id: entities/protocol-buffers
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

Protocol Buffers (protobuf) is Google's binary encoding format and IDL, open-sourced in 2007–08. It is the foundation of gRPC and widely used inside Google and elsewhere for both persistent storage and RPC. Like Thrift, it identifies fields by numeric tags so the encoded form omits field names.

## Characteristics

- Single binary encoding format (no "compact" vs "binary" variants like Thrift).
- Fields declared with required, optional, or repeated; tag numbers are part of the schema contract.
- Variable-length integer encoding compresses small numbers.
- Encodes the DDIA example record in 33 bytes — very close to Thrift CompactProtocol.
- Schema evolution rules: never reuse tags, never make new fields required, can promote optional → repeated, can change names freely.
- Code generation is required to use protobuf in most languages.

## Common Strategies

- Use with gRPC for service-to-service communication.
- Persist long-lived messages with stable tag assignments; reserve tag ranges in the IDL.
- Treat the .proto file as the schema source of truth; check it into version control.
- Maintain backward and forward compatibility by adding only optional/repeated fields.

## Related Entities

- [[entities/apache-thrift]]
- [[entities/apache-avro]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
