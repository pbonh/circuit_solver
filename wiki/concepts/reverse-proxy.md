---
title: "Reverse Proxy"
type: concept
tags: [web, deployment, networking]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt"]
confidence: high
---

## Definition

A reverse proxy is a server that sits in front of one or more backend application servers and forwards client requests to them, returning the responses back to the client. NGINX functioning in front of GUNICORN is the canonical Python web-app deployment pattern.

## How It Works

The reverse proxy terminates the client connection, optionally handling TLS, static-file serving, caching, and access control. It then opens an upstream connection to the backend (over TCP port or Unix domain socket) and forwards the request. The book's NGINX configuration uses `proxy_pass http://unix:/path/app.sock;` for socket connections and `proxy_pass http://127.0.0.1:5000;` for TCP connections, routing different URL prefixes (`/hello`, `/atads`) to different backend services.

## Key Parameters

- Upstream backend address (socket or TCP)
- URL routing rules (`location` blocks)
- TLS termination configuration
- Buffering / timeout settings

## When To Use

- Exposing a Python WSGI app to the public Internet safely
- Serving multiple apps on a single host through URL prefixes
- Adding TLS, caching, or rate-limiting in front of an application server
- Decoupling client-facing concerns (HTTPS, static files) from app logic

## Risks & Pitfalls

- Trailing slash and `proxy_pass` URL termination semantics differ subtly (the book's `:/` workaround for `/hello`)
- Socket file permissions must allow both proxy and backend users
- Header forwarding (`X-Forwarded-For`, `Host`) needs explicit configuration
- Misconfigured timeouts disconnect slow long-polling connections

## Related Concepts

- [[concepts/wsgi]]
- [[concepts/systemd-service]]
- [[entities/nginx]]
- [[entities/gunicorn]]

## Sources

- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
