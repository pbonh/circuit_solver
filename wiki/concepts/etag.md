---
title: "ETag"
type: concept
tags: [caching, networking, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

An ETag is an opaque identifier that an HTTP server attaches to a response, representing the current version of a resource. Caches use ETags to revalidate stored copies via conditional requests, allowing the server to respond with 304 Not Modified instead of resending the body.

## How It Works

Server: `ETag: "v1-2026-05-15"` in the response. Cache later sends `If-None-Match: "v1-2026-05-15"`. If the resource is unchanged, server returns 304 with no body; otherwise 200 with the new content and a new ETag. ETags can be content hashes or version strings.

## Key Parameters

- ETag generation algorithm.
- Strong vs. weak ETag semantics.

## When To Use

Whenever HTTP caching is in play and revalidation is preferable to TTL-only expiry, especially for large infrequently changing resources.

## Risks & Pitfalls

- Non-deterministic ETag generation breaks caching.
- Server must support `If-None-Match` evaluation efficiently.

## Related Concepts

- [[concepts/http-caching]]
- [[concepts/cdn]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
