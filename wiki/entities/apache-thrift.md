---
title: Apache Thrift
type: entity
id: entity-apache-thrift
tags:
- well-established
- encoding
- schema-evolution
- rpc
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
---

## Overview

Apache Thrift is a binary encoding format, IDL, and RPC framework originally developed at Facebook and open-sourced in 2007–08. It is closely related to Protocol Buffers in design — both use numeric field tags — but offers multiple wire protocols (BinaryProtocol, CompactProtocol, DenseProtocol, JSON variants).

## Characteristics

- IDL defines structs with numbered, typed fields marked required or optional.
- Multiple binary encodings: BinaryProtocol (verbose), CompactProtocol (variable-length integers, packed type+tag — DDIA example is 34 bytes), DenseProtocol (C++ only).
- Has a dedicated list datatype with element-type parameterization; allows nested lists.
- Includes an RPC framework as part of the toolkit (used by Twitter Finagle).
- Schema-evolution rules identical to Protocol Buffers in spirit: tags identify fields, new fields must be optional, never reuse tags.

## Common Strategies

- Use CompactProtocol for production deployments to minimize bytes on the wire.
- Pair with Finagle or Thrift's own RPC for service communication.
- Use the dedicated list type when nested collections are needed (not possible in Protobuf without a wrapper message).

## Related Entities

- [[entities/protocol-buffers]]
- [[entities/apache-avro]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
