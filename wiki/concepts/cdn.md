---
title: "Content Delivery Network (CDN)"
type: concept
tags: [caching, networking, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

A Content Delivery Network (CDN) is a globally distributed network of edge caches strategically located close to end users. CDNs serve cached static content (images, videos, scripts) and accelerate dynamic content delivery by terminating connections near users.

## How It Works

Clients are routed (typically via DNS or anycast) to the nearest edge node. If the requested resource is in the edge cache it is returned immediately; otherwise the edge fetches from the origin server, caches the response, and returns it. Akamai operates 2,000+ POPs and reportedly carries up to 30% of global internet traffic.

## Key Parameters

- TTL per resource type.
- Cache-key strategy.
- Origin-shielding configuration.

## When To Use

Media-heavy sites, software downloads, static-asset offload, geographic acceleration.

## Risks & Pitfalls

- Cache invalidation across many edges is complex.
- Misconfigured TTLs serve stale content.

## Related Concepts

- [[concepts/http-caching]]
- [[concepts/distributed-cache]]
- [[concepts/etag]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
