---
title: Data Encoding (Serialization)
type: claim
id: claim-data-encoding
tags:
- well-established
- encoding
- distributed-systems
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

Data encoding (a.k.a. serialization or marshalling) is the translation of in-memory data structures (objects, structs, lists, hash tables, trees) into a self-contained byte sequence that can be written to a file or transmitted over a network. Decoding (deserialization, parsing) is the reverse. The choice of format affects compactness, performance, interoperability, and schema evolvability.

## How It Works

Several format families:

- **Language-specific** (Java Serializable, Python pickle, Ruby Marshal, Kryo): convenient but tied to one language, often insecure (decoding can instantiate arbitrary classes), generally bad performance and weak versioning.
- **Textual** (JSON, XML, CSV): universal, human-readable, weak typing (no integer vs float in JSON; no binary strings without Base64), optional schema languages.
- **Binary JSON variants** (MessagePack, BSON, BJSON, Smile): more compact but still self-describing with field names embedded.
- **Schema-driven binary** (Protocol Buffers, Thrift, Avro): require a schema, omit field names from data (use tags or implicit ordering), much more compact, support controlled schema evolution.

Example record encoded in ~32 bytes with Avro vs 81 bytes with JSON vs 66 bytes with MessagePack vs 33–34 bytes with Protobuf/Thrift CompactProtocol.

## Key Parameters

- Compactness on the wire/disk.
- Schema requirement (yes for Protobuf/Thrift/Avro, optional for JSON/XML).
- Forward/backward compatibility rules.
- Language and tooling support.
- Whether code generation is required.

## When To Use

- Use textual formats (JSON) for cross-organizational APIs and human-debuggable workflows.
- Use schema-driven binary (Avro, Protobuf, Thrift) for internal high-volume RPC, persistent storage, or message streams where compactness, type-safety, and evolution control matter.
- Avoid language-specific serialization for anything non-transient.

## Risks & Pitfalls

- JSON's number ambiguity (no int vs float; >2^53 loses precision in JavaScript — Twitter's tweet-ID workaround).
- Reusing field tags in Protobuf/Thrift silently corrupts data.
- Avro requires the writer's schema be available to the reader — schema registry needed for streams and databases.
- Insecure deserialization (Java, Ruby, Python) has been the source of many remote-code-execution CVEs.

## Related Concepts

- [[concepts/schema-evolution]]
- [[concepts/backward-and-forward-compatibility]]
- [[concepts/remote-procedure-call]]
- [[entities/apache-avro]]
- [[entities/protocol-buffers]]
- [[entities/apache-thrift]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
