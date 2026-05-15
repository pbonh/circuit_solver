---
title: "GUNICORN"
type: entity
tags: [python, web, wsgi, deployment]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt"]
confidence: high
---

## Overview

GUNICORN ("Green Unicorn") is a WSGI HTTP server for Python applications. In the book it hosts the Flask/Dash dashboards behind NGINX, providing multi-worker concurrency suitable for production deployment.

## Characteristics

- Pre-fork worker model: a master process forks multiple worker processes that share the listening socket.
- Binds to a TCP port or Unix socket (`--bind 0.0.0.0:5000` or `--bind unix:/path/app.sock`).
- Accepts a WSGI app reference such as `wsgi:app` (equivalent to `from wsgi import app`).
- Lightweight compared to alternatives; well-suited to small-team deployments.
- Not intended as a public-facing server; sits behind NGINX in a reverse-proxy configuration.

## Common Strategies

- Install inside the project's virtual environment so the venv's `gunicorn` binary is used.
- For Dash apps, point gunicorn at `atads:server` (the Flask instance Dash wraps), not `atads:app`.
- Use Unix sockets to avoid port conflicts between multiple services.
- Invoke from a systemd unit file's `ExecStart` so the service auto-restarts on reboot.
- Tune worker count to roughly `2 * CPU + 1` for IO-bound dashboards.

## Related Entities

- [[entities/nginx]]
- [[entities/flask]]
- [[entities/dash]]
- [[entities/uwsgi]]
- [[entities/systemd]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
