---
title: "Virtual Build and Test"
type: concept
tags: [simulation, modeling, methodology, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/00-preface.txt"]
confidence: high
---

## Definition

Virtual Build and Test is a methodology that designs, builds, and tests Systems of Systems entirely within virtual reality — whether on a single computer or across networked distributed simulations augmented with physically analogous and immersive environments — before any physical deployment. It is the central organizing theme of the Zeigler/Sarjoughian guide to modeling and simulation of SoS.

## How It Works

Models of new components and stubs of existing components are integrated within a simulation environment. The same models are then transferable to a real-time execution engine, interfaced via data-distribution middleware to actual existing components. The DEVS formalism is specifically designed to allow this transfer with minimal model rewriting — only the underlying engine changes between simulation, distributed simulation, and operational deployment.

## Key Parameters

- Single-machine or distributed simulation execution
- Stubs/abstractions for existing components
- Middleware (e.g., Web services, data-distribution service) for integration
- Re-targetable simulation engines (simulation, real-time, distributed)
- Model lifecycle preserved from design through fielded operation

## When To Use

- Healthcare-system coordination experiments
- Energy-efficient building design
- Agricultural policy and crop-genetics screening
- Wildfire suppression strategy development
- Drug-action biology screening
- Any setting where physical experimentation is dangerous, costly, unethical, or risky

## Risks & Pitfalls

- Stubs may not adequately capture existing-component behavior
- Disconnect between simulation engine and real-time engine if not carefully aligned
- Model lifecycle benefits depend on disciplined version control and middleware compatibility
- Risk of mistaking validated simulation for validated reality

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/system-entity-structure]]
- [[concepts/systems-of-systems]]

## Sources

- [[summaries/modeling-simulation-systems-00-preface]]
- [[summaries/modeling-simulation-systems-03-1-modeling-and-simulation-of-systems-of-systems]]
