---
title: "Callback"
type: concept
tags: [python, dash, reactive-programming, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt"]
confidence: high
---

## Definition

A callback is a function invoked by a framework when a registered event occurs. In Dash applications, callbacks are decorated Python functions that consume values from layout components (Inputs) and return new values to other layout components (Outputs), making the dashboard reactive.

## How It Works

In Dash:
1. A `@callback` decorator is placed above an update function.
2. The decorator's `Output(component_id, prop)` declares what the function returns; `Input(component_id, prop)` declares what triggers it.
3. When any Input value changes, the framework calls the function with current Input values; whatever it returns is assigned to the Output's prop.
4. Multiple Inputs require matching positional arguments in the update function.

## Key Parameters

- Output component_id + property (often "figure" for graphs, "value" for sliders)
- Input component_id + property
- State (Input-like but does not trigger; only read on callback)
- Argument order in update function

## When To Use

- Connecting a widget's value to a chart's data
- Cascading dynamic menus where one menu's items depend on another
- Coordinating multi-panel dashboards from a shared set of controls

## Risks & Pitfalls

- Misordered Input declarations vs. update function parameters silently corrupt values
- Tangled callback graphs are hard to trace
- Single callback driving many outputs can refetch data unnecessarily; splitting callbacks improves performance but requires careful ID management

## Related Concepts

- [[concepts/reactive-programming]]
- [[concepts/python-decorator]]
- [[concepts/dashboard]]
- [[entities/dash]]

## Sources

- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables]]
