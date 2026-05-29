---
title: Reactive Programming
type: claim
id: claim-reactive-programming
tags:
- python
- dashboard
- reactive-programming
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/06-chapter-2-reactive-programming-with-plotly-and-dash.txt
confidence:
  base: 0.85
---

## Definition

Reactive programming is a paradigm in which a program is organized around responding to discrete events (keyboard, mouse, network, timer) rather than executing a linear top-to-bottom procedure. In dashboards, reactive code redraws charts whenever an input widget changes.

## How It Works

The core algorithm is wrapped in a framework that:
1. Defines layout components (widgets, charts) with identifiable IDs.
2. Registers callbacks that subscribe to specific input events and produce outputs targeted at other components.
3. Runs a main loop that listens for events and invokes the matching callbacks, threading inputs through update functions and pushing returned values back to the targeted outputs.

In Dash, callbacks are decorated Python functions whose `Input()`/`Output()` declarations bind to layout component IDs.

## Key Parameters

- Event sources (widget value changes, timer ticks, server pushes)
- Callback wiring (input/output IDs, data types)
- Update-function ordering (arguments must match Input declaration order)
- Debouncing or throttling for rapidly changing inputs

## When To Use

- GUI applications that must respond to user actions
- Web dashboards where chart content depends on user-selected filters
- Real-time monitoring views
- Any application where the linear FORTRAN-style "process file from start to end" pattern no longer fits

## Risks & Pitfalls

- Callback graphs can become tangled and hard to debug if not organized
- Ordering of Input arguments must match update-function parameters or values silently misalign
- Frequent rebuilds of expensive figures can stall UI responsiveness
- Multiple cascading callbacks can fire on a single user change, doing redundant work

## Related Concepts

- [[concepts/callback]]
- [[concepts/python-decorator]]
- [[concepts/dashboard]]
- [[entities/dash]]
- [[entities/plotly]]

## Sources

- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
