---
title: WSGI
type: claim
id: claim-wsgi
tags:
- python
- web
- deployment
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
confidence:
  base: 0.85
---

## Definition

WSGI (Web Server Gateway Interface, PEP 3333) is the Python specification that defines how a web server forwards HTTP requests to a Python application and receives responses back. It is the bridge that lets servers like NGINX (via GUNICORN or uWSGI) talk to Python frameworks like Flask and Dash.

## How It Works

A WSGI application is any callable accepting `(environ, start_response)` and returning an iterable of byte strings. Frameworks like Flask expose an instance (e.g., `app`) that fulfills this protocol. A WSGI server (GUNICORN, uWSGI) hosts one or more worker processes that import the application and dispatch incoming requests to it. NGINX, in front of GUNICORN, handles TLS, static files, and routing, forwarding dynamic requests over either a TCP port or a Unix domain socket.

## Key Parameters

- Bind address (TCP `host:port` or Unix socket path)
- Worker count
- WSGI app reference (`module:attribute`, e.g., `wsgi:app`)
- Timeout, keepalive, and reload settings

## When To Use

- Hosting a Flask or Dash application on a Unix server
- Standardizing how web servers integrate with Python apps
- Replacing the development server with a production WSGI server

## Risks & Pitfalls

- WSGI is synchronous; long-running or websocket connections may need ASGI instead
- Worker model means in-process state is per-worker
- Confusing nomenclature: WSGI is a spec, uWSGI is a server (and so is GUNICORN)

## Related Concepts

- [[concepts/reverse-proxy]]
- [[concepts/virtual-environment]]
- [[entities/flask]]
- [[entities/gunicorn]]
- [[entities/uwsgi]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
