---
title: Prototyping Python Dashboards — Introduction
type: source
id: source-prototyping-python-dashboards-04-introduction
kind: derived-summary
tags:
- python
- dashboard
- visualization
- prototyping
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/04-introduction.txt
---

## Key Points

- The book is written for researchers, grad students, and faculty who code as a necessary skill rather than a primary discipline, and who need complete worked examples of building and deploying a dashboard.
- Dashboards encapsulate data management, display, and access in a self-contained project that allows remote collaboration and central data stewardship.
- A web-based dashboard simplifies development (single platform) and solves distribution (any browser, anywhere on the Internet).
- The book builds an end-to-end example around the FAA's ATADS dataset, which tracks daily flight operations at 500+ US airports including civilian/military, air carrier/air taxi, local/itinerant, and IFR vs. VFR categories.
- The toolchain used is Python with PLOTLY and DASH (both MIT-licensed), developed in the Spyder IDE under Anaconda, with NGINX and GUNICORN handling deployment.
- The author emphasizes prototyping philosophy: functionality first, brute-force solutions, regular refactoring, and self-documenting code; dashboards should evolve incrementally from a minimal "display one time series" goal.
- Organizational advice: keep a hardcover paper notebook for project notes with a table of contents and dated entries; periodic working-folder backups precede adopting Git.
- The book provides a "complete solution" including data scraping, format conversion, reactive programming, CSS layout, Unix server setup, and a WordPress web portal.

## Relevant Concepts

- [[concepts/dashboard]] — the central artifact the book teaches you to build, deploy, and share.
- [[concepts/data-visualization]] — the underlying motivation: humans are visual creatures and graphics reveal hidden features.
- [[concepts/reactive-programming]] — paradigm that supports dashboard interactivity through user events.
- [[entities/plotly]] — graphics library used throughout the book.
- [[entities/dash]] — interactive dashboard framework layered atop Plotly.
- [[concepts/python]] — the implementation language for the entire stack.
- [[concepts/prototyping]] — design philosophy emphasizing functionality, iteration, and self-documenting code.
- [[concepts/object-oriented-design]] — recommended for sharing code across team members.
- [[entities/atads-dataset]] — FAA dataset of daily airport operations counts used as the example throughout the book.
- [[entities/nginx]] — lightweight web server used for production deployment.
- [[entities/gunicorn]] — WSGI server hosting the Python application behind NGINX.
- [[entities/spyder-ide]] — author's IDE choice, run under Anaconda.

## Source Metadata

- Source type: book chapter (front matter)
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: Introduction
- File path: raw/PrototypingPythonDashboards/_txt/04-introduction.txt
- Author: Padraig Houlahan
