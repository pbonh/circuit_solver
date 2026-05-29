---
title: Amdahl's Law
type: claim
id: claim-amdahls-law
tags:
- concurrency
- performance
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.85
---

## Definition

Amdahl's law gives the theoretical maximum speedup obtainable by parallelizing a computation: speedup = 1 / ((1 - p) + p/N), where p is the parallelizable fraction and N is the number of processors. The serial fraction (1 - p) caps speedup regardless of how many cores you throw at the problem.

## How It Works

If 95% of code is parallel, adding more than ~2,048 cores yields essentially no further speedup. If only 50% is parallel, more than 8 cores buys almost nothing. The corollary is that minimizing serial sections (critical sections, locks, coordination) is more valuable than buying more hardware.

## Key Parameters

- Parallelizable fraction `p`.
- Number of cores `N`.

## When To Use

Any time you are reasoning about whether scale-up (or thread-pool sizing) will actually deliver throughput improvements.

## Risks & Pitfalls

- Holding monitor locks in hot paths kills scalability.
- The "p" fraction often shrinks unexpectedly as workloads grow.

## Related Concepts

- [[concepts/concurrency]]
- [[concepts/thread-pool]]
- [[concepts/vertical-scaling]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
