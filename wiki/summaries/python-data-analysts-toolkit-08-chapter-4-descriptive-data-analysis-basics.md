---
title: "Python Data Analyst's Toolkit — Chapter 4: Descriptive Data Analysis Basics"
type: summary
tags: [python, data-analysis, statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/08-chapter-4-descriptive-data-analysis-basics.txt"]
confidence: high
---

## Key Points

- Descriptive (exploratory) data analysis analyzes past data through summarization, aggregation, and visualization, in contrast to predictive analytics that forecasts the future.
- The book defines a five-step descriptive analysis workflow: (1) data retrieval, (2) cursory review and problem identification, (3) data wrangling, (4) data exploration and visualization, (5) publishing and presenting findings.
- Data wrangling consumes roughly 80% of an analyst's time and involves tidying (mapping variables to columns), cleansing (missing values, outliers, type fixes), and enrichment (adding fields/sources).
- Data structures come in three flavors: structured (rows/columns, spreadsheets, relational databases), unstructured (photos, videos, documents), and semi-structured (JSON, XML).
- Data levels: continuous splits into ratio (true zero, all arithmetic valid) and interval (no true zero, addition/subtraction only); categorical splits into nominal (unordered) and ordinal (ordered).
- A true zero point means the absence of a value; height/weight/age are ratio; Celsius/Fahrenheit temperature, pH, year-of-birth, and GRE scores are interval.
- The Titanic dataset is used to demonstrate variable classification: nominal (PassengerId, Survived, Name, Sex, Cabin, Embarked), ordinal (Pclass), ratio (Age, SibSp, Parch, Fare).
- Plot choice depends on data level: bar/pie charts for nominal and ordinal, histograms for continuous, box plots for one continuous vs. one categorical, scatter plots for two continuous, stacked or clustered bar charts for two categorical.
- Mathematical operations track data level: division/multiplication require ratio data; addition/subtraction allowed on ratio and interval; mode applies to all levels; median requires order.
- Jupyter notebooks double as execution and presentation media for descriptive analyses, exportable to PDF for sharing.

## Relevant Concepts

- [[concepts/exploratory-data-analysis]] — synonym used here for descriptive data analysis.
- [[concepts/data-wrangling]] — central step in the workflow.
- [[concepts/data-levels]] — the nominal/ordinal/interval/ratio taxonomy.
- [[concepts/descriptive-statistics]] — measures derived from these data types.
- [[concepts/data-visualization]] — choosing charts by data level.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 4 — Descriptive Data Analysis Basics
- File path: raw/PythonDataAnalystsToolkit/_txt/08-chapter-4-descriptive-data-analysis-basics.txt
- Author: Gayathri Rajagopalan
