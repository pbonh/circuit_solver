---
title: "UFW (Uncomplicated Firewall)"
type: entity
tags: [unix, security, networking]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt"]
confidence: medium
---

## Overview

UFW (Uncomplicated Firewall) is the default firewall management tool on Ubuntu, wrapping iptables/netfilter with a simpler CLI. The book uses it to open the development port (`sudo ufw allow 5000`) and to restrict the public surface to essential ports in production.

## Characteristics

- Allow/deny rules expressed in plain English (`ufw allow 22`, `ufw deny from 1.2.3.4`).
- Status query: `ufw status`.
- Enabling/disabling: `ufw enable` / `ufw disable`.
- Persistent across reboots once enabled.

## Common Strategies

- Open only the development ports used by Flask/GUNICORN/uWSGI while iterating.
- In production keep SSH, HTTPS, DNS, and (optionally) FTP open; close everything else.
- Combine with Fail2ban for time-bounded bans on suspicious IPs.
- Pair with Let's Encrypt-managed HTTPS so the firewall surface includes 443 instead of 80 alone.

## Related Entities

- [[entities/ubuntu]]
- [[entities/fail2ban]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
