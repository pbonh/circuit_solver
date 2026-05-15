---
title: "Modeling and Simulation of Systems — Chapter 8: Automated and Rule-Based Pruning and Experimental Execution"
type: summary
tags: [simulation, modeling, ses, pruning, experimental-frames, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt"]
confidence: high
---

## Key Points

- Combinatorial SES spaces quickly outstrip manual pruning; MS4 Me supports two automated strategies — enumerative pruning (generate every PES once) and random pruning (sample with uniform probability).
- An empty pruning file represents the entire solution space; a partial pruning file represents a constrained subspace; a complete pruning file specifies exactly one PES.
- Selection rules come in context-free and context-sensitive forms. Example context-sensitive: `select mediumPower from computePower for CPU under HP under Computer under JobContext!`.
- Conflict resolution: when multiple rules match an entity occurrence, the rule with the longest matching partial-context path wins; an empty context (context-free rule) is always a fallback.
- Conditional rule-based pruning: `if select X from Y for Z then select A from B for C!` placed in the `*.ses` file (not the pes script) constrains pruning choices in terms of other choices.
- `if not` (unless) rules expand to a set of explicit if-then rules over the complement of the named choice — handy for "default unless exception" patterns like patient-activity sampling rates.
- After applying all rules, any specialization choice still open is filled in by random selection — enabling Monte Carlo exploration of the residual subspace.
- Experimental Frame (EF) concept: composed of Generators (drive input trajectories), Acceptors (check termination conditions), and Transducers (collect data). The simulator runs in control of a Model-and-Frame pair.
- Pseudo-code execution loop: initialize dataStore, set NumReps, create ModelAndFrame, attach Simulator, start, iterate while querying Acceptor, on termination query Transducer and record.
- Three control levels: (1) handwritten root-execution method controls a fixed model/EF; (2) SES-driven control loads (ses, pes) files and prunes/transforms before simulating; (3) wraps SES-driven control in an outer loop sampling random PESs in parallel for distributed simulation.
- The architecture lends itself to multicore and distributed execution platforms — generated models can be farmed out per core.

## Relevant Concepts

- [[concepts/automated-pruning]] — enumerative and random pruning algorithms.
- [[concepts/context-sensitive-pruning]] — partial-context selection rules with longest-match conflict resolution.
- [[concepts/rule-based-pruning]] — if-then and if-not conditional rules in *.ses files.
- [[concepts/experimental-frame]] — Generator/Acceptor/Transducer trio surrounding a model.
- [[concepts/simulation-executive]] — SES-driven multi-run execution loop with parallel/distributed support.
- [[concepts/ses-pruning]] — base pruning operation that automated pruning extends.
- [[concepts/pruned-entity-structure]] — output of pruning.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 8 — Automated and Rule-Based Pruning and Experimental Execution
- File path: `raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
