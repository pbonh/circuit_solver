---
title: Fail2ban
type: entity
id: entities/fail2ban
tags:
- unix
- security
- intrusion-prevention
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt
---

## Overview

Fail2ban is a log-scanning intrusion-prevention tool that monitors authentication and web logs for repeated failures and temporarily bans the offending IP addresses at the firewall level.

## Characteristics

- Reads log files (`/var/log/auth.log`, NGINX access logs, etc.) and matches them against per-service filter expressions.
- Configurable thresholds: failures over a time window trigger a ban.
- Ban durations can range from minutes to indefinite.
- Actions typically modify iptables/UFW rules directly.

## Common Strategies

- Pair with UFW so bans are immediately enforced.
- Tune thresholds carefully to avoid banning yourself.
- Combine with HTTPS-only access (Let's Encrypt) and an institutional firewall for layered defense.
- Monitor jail status with `fail2ban-client status`.

## Related Entities

- [[entities/ubuntu]]
- [[entities/ufw]]
- [[entities/letsencrypt]]
- [[entities/nginx]]

## Sources

- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
