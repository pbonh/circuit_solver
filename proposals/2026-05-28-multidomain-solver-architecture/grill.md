---
change-id: 2026-05-28-multidomain-solver-architecture
created: 2026-05-28
---

# Grill

Each entry below interrogates the proposal against the KG. Every entry carries
its own frontmatter (an `id` and an `addressed:` flag) and cites the wiki page(s)
it draws from with the claim's `effective` shown inline. **The proposal cannot
advance to specs until every entry is marked addressed** in the Responses
section (flip the flag to `true`).

Entry shape (repeat per entry, in the relevant section):

```
<!-- entry
id: <kebab-id>
addressed: true|false
-->
- **<short title>** — cites [[<wiki-page> | <edge>]] (effective <0.00>). <body>.
```

## Open Questions

<!-- entry
id: oq-v1-decision-tree-reopen
addressed: true
-->
- **Which v1 decision-tree branches reopen for an industrial target?** — cites [[grills/circuit-solver | mentions]] (type: question). The accepted decision tree bounded *v1* scope (in-process PyO3, russell+faer, external digital co-sim, closed enum). An industrial-strength, all-three-domain target pressures branches 2 (solver backend) and the digital boundary. Which branches are explicitly reopened, and which are settled.

<!-- entry
id: oq-native-digital-scope
addressed: true
-->
- **Native event-driven digital engine vs. external co-simulation?** — cites [[concepts/discrete-event-system-specification | mentions]] (effective 0.95). The proposal leaves the digital-engine boundary open. This is load-bearing for "industrial-strength digital" and must be resolved before specs can describe observable behavior.

<!-- entry
id: oq-steady-state-scope
addressed: true
-->
- **Is a steady-state / RF engine (harmonic balance / shooting) in scope?** — cites [[concepts/shooting-method | mentions]] (effective 0.85). "All three domains, industrially" may imply RF/steady-state, which the KG supports but v1 excludes. Unscoped, it inflates the spec surface unpredictably.

## Counter-Claims

<!-- entry
id: cc-adr0004-external-cosim
addressed: true
-->
- **An accepted ADR already chose external digital co-simulation.** — cites [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler | contradicts]] (effective 1.045). The proposal's "native digital engine" option stands against ADR-0004, an accepted decision at the highest effective confidence. A counter-claim this strong cannot be silently overridden.

<!-- entry
id: cc-adr0005-closed-enum
addressed: true
-->
- **Closed-enum dispatch was accepted precisely to forbid runtime model extensibility.** — cites [[decisions/0005-closed-enum-device-model-dispatch | contradicts]] (effective 1.045). The proposal's "industrial model library / controlled extensibility seam" is in direct tension with ADR-0005, which accepted that a new model is a breaking recompile.

<!-- entry
id: cc-adr0002-pure-rust
addressed: true
-->
- **Pure-Rust solver was accepted, ruling out a C/C++ fallback for hard matrices.** — cites [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer | contradicts]] (effective 1.045). The proposal raises "industrial matrix sizes / conditioning," which historically motivates KLU/SuperLU. ADR-0002 accepted no C/C++ FFI — a strong counter to any such fallback.

## Hidden-Assumption Challenges

<!-- entry
id: ha-golden-reference
addressed: true
-->
- **The entire validation premise rests on an under-grounded claim.** — cites [[concepts/golden-reference | refines]] (effective 0.70). The proposal validates all three domains against golden references (ngspice, Icarus), but `golden-reference` is a spec-stub sourced only from the spec page — below the 0.85 dismissal threshold. The methodology it names is not itself grounded in the textbook corpus.

<!-- entry
id: ha-event-trace-equivalence
addressed: true
-->
- **The digital/mixed-signal correctness metric is a sub-threshold stub.** — cites [[concepts/event-trace-equivalence | refines]] (effective 0.70). Digital and mixed-signal acceptance depends on event-trace equivalence, but the claim sits at the 0.70 floor (spec-stub, single non-textbook source).

<!-- entry
id: ha-value-change-dump
addressed: true
-->
- **Digital trace interchange leans on a stub claim.** — cites [[concepts/value-change-dump | refines]] (effective 0.70). VCD is the assumed interchange for digital traces from the external simulator, yet the claim is a 0.70 stub.

<!-- entry
id: ha-simulation-foundational
addressed: true
-->
- **A foundational term the proposal leans on is itself sub-threshold.** — cites [[concepts/simulation | refines]] (effective 0.70). The proposal speaks of "simulation engines" across domains while the foundational `simulation` claim is at 0.70.

