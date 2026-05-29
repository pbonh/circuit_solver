---
title: Cold Start
type: claim
id: concepts/cold-start
tags:
- cloud
- performance
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A cold start is the latency penalty incurred the first time a serverless function (or container) is invoked, as the platform downloads the code, initializes the runtime, and runs any startup logic.

## How It Works

After a function has been idle for a few minutes, the platform reclaims the instance. The next invocation must spin up a new instance — pulling the image, starting the runtime (Node.js/Go in hundreds of ms; Java/.NET in 1-3 s), and running the handler's initialization code.

## Key Parameters

- Runtime / language choice.
- Memory allocation (more memory = faster init).
- Provisioned concurrency for pre-warming.

## When To Use

Always relevant to FaaS design; influences runtime choice and startup-logic placement.

## Risks & Pitfalls

- Cold starts visibly degrade tail latency.
- Provisioned concurrency reduces cold starts at extra cost.

## Related Concepts

- [[concepts/serverless]]
- [[concepts/provisioned-concurrency]]
- [[concepts/long-tail-latency]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
