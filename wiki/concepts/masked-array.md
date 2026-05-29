---
title: Masked Array (NumPy)
type: claim
id: claim-masked-array
tags:
- python
- numpy
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt
confidence:
  base: 0.65
---

## Definition

A masked array is a NumPy structure that pairs a data array with a Boolean mask indicating which entries are invalid or missing. Available via the `numpy.ma` submodule, it lets aggregate functions skip masked entries automatically.

## How It Works

`numpy.ma.masked_array(data, mask)` takes two parallel arrays; mask value `1` marks an element as invalid, `0` marks it valid. The book shows assigning a new value to a masked element unmasks it. Statistical operations ignore masked entries.

## Key Parameters

- Mask array (same shape as data)
- Fill value used when materializing
- Whether the mask is shared or copied

## When To Use

- Datasets with sentinel or sensor-failure values
- Streaming data where some samples are bad
- Cleaner alternative to `np.nan` for integer dtypes

## Risks & Pitfalls

- Performance overhead vs. raw arrays
- Limited support in some libraries downstream
- Forgetting to set the mask leaves invalid values in computations

## Related Concepts

- [[concepts/ndarray]]
- [[concepts/data-wrangling]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
