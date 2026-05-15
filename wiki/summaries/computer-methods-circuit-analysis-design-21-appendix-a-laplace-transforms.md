---
title: "Computer Methods for Circuit Analysis and Design — Appendix A: Laplace Transforms"
type: summary
tags: [foundational, math, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/21-appendix-a-laplace-transforms.txt"]
confidence: high
---

## Key Points

- Defines the one-sided Laplace transform V(s) = integral_{0-}^{infinity} v(t) e^{-st} dt and its inverse v(t) = (1/2 pi j) integral_{c-j*infinity}^{c+j*infinity} V(s) e^{st} ds.
- Lists key properties:
  - Linearity: L[v_1 + v_2] = V_1(s) + V_2(s).
  - Scaling: L[v(k t)] = (1/k) V(s/k).
  - Time shift: L[v(t-T)] = e^{-sT} V(s).
  - Frequency shift: L[e^{-alpha t} v(t)] = V(s + alpha).
  - Differentiation: L[v'(t)] = sV(s) - v(0-).
  - Higher derivatives: L[v^{(n)}(t)] = s^n V(s) - sum_{k=1}^n v^{(n-k)}(0-) s^{k-1}.
  - Integration: L[integral_0^t v(tau) dtau] = V(s)/s.
- Initial-value theorem and final-value theorem (used for boundary conditions).
- Reference Laplace transform pairs collected in tables.

## Relevant Concepts

- [[concepts/laplace-transform]] — Already covered; this appendix supports it.

## Source Metadata

- Source type: book appendix
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix A — Laplace Transforms
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/21-appendix-a-laplace-transforms.txt`
- Authors: Jiri Vlach, Kishore Singhal
