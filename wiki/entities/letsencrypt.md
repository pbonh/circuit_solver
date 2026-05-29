---
title: Let's Encrypt
type: entity
id: entity-letsencrypt
tags:
- web
- security
- https
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt
---

## Overview

Let's Encrypt is a free, automated certificate authority that issues TLS certificates for HTTPS via the ACME protocol. The book recommends it for upgrading an HTTP-only server to HTTPS at no cost.

## Characteristics

- Free certificates, 90-day lifetimes, automated renewal.
- ACME protocol verifies domain ownership via HTTP-01 or DNS-01 challenges.
- Certbot is the canonical ACME client on Ubuntu.
- Plugins exist for NGINX and Apache that modify server blocks automatically.

## Common Strategies

- Install certbot via `apt`, run with the NGINX plugin so it edits the server blocks in place.
- Opt into automatic HTTP→HTTPS rewrites during certbot installation.
- Schedule renewal via the systemd timer that certbot installs.
- Donate when using in non-hobby production contexts.

## Related Entities

- [[entities/nginx]]
- [[entities/ubuntu]]
- [[entities/ufw]]

## Sources

- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
