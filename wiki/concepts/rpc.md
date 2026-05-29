---
title: Remote Procedure Call (RPC)
type: claim
id: concepts/rpc
tags:
- distributed-systems
- networking
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Remote Procedure Call (RPC) is a programming abstraction that lets a client invoke a procedure or method on a remote server with syntax similar to a local call. The runtime handles marshalling arguments, transport, dispatch, and unmarshalling results.

## How It Works

Generated stubs and skeletons serialize call arguments into network packets and reconstruct them on the other end. The client's call blocks until the server returns (or an error / timeout occurs). Historical RPC frameworks include DCE RPC, CORBA, Java RMI, and XML web services; modern equivalents are gRPC (HTTP/2 + Protocol Buffers) and REST-over-HTTP.

## Key Parameters

- Interface description language.
- Serialization format.
- Timeout and retry policy.

## When To Use

Inter-service communication when a synchronous call/response model fits, especially within a controlled microservice mesh.

## Risks & Pitfalls

- "Location transparency" hides real failure modes (network errors).
- Cross-language marshalling can corrupt types subtly.
- Tight coupling to interface signatures impedes evolution.

## Related Concepts

- [[concepts/rest]]
- [[concepts/microservices]]
- [[concepts/tcp]]
- [[concepts/partial-failure]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
