---
title: Ubuntu
type: entity
id: entity-ubuntu
tags:
- unix
- linux
- deployment
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
---

## Overview

Ubuntu is a Debian-derived Linux distribution widely used for server deployments. The book hosts its ATADS dashboard on an Ubuntu server (a cheap "droplet" instance) running the full NGINX + GUNICORN + WordPress + MySQL stack.

## Characteristics

- Debian-family package manager (`apt`).
- Long-term-support (LTS) releases on a two-year cadence.
- Default firewall is UFW; default init system is systemd.
- Provides Python 3 in base packages plus easy installation of `venv`, `pip`, and supporting libraries via `apt install`.

## Common Strategies

- Use a small, throwaway droplet/VPS as a sandbox to build out each layer (Python, Flask, GUNICORN, NGINX) progressively.
- Manage Python isolation with venv per project.
- Configure UFW to permit only essential ports (SSH/HTTPS/DNS/FTP).
- Install Fail2ban and Let's Encrypt for layered security and HTTPS.

## Related Entities

- [[entities/nginx]]
- [[entities/gunicorn]]
- [[entities/systemd]]
- [[entities/ufw]]
- [[entities/fail2ban]]
- [[entities/letsencrypt]]

## Sources

- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
