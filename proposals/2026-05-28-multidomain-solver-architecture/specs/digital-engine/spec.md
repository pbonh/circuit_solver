---
capability: digital-engine
created: 2026-05-28
---

# Feature: Native Event-Driven Digital Engine

An in-process native digital kernel built on the DEVS / event-driven-architecture method core, replacing v1's external co-simulation (supersedes ADR-0004). Validated against the Icarus Verilog golden trace via event-trace equivalence. Scope is gate-/logic-level event-driven evaluation; full HDL elaboration is not in scope.

## Scenarios

<!-- traces-grill: oq-native-digital-scope -->
**Scenario: The native kernel advances by processing the event queue**
```gherkin
Given a gate-level digital testbench loaded into the native kernel
When the kernel is run until a target simulation time
Then it processes scheduled events in nondecreasing time order and the resulting trace matches the Icarus golden trace under event-trace equivalence
```

### Scenario: Zero-delay combinational settling converges
```gherkin
Given a combinational cone with zero-delay gates and a primary-input change
When the native kernel settles the delta cycle
Then all internal nets reach a stable value with no infinite delta loop (oscillation is reported, not hung)
```

### Scenario: The native kernel integrates with optimistic rollback
<!-- traces-grill: cc-adr0004-external-cosim -->
```gherkin
Given the native digital kernel under the Mixed-Signal Scheduler
When the analog side rolls back past a digital event that was optimistically processed
Then the kernel restores its event queue and net state to the checkpoint, consistent with the superseding sync ADR
```
