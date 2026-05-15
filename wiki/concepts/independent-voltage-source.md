---
title: "Independent Voltage Source"
type: concept
tags: [foundational, analog, well-established, device-model]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

An independent voltage source maintains a prescribed voltage e across its terminals regardless of the current through it. Its i-v characteristic in the i-v plane is a horizontal line at voltage e; shifting e moves this line up or down. Setting e = 0 yields a short circuit (current can flow freely, voltage is zero).

## How It Works

The source can be either DC or time-varying. Real sources are modeled by an ideal voltage source in series with internal resistance Rs (Thevenin form). In MNA, voltage sources require an extra branch-current unknown and KVL equation because they cannot be expressed in admittance form.

## Key Parameters

- Prescribed voltage e(t) or E (DC).
- Internal resistance Rs (when modeling realistic sources).

## When To Use

- Modeling batteries, signal generators, supply rails.
- Probing the network response (small-signal AC, transient stimuli).
- Forcing a node potential in nodal/MNA formulation.

## Risks & Pitfalls

- An ideal voltage source can deliver infinite current — physically unrealistic.
- Two voltage sources connected in parallel with different values produce an inconsistent equation.
- Voltage source between two non-ground nodes complicates nodal analysis, motivating MNA.

## Related Concepts

- [[concepts/independent-current-source]]
- [[concepts/thevenin-norton-equivalents]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
