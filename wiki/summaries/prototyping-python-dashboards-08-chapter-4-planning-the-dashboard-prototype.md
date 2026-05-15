---
title: "Prototyping Python Dashboards — Chapter 4: Planning the Dashboard Prototype"
type: summary
tags: [python, dashboard, prototyping, regression, design, architecture]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/08-chapter-4-planning-the-dashboard-prototype.txt"]
confidence: high
---

## Key Points

- The project goal is to expose the ATADS daily airport operations data to airline/airport managers, researchers, and students through an interactive, accessible dashboard.
- Project elements break into four areas: data import and manipulation, reactive code development, Linux service creation, and a Unix web-server deployment (NGINX), plus a WordPress-based portal for documentation/feedback.
- Behind the scenes the system needs Nginx (reverse proxy), GUNICORN (WSGI server), Flask (WSGI app), and MySQL (for WordPress) — all of which can run on a single virtual Ubuntu host but can be split for scaling.
- The Flask vs. GUNICORN analogy: Flask is a single-engine plane fine for development, while GUNICORN is a commercial jet for production with multi-worker concurrency under heavy load.
- Eight dashboard capabilities to deliver: airport selection, year-range selection, operational-metric selection, time-series graph display, smoothing, overlapping multi-airport graphs, linear/quadratic trend display, and periodicity exploration.
- Trends are extracted via polynomial curve fitting: linear `y = a0 + a1*t` and quadratic `y = a0 + a1*t + a2*t^2` provide easy-to-interpret summaries; coefficients are determined via least-squares regression encapsulated in Python's `poly` libraries.
- Linear coefficient `a1` is the slope/trend/rate; `a0` is the intercept; for quadratic, slope changes with t as `a1 + 2*a2*t`.
- The author cautions against high-order polynomial fits because they overfit and produce uninterpretable coefficients — linear and quadratic are the practical choices.
- Code is split across three Python files for maintainability: `atads.py` (top-level), `atads_figures.py` (data + chart construction), and `atads_layout.py` (widgets); each non-trivial component is encapsulated in a class.

## Relevant Concepts

- [[concepts/dashboard]] — the artifact being designed.
- [[concepts/prototyping]] — iterative design philosophy applied throughout the project.
- [[concepts/reactive-programming]] — needed for menu/slider interactivity.
- [[concepts/regression]] — curve fitting used to extract trends from time series.
- [[concepts/polynomial]] — linear/quadratic curve families used for trend display.
- [[concepts/time-series]] — the form the airport data takes.
- [[concepts/object-oriented-design]] — drives the three-file class-based layout.
- [[entities/nginx]] — outward-facing web server.
- [[entities/gunicorn]] — WSGI server bridging NGINX to Python.
- [[entities/flask]] — Python WSGI web framework used during development.
- [[entities/wordpress]] — blogging/portal software for documentation.
- [[entities/mysql]] — database backing WordPress.
- [[entities/atads-dataset]] — the data the dashboard surfaces.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 4 — Planning the Dashboard Prototype
- File path: raw/PrototypingPythonDashboards/_txt/08-chapter-4-planning-the-dashboard-prototype.txt
- Author: Padraig Houlahan
