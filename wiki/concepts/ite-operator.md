---
title: ITE Operator (If-Then-Else)
type: claim
id: concepts/ite-operator
tags:
- bdd
- foundational
- logic
- operator
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/06-3-binary-decision-diagram-for-symbolic-analysis.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

`ITE(F, G, H) = F * G + bar(F) * H` is the universal three-argument Boolean operator on BDDs introduced by Brace, Rudell, and Bryant (1990). Every binary Boolean operator can be expressed as an ITE, so a BDD package only needs one efficient implementation.

## How It Works

`ITE(F, G, H) = x_k * ITE(F_{x_k}, G_{x_k}, H_{x_k}) + bar(x_k) * ITE(F_{bar(x_k)}, G_{bar(x_k)}, H_{bar(x_k)})`, recursing on the topmost variable of `F`, `G`, `H`. Terminal cases short-circuit: `ITE(1,G,H)=G`, `ITE(0,G,H)=H`, etc. Memoization on `(F,G,H)` pointer triples gives near-linear-amortized performance.

## Key Parameters

- Computed-table (memoization) size.
- Terminal-case detection set.

## When To Use

- Implementing any Boolean operator over BDDs (AND = ITE(F,G,0), OR = ITE(F,1,G), XOR = ITE(F, bar(G), G), etc.).

## Risks & Pitfalls

- Without memoization, complexity blows up.
- Hash-table eviction policy affects performance.

## Related Concepts

- [[concepts/binary-decision-diagram]]
- [[concepts/robdd]]
- [[concepts/shannon-expansion]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-06-3-binary-decision-diagram-for-symbolic-analysis]]
