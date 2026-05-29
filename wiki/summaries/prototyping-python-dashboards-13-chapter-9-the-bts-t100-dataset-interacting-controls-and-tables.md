---
title: 'Prototyping Python Dashboards — Chapter 9: The BTS T100 Dataset — Interacting
  Controls and Tables'
type: source
id: summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables
kind: publication
tags:
- python
- dashboard
- dash
- plotly
- callback
- dataframe
- visualization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables.txt
---

## Key Points

- The Bureau of Transportation Statistics (BTS) T100 Domestic Segment dataset (T100dm) reports passenger, mail, and cargo volumes between US airports, submitted monthly by airlines.
- Unlike ATADS (per-airport, standalone), T100dm is relational — connecting airports through segment-level traffic — which forces the dashboard's controls to interact dynamically.
- Five operating modes are defined: H (Hub), S (Segment), SC (Segment by Carrier), HC (Hub Carrier), and C (Carrier); each mode dictates which of the Hub/Connecting Airport/Carrier menus are populated and which remain blank.
- Cascading menus are implemented in a three-step callback: 1) read the mode parameter, 2) set boolean flags `hmenu`/`smenu`/`cmenu` and extract the relevant filtered dataframes via `get_hub_df`/`get_segment_df`/`get_carrier_df`/`get_hub_carrier_df`/`get_segment_carrier_df`, 3) populate or disable each menu based on flag state.
- Histograms (rather than line plots) render the monthly traffic via Plotly's `go.Bar()` trace; a single first-order trendline option is supported through `get_poly(1)`.
- `get_totals_by_month()` aggregates a filtered dataframe to monthly totals (with a 1-12 month column) and is reused for hub, segment, and carrier modes.
- A Dash `DataTable` displays the chart's underlying data and offers a built-in Export button so users can download the data for use in Excel.
- The author cautions against overusing pandas' aggregation/grouping/splitting features in production code — choose the simplest, most brute-force solution that still reads clearly when revisited months later.

## Relevant Concepts

- [[concepts/dashboard]] — the BTS prototype extending the ATADS template.
- [[entities/dash]] — framework providing `DataTable` and cascading callbacks.
- [[concepts/callback]] — multi-input multi-output callbacks driving menu population.
- [[entities/plotly]] — `go.Bar()` provides the histogram traces.
- [[concepts/dataframe]] — filtered subsets returned by the various `get_*_df()` utilities.
- [[entities/pandas]] — grouping/aggregation behind `get_totals_by_month()`.
- [[entities/bts-t100-dataset]] — the relational airport traffic dataset central to this chapter.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 9 — The BTS T100 Dataset: Interacting Controls and Tables
- File path: raw/PrototypingPythonDashboards/_txt/13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables.txt
- Author: Padraig Houlahan
