---
title: API Gateway
type: claim
id: concepts/api-gateway
tags:
- microservices
- networking
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

An API gateway is a managed front door for a collection of backend microservices. It proxies client requests to the correct internal service, applying cross-cutting concerns: routing, authentication, authorization, throttling, caching, observability, and protocol translation.

## How It Works

Clients call the gateway's public URL; the gateway authenticates and authorizes the request, applies rate limits, and forwards to the configured upstream service. Mappings and policies are maintained centrally. Examples: NGINX Plus, Kong, AWS API Gateway.

## Key Parameters

- Routing rules / API contract.
- Rate-limit policies.
- Auth scheme.
- Caching policy.

## When To Use

Any microservices architecture with external clients; provides a stable contract and insulates clients from internal refactors.

## Risks & Pitfalls

- Can become a bottleneck or single point of failure.
- Configuration sprawl with hundreds of microservices.

## Related Concepts

- [[concepts/microservices]]
- [[concepts/load-balancing]]
- [[concepts/throttling]]
- [[concepts/backend-for-frontend]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
