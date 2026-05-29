---
capability: mixed-signal-cosim
created: 2026-05-28
---

# Feature: Mixed-Signal Co-Simulation (Optimistic Shared Scheduler)

Analog/digital co-simulation mediated by the shared Mixed-Signal Scheduler using optimistic time advance with rollback (ADR-0004). The digital domain stays an EXTERNAL event-driven simulator; this change does not embed a native digital engine (grill cc-adr0004-external-cosim, oq-native-digital-scope deferred to design).

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
