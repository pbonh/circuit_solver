---
title: "Chrome Developer Tools"
type: entity
tags: [web, debugging, css, devtools]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/07-chapter-3-working-with-online-data.txt"]
confidence: high
---

## Overview

Chrome Developer Tools (DevTools) is the browser-integrated suite for inspecting and debugging web pages. The book uses it both to identify form element names while screen-scraping the FAA ATADS site and to diagnose CSS-padding issues in the WordPress portal.

## Characteristics

- Opened via the three-dots menu → More Tools → Developer Tools (or `Ctrl+Shift+I` / `Cmd+Opt+I`).
- Panels include Elements, Styles, Network, Console, Performance, Application, Sources.
- The Elements panel hovers over DOM nodes and highlights the matching region in the page; clicking expands hidden layers via small triangles.
- The Styles panel shows applied CSS rules and lets you toggle individual declarations on/off with checkboxes.

## Common Strategies

- Hover the Elements tree to find the DOM node corresponding to a visual region; click to reveal its CSS.
- Toggle Styles checkboxes to test which rule is responsible for a quirk before committing the override.
- Read the form-field `name` attribute (e.g., `fm_r`) before writing a Selenium selector.
- Inject experimental CSS into the Styles panel and then move successful rules into the project's stylesheet.

## Related Entities

- [[entities/selenium]]
- [[entities/chromedriver]]
- [[entities/wordpress]]

## Sources

- [[summaries/prototyping-python-dashboards-14-chapter-10-creating-a-web-portal]]
