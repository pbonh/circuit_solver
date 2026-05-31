---
capability: mixed-signal-cosim
created: 2026-05-28
---

# Feature: Mixed-Signal Co-Simulation (Optimistic Shared Scheduler)

Analog/digital co-simulation mediated by the shared Mixed-Signal Scheduler using optimistic time advance with rollback. The digital domain is served by the in-process native digital kernel (circuit-solver-digital crate), which is a peer crate depended on by the orchestration crate (ADR-0004 superseded; native engine confirmed at design halt Q1).

## Scenarios

<!-- traces-grill: cc-adr0004-external-cosim -->
**Scenario: Digital-driven analog load co-simulates with rollback**
```gherkin
Given a digital stimulus driving an analog load across the analog/digital boundary
And the Mixed-Signal Scheduler owning both kernels
When the scheduler advances the analog kernel optimistically past a predicted digital event that later mispredicts
Then it rolls back to the last checkpoint and the final output matches the golden reference within tolerance
```

### Scenario: Comparator plus DFF produces correct mixed-signal behavior
```gherkin
Given a comparator feeding a D flip-flop testbench
When the co-simulation runs to completion
Then the captured digital output matches the expected event trace
```

### Scenario: Level shifter crosses domains correctly
```gherkin
Given a level-shifter testbench bridging two supply domains
When the co-simulation runs
Then the shifted output level matches the golden reference within tolerance
```

### Scenario: The scheduler drives the in-process native digital kernel
<!-- traces-grill: cc-adr0004-external-cosim -->
```gherkin
Given the scheduler needs the digital domain evaluated
When it requires digital progress
Then it issues a run-until command to the in-process native event-driven kernel (no cross-process IPC), per the ADR superseding 0004
```

### Scenario: The scheduler accesses the native kernel only via the digital crate's public API
```gherkin
Given the orchestration crate (owning the Mixed-Signal Scheduler) and the digital crate as peer workspace members
When the scheduler issues a run-until command to the native digital kernel
Then it calls only items exported by the circuit-solver-digital crate's public API; no internal module paths are accessed cross-crate
```
