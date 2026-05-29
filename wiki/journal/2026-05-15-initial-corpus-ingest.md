---
title: Initial Corpus Ingest — 2026-05-15
type: source
id: journal/2026-05-15-initial-corpus-ingest
kind: journal
tags:
- meta
- ingest
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/
---

## Purpose

Record the design and execution of the initial parallel-agent ingest of every textbook and paper under `raw/` into the circuit_solver wiki.

## Corpus

15 sources under `raw/` (14 textbooks + 1 paper):

| Source | Slug | Chapters |
|--------|------|----------|
| Advanced Symbolic Analysis for VLSI Systems | `advanced-symbolic-analysis-for-vlsi-systems` | 20 (17 ingest units after skipping toc/index/bibliography) |
| Computer Methods for Circuit Analysis and Design | `computer-methods-circuit-analysis-design` | 29 (~26 ingest units) |
| Data Analysis and Visualizations with Python | `data-analysis-visualizations-python` | 13 (11 ingest units) |
| Designing Data-Intensive Applications | `ddia` | 9 |
| Foundations of Scalable Systems | `foundations-scalable-systems` | 10 |
| Graphs in VLSI | `graphs-in-vlsi` | 21 |
| Guide to Graph Algorithms | `guide-to-graph-algorithms` | 10 |
| Modeling and Simulation of Systems | `modeling-simulation-systems` | 25 |
| Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) | `sze-physics-semiconductor-devices` | 23 |
| Prototyping Python Dashboards | `prototyping-python-dashboards` | 19 (17 ingest units) |
| Python Data Analyst's Toolkit | `python-data-analysts-toolkit` | 15 |
| The Rust Programming Language | `rust-book` | 29 |
| Solving Ordinary Differential Equations II (Hairer & Wanner) | `hairer-ode-ii` | 10 |
| Systems for Big Graph Analytics | `systems-big-graph-analytics` | 5 (4 ingest units) |
| Simulation of Analog and Mixed-Signal Circuits (Kundert, BCTM '98) | `simulation-whitepaper-v1` | 1 |

Total ingest units: ~239.

## Architecture

- **Per-book parallel agents.** One general-purpose agent dispatched per book, all running concurrently in the background. Each agent receives only its book identity (raw subdir, slug, title) and reads a shared workflow spec at `.ingest-instructions.md`.
- **Shared workflow spec.** Pinned per-page schema, slug conventions, link conventions, tag taxonomy, and the per-chapter workflow (read chapter `.txt`, write `wiki/summaries/<book-slug>-<chapter-stem>.md`, create concept/entity pages on first mention, append to per-book partial log, final dangling-link sweep).
- **Concurrency safety.** "Read-then-write-if-missing" on shared concept/entity pages: an agent only creates a concept/entity page if the file does not yet exist; it never modifies pages another agent already wrote. This rules out write-write races and produces a stable graph. The trade-off is that each concept page's `## Sources` list reflects only the first author's chapter — `## Sources` is reconciled in a final pass that scans every summary for `[[concepts/...]]` / `[[entities/...]]` references and rebuilds the cross-source lists.
- **Per-book partial logs** at `wiki/log.md.<book-slug>.partial` avoid contention on the shared `wiki/log.md` and `wiki/index.md`. Parent agent (this conversation) merges them and rebuilds the index at the end.
- **Reconciliation script** staged at `.ingest-reconcile.py` performs: partial-log concatenation, cross-source `## Sources` aggregation, `wiki/index.md` rebuild from on-disk page state, and a final dangling-link sweep with optional stub creation.

## Foreground work in the parent

- Ingest of `simulation_whitepaper_v1` (Kundert's BCTM 1998 tutorial) — 1 summary at `summaries/kundert-bctm98-simulation-tutorial`, 27 concept pages spanning Newton-Raphson, DC/AC/noise/transient analyses, homotopy methods, integration methods, LTE control, numerical damping, charge conservation, top-down design, mixed-level simulation, AHDLs/MS-HDLs, and 5 entity pages (Spectre, Verilog-AMS, VHDL-AMS, Ken Kundert, Cadence).
- Reconciliation script authoring at `.ingest-reconcile.py`.
- This journal entry.

## Lessons learned (in-flight)

- The "shared instructions file" approach kept individual agent prompts to ~300–600 chars while still encoding the full workflow — cheaper dispatch, consistent behavior across agents.
- Read-then-write-if-missing converges to a workable graph even under heavy parallel write pressure. A few overlapping concept-page creation attempts surfaced during the parent's own ingest (`modified-nodal-analysis` was created by an EDA-book agent between this agent's existence check and Write attempt; the Write was rejected and we kept moving). No data was lost — the parent simply did not redundantly clobber the other agent's page.
- Foundational concepts cluster across books (MNA, KCL, Newton-Raphson, integration methods, ODE/DAE, sparse matrices, graph theory, distributed-systems primitives). Whoever runs first writes the canonical definition; aggregation of cross-source citations happens in reconciliation.

## Status (snapshot at writing)

- 3 of 15 sources fully complete (Data Analysis & Visualizations Python, Systems for Big Graph Analytics, simulation whitepaper).
- 12 book agents still running.
- Wiki has 133 summaries, 616 concept pages, 109 entity pages so far.
- Reconciliation (`task #16`) waits on the last book agent.
