---
title: DevOps
type: claim
id: concepts/devops
tags:
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

DevOps is a set of practices and tools that unify software development (Dev) and IT operations (Ops) to shorten the system development life cycle and deliver software continuously, with high quality. Bass et al. define it as "a set of practices intended to reduce the time between committing a change to a system and the change being placed into normal production, while ensuring high quality."

## How It Works

Core practices include continuous integration, continuous delivery / deployment, infrastructure as code, automated testing, monitoring and observability, and team ownership of deployment. Teams typically rotate 24-hour on-call duty.

## Key Parameters

- Deployment frequency and cycle time.
- Mean time to recovery (MTTR).
- Change failure rate.

## When To Use

Mandatory practice for any scalable system with frequent change.

## Risks & Pitfalls

- "DevOps" sometimes degenerates into a tooling buzzword without organizational change.
- Without observability, automation can deploy bad code faster.

## Related Concepts

- [[concepts/continuous-delivery]]
- [[concepts/infrastructure-as-code]]
- [[concepts/observability]]
- [[concepts/container]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
