# Design — Circuit Solver Delta

## Overview

Circuit Solver Delta is a Rust library and CLI that implements an analog/digital/mixed-signal circuit simulator grounded in SPICE methodology and improved by three decades of numerical analysis research. The implementation proceeds in layers: a netlist parser produces a circuit IR; the MNA engine stamps elements into a sparse matrix; the solver core executes Newton-Raphson iteration with Radau IIA or BDF integration; the mixed-signal bridge mediates analog/digital crossings; the analysis runner orchestrates analysis modes; and the output pipeline serializes waveforms.

Every design decision below is tied to a spec requirement and an ADR. The learning brief (`wiki/pages/analyses/circuit-solver-delta-learning-brief.md`) is the evidence base.

## Decisions

| ADR | Decision | Design impact |
|-----|----------|---------------|
| [ADR-0001](decisions/0001-rust-implementation-language.md) | Rust as implementation language | All crates implemented in Rust; no GC, ownership-enforced sparse matrix lifetimes, `rayon` for parallel element stamping |
| [ADR-0002](decisions/0002-radau-iia-primary-integrator.md) | Radau IIA as primary integrator, BDF2 as fallback | `solverCore::radauIntegrator` is the default; `bdfIntegrator` selectable via config; Radau stages drive NR sub-iterations |
| [ADR-0003](decisions/0003-mna-as-canonical-formulation.md) | MNA as canonical circuit formulation | `mnaEngine` owns the sparse MNA matrix `G·x = b`; voltage/inductor branches extend the system with branch-current rows |
| [ADR-0004](decisions/0004-threshold-crossing-mixed-signal-bridge.md) | Threshold-crossing event scheduler for mixed-signal bridge | `mixedSignalBridge` bisection-detects crossings; analog integrator aligns timestep grid to upcoming events; `transition`/`slew` waveforms injected back |
| [ADR-0005](decisions/0005-sparse-direct-lu-solver.md) | Sparse direct LU with Markowitz ordering as inner-loop solver | `sparseLU` component inside `solverCore`; Markowitz minimum-degree ordering minimizes fill-in; pivoting for numerical stability |

## Architecture Mapping

See `architecture.md` for the full C4 model. Key containers and their design roles:

| Container (C4) | Crate / module | Responsibility |
|----------------|----------------|----------------|
| `netlParser` | `circuit_solver_delta::parser` | Tokenize and parse SPICE `.sp`/`.cir` and Verilog-AMS `.vams`; emit `CircuitGraph` IR |
| `mnaEngine` | `circuit_solver_delta::mna` | Stamp elements into `SparseMatrix<f64>`; topology validation; Jacobian management |
| `deviceRegistry` | `circuit_solver_delta::devices` | Device model trait `DeviceModel` + implementations: `Resistor`, `Capacitor`, `Inductor`, `VoltageSource`, `CurrentSource`, `Diode`, `Mosfet1`, `BjtEM`, and behavioral `VerilogAmsBlock` |
| `solverCore` | `circuit_solver_delta::solver` | `NewtonRaphson`, `RadauIIA`, `Bdf`, `SparseLU`, `HomotopyEngine` |
| `mixedSignalBridge` | `circuit_solver_delta::mixed_signal` | `ThresholdDetector`, `WaveformInjector`, `EventScheduler` |
| `analysisRunner` | `circuit_solver_delta::analysis` | `DcAnalysis`, `AcAnalysis`, `TransientAnalysis`, `NoiseAnalysis`, `FourierAnalysis` |
| `outputPipeline` | `circuit_solver_delta::output` | `NutmegWriter`, `VcdWriter`, `ParquetWriter`, `PyResult` (PyO3) |

## Data Flow and Interface Contracts

### 1. Netlist → Circuit IR

```
Input: SPICE netlist text or Verilog-AMS source
Output: CircuitGraph { nodes: Vec<NodeId>, elements: Vec<Box<dyn DeviceModel>> }
```

Parser emits `CircuitGraph`. Node `0` (ground) is always present. Each element implements `DeviceModel::terminals() -> &[NodeId]` and `DeviceModel::stamp(&mut MnaMatrix, &[f64]) -> ()`.

### 2. MNA Stamping

```
Input: CircuitGraph
Output: MnaSystem { G: SparseMatrix<f64>, b: Vec<f64>, var_map: VarMap }
```

`VarMap` maps each node and branch-current variable to a row/column index. Topology validation runs before stamping: floating-node check (BFS from ground), voltage-source-loop check (DFS on V-L subgraph). Stamping iterates elements and calls `DeviceModel::stamp`.

### 3. DC Analysis (Newton-Raphson)

```
Input: MnaSystem, DcOptions { gmin: f64, max_iter: usize, v_tol: f64, i_tol: f64 }
Output: DcSolution { x: Vec<f64>, converged: bool, iters: usize }
```