<!-- entry
id: ha-gil-release
addressed: true
-->
- **The zero-copy/GIL-release frontend assumption depends on a sub-threshold claim.** — cites [[concepts/global-interpreter-lock | refines]] (effective 0.70). ADR-0001's zero-copy PyO3 frontend and the spec's GIL-release scenario rely on GIL semantics, but `global-interpreter-lock` is a 0.70 stub.

## Failure-Pattern Warnings

_Explicitly empty: no Source page is tagged `kind: post-mortem`; the corpus is `kind: publication` textbooks and `kind: journal` ingest meta. No failure-pattern source applies._

## Responses

_The proposer answers each entry here and flips its `addressed:` flag to `true`._

<!-- response-to: oq-v1-decision-tree-reopen -->
- **oq-v1-decision-tree-reopen** — Reopened, as open questions only (no decision taken at seed): the native-vs-external digital boundary (branch behind ADR-0004) and device-model extensibility (ADR-0005). Settled for this change: in-process PyO3 (ADR-0001) and two-pass flattening (ADR-0003). Recorded for the design/ADR stages; this seed takes no architectural decision.

<!-- response-to: oq-native-digital-scope -->
- **oq-native-digital-scope** — Carried as an Open Question into design (mode pause_and_ask). Specs will be written only for the analog and mixed-signal behavior that is invariant to this choice; the digital-engine scenarios are deferred until the design stage resolves the boundary.

<!-- response-to: oq-steady-state-scope -->
- **oq-steady-state-scope** — Explicitly OUT of scope for this change; logged as a follow-on. The proposal's Proposed Change is amended in intent to mark steady-state as deferred, keeping the spec surface bounded to DC/AC/transient/noise + event-driven digital + mixed-signal.

<!-- response-to: cc-adr0004-external-cosim -->
- **cc-adr0004-external-cosim** — Acknowledged. The proposal does not override ADR-0004; it surfaces the native-engine option as an Open Question. If design elects it, a *superseding* ADR is required — flagged for the record-adr stage. No specs will assume a native engine.

<!-- response-to: cc-adr0005-closed-enum -->
- **cc-adr0005-closed-enum** — Acknowledged. Expanding the *closed* enum with more in-tree models (MOSFET/BSIM) is consistent with ADR-0005; a *runtime-extensible* seam is NOT, and is demoted to an Open Question requiring a superseding ADR. Specs target in-tree model additions only.

<!-- response-to: cc-adr0002-pure-rust -->
- **cc-adr0002-pure-rust** — Acknowledged and kept: the change stays within the pure-Rust constraint. Scaling risk is converted into an Open Question (does russell+faer hold at industrial sizes?) to be answered with benchmarks at the design stage, not by reintroducing FFI.

<!-- response-to: ha-golden-reference -->
- **ha-golden-reference** — Addressed: this fragility is real and is the proposer's responsibility to retire. The specs stage will pin the concrete reference methodology (tool versions, tolerances already in specs/circuit-solver: 5%/0.5dB/100uV/2dB) and cite the analog/digital textbook claims, raising the claim's grounding rather than relying on the stub.

<!-- response-to: ha-event-trace-equivalence -->
- **ha-event-trace-equivalence** — Addressed: event-trace equivalence (not byte-level VCD) is already the chosen digital metric in specs/circuit-solver. Specs will define it operationally (ordered (time, net, value) tuples within tolerance), making the dependency explicit and testable instead of leaning on the bare claim.

<!-- response-to: ha-value-change-dump -->
- **ha-value-change-dump** — Addressed: VCD is an interchange detail, not a correctness contract — the contract is event-trace equivalence (above). Specs treat VCD as the Icarus output format only; no behavior depends on VCD internals, bounding the exposure of this weak claim.

<!-- response-to: ha-simulation-foundational -->
- **ha-simulation-foundational** — Addressed: the proposal does not rest its method content on the generic `simulation` claim — it rests on the specific, high-confidence engine claims (MNA 0.95, Newton-Raphson 0.95, DEVS 0.95, mixed-level-simulation 0.95). The generic claim is narrative scaffolding only; no spec depends on it.

<!-- response-to: ha-gil-release -->
- **ha-gil-release** — Addressed: the GIL-release contract is already a dedicated spec scenario (specs/circuit-solver) with an observable test (long solve releases the GIL; a concurrent Python thread progresses). Specs verify the behavior empirically rather than assuming the claim, retiring the fragility.
