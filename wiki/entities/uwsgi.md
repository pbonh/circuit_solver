---
title: "uWSGI"
type: entity
tags: [python, web, wsgi, deployment]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt"]
confidence: medium
---

## Overview

uWSGI is a multi-protocol application server commonly used to host Python WSGI applications. Despite its name, it is a server, not a specification (which causes the regular confusion with the WSGI standard itself).

## Characteristics

- Supports many protocols: HTTP, uwsgi (its native binary protocol), and WSGI.
- Process-and-thread worker model, similar to GUNICORN.
- Configured via command-line flags, INI files, or YAML.
- The book uses it briefly to illustrate the WSGI server role before settling on GUNICORN.

## Common Strategies

- Install into the project's venv via `pip install uwsgi`.
- Run with `uwsgi --http 0.0.0.0:5000 -w wsgi:app` to expose the Flask app over HTTP at port 5000.
- Use `wsgi.py` as the entry point that imports the app instance.

## Related Entities

- [[entities/gunicorn]]
- [[entities/flask]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
