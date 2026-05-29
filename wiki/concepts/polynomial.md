---
title: Polynomial
type: claim
id: concepts/polynomial
tags:
- math
- modeling
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/08-chapter-4-planning-the-dashboard-prototype.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A polynomial is a function of the form `y = a0 + a1*t + a2*t^2 + ... + an*t^n`, parameterized by its coefficients `a0..an`; the highest power `n` is the polynomial's order. Low-order polynomials (linear, quadratic) are the user-friendly curves used for trend display in the book's dashboards.

## How It Works

For a linear polynomial, `a1` is the slope (rate of change per unit `t`) and `a0` is the intercept (value at `t=0`). For a quadratic, the slope is `a1 + 2*a2*t` and itself varies with `t`. Coefficients are fit via regression libraries (`numpy.polynomial.polynomial.polyfit`) and evaluated with `polyval`. The book demonstrates rebasing coefficients from the year-0 origin to a project-relative origin to make equation strings comprehensible.

## Key Parameters

- Order (number of terms)
- Coefficient array
- Reference origin for the independent variable
- Domain over which the fit is meaningful

## When To Use

- Capturing trend / slope summaries on plotted time series
- Building simple forecasts (linear and quadratic only)
- Communicating quantitative change-rates to non-specialists

## Risks & Pitfalls

- Higher-order polynomials overfit and lose interpretability
- Coefficients computed against absolute origins (0 A.D.) are not intuitive without rebasing
- Polynomial extrapolation diverges quickly outside the fit's domain

## Related Concepts

- [[concepts/regression]]
- [[concepts/linear-regression]]
- [[concepts/calculus]]
- [[concepts/time-series]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
