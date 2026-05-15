---
title: "Backend for Frontend (BFF)"
type: concept
tags: [microservices, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: medium
---

## Definition

The Backend-for-Frontend (BFF) pattern deploys a dedicated backend service per client channel (e.g., one for web, one for mobile, one for partners). Each BFF aggregates calls to underlying microservices and shapes responses for its specific client.

## How It Works

Clients communicate only with their BFF, which delegates to internal services. BFFs can be scaled independently based on the load of their respective channel. Pattern attributed to Sam Newman.

## Key Parameters

- Number of BFFs (one per significant client channel).
- API contract between BFF and clients.

## When To Use

When clients with different needs (mobile bandwidth/latency vs. web feature richness) share underlying microservices and benefit from differentiated facades.

## Risks & Pitfalls

- Duplicates logic across BFFs if not careful.
- Adds an additional hop.

## Related Concepts

- [[concepts/api-gateway]]
- [[concepts/microservices]]
- [[concepts/horizontal-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
