---
title: Python Virtual Environment
type: claim
id: concepts/virtual-environment
tags:
- python
- deployment
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/11-chapter-7-hosting-an-application-on-a-unix-server.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A Python virtual environment is a self-contained directory that isolates a project's Python interpreter and dependencies from the system Python and from other projects. Created with the standard-library `venv` package, it lets dashboards install specific library versions without affecting the rest of the OS.

## How It Works

`python -m venv hwenv` creates a directory `hwenv/` containing a private `bin/`, `lib/`, and an `activate` script. Sourcing `hwenv/bin/activate` modifies `PATH` and `PYTHONHOME` so that `python` and `pip` resolve to the venv's copies. Packages installed via `pip install ...` go into the venv's `site-packages`. The `deactivate` command unwinds the shell changes.

## Key Parameters

- Activation script path (`bin/activate` on Unix, `Scripts\activate` on Windows)
- Python interpreter version baked into the venv
- Whether system site-packages are inherited (`--system-site-packages` flag)

## When To Use

- Any nontrivial Python project to lock its dependencies
- Deploying separate dashboards on the same host
- Avoiding library-version conflicts between projects
- Reproducing builds across machines via `requirements.txt`

## Risks & Pitfalls

- Forgetting to activate the venv leads to installing into the system Python
- Moving a venv directory breaks it (paths are baked in); recreate instead
- Different projects on different Python minor versions still need separate venvs
- venvs do not isolate non-Python system libraries

## Related Concepts

- [[concepts/python]]
- [[concepts/wsgi]]
- [[entities/flask]]
- [[entities/gunicorn]]

## Sources

- [[summaries/prototyping-python-dashboards-11-chapter-7-hosting-an-application-on-a-unix-server]]
