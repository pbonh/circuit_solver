---
title: Poisson Distribution
type: claim
id: claim-poisson-distribution
tags:
- statistics
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
confidence:
  base: 0.65
---

## Definition

The Poisson distribution models the number of events occurring in a fixed interval (of time, distance, area, volume) given a constant average rate lambda. PMF: `P(X=r) = lambda^r * exp(-lambda) / r!`.

## How It Works

Events are independent, randomly occurring, and cannot happen simultaneously. Mean = variance = lambda. Skewed for small lambda; approaches normal for large lambda. SciPy provides `stats.poisson.pmf` and `stats.poisson.cdf`.

## Key Parameters

- Average rate lambda
- Number of occurrences r
- Interval over which lambda is measured

## When To Use

- Arrival counts at queues / call centers
- Defect counts per unit area
- Accident frequencies per period

## Risks & Pitfalls

- Underestimating overdispersion (variance > mean)
- Assuming constant rate when it varies
- Treating clearly bounded counts as Poisson

## Related Concepts

- [[concepts/probability-distributions]]
- [[concepts/binomial-distribution]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
