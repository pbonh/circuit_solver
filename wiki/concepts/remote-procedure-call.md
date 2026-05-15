---
title: "Remote Procedure Call (RPC)"
type: concept
tags: [distributed-systems, well-established, networking, communication]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: high
---

## Definition

A remote procedure call (RPC) is a programming-level abstraction (dating to the 1970s, Birrell & Nelson 1984) that makes a request to a remote service look like an ordinary local function call. The goal is "location transparency": the caller does not see the network. Modern frameworks include gRPC (Protocol Buffers over HTTP/2), Apache Thrift, Avro RPC, Twitter Finagle, and LinkedIn Rest.li.

## How It Works

- The client invokes a stub generated from an IDL (Thrift, Protobuf, Avro). The stub encodes arguments, sends them over the network, awaits a response, and decodes it.
- The server-side framework receives the encoded request, dispatches to a handler, encodes the result, and returns it.
- Compatibility evolution follows the underlying encoding (Thrift/Protobuf/Avro rules); RESTful APIs typically use JSON without a formal schema and rely on conventions for adding optional fields.
- Modern frameworks acknowledge that RPC is not a local call: Finagle and Rest.li use futures, gRPC supports request/response streams, and service discovery resolves IP:port at runtime.

## Key Parameters

- Encoding format and IDL (Thrift, Protobuf, Avro, JSON).
- Transport (HTTP/1, HTTP/2, gRPC, raw TCP).
- Retry/timeout/circuit-breaker policy.
- Service discovery mechanism.

## When To Use

For tight, controlled service-to-service communication within an organization, when compact encoding and strict schemas justify the tooling. REST/JSON remains preferred for public APIs and for ease of debugging.

## Risks & Pitfalls

The "RPC is just a function call" abstraction is fundamentally flawed:
- Network requests can be lost, timeout, or have ambiguous outcomes (success on server, response lost).
- Latency is highly variable; local calls are not.
- Retries can cause duplicate execution unless idempotence is built in.
- Passing pointers/references doesn't work — everything must be serialized.
- Languages have mismatched type systems (e.g., JavaScript's 2^53 integer limit).

Treat remote calls as remote calls, not local calls.

## Related Concepts

- [[concepts/data-encoding]]
- [[concepts/schema-evolution]]
- [[concepts/message-broker]]
- [[concepts/backward-and-forward-compatibility]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
