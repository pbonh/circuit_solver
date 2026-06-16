# Proposal: Circuit Solver Delta — Analog/Digital/Mixed-Signal Circuit Simulator

## Why

Modern analog and mixed-signal IC verification requires a simulator that correctly handles stiff nonlinear DAEs, formal analog-digital co-simulation, and large-circuit scalability — gaps that SPICE-legacy tools address poorly. The wiki evidence ([[simulation-analog-mixed-signal-circuits]], [[computer-methods-circuit-analysis-design]], [[solving-ode-ii-stiff-dae]]) establishes a well-understood algorithmic foundation; implementing it in Rust gives memory-safe, GC-free performance without the legacy debt of Fortran/C SPICE codebases.

## What Changes

- **NEW**: MNA circuit formulation engine — stamps R, L, C, V, I, VCCS, VCVS, CCCS, CCVS into the sparse MNA matrix from a parsed netlist
- **NEW**: Nonlinear DC solver — Newton-Raphson with KCL convergence check, Gmin insertion, Gmin-stepping and source-stepping homotopy recovery
- **NEW**: Stiff transient integrator — variable-step Radau IIA (primary) and BDF1/BDF2 (fallback), LTE control in voltage domain, per-timestep NR inner loop
- **NEW**: Device model registry — SPICE-compatible diode, MOSFET (Level 1/3), BJT (Ebers-Moll), R/L/C, and behavioral macromodels; smooth C2 models required
- **NEW**: Analysis orchestrator — DC, AC (small-signal), transient, noise, and Fourier analysis modes
- **NEW**: Mixed-signal bridge — threshold-crossing analog→digital event detection and waveform-injection digital→analog; Verilog-AMS behavioral block integration via DEVS-inspired scheduler
- **NEW**: Output pipeline — waveform data in Nutmeg/VCD/Parquet format; Python hook for pandas/NumPy post-processing
- **NEW**: Sparse linear algebra kernel — Markowitz-ordered sparse LU factorization (or KLU-compatible interface) as the inner-loop bottleneck solver

## Capabilities

- New:
  - `mna-formulation` — netlist → MNA matrix stamping and topology validation
  - `nonlinear-dc-solver` — Newton-Raphson DC analysis with homotopy recovery
  - `transient-solver` — Radau IIA / BDF stiff transient integration
  - `device-models` — SPICE-compatible device model library
  - `mixed-signal` — analog/digital co-simulation bridge
  - `analysis-output` — analysis orchestration and waveform output

## Impact

- New Rust library/binary: `circuit-solver-delta` crate
- Dependencies: `nalgebra` or `faer` (dense LA), `sprs` or custom sparse module (sparse LA), `rayon` (parallelism), `pest` or `nom` (netlist parser)
- External interface: SPICE netlist (.sp/.cir) and Verilog-AMS (.vams) input; Nutmeg/VCD/Parquet output
- No existing codebase modified — greenfield implementation grounded in wiki evidence
- Planning evidence: [[circuit-solver-delta-learning-brief]], [[simulation-analog-mixed-signal-circuits]], [[computer-methods-circuit-analysis-design]], [[solving-ode-ii-stiff-dae]], [[verilog-ams]], [[rust-systems-programming]]
