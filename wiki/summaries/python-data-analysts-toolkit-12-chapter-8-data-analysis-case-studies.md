---
title: 'Python Data Analyst''s Toolkit — Chapter 8: Data Analysis Case Studies'
type: source
id: source-python-data-analysts-toolkit-12-chapter-8-data-analysis-case-studies
kind: derived-summary
tags:
- python
- pandas
- data-analysis
- visualization
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/12-chapter-8-data-analysis-case-studies.txt
---

## Key Points

- The chapter walks through three end-to-end case studies that exercise the full descriptive-analysis pipeline: import, examine, wrangle, visualize, and summarize.
- Standard methodology per case: (1) import libraries/data, `head`/`info`/`shape`/`describe`; (2) wrangle types, rename, drop, restructure, replace, handle nulls, aggregate; (3) visualize with univariate, bivariate, multivariate plots; (4) summarize insights and recommendations.
- Case 1 (unstructured web data): scraping the Wikipedia top-50 highest-grossing 2018 films in France using the `requests` library plus `pd.read_html`; cleaning the "Gross" column with chained `str.replace` (regex), casting to int64, parsing dates with `pd.DatetimeIndex` to extract `Month`.
- Case 1 findings: top three highest-grossing films were Avengers, La Ch'tite Famille, and Les Tuche 3; monthly revenue varies considerably with release timing.
- Case 2 (Delhi air quality): the NSIT Dwarka CSV contains daily PM2.5, SO2, ozone, and NO2 readings. Initial null counts were misleading because missing values appeared as the string "None"; replacing with `np.nan` revealed many more gaps.
- Case 2 method: convert date columns with `pd.to_datetime`, numeric pollutant columns with `pd.to_numeric(..., errors='coerce')`; restrict analysis to 2016-2019 because 2014-2015 had too many missing observations; drop nulls in remaining years; extract year and month via `pd.DatetimeIndex`.
- Case 2 visualizations: yearly bar charts with annotated standard lines (`hlines`); stacked horizontal bar charts of binned PM2.5 concentration intervals per year; monthly bar charts showing critical PM2.5 days.
- Case 2 findings: only PM2.5 consistently exceeds the annual standard; critical PM2.5 days cluster in January, November, December — guiding when traffic and construction restrictions should be imposed.
- Case 3 (worldwide COVID-19): Excel data of cases/deaths by country and date (Dec 2019 - Jun 2020 from ECDC). Wrangling steps include renaming columns, dropping a redundant code column, removing the one-day December 2019 slice, and dropping NaN rows (<1% of data).
- Case 3 aggregation: `groupby('country')[['cases','deaths']].sum()` plus a derived `mortality_rate` column; visualizations include top-20 mortality bar chart, top-10 cases pie chart, top-5 fatalities bar chart, and per-month line charts.
- Case 3 lockdown analysis: side-by-side line plots for UK, India, Italy, Germany show that countries imposing March lockdowns generally saw decreasing cases afterward (India was the exception, with continued increase).
- Throughout, the cases demonstrate the `requests` module for HTTP, regex-driven string cleanup, `astype` for dtype coercion, `pivot` for cross-tabulations, `groupby` aggregation, and the object-oriented Matplotlib API for multi-panel figures.

## Relevant Concepts

- [[concepts/web-scraping]] — using `requests` + `pd.read_html` to ingest HTML tables.
- [[concepts/data-wrangling]] — the central activity in all three cases.
- [[concepts/exploratory-data-analysis]] — the umbrella methodology demonstrated.
- [[concepts/missing-data-imputation]] — handling "None" strings, `coerce` parsing, dropping nulls.
- [[concepts/data-visualization]] — choice of bar, line, pie, stacked-bar, multi-panel plots.
- [[concepts/split-apply-combine]] — `groupby` used in every case study.
- [[entities/pandas]] — primary library throughout.
- [[entities/matplotlib]] — used for plotting all visualizations.
- [[entities/seaborn]] — used for bar plots in case 1.
- [[entities/requests-library]] — used to pull data from web pages.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 8 — Data Analysis Case Studies
- File path: raw/PythonDataAnalystsToolkit/_txt/12-chapter-8-data-analysis-case-studies.txt
- Author: Gayathri Rajagopalan
