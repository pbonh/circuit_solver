---
title: HTML
type: claim
id: concepts/html
tags:
- web
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/14-chapter-10-creating-a-web-portal.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

HTML (HyperText Markup Language) is the markup language that structures web pages. Elements (`<div>`, `<p>`, `<a>`, `<table>`) form a hierarchical document tree that browsers render and that scripts and stylesheets can target.

## How It Works

A web page is a tree of HTML elements with attributes (class, id, src). Browsers parse the markup, build the DOM, and render it according to associated CSS rules and JavaScript. Dash applications generate HTML programmatically through Python wrappers like `html.Div`, `html.Label`, `html.Br`; the book also demonstrates a hand-written minimal HTML portal page that links to the deployed dashboards.

## Key Parameters

- Element tags and attributes
- Document hierarchy (parent/child/sibling relationships)
- Semantic vs. presentational elements
- Linking to external stylesheets and scripts

## When To Use

- Authoring landing pages and portals
- Building static documentation around a dashboard
- Wrapping Dash widgets in semantic structure
- Embedding tables of summary information

## Risks & Pitfalls

- Inconsistent rendering across browsers (mitigated by modern CSS and HTML5)
- Markup-vs-structure confusion when CSS frameworks invade semantic naming
- Accessibility regressions when semantic elements are replaced with generic `div`s

## Related Concepts

- [[concepts/css]]
- [[concepts/css-grid]]
- [[concepts/dashboard]]
- [[concepts/web-portal]]

## Sources

- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
