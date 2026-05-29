---
title: 'Python Data Analyst''s Toolkit — Chapter 9: Statistics and Probability with
  Python'
type: source
id: summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python
kind: publication
tags:
- python
- statistics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
---

## Key Points

- Permutations (`nPr = n!/(n-r)!`) count ordered arrangements; combinations (`nCr = n!/(r!(n-r)!)`) count unordered selections.
- Basic probability: P(X) = N(X)/N(S). Key rules covered: addition (`P(AUB) = P(A)+P(B)-P(AnB)`), special addition (mutually exclusive events), multiplication with conditional probability, special multiplication (independent events).
- Conditional probability: `P(A|B) = P(AnB)/P(B)`; Bayes theorem: `P(A|B) = P(B|A)*P(A)/P(B)`. Examples: medical diagnostic test (positive test only 25.6% means actually ill given low base rate) and email spam classification.
- SciPy's `scipy.stats` submodule provides functions for distributions, tests, distance calculations, correlations, and contingency tables.
- Random variables: discrete (PMF, e.g., Likert) vs. continuous (PDF / CDF). Common distributions covered: binomial (`stats.binom.pmf`, `stats.binom.cdf`), Poisson (`stats.poisson.pmf`, `stats.poisson.cdf`), normal (`stats.norm.cdf`, `stats.norm.ppf`).
- Binomial properties: mean=np, variance=npq. Poisson: mean=variance=lambda. Normal is symmetric, defined by mu and sigma; the standard normal has mu=0, sigma=1 and the empirical rule 68/95/99.8.
- Descriptive measures: central tendency (mean, median, mode, percentile, quartile), dispersion (range, IQR, variance, standard deviation), and shape (skewness, kurtosis: mesokurtic, leptokurtic, platykurtic). The Pandas `describe` method bundles many of these in one call.
- Sampling: probability sampling (simple random, stratified, systematic, cluster) vs. non-probability sampling (convenience, purposive — including quota and snowball). Central limit theorem: sample means are normally distributed with standard error `sigma/sqrt(n)`.
- Confidence intervals via `stats.t.interval` (unknown sigma) or `stats.norm.interval` (known sigma). Sampling errors include sampling, coverage, nonresponse, and measurement errors.
- Hypothesis testing framework: null (H0) vs. alternate (H1); type I error (level of significance alpha) vs. type II (beta); one-sample vs. two-sample; one-tail vs. two-tail; critical statistic; p-value (reject H0 if p < 0.05).
- Parametric tests covered with worked examples and SciPy functions: one-sample z-test, two-sample z-test, one-sample / two-sample / paired t-tests (`stats.ttest_1samp`, `stats.ttest_ind`, `stats.ttest_rel`), proportion z-tests, ANOVA (`stats.f_oneway`) for comparing more than two means using the F-distribution.
- Non-parametric chi-square test of association (`stats.chi2_contingency`) for two categorical variables; uses observed vs. expected frequencies with degrees of freedom `(r-1)*(c-1)`.
- p-value caveats: it does not prove or disprove hypotheses; depends on sample size; confidence intervals are often more interpretable.

## Relevant Concepts

- [[concepts/probability]] — foundation for the chapter.
- [[concepts/bayes-theorem]] — posterior probability given evidence.
- [[concepts/permutations-and-combinations]] — counting selections and arrangements.
- [[concepts/probability-distributions]] — binomial, Poisson, normal.
- [[concepts/normal-distribution]] — symmetric bell curve, standard normal transform.
- [[concepts/binomial-distribution]] — discrete trials with two outcomes.
- [[concepts/poisson-distribution]] — counts of events over an interval.
- [[concepts/descriptive-statistics]] — central tendency, dispersion, shape.
- [[concepts/central-limit-theorem]] — sampling distribution of means.
- [[concepts/hypothesis-testing]] — null/alternate framework, p-values.
- [[concepts/sampling-methods]] — probability and non-probability sampling.
- [[concepts/confidence-interval]] — interval estimation for population parameters.
- [[concepts/z-test]] — population mean / proportion with known sigma.
- [[concepts/t-test]] — small samples, unknown sigma.
- [[concepts/anova]] — comparing means of three or more populations.
- [[concepts/chi-square-test]] — non-parametric test of categorical association.
- [[entities/scipy]] — the library used for statistical functions.
- [[entities/william-sealy-gosset]] — published Student's t-distribution under the pen name "Student".

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 9 — Statistics and Probability with Python
- File path: raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
- Author: Gayathri Rajagopalan
