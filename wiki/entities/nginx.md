---
title: "NGINX"
type: entity
tags: [web, reverse-proxy, deployment]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt"]
confidence: high
---

## Overview

NGINX is a lightweight, high-performance web server and reverse proxy. In the book it terminates client HTTP(S) connections and forwards dynamic requests upstream to GUNICORN, while also serving static portal pages.

## Characteristics

- Event-driven architecture with low per-connection overhead.
- Configuration via `nginx.conf` plus per-site server blocks in `/etc/nginx/sites-available` (symlinked into `sites-enabled`).
- Supports both TCP and Unix-socket upstreams (`proxy_pass`).
- Handles TLS termination (typically with Let's Encrypt certificates).
- Routes by URL prefix (`location /hello`, `location /atads`).

## Common Strategies

- Place NGINX in front of GUNICORN as a reverse proxy.
- Use Unix sockets for multiple lightweight services (avoids port collisions); the book reserves TCP for Dash because it has been less reliable over sockets.
- Append `:/` to `proxy_pass` URLs when changing a `location` from `/` to a sub-path to avoid 404s.
- Pair with Let's Encrypt for free HTTPS and configure NGINX to rewrite HTTP requests as HTTPS.
- Restrict open ports via UFW to SSH, HTTPS, DNS, and (sparingly) FTP.

## Related Entities

- [[entities/gunicorn]]
- [[entities/flask]]
- [[entities/dash]]
- [[entities/letsencrypt]]
- [[entities/ufw]]
- [[entities/ubuntu]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
