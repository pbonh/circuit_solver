---
title: OpenAPI
type: claim
id: claim-openapi
tags:
- distributed-systems
- networking
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.65
---

## Definition

OpenAPI (formerly Swagger) is a specification for describing HTTP APIs in YAML or JSON. It defines paths, operations, request/response schemas, and authentication. SwaggerHub is the de facto standard authoring tool.

## How It Works

Authors describe each endpoint, its parameters, and its response shapes (referencing JSON Schema components). Tooling generates client SDKs, server stubs, documentation portals, and contract-tests from the spec. Current major version is 3.0.

## Key Parameters

- OpenAPI version.
- Schema component definitions.
- Authentication scheme (OAuth2, API keys, etc.).

## When To Use

Whenever you need a versioned, machine-readable contract between API producers and consumers.

## Risks & Pitfalls

- Spec and implementation drift if not validated in CI.
- Overly complex schemas become a documentation chore.

## Related Concepts

- [[concepts/rest]]
- [[concepts/api-gateway]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
