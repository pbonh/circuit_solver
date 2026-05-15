---
title: "Kernel Density Estimate"
type: concept
tags: [python, visualization, statistics, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt"]
confidence: medium
---

## Definition

A kernel density estimate (KDE) is a smoothed, non-parametric estimate of the probability distribution of a continuous variable. The chapter introduces it via Seaborn's `sns.kdeplot(series)` and the merged violin plot.

## How It Works

KDE places a kernel function (typically Gaussian) at each data point and sums these to produce a continuous density curve. The bandwidth parameter controls smoothness — small bandwidth follows the data closely; large bandwidth produces a flatter curve.

## Key Parameters

- Kernel function (default Gaussian)
- Bandwidth (smoothing parameter)
- Optional hue for multiple groups

## When To Use

- Comparing distributions of continuous variables
- Smooth alternative to histograms
- Inside violin plots

## Risks & Pitfalls

- Bandwidth choice strongly affects appearance
- Boundary effects near data limits
- Cannot accurately represent multimodality with too-large bandwidth

## Related Concepts

- [[concepts/data-visualization]]
- [[concepts/descriptive-statistics]]
- [[concepts/probability]]

## Sources

- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
