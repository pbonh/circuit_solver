---
title: Thevenin and Norton Equivalents
type: claim
id: claim-thevenin-norton-equivalents
tags:
- foundational
- analog
- dc
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.85
---

## Definition

Any linear network seen between two terminals A-A' can be replaced by a Thevenin equivalent (ideal voltage source E in series with resistance Rs) or a Norton equivalent (ideal current source J = E/Rs in parallel with Rs), where E is the open-circuit voltage and J is the short-circuit current.

## How It Works

To find the equivalents:
1. Open-circuit the load to measure E = V_oc at A-A', or short-circuit to measure J = I_sc.
2. Deactivate all independent sources (replace voltage sources by shorts, current sources by opens; leave dependent sources / transducers intact).
3. Apply a unit voltage source at A-A' and compute the current I; then Rs = 1/I. Or apply a unit current source and compute the voltage V; then Rs = V.

The voltage source E in series with Rs is equivalent (at terminals A-A') to the current source J = E / Rs in parallel with Rs.

## Key Parameters

- E (Thevenin voltage, equal to V_oc).
- J (Norton current, equal to I_sc).
- Rs (source resistance, common to both forms).

## When To Use

- Simplifying complex linear sub-networks before further analysis.
- Computing power transfer to a load.
- Building intuition for the behavior of port-driven networks.

## Risks & Pitfalls

- Equivalents hold only at the chosen pair of terminals; internal voltages/currents are not preserved.
- Power dissipated by the equivalent source resistor is not the same as power dissipated by the original network.
- The procedure fails for circuits containing controlled sources whose controlling variable is internal to the deactivated portion — extra care is required.

## Related Concepts

- [[concepts/independent-voltage-source]]
- [[concepts/independent-current-source]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
