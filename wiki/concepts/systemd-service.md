---
title: Systemd Service
type: claim
id: claim-systemd-service
tags:
- unix
- deployment
- systemd
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/12-chapter-8-deploying-your-project-as-a-unix-service.txt
confidence:
  base: 0.85
---

## Definition

A systemd service is a long-running daemon managed by the systemd init system on modern Linux distributions. Services are described by `.service` unit files (typically in `/etc/systemd/system/`) and controlled with `systemctl start|stop|enable|disable|status`.

## How It Works

A unit file has three primary blocks: `[Unit]` (description, dependencies like `After=network.target`), `[Service]` (user, group, working directory, `ExecStart` command), and `[Install]` (`WantedBy=multi-user.target` so the service starts on boot). The book's `atads.service` uses `ExecStart` to run `gunicorn --bind 0.0.0.0:5000 atads:server` from inside the project's virtual environment.

## Key Parameters

- `ExecStart` command (typically a `gunicorn`/binary inside a venv)
- `User` / `Group` running the process
- `WorkingDirectory`
- Restart policy (`Restart=on-failure`)
- `WantedBy` install target

## When To Use

- Auto-starting a dashboard on server boot
- Running multi-worker WSGI apps as background daemons
- Managing application lifecycle through standard Unix tooling
- Centralized logging via journald

## Risks & Pitfalls

- File and socket permissions must match the configured user/group
- Forgetting `systemctl daemon-reload` after editing a unit file
- `Restart=always` masking persistent failures
- Path resolution differs from interactive shells (no profile sourced)

## Related Concepts

- [[concepts/reverse-proxy]]
- [[concepts/wsgi]]
- [[entities/systemd]]
- [[entities/gunicorn]]
- [[entities/ubuntu]]

## Sources

- [[summaries/prototyping-python-dashboards-12-chapter-8-deploying-your-project-as-a-unix-service]]
