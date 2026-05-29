---
title: systemd
type: entity
id: entities/systemd
tags:
- unix
- deployment
- init-system
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt
---

## Overview

systemd is the init system and service manager used by most modern Linux distributions (including Ubuntu). It supervises long-running services described by unit files, exposes them via the `systemctl` CLI, and integrates logging via journald.

## Characteristics

- Unit files live under `/etc/systemd/system/` (operator) or `/lib/systemd/system/` (packaged).
- Unit types include `service`, `socket`, `timer`, `target`.
- Services advertise dependencies (`After=`, `Requires=`) and install targets (`WantedBy=multi-user.target`).
- The `systemctl` command supports `start`, `stop`, `enable`, `disable`, `restart`, and `status` actions.

## Common Strategies

- Wrap GUNICORN invocations in a systemd unit so they auto-start on boot.
- Use `User=` and `Group=` to drop privileges from the deploying root account.
- Read failure context via `systemctl status <name>` (which integrates the most recent log lines from journald).
- Reload the daemon configuration with `systemctl daemon-reload` after editing unit files.

## Related Entities

- [[entities/gunicorn]]
- [[entities/nginx]]
- [[entities/ubuntu]]

## Sources

- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
