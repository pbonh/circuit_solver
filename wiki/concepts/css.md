---
title: CSS
type: claim
id: claim-css
tags:
- web
- css
- layout
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/09-chapter-5-our-first-dashboard.txt
confidence:
  base: 0.85
---

## Definition

CSS (Cascading Style Sheets) is a declarative language for styling HTML elements. It controls layout, sizing, color, typography, borders, and animation, and is essential for arranging Plotly/Dash widgets and charts into a polished dashboard.

## How It Works

CSS rules pair selectors (`.banner`, `#chart`, `div > p`) with property/value blocks. The browser cascades and inherits rules, applying matched declarations to DOM elements. Dash exposes `className=` parameters on `html.Div` that the application's CSS file targets. The book uses CSS grids to define rows/columns and class blocks like `.banner{}`, `.instructions{}`, and `.chart{}` to size and position individual panels.

## Key Parameters

- Selectors (class, id, descendant, combinator)
- Box-model properties (width, height, padding, margin, border)
- Layout systems (grid, flex, block, inline)
- Cascading specificity rules

## When To Use

- Controlling dashboard layout in Plotly/Dash apps
- Producing visually consistent panels (rounded corners, colors, padding)
- Overriding WordPress theme defaults
- Supporting responsive layouts on different screen widths

## Risks & Pitfalls

- Browsers interpret CSS inconsistently; complex rules may render differently
- Stray CSS files in subdirectories can be auto-loaded by Dash and silently override styling
- Specificity wars can be hard to debug
- Premature unification of common rules can create maintenance brittleness for small projects

## Related Concepts

- [[concepts/css-grid]]
- [[concepts/html]]
- [[concepts/dashboard]]
- [[entities/dash]]

## Sources

- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
