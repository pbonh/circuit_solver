---
title: Flask
type: entity
id: entity-flask
tags:
- python
- web
- wsgi
- dashboard
- framework
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
---

## Overview

Flask is a lightweight Python WSGI web framework used both directly and as the engine that Dash builds on top of. In *Prototyping Python Dashboards* it appears in the Hello World deployment example and, more importantly, as the server instance exposed by the Dash dashboard for production hosting.

## Characteristics

- Minimal core with routing via decorators (`@app.route("/")`).
- Single-threaded development server (not for production).
- Exposes a callable WSGI `app` (or `server`) that GUNICORN/uWSGI can host.
- Built-in debug reloader (`app.run(debug=True, host='0.0.0.0')`).

## Common Strategies

- Use Flask's development server only during local development; switch to GUNICORN for deployment.
- For Dash apps, explicitly create `server = Flask(__name__)` and pass it to `dash.Dash(server=server)` so a single Flask instance can be hosted as a WSGI app.
- Import `server` (not `app`) in `wsgi.py` for Dash deployments.
- Bind to `0.0.0.0` instead of `127.0.0.1` when remote access is required.

## Related Entities

- [[entities/dash]]
- [[entities/gunicorn]]
- [[entities/uwsgi]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
