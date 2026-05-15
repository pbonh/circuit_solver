---
title: "Bayes Theorem"
type: concept
tags: [statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt"]
confidence: high
---

## Definition

Bayes theorem expresses the posterior probability of a hypothesis A given evidence B as `P(A|B) = P(B|A) * P(A) / P(B)`. It lets us update prior beliefs with new evidence.

## How It Works

P(A) is the prior; P(B|A) is the likelihood; P(B) is the marginal probability of the evidence (often expanded as `P(B|A)*P(A) + P(B|~A)*P(~A)`); P(A|B) is the posterior. The chapter applies the theorem to medical diagnostics (low base rate makes a positive test only weakly informative) and email spam classification.

## Key Parameters

- Prior probability P(A)
- Likelihood P(B|A)
- Marginal P(B) and its expansion
- Sensitivity / specificity of evidence

## When To Use

- Updating beliefs given test outcomes
- Naive Bayes classifiers
- Forensic and risk assessments

## Risks & Pitfalls

- Ignoring the base rate (base-rate fallacy)
- Assuming conditional independence when it doesn't hold
- Estimating likelihoods from biased samples

## Related Concepts

- [[concepts/probability]]
- [[concepts/hypothesis-testing]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
