---
title: 'Data Analysis and Visualizations with Python — Chapter 8: Case Studies'
type: source
id: source-data-analysis-visualizations-python-11-chapter-8-case-studies
kind: derived-summary
tags:
- python
- pandas
- case-study
- data-analysis
- data-visualization
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/11-chapter-8-case-studies.txt
---

## Key Points

- Two end-to-end case studies illustrate the complete data-science workflow: problem definition, data gathering, cleaning, analysis, visualization, and findings.
- Case Study 1 analyzes the leading causes of death in the United States from 1999 to 2015 using NCHS data from data.gov.
- Case Study 1 pipeline: `pd.read_csv`, `data.shape` (15,028 rows × 6 columns), `dropna()` (14,917 valid), `data["Cause Name"].unique()`, filtering out "All Causes" and "United States" aggregates, and `.sum()` to total 69,279,057 deaths over the period.
- Case Study 1 analytics: `data.groupby(["Year"]).sum()` to plot annual death trends (decline 2002–2009, growth 2010–2013, sharp jump 2013–2014); top-10 states by `groupby("State").sum().sort_values("Deaths", descending)` (California first, Florida second); top-10 causes by similar pattern (heart disease leads, cancer second).
- Case Study 2 analyzes gun deaths in the United States from 2012 to 2014 using the FiveThirtyEight guns-data dataset from GitHub.
- Case Study 2 ingestion: `pd.read_csv('Death data.csv', index_col=0)` yielding 100,798 records × 10 columns, normalized via `dataset.columns = map(str.capitalize, dataset.columns)`, sorted by Year and Month.
- Case Study 2 analytics: `dataset_Gun.Sex.value_counts()` (86,349 males, 14,449 females); filtering by intent (`Intent == "Suicide"`) and visualizing with `.plot.bar(title=...)`; converting counts to per-100,000 rates via `value_counts() * 100 / 100000`.
- Case Study 2 findings: suicide gun deaths overwhelmingly male (>50,000 vs <10,000); whites have the highest death rate, then Blacks, then Hispanics; suicide and homicide dominate while accidents are minor; suicide rate stable at ~21 per 100,000 across all three years.
- Reinforces the canonical workflow: (1) determine the problem, (2) determine the main questions, (3) find a reliable data source, (4) explore and clean, (5) analyze and visualize, (6) discuss findings and make recommendations.

## Relevant Concepts

- [[concepts/python]] — runtime language.
- [[concepts/data-science]] — the chapter demonstrates the discipline end-to-end.
- [[concepts/data-analysis]] — analysis is the core activity.
- [[concepts/data-cleaning]] — `dropna`, anomaly filtering, column normalization.
- [[concepts/data-visualization]] — bar charts and trend plots interpret the data.
- [[concepts/data-aggregation]] — `groupby().sum()` and `value_counts()` drive both studies.
- [[concepts/exploratory-data-analysis]] — uniques, shapes, and trends precede formal analysis.
- [[entities/pandas]] — DataFrame operations everywhere.
- [[entities/matplotlib]] — `%matplotlib inline` and bar/line plots.
- [[entities/seaborn]] — `sns.set(style='white', color_codes=True)` in Case Study 2.

## Source Metadata

- Source type: book chapter
- Book title: Data Analysis and Visualizations with Python
- Chapter: 8 — Case Studies
- File path: raw/DataAnalysisAndVisualizationsPython/_txt/11-chapter-8-case-studies.txt
- Author: Ossama Embarak
