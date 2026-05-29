---
title: Einstein Relation (Semiconductor)
type: claim
id: claim-einstein-relation
tags:
- semiconductor
- device-physics
- transport
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
---

## Definition

For a nondegenerate semiconductor in thermal equilibrium, the Einstein relation D = (kT/q) mu links the diffusion coefficient D and the mobility mu of a charge carrier. At 300 K, kT/q = 0.0259 V, so D in cm^2/s equals 0.0259 mu in cm^2/V-s.

## How It Works

The relation is derived by demanding that, in a nonuniformly doped semiconductor at equilibrium (constant E_F), the drift current driven by the built-in field exactly cancels the diffusion current driven by the doping gradient. The same relation holds for both electrons and holes; it generalizes to a Fermi-Dirac form (involving the F_{1/2} integral) in the degenerate regime.

## Key Parameters

- Temperature T.
- Mobility mu_n, mu_p.
- Carrier degeneracy (correction factor in heavy doping).

## When To Use

- Whenever D is needed but only mu has been measured (and vice versa).
- In drift-diffusion device equations to reduce one unknown.

## Risks & Pitfalls

- Strictly valid only in equilibrium and nondegenerate limit.
- In degenerate semiconductors, must use the generalized form involving F_{-1/2}/F_{1/2}.

## Related Concepts

- [[concepts/carrier-mobility]]
- [[concepts/drift-diffusion-equation]]
- [[concepts/fermi-dirac-distribution]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
