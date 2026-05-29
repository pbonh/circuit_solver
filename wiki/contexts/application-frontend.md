---
title: Application Frontend
type: entity
id: entity-context-application-frontend
tags:
- frontend
- python
- cli
- circuit-solver
- bounded-context
created: 2026-05-17
updated: 2026-05-17
sources:
- concepts/python
- concepts/rust-language
---

## Model

The application-frontend context owns the user interface and result presentation. Core entities:
- `CircuitSolverCLI` — the command-line entry point.
- `PythonAPI` — the programmatic Python interface (`python -m circuit_solver`).
- `ResultFormatter` — converters from `AnalysisResult` to CSV, JSON, NumPy arrays, or plotting objects.
- `Session` — a persistent solver session holding netlist, models, and results.

Key invariants: CLI arguments map to valid analysis requests. Python API calls are type-safe and raise exceptions on invalid requests. Result files are flushed before the process exits.

## Boundary

- Starts at user invocation (shell command or Python import).
- Ends at rendered output (terminal tables, plot windows, files on disk).
- Adjacent contexts:
  - `analysis-orchestration` receives `AnalysisRequest`s and returns `AnalysisResult`s.
- Artifacts crossing the boundary: `AnalysisRequest`, `AnalysisResult`, `PlotSpec`, `SessionHandle`.

## Ubiquitous Language

- `Solver` — the top-level object the user interacts with.
- `API` — the programmatic Python interface.
- `CLI` — the command-line interface.
- `Script` — a Python file that drives the solver.
- `NetlistFile` — the input circuit description file.
- `ResultFile` — the output data file (CSV, JSON, etc.).
- `Plot` — a graphical visualization of a waveform or frequency response.
- `Command` — a CLI subcommand or API call.
- `Session` — a persistent interactive solver instance.

## Relationships

- [[context-maps/circuit-solver]]

## Architecture

- [[architecture/circuit-solver]] — C4 diagrams showing the application-frontend context as the PyO3-based entry point and result presenter.

## Related Decisions

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph]] — Governs the PyO3 binding mechanism and immutable graph ownership model for the Python API.
