---
title: 'Data Analysis and Visualizations with Python — Chapter 5: Data Gathering and
  Cleaning'
type: source
id: source-data-analysis-visualizations-python-08-chapter-5-data-gathering-and-cleaning
kind: derived-summary
tags:
- python
- pandas
- data-cleaning
- data-gathering
- json
- html
- xml
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/08-chapter-5-data-gathering-and-cleaning.txt
---

## Key Points

- Identifies five canonical data-science steps: acquisition, cleaning, exploratory analysis, modeling, and visualization.
- Lists the Python libraries used throughout: Pandas for tabular manipulation, NumPy for array math, SciPy for numerical integration/optimization, and Matplotlib for plots.
- Demonstrates missing-value detection via `df['col'].isnull()` and `notnull()`, which return Boolean Series indicating NaN positions.
- Covers four NaN-handling strategies: `fillna(scalar)`, `fillna(method='pad')` (forward fill), `fillna(method='bfill')` (backward fill), and `dropna()` to remove rows containing NaN; also `replace(np.nan, value)`.
- Reads CSV files with `pd.read_csv("Sales.csv")`, previewing with `.head()` and `.tail()`, limiting rows with `nrows=`, and selecting columns with `usecols=[idx]` or `usecols=[label]`.
- Renames columns inplace via `df.rename(columns={"old":"new"}, inplace=True)` and finds unique column values with `df['col'].unique()`.
- Converts sentinel values to NaN during load with `na_values=["n.a.", "not avilable", -1]`, including per-column dict form for different sentinels per column.
- Defines custom converter functions and applies them via `converters={'COL': cleaner_func}` to substitute sane defaults (e.g., 0 for numeric, "AbuDhabi" for region).
- Merges DataFrames via `a.merge(b, on="Country Name")` (specified key) or `a.merge(b)` (auto-detect common columns).
- Stacks DataFrames vertically via `pd.concat((d1, d2), axis=0)` after `reset_index()`.
- Demonstrates dropping columns with `df.drop('2014', axis=1, inplace=True)` or by passing a list of column names.
- Reads JSON via `json.loads(string)` and `json.load(open_file)`; accesses nested keys like `info["email"]["hide"]`; iterates list-of-dicts JSON to compute aggregates over a `comments` array.
- Reads HTML via `urllib.request.urlopen` + Beautiful Soup (`BeautifulSoup(doc, 'html.parser')`); navigates with `.title`, `.title.string`, `.title.parent.name`, `.a`, `.p['class']`; queries with `find_all('a')`, `find(id="link2")`; extracts text with `get_text()`.
- Parses XML via `xml.etree.ElementTree`: `ET.fromstring(doc)`, `findall('student')`, per-element `.get('name')` for attributes and `.find('rank').text` for child text.
- Closes with exercises covering Excel ingestion via `pandas.read_excel` with sheet selection, descriptive statistics on the Budget column, conditional filtering for USA+Duration<50, computing an Avg Reviews column, and multi-column sort.

## Relevant Concepts

- [[concepts/python]] — language and runtime.
- [[concepts/data-analysis]] — the discipline this chapter equips.
- [[concepts/missing-data-handling]] — half the chapter focuses on NaN strategies.
- [[concepts/data-cleaning]] — broader umbrella covering merges, type fixes, sentinel replacement.
- [[concepts/data-extraction]] — extracting structured records from JSON/HTML/XML.
- [[concepts/dataframe]] — primary data structure for all examples.
- [[entities/pandas]] — read_csv/merge/concat/fillna/dropna/replace are all Pandas.
- [[entities/numpy]] — NaN sentinel and random matrices.
- [[entities/scipy]] — listed among the Python data-science libraries.
- [[entities/matplotlib]] — listed among the visualization libraries used downstream.
- [[entities/beautiful-soup]] — used for HTML parsing.

## Source Metadata

- Source type: book chapter
- Book title: Data Analysis and Visualizations with Python
- Chapter: 5 — Data Gathering and Cleaning
- File path: raw/DataAnalysisAndVisualizationsPython/_txt/08-chapter-5-data-gathering-and-cleaning.txt
- Author: Ossama Embarak
