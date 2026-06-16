---
title: Circuit Solver Delta — Learning Brief
type: analysis
slug: circuit-solver-delta-learning-brief
created: 2026-06-16
updated: 2026-06-16
summary: Cited learning brief for the Circuit Solver Delta project — synthesizes wiki evidence for building a Rust-based analog/digital/mixed-signal circuit simulator.
tags: [circuit-simulation, analog, mixed-signal, rust, numerical-methods, mna, planning]
sources: [simulation-analog-mixed-signal-circuits, computer-methods-circuit-analysis-design, solving-ode-ii-stiff-dae, graphs-in-vlsi, advanced-symbolic-analysis-vlsi, modeling-simulation-systems, rust-programming-language]
status: active
---

# Circuit Solver Delta — Learning Brief

## Research Question

What algorithms, formulations, and architectural patterns does the wiki evidence support for building a correct, performant, Rust-based analog/digital/mixed-signal circuit simulator?

## Supported Findings

### 1. Circuit Equation Formulation (MNA)

[[computer-methods-circuit-analysis-design]] establishes **Modified Nodal Analysis (MNA)** as the correct formulation for SPICE-class simulators. MNA extends pure nodal analysis to handle voltage sources, inductors, mutual inductances, and controlled sources by augmenting the nodal admittance matrix with additional branch-current variables. The resulting system `G·x = b` (where G is the MNA conductance/stamp matrix and x contains node voltages plus branch currents) is the standard representation in SPICE, Spectre, and HSpice.

Key constraint: [[vlsi-graph-methods]] shows the MNA matrix is precisely the **circuit Laplacian** — a positive semi-definite sparse matrix with structure directly reflecting the circuit topology. Sparsity is critical: sparse LU factorization is O(n^1.5) for planar circuits (vs. O(n^3) dense), and minimum-degree / Markowitz ordering minimizes fill-in.

### 2. Nonlinear DC Solver

[[newton-raphson]] (confirmed by [[simulation-analog-mixed-signal-circuits]] §NR and [[computer-methods-circuit-analysis-design]] §12) is the required solver for DC analysis. Key findings:
- Convergence requires: smooth device models, isolated equilibrium, good initial guess
- [[simulation-analog-mixed-signal-circuits]] documents that SPICE's **ΔI check is subject to false convergence**; Spectre's **KCL check** is preferred and has no performance cost
- Gmin insertion (≈10^-12 S across nonlinear devices) prevents floating-node singularities
- Homotopy: [[homotopy-methods]] documents Gmin stepping, source stepping, and pseudo-transient continuation for convergence recovery when NR fails

### 3. Transient Integration (Stiff DAEs)

