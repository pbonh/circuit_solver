---
title: HTTP Caching
type: claim
id: claim-http-caching
tags:
- caching
- networking
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
---

## Definition

HTTP caching is the suite of mechanisms baked into the HTTP protocol that allow intermediaries (browsers, proxies, CDNs) to store and reuse responses, dramatically reducing origin load and latency.

## How It Works

Origin servers attach cache directives in response headers: `Cache-Control` (public/private/no-store/no-cache/max-age), `Expires`, `Last-Modified`, and `ETag`. Caches use these to determine freshness; when a resource becomes stale, the cache uses conditional requests (`If-None-Match`, `If-Modified-Since`) to revalidate. A 304 Not Modified response is far cheaper than a 200 with the full body.

## Key Parameters

- Cache-Control directives.
- TTL via max-age or Expires.
- ETag generation strategy.

## When To Use

Any HTTP API or web resource that is read-mostly.

## Risks & Pitfalls

- Aggressive caching of dynamic content serves stale data.
- ETags computed from changing internal state cause spurious cache misses.

## Related Concepts

- [[concepts/cdn]]
- [[concepts/etag]]
- [[concepts/distributed-cache]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
