---
capability: digital-equivalence
created: 2026-05-28
---

# Feature: Digital Correctness Metric (Event-Trace Equivalence)

Digital and mixed-signal correctness is judged by event-trace equivalence against the Icarus Verilog golden trace, defined operationally so the acceptance criterion does not rest on the bare 0.70 stub claims (grill ha-event-trace-equivalence, ha-value-change-dump).

## Scenarios

<!-- traces-grill: ha-event-trace-equivalence -->
**Scenario: Equivalence is judged on ordered events, not byte-level VCD**
```gherkin
Given the engine's digital output trace and the Icarus golden trace for the same testbench
When the two traces are compared
Then equivalence holds iff the ordered (time, net, value) event sequences agree within the timing tolerance (not byte-level VCD identity)
```

### Scenario: VCD is treated as interchange only
<!-- traces-grill: ha-value-change-dump -->
```gherkin
Given the external simulator emits a VCD file
When the engine ingests it
Then VCD is parsed only into the (time, net, value) event model and no acceptance criterion depends on VCD byte layout
```
