---
title: "Analysis Orchestration"
type: context
tags: [analysis, circuit-solver, bounded-context]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "concepts/dc-analysis"
  - "concepts/ac-analysis"
  - "concepts/transient-analysis"
  - "concepts/noise-analysis"
confidence: high
---

## Model

The analysis-orchestration context owns the user-facing analysis types and their control loops. Core entities:
- `DCAnalysis` — operating-point and sweep computation.
- `ACAnalysis` — small-signal frequency-domain sweep around a DC operating point.
- `TransientAnalysis` — time-domain simulation with adaptive timestepping.
- `NoiseAnalysis` — small-signal noise spectral density computation.
- `OperatingPoint` — the steady-state solution vector used to linearize the circuit.
- `AnalysisResult` — the unified output structure containing waveforms, frequency responses, or scalar operating-point data.
- `Sweep` — a parameterized sequence (linear, decade, octave) of analysis points.
- `Waveform` — a time-domain signal sampled at accepted timesteps.
- `FrequencyResponse` — a complex transfer function sampled at frequency points.

Key invariants: AC and noise analyses require a converged DC operating point. Transient analysis starts from a converged DC point unless UIC is specified. Adaptive timestepping rejects and retries when LTE exceeds tolerance. The result structure is complete and immutable once the analysis finishes.

## Boundary

- Starts at user analysis requests (via CLI, Python API, or programmatic calls).
- Ends at `AnalysisResult` objects passed to the application-frontend context.
- Adjacent contexts:
  - `numeric-solver` is called repeatedly to solve the equations for each analysis point.
  - `application-frontend` receives the final results for formatting, plotting, or serialization.
- Artifacts crossing the boundary: `AnalysisRequest`, `AnalysisResult`, `OperatingPoint`, `ConvergenceReport`.

## Ubiquitous Language

- `Analysis` — a specific simulation type requested by the user.
- `OperatingPoint` — the DC steady-state solution used as a reference.
- `Sweep` — a sequence of analysis points (voltage, frequency, or time).
- `FrequencyPoint` — one point in an AC or noise sweep.
- `Timestep` — the interval between consecutive transient timepoints.
- `Waveform` — a time-domain voltage or current signal.
- `TransferFunction` — the complex ratio of output to input in AC analysis.
- `SmallSignal` — the linearized behavior around an operating point.
- `LargeSignal` — the full nonlinear time-domain behavior.
- `Result` — the unified container for all analysis outputs.
- `Convergence` — success or failure of the overall analysis.
- `UIC` — Use Initial Conditions, bypassing the DC operating-point calculation.

## Relationships

- [[context-maps/circuit-solver]]

## Architecture

- [[architecture/circuit-solver]] — C4 diagrams showing the analysis-orchestration context driving the control loops and interacting with the mixed-signal scheduler.
