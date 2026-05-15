---
title: "Sparse Tableau Approach (Hachtel-Brayton-Gustavson)"
type: concept
tags: [foundational, sparse-matrix, netlist, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt"]
confidence: high
---

## Definition

The sparse tableau approach (Hachtel, Brayton, Gustavson, 1971) is the formulation of network equations as a single sparse algebraic-differential system T X = W in which KCL, KVL, and all branch constitutive equations are collected. It is the most general formulation; modified nodal analysis can be derived as a special case via block elimination.

## How It Works

The tableau matrix has the structure:
| I  -A^T  0 | | V_b |   | 0 |
| Y_b  Z_b  0 | | I_b | = | W_b |
| 0   0    A | | V_n |   | 0 |

Where:
- The top block is KVL: V_b - A^T V_n = 0.
- The middle block is the constitutive equations: Y_b V_b + Z_b I_b = W_b.
- The bottom block is KCL: A I_b = 0.

The matrix is large (size 2b + n) but extremely sparse, with regular block structure.

## Key Parameters

- b (number of branches), n (number of ungrounded nodes).
- Matrix density: typically 10-15% for circuit-like topologies.
- Choice of admittance vs. impedance entry for each element (Vlach & Singhal use Y for capacitors and Z for inductors to keep s in the numerator).

## When To Use

- Theoretical analysis (the tableau is the most direct expression of the network equations).
- Cases where all branch voltages and currents are required as solutions.
- Educational presentation of formulation hierarchy.

## Risks & Pitfalls

- Matrix is much larger than MNA, demanding a strong sparse solver.
- Coding sparse routines for tableau structure is complicated (the structure does not directly fit standard banded or symmetric storage).
- Rarely used in production simulators today; MNA is universally preferred.

## Related Concepts

- [[concepts/tableau-formulation]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/two-graph-formulation]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
