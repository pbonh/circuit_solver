---
title: "Binomial Distribution"
type: concept
tags: [statistics, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt"]
confidence: medium
---

## Definition

The binomial distribution models the number of successes in n independent Bernoulli trials, each with success probability p. PMF: `P(X=r) = nCr * p^r * q^(n-r)` with q=1-p.

## How It Works

Each trial has exactly two outcomes; trials are independent; p is constant. Mean = np; variance = npq. For large n with small p, it approaches a Poisson distribution; for moderate p, it approaches normal. SciPy exposes `stats.binom.pmf(r, n, p)` and `stats.binom.cdf(r, n, p)`.

## Key Parameters

- Number of trials n
- Success probability p
- Number of successes r

## When To Use

- Coin tosses, yes/no surveys, defect counts
- Quality-control sampling
- Modeling success counts in fixed-size experiments

## Risks & Pitfalls

- Violations of independence between trials
- p not constant across trials
- Confusing PMF/CDF semantics

## Related Concepts

- [[concepts/probability-distributions]]
- [[concepts/poisson-distribution]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
