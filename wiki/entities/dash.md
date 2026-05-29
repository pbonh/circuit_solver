---
title: Dash
type: entity
id: entity-dash
tags:
- python
- dashboard
- web
- plotly
- reactive-programming
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt
---

## Overview

Dash is a Python framework from Plotly for building interactive web applications without writing JavaScript, HTML, or CSS by hand (though CSS becomes helpful in practice). Dash apps combine layout components, Plotly figures, and callback functions into a single Python codebase that renders in the browser.

## Characteristics

- MIT-licensed open-source framework distributed by Plotly.
- App structure: `app = dash.Dash()`, an `app.layout` of components, `@callback`-decorated update functions wiring Inputs to Outputs, and `app.run()`.
- Layout components live in `dash.html` (semantic wrappers like `html.Div`, `html.Label`) and `dash.dcc` (controls and graphs: `dcc.Graph`, `dcc.RangeSlider`, `dcc.Dropdown`).
- A `DataTable` component supports tabular display with built-in CSV export.
- Internally builds on Flask, so the Dash app exposes a `server` attribute (a Flask instance) for production WSGI deployment.

## Common Strategies

- Group widget creation and figure creation into separate classes (`atads_layout`, `atads_figures`) for maintainability.
- Use unique component `id`s and wire them via `Input()`/`Output()` declarations whose order matches the update-function arguments.
- For Dash apps deployed via gunicorn, expose `server = Flask(__name__)` and instantiate Dash with `dash.Dash(__name__, server=server)`; the WSGI entry point references `server`, not `app`.
- Prefer TCP-bound deployment over Unix sockets for Dash specifically.
- Use a CSS file co-located with the app for grid-based layout via `className` selectors.

## Related Entities

- [[entities/plotly]]
- [[entities/flask]]
- [[entities/gunicorn]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
- [[summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables]]