NR loop:
1. Evaluate all device models at current `x` → Jacobian contributions stamped into `dG`
2. `SparseLU::factor(G + dG)` and `solve(b - f(x))` → `Δx`
3. KCL residue check: `||G·x - b||∞ < i_tol` AND `||Δx||∞ < v_tol`
4. If not converged within `max_iter`: invoke `HomotopyEngine` (Gmin stepping then source stepping)

### 4. Transient Analysis (Radau IIA / BDF)

```
Input: DcSolution (initial x0), TransientOptions { t_stop, h0, h_max, h_min, lte_tol, method }
Output: TransientSolution { times: Vec<f64>, waveforms: HashMap<NodeId, Vec<f64>> }
```

Outer loop (time advance):
1. Propose `h` from current step controller
2. `RadauIIA::step(t, x, h)` or `Bdf::step(t, x, h)` — forms the stage system, calls NR inner loop
3. Estimate LTE in voltage domain: `||x_high - x_low||∞` (embedded estimate or Richardson extrapolation)
4. Accept if `LTE < lte_tol`, reject and halve `h` otherwise
5. After accept: grow `h` if `LTE < lte_tol/10`, cap at `h_max`
6. Record accepted `x` into waveform buffers

Inner NR (per Radau stage or BDF step): same as DC NR with the integration method's discretization terms added to the Jacobian.

Integration failure: 5 consecutive halvings below `h_min` → `IntegrationError { timepoint, min_h }`.

### 5. AC Analysis

```
Input: DcSolution, AcOptions { f_start, f_stop, n_points, sweep: Log | Lin }
Output: AcSolution { freqs: Vec<f64>, H: HashMap<NodeId, Vec<Complex64>> }
```

Linearize about DC operating point: compute `G_0 = G + dG|_{x=x_dc}`. At each frequency `ω`: solve `(G_0 + jωC)·V = b_ac` via `SparseLU`. Capacitor/inductor stamps are purely imaginary in AC.

Note: AC is invalid for circuits with strong nonlinearity at signal amplitudes (oscillators, mixers). The `AcAnalysis` struct reports a warning if the circuit contains a `VerilogAmsBlock` with `idt`/`ddt` operators and a DC solution with device currents above a threshold.

### 6. Mixed-Signal Bridge

```
Analog → Digital:
  ThresholdDetector { node: NodeId, v_high: f64, v_low: f64 }
  Event: DigitalEvent { time: f64, node: NodeId, state: Logic }

Digital → Analog:
  WaveformInjector { node: NodeId, events: Vec<(f64, f64)>, tr: f64, tf: f64 }
  Injects piecewise-linear waveform stamped as time-varying voltage source
```

Threshold crossing detection: when `x[node]` crosses `v_high` or `v_low` during a transient step, bisect within the step to find `t_cross` exactly, roll back to `t_cross`, emit `DigitalEvent`, restart from `t_cross`.

`EventScheduler` holds a priority queue of pending digital events. Before each transient step, `h` is capped to `t_next_event - t_current` to align the grid.

### 7. Device Model Trait

```rust
pub trait DeviceModel: Send + Sync {
    fn terminals(&self) -> &[NodeId];
    fn stamp_linear(&self, mna: &mut MnaMatrix);          // DC/AC linear stamp
    fn stamp_nonlinear(&self, mna: &mut MnaMatrix, x: &[f64]); // NR Jacobian stamp
    fn is_smooth(&self) -> bool;                            // C2 continuity check
}
```

All implementations MUST pass `is_smooth()` — a model returning `false` triggers a warning and prevents NR convergence. Diode clamping: `I_D = I_s * (exp(V_D / n*V_T) - 1)` clamped at `V_D > 40 * V_T` using the tangent-line extrapolation.

### 8. Output Pipeline

Nutmeg binary: standard SPICE format (header + binary block); compatible with ngscope, WaveView. VCD: standard IEEE 1364 format for digital state traces. Parquet: columnar; columns = node names + time index; compatible with pandas `read_parquet`. PyO3: `PySimResult` exposes `.time()`, `.voltage(node)`, `.current(element)` as numpy arrays.

### Error Handling

All analysis structs return `Result<_, SimError>` where `SimError` is a Rust enum:
- `TopologyError(FloatingNode | VoltageLoop | InductorLoop)`
- `ConvergenceError(DcConvergence | IntegrationFailure { timepoint, min_h })`
- `DeviceError(NonSmoothModel | BadParameters)`
- `ParseError(NetlistSyntax | UnsupportedElement)`

Errors are propagated; no panic in library code. The CLI layer formats errors for the user.

## Open Questions

- Symbolic analysis (BDD/DDD for transfer functions) is deferred; no ADR needed for the first release — flag as a follow-up decision when `analysis-output` specs are stable.
- GPU-accelerated sparse LU (`cuSPARSE`) is an optimization target post-v1; ADR-0005 defers this explicitly.
- BSIM4 MOSFET model (vs. Level 1/3 implemented here) requires a separate model ingestion effort; flag as a separate change.
