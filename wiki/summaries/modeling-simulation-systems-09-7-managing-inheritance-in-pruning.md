---
title: "Modeling and Simulation of Systems — Chapter 7: Managing Inheritance in Pruning"
type: summary
tags: [simulation, modeling, ses, inheritance, java, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/09-7-managing-inheritance-in-pruning.txt"]
confidence: high
---

## Key Points

- Underscores embedded in SES entity names (e.g., `Slow_GeneratorOfJobs`) drive both instance creation and Java-class inheritance during transformation.
- The default rule: in a name like `Slow_GeneratorOfJobs`, the last token (parent, `GeneratorOfJobs`) becomes the superclass. For nested names like `Random_Slow_GeneratorOfJobs`, the last token is the default superclass.
- The auto-generated subclass merely forwards constructor arguments to the parent's single-string constructor; the parent's `name` field carries the underscore-name and can be parsed to configure parent behavior per the child.
- "Configuring the base class" pattern: the parent's `initialize()` calls a helper (e.g., `interpretNameAsPeriod()`) that inspects the prefix (Slow/Fast) and sets behavior parameters (`Period`) accordingly. Code can live in tagged FDDEVS blocks.
- "Inheriting from the child" pattern: when the child (e.g., Reactive, Proactive) is a fully implemented behavior model and the parent is just a placeholder, use `inherit from Reactive!` and `inherit from Proactive!` in the pruning file to override the default.
- Specializations naturally produce underscore names during pruning; the pruning script controls which side of the underscore drives inheritance.
- A SimonSays game example illustrates how Compliant and NonCompliant FDDEVS behavior classes serve as the inheritance source for player instances via `inherit from Compliant/NonCompliant!`.
- Underscore-based naming is also used to create multiple instances of the same base class (e.g., `First_GeneratorOfJobs` and `Second_GeneratorOfJobs`) as components of the same coupled model.

## Relevant Concepts

- [[concepts/ses-inheritance]] — underscore-name inheritance mechanics.
- [[concepts/ses-specialization]] — origin of underscore names during pruning.
- [[concepts/ses-pruning]] — pruning script directives controlling inheritance.
- [[concepts/atomic-devs-model]] — Java class hierarchy generated from FDDEVS.
- [[concepts/object-oriented-simulation]] — OO inheritance underpinning the mechanism.
- [[concepts/dnl-elaboration]] — tagged-block hooks used for parent-configuration helpers.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 7 — Managing Inheritance in Pruning
- File path: `raw/ModelingAndSimulationOfSystems/_txt/09-7-managing-inheritance-in-pruning.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
