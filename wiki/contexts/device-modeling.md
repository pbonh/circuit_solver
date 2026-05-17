---
title: "Device Modeling"
type: context
tags: [device-model, circuit-solver, bounded-context]
created: 2026-05-17
updated: 2026-05-17
sources:
  - "concepts/device-modeling"
  - "concepts/diode-model"
  - "concepts/bjt-model"
  - "concepts/fet-model"
confidence: high
---

## Model

The device-modeling context owns the electrical behavior of nonlinear and linear devices. Core entities:
- `DeviceModel` — an abstract model defining the constitutive equations for a device class.
- `ModelParameters` — the parameter set (e.g., IS, N for diode; KP, VTO for MOS) associated with a model instance.
- `DiodeModel`, `BJTModel`, `MOSFETModel` — concrete model families with their equation sets.
- `LinearizedModel` — the small-signal or companion-model linearization of a device at an operating point, expressed as a conductance matrix and equivalent source vector.
- `ModelLibrary` — the registry of named models available during netlist elaboration.

Key invariants: Every model name referenced by an element in the netlist-graph resolves to a model definition. Model equations are continuous and differentiable (Lipschitz) within their valid ranges. Linearized models are consistent with the full nonlinear equations (the Jacobian is correct).

## Boundary

- Starts at model parameter definitions (from `.model` cards or programmatic API).
- Ends at `LinearizedModel` stamps delivered to the numeric-solver context for [[concepts/modified-nodal-analysis]] assembly.
- Adjacent contexts:
  - `netlist-graph` provides element instances with model references and terminal mappings.
  - `numeric-solver` receives the stamp contributions and requests re-linearization at new iterates.
- Artifacts crossing the boundary: `LinearizedModel`, `ModelParameters`, `OperatingPointRequest`.

## Ubiquitous Language

- `DeviceModel` — the equations and parameters defining a device class.
- `ModelParameters` — numeric coefficients extracted from process data.
- `OperatingPoint` — the bias condition (terminal voltages/currents) at which a device is linearized.
- `Linearization` — the process of computing a local tangent (Jacobian) to the device equations.
- `Stamp` — the matrix entries a device contributes to the MNA system.
- `Jacobian` — the derivative of terminal currents with respect to terminal voltages.
- `Conductance` — the real part of the small-signal admittance.
- `Transconductance` — a controlled-source gain in the Jacobian.
- `Capacitance` — the charge-derivative term in the dynamic Jacobian.
- `CompanionModel` — the discrete-time equivalent circuit produced by an integration method.

## Relationships

- [[context-maps/circuit-solver]]

## Architecture

- [[architecture/circuit-solver]] — C4 diagrams showing the device-modeling context as the stamp supplier to the numeric-solver container.
