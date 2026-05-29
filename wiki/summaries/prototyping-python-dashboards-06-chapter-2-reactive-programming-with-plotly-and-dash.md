---
title: 'Prototyping Python Dashboards — Chapter 2: Reactive Programming with PLOTLY
  and DASH'
type: source
id: summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash
kind: publication
tags:
- python
- plotly
- dash
- reactive-programming
- visualization
- dashboard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt
---

## Key Points

- Reactive programming wraps core algorithms in a framework that responds to keyboard and mouse input; the wrapper code is reusable across projects.
- PLOTLY produces interactive, browser-rendered charts with built-in zoom, pan, save, and cursor-driven point inspection.
- Core PLOTLY routines: `go.Figure()` creates a figure, `add_trace()` adds curves/points, `update_layout()` controls layout, `add_annotation()` adds text, and `add_vrect()` adds vertical rectangular regions.
- "Paper coordinates" treat the chart region as 0-1 on both axes regardless of actual data scaling, so annotations stay placed when data ranges change.
- An OOD version of a PLOTLY example wraps the chart construction in a class (`my_chart`) with `__init__(self, x, y)` that builds and displays the figure as soon as the object is instantiated.
- DASH is a Python framework for interactive web applications that does not require HTML/CSS/JavaScript expertise, though some CSS becomes helpful in practice.
- A DASH application consists of: an `app = dash.Dash()` instance, an `app.layout` containing components, `@callback` decorators tracking inputs/outputs, an `update_figure()` function rebuilding charts, and an `app.run()` call.
- Layout components (e.g., `dcc.Graph`, `dcc.RangeSlider`) require an `id` that callbacks reference via `Input(id, type)` and `Output(id, type)` specifiers; the order of `Input()` calls must match the update function's argument order.
- Python decorators (denoted `@`) are mechanisms that enhance an existing function without editing it; `@callback` is one such decorator.
- `RangeSlider` returns a list of two values which the callback wires into a dataframe filter for chart updates; `html.Div` in `app.layout` shows DASH does generate HTML behind the scenes.

## Relevant Concepts

- [[concepts/reactive-programming]] — the paradigm the chapter introduces and uses throughout the book.
- [[entities/plotly]] — graphics library producing the chart objects.
- [[entities/dash]] — interactive web framework layering callbacks over PLOTLY.
- [[concepts/callback]] — Dash mechanism tying widget inputs to figure outputs.
- [[concepts/python-decorator]] — language feature implementing `@callback`.
- [[concepts/object-oriented-design]] — recommended class-based organization for chart code.
- [[concepts/dataframe]] — input data type filtered by callbacks.
- [[concepts/dashboard]] — the artifact this chapter teaches you to build.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 2 — Reactive Programming with PLOTLY and DASH
- File path: raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt
- Author: Padraig Houlahan
