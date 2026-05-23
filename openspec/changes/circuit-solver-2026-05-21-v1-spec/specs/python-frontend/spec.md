---
title: "Spec: Python Frontend"
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
capability: python-frontend
created: 2026-05-21
updated: 2026-05-21
---

# Capability: Python Frontend

PyO3 extension module exposing a builder API for incremental circuit
construction, immutable CircuitGraph handles, per-request
AnalysisRequest submission, and NumPy-compatible result arrays, with
GIL release around native solver work.

## Glossary (inlined from manifest)

- `Circuit` — the top-level object representing a netlist and its
  associated models.
- `Simulator` — the runtime that executes analyses on a circuit.
- `Analysis` — a specific simulation type requested by the user (DC,
  AC, transient, noise).
- `Netlist` — the textual or programmatic circuit description.
- `Result` — the unified output structure for any analysis.
- `OperatingPoint` — the DC steady-state solution used as a reference
  for AC/noise/transient.
- `Waveform` — a time-domain voltage or current signal.
- `TransferFunction` — the complex ratio of output to input in AC
  analysis.
- `SmallSignal` — the linearized behavior around an operating point.
- `LargeSignal` — the full nonlinear time-domain behavior.
- `Sweep` — a sequence of analysis points (voltage, frequency, or
  time).
- `Convergence` — success or failure of the overall analysis or
  per-iteration solve.
- `Golden Reference` — a trusted external simulator against which
  results are compared.
- `Conformance` — passing the tolerance-bounded comparison against a
  golden reference.

## Personas

- **CircuitDesigner** — an engineer who constructs circuits
  interactively via Python and submits analyses; has authority to build
  circuits, submit analysis requests, and read results.
- **PythonDeveloper** — a developer who integrates circuit-solver into
  Python toolchains, notebooks, and automation scripts; has authority
  to inspect result array types, memory layout, and API surface.

## Acceptance Criteria

- The `circuit_solver` Python module exposes a builder API for
  incremental Circuit construction: adding elements, wires, models,
  and subcircuits via Python method calls.
- Calling `build()` on the builder produces an immutable
  `CircuitGraph` handle; subsequent mutations on the builder do not
  affect previously built graphs per ADR-0001.
- An `AnalysisRequest` is constructed from Python with analysis type,
  sweep parameters, and options; submitting it returns a `Result`
  object.
- Result arrays (node voltages, branch currents, Waveforms,
  TransferFunctions) are NumPy-compatible views into Rust-owned memory
  with zero-copy semantics.
- The GIL is released during native solver execution, allowing other
  Python threads to proceed while a simulation runs.
- Attempting to mutate a built CircuitGraph raises an immutable-handle
  error.

## Scenarios

### Scenario: Incremental circuit construction via builder API
```gherkin
Given PythonDeveloper imports the circuit_solver module
When PythonDeveloper creates a CircuitBuilder and adds a resistor "R1" between nodes "n1" and "n2" with value 1 kΩ
And PythonDeveloper adds a voltage source "V1" between nodes "n2" and "0" with value 5 V
And PythonDeveloper calls builder.build()
Then the returned object is an immutable CircuitGraph
And the CircuitGraph contains two elements and three nodes
```

### Scenario: Immutable circuit graph prevents post-build mutation
```gherkin
Given CircuitDesigner has built a CircuitGraph via the builder API
When CircuitDesigner attempts to call an add-element method on the CircuitGraph
Then a Python exception of type "ImmutableHandleError" is raised
And the CircuitGraph remains unchanged
```

### Scenario: Builder isolation across multiple builds
```gherkin
Given CircuitDesigner creates a CircuitBuilder and adds a resistor "R1"
And CircuitDesigner calls builder.build() producing graph_a
And CircuitDesigner adds another resistor "R2" to the same builder
When CircuitDesigner calls builder.build() a second time producing graph_b
Then graph_a contains one element
And graph_b contains two elements
And graph_a is not affected by the addition of "R2"
```

### Scenario: Analysis request and result retrieval
```gherkin
Given CircuitDesigner has built a CircuitGraph containing a resistive divider
When CircuitDesigner creates an AnalysisRequest for DC operating point
And CircuitDesigner submits the AnalysisRequest to the Simulator
Then the Simulator returns a Result object
And the Result contains node voltages accessible by node name
And the voltage at node "n1" is approximately 5 V within the tolerance envelope
```

### Scenario: Zero-copy NumPy result arrays
```gherkin
Given CircuitDesigner has obtained a Result from a transient Analysis
When PythonDeveloper accesses the Waveform array for node "n1"
Then the returned object is a NumPy ndarray of dtype float64
And the array's underlying buffer is a view into Rust-owned memory (no copy is performed)
And the array length equals the number of time points in the Result
```

### Scenario: GIL release during simulation
```gherkin
Given PythonDeveloper has two Python threads
And thread A submits a transient AnalysisRequest that takes several seconds
And thread B increments a Python counter in a loop
When thread A's simulation begins executing
Then thread B's counter continues to increment without being blocked by thread A
And thread A eventually receives its Result
```

### Scenario: SPICE netlist file parsing
```gherkin
Given CircuitDesigner has a SPICE netlist file on disk
When CircuitDesigner calls circuit_solver.parse_netlist(path)
Then the returned object is a CircuitGraph
And the CircuitGraph contains all elements, models, and subcircuits declared in the netlist
And the CircuitGraph is identical to one built incrementally with the same topology
```

### Scenario: Error on malformed netlist
```gherkin
Given CircuitDesigner has a SPICE netlist file with an unrecognized device letter
When CircuitDesigner calls circuit_solver.parse_netlist(path)
Then a Python exception of type "NetlistParseError" is raised
And the exception message identifies the line number and the unrecognized token
```

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `zero-copy-numpy-result-arrays`** — task `t_c7037c7a` (key `2026-05-21-v1-spec:task-58:bd85ae4288f3caedb4e14dbea0dbf41d78773a5e7054119ba3e44ce015b9caed`) merged at `e029b4899adf11ed0e8a5f794346b994046f4c97` by `scientia-integrator`. Verification: `cargo test -p circuit-solver-py --no-default-features → 105 passed / 0 failed / 0 ignored; result.rs integration binary → 24 passed / 0 failed / 0 ignored`. Residual risk: Cosmetic Cargo.toml comment/default-features inconsistency; ABI-matching numpy venv required for integration tests; from_channels adapter dormant pending Simulator.run wiring. Changed files: 5.
