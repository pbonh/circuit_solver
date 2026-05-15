---
title: "Cron"
type: concept
tags: [unix, automation, scheduling]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/17-appendix-a-utilities-for-managing-atads-data.txt"]
confidence: low
---

## Definition

Cron is the Unix time-based job scheduler used to run commands or scripts at recurring intervals defined in a crontab file. The book suggests using cron to automate the monthly refresh of the ATADS data pipeline.

## How It Works

Each user (and the system) has a crontab; entries follow `minute hour day month weekday command`. The cron daemon wakes once per minute, checks crontab entries, and forks a process to run any matching command. Output goes to the user's mail unless redirected. For browser-driven scraping jobs, a virtual desktop or headless mode is required because cron jobs lack an interactive display.

## Key Parameters

- Schedule fields (minute, hour, day-of-month, month, day-of-week)
- Environment variables in the crontab header
- stdout/stderr redirection
- Path resolution (cron does not source interactive profiles)

## When To Use

- Refreshing scraped datasets nightly or monthly
- Running periodic backups
- Rotating log files
- Generating reports on a regular cadence

## Risks & Pitfalls

- Cron's environment is minimal; absolute paths and explicit env vars are essential
- Silent failures unless output is captured and monitored
- Concurrent runs if a job exceeds its schedule interval
- Modern systems may prefer systemd timers over cron for tighter integration

## Related Concepts

- [[concepts/screen-scraping]]
- [[concepts/systemd-service]]
- [[concepts/data-cleaning]]

## Sources

- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
