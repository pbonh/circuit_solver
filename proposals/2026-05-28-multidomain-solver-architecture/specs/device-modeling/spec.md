---
capability: device-modeling
created: 2026-05-28
---

# Feature: Device Model Engine (Closed-Enum Stamps)

Nonlinear device stamps evaluated in the Newton-Raphson loop via the closed `enum DeviceModel` (ADR-0005): compile-time monomorphized dispatch, no runtime extensibility. Industrial coverage is achieved by adding in-tree variants, not a runtime plugin seam (grill cc-adr0005-closed-enum).

## Scenarios

<!-- traces-grill: cc-adr0005-closed-enum -->
**Scenario: A new MOSFET model is added as an in-tree enum variant**
```gherkin
Given the closed `enum DeviceModel` extended in-tree with a MOSFET variant
When a MOSFET instance is stamped inside the Newton-Raphson loop
Then dispatch is statically monomorphized (no vtable / dynamic dispatch) and the variant is compiled in
```

### Scenario: Diode and BJT stamps match the reference models
```gherkin
Given a diode and a BJT test circuit
When the engine evaluates their stamps over a DC sweep
Then the I-V characteristics match the Computer-Methods reference models within 5%
```

### Scenario: Runtime model registration is rejected
<!-- traces-grill: cc-adr0005-closed-enum -->
```gherkin
Given the device-model API surface
When code attempts to register a new device model at runtime
Then no such API exists (a new model requires recompilation), preserving the closed-enum decision
```

### Scenario: A model family is generated via the in-tree codegen seam
<!-- traces-grill: cc-adr0005-closed-enum -->
```gherkin
Given a model family declared through the compile-time macro/codegen seam
When the crate is built
Then the generated variants are members of the closed `enum DeviceModel`, dispatched by static monomorphization with no runtime registration
```