[[differential-algebraic-equations]] confirms circuit equations are index-1 DAEs. [[solving-ode-ii-stiff-dae]] (Hairer & Wanner) is the definitive reference:
- BDF1-2 are A-stable; BDF3-6 are A(α)-stable — sufficient for most SPICE transient
- **Radau IIA is the gold standard** for stiff DAEs: stiffly accurate (L-stable), high order, error-controlled, self-starting; Hairer's RADAU5 code is the reference implementation
- [[bdf-methods]] documents BDF variable-step/variable-order as the SPICE standard (Gear's method = BDF2 with step control)
- [[stiff-ode-methods]] confirms Radau IIA, SDIRK, Rosenbrock as the modern alternatives; Rosenbrock is cheaper per step but less accurate on high-index problems
- [[integration-methods]] documents that TR (Trapezoidal Rule) rings on stiff circuits; BE/G2 are safer; Spectre controls LTE in voltage (not charge), which is more accurate

### 4. Device Models

[[mosfet-physics]] and [[pn-junction]] provide physics grounding. [[computer-methods-circuit-analysis-design]] §11 gives the numerical forms:
- **Diode**: Shockley equation + junction capacitance + series resistance
- **MOSFET**: Level 1/2/3 I-V, Meyer capacitance (SPICE legacy); BSIM4 is the modern standard
- **BJT**: Ebers-Moll / Gummel-Poon
- All models must be smooth (C2 at minimum) for NR convergence
- [[simulation-analog-mixed-signal-circuits]] §device notes behavioral macromodels (e.g., opamp behavioral) as essential for mixed-level simulation

### 5. Mixed-Signal / Analog-Digital Co-Simulation

[[verilog-ams]] (confirmed by [[simulation-analog-mixed-signal-circuits]] §Verilog-AMS) provides the language foundation:
- Verilog-AMS supports both **event-driven** digital (`initial`/`always`) and **continuous-time** analog (`analog` block) in one language
- Conservative models (potential + flow) and signal-flow models
- Operators: `idt`/`ddt`, `transition`/`slew`, `idtmod` (circular integrator)
- **Threshold crossing** is the analog→digital event mechanism; **waveform generation** is the digital→analog mechanism

[[devs-simulation]] (from [[modeling-simulation-systems]]) provides the **formal discrete-event framework**:
- DEVS atomic models: state, internal/external transition functions, output function, time advance
- DEVS coupled models: component composition with port coupling
- DEVS is more rigorous than ad-hoc event scheduling; event causality is explicit
- The analog solver drives to the next digital event; the digital scheduler drives to the next analog discontinuity

[[simulation-analog-mixed-signal-circuits]] §Mixed-Level confirms that **mixed-level simulation** (one block at transistor level, rest behavioral) is the only practical path for verifying large mixed-signal systems (>10,000 transistors).

### 6. Graph-Theoretic Exploitation

[[vlsi-graph-methods]] and [[treewidth-and-graph-structure]] open optimization opportunities:
- VLSI netlists typically have **small treewidth** → FPT algorithms enable linear-time dynamic programming on the circuit graph
- [[power-grid-analysis]] shows effective-resistance / Laplacian methods for power grid IR drop
- [[graphs-in-vlsi]] documents domain-decomposition methods for distributed MNA solve — relevant for large parallel simulation

### 7. Symbolic Analysis

[[advanced-symbolic-analysis-vlsi]] (Shi, Tan & Tlelo-Cuautle) and [[symbolic-circuit-analysis]] cover BDD/DDD/GPDD methods for closed-form transfer functions. Key finding: symbolic analysis can replace Monte Carlo for yield/sensitivity if the circuit is not too large. GPU Monte Carlo via hierarchical symbolic analysis is the frontier.

### 8. Implementation Substrate

[[rust-programming-language]] and [[rust-systems-programming]] confirm Rust as the implementation language:
- Ownership model: memory safety without GC — no allocator pressure in the inner NR loop
- Fearless concurrency: safe parallel matrix operations (e.g., parallel element stamping, parallel LU)
- No GC pauses — critical for timestep-controlled transient simulation
- Ecosystem: `nalgebra`, `faer`, `sprs` for sparse matrices; `rayon` for parallelism

[[python-data-science]] confirms Python (pandas/NumPy/matplotlib) as the post-processing layer.

## Inferences (Require Validation)

- [INFERENCE] Radau IIA will outperform BDF2 for stiff mixed-signal circuits with oscillator subcircuits; BDF2 may suffice for purely digital-timing-driven simulations. Needs benchmarking.
- [INFERENCE] DEVS formalism is rigorous but may add scheduling overhead for simple digital-only blocks; a lighter threshold-crossing mechanism may be sufficient for the initial mixed-signal bridge.
- [INFERENCE] Treewidth-based FPT algorithms will only outperform sparse LU for circuits with treewidth ≤ 15–20; netlists with high-fanout buses or complete interconnect may not benefit.
- [INFERENCE] A pure Rust symbolic analysis engine (BDD/DDD) is ambitious; a FFI bridge to an existing library (e.g., CUDD) is more pragmatic for the first release.

## Constraints

- Device models must be smooth (C2) for NR convergence; discontinuities require event detection
- The KCL convergence check (not ΔI) must be the primary convergence criterion
- Transient integration must be variable-step with LTE control in the voltage domain (Spectre approach)
- The analog/digital interface must be formally defined: analog → event (threshold crossing), digital → analog (waveform injection)
- Sparse LU with Markowitz/minimum-degree ordering is required for performance at circuit scale

## Open Questions for Planning

1. **Radau IIA vs BDF2**: Which integration family is the default? (ADR needed)
2. **MNA vs state-space**: Should the formulation layer expose state-space (for control-theory integration) or stay MNA-centric? (ADR needed)
3. **Mixed-signal bridge**: Full DEVS vs threshold-crossing event scheduler? (ADR needed)
4. **Sparse solver**: Sparse direct LU (e.g., KLU) vs iterative (e.g., GMRES+ILU)? (ADR needed)
5. **Symbolic analysis**: BDD/DDD in Rust vs FFI to CUDD? (ADR needed)

## Relevant Wiki Slugs

**Concepts**: [[spice-simulation]], [[newton-raphson]], [[integration-methods]], [[bdf-methods]], [[stiff-ode-methods]], [[runge-kutta-methods]], [[differential-algebraic-equations]], [[homotopy-methods]], [[verilog-ams]], [[devs-simulation]], [[mosfet-physics]], [[pn-junction]], [[symbolic-circuit-analysis]], [[vlsi-graph-methods]], [[treewidth-and-graph-structure]], [[power-grid-analysis]], [[rust-systems-programming]], [[python-data-science]]

**Sources**: [[simulation-analog-mixed-signal-circuits]], [[computer-methods-circuit-analysis-design]], [[solving-ode-ii-stiff-dae]], [[graphs-in-vlsi]], [[advanced-symbolic-analysis-vlsi]], [[modeling-simulation-systems]], [[rust-programming-language]]

**Entities**: [[ken-kundert]]
