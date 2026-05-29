---
title: REST
type: claim
id: concepts/rest
tags:
- distributed-systems
- networking
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

REST (Representational State Transfer) is an architectural style defined by Roy Fielding for networked applications. In practice "RESTful" usually denotes HTTP APIs that expose resources at URIs and manipulate them with the HTTP verbs GET, POST, PUT, PATCH, and DELETE, with payloads typically encoded as JSON.

## How It Works

Each resource has a URI; operations on the resource are expressed via HTTP verbs (GET reads, POST creates, PUT replaces, PATCH partially updates, DELETE removes). HTTP status codes communicate outcomes (200 success, 404 not found, 503 unavailable, etc.). The API contract is often described in OpenAPI.

## Key Parameters

- URI design and resource modeling.
- HTTP verb / status-code conventions.
- Content negotiation (Accept / Content-Type).

## When To Use

The default style for internet-facing APIs and most internal service-to-service communication when a synchronous request/response works.

## Risks & Pitfalls

- "Chatty" APIs that require many calls to assemble a result balloon latency.
- Pure REST and "HTTP CRUD" are not the same; full REST is rarely implemented.
- Versioning strategies are easy to get wrong.

## Related Concepts

- [[concepts/openapi]]
- [[concepts/rpc]]
- [[concepts/http-caching]]
- [[concepts/microservices]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
