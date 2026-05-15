---
title: "Computer Methods for Circuit Analysis and Design — Chapter 5: Sensitivities"
type: summary
tags: [foundational, analog, sensitivity, ac, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt"]
confidence: high
---

## Key Points

- Three reasons to study sensitivity: (1) understand how parameter variations influence response; (2) compare network alternatives with the same nominal response; (3) supply gradients for optimization.
- Four sensitivity definitions: differential D_h^F = dF/dh; normalized S_h^F = (h/F)(dF/dh); semi-normalized S_h^bar = h (dF/dh) (used when F=0); semi-normalized S_h^tilde = (1/F)(dF/dh) (used when h=0, i.e. h is a parasitic). All four reduce to the differential when both F=0 and h=0.
- Network function sensitivity decomposes via T = N/D as S_h^T = S_h^N - S_h^D. From T = |T| e^{j phi}, the real and imaginary parts of S_h^T equal S_h^|T| and S_h^phi/phi, respectively. Sensitivity depends on frequency.
- Pole/zero sensitivity for a polynomial P with root z(h): dz/dh = -(dP/dh)/(dP/ds)|_{s=z}, with normalized form S_h^z = (h/z)(dz/dh). For complex roots z = a + jb, sensitivities of real and imaginary parts follow by Re/Im.
- Q and omega_0 sensitivity: for a complex pole pair p, p_bar at coordinates (a, b), Q = -omega_0/(2a) and omega_0^2 = a^2 + b^2. Then S_h^Q = S_h^omega_0 - S_h^a; S_h^omega_0 = (1/omega_0^2)(a^2 S_h^a + b^2 S_h^b). In high-Q circuits (a^2 << b^2), S_h^omega_0 ≈ S_h^b.
- Tuned-circuit example: for a parallel RLC, S_C^omega_0 = S_L^omega_0 = -1/2, S_G^omega_0 = 0, S_G^Q = -1, S_C^Q = -S_L^Q = 1/2. All magnitudes ≤ 1.
- Multiparameter sensitivity: dF/F = sum S_{h_i}^F (dh_i/h_i). Three measures:
  - Worst-case (WCMS): WCMS = sum |S_{h_i}^F|, with |delta F / F| ≤ t * WCMS for equal tolerance t.
  - Tracking (MTS_k): MTS_k = |sum over type-k elements S_{h_i}^F|, accounting for elements that track together (IC capacitors, etc.).
  - Statistical (MSS): MSS = [sum (S_{h_i}^F)^2]^{1/2}, used with normal distribution of element variations.
- Sensitivities to parasitics: an element with nominal value zero (parasitic) still has a defined dF/dh, even though normalized sensitivity is degenerate. Increment formula: delta F / F = sum (nonzero) S_{h_i}^F (delta h_i / h_i) + sum (zero) S_{v_j}^F delta v_j.
- Ideal OPAMPs as parasitics: define B = -1/A; the OPAMP constitutive equation V_j' - V_j + B V_n = 0 reduces to V_j' = V_j when B = 0. Pole sensitivity dp/dB and transfer sensitivity dT/dB are well-defined limits, computable for networks of any size. The Moschytz gain-sensitivity product Gamma_h^F = A * S_A^F equals S_B^F exactly (Eq. 5.3.10), confirming that B-sensitivities are the natural infinite-A limit.
- OPAMP gain-bandwidth: A(s) = A_0 omega_b / (s + omega_b). At finite frequencies and gain-bandwidth product A_0 omega_b, B_actual = -s/(A_0 omega_b) - 1/A_0, providing a small perturbation that can be folded into the parasitic sensitivity framework.

## Relevant Concepts

- [[concepts/sensitivity-analysis]] — Already covered; this chapter extends with definitions for Q, omega_0, parasitics.
- [[concepts/normalized-sensitivity]] — S_h^F = (h/F)(dF/dh).
- [[concepts/semi-normalized-sensitivity]] — Used when F = 0 or h = 0.
- [[concepts/pole-zero-sensitivity]] — dz/dh and dp/dh for roots of network polynomials.
- [[concepts/q-omega-sensitivity]] — Sensitivity of quality factor and natural frequency.
- [[concepts/multiparameter-sensitivity]] — Worst-case, tracking, and statistical measures.
- [[concepts/parasitic-sensitivity]] — Sensitivity with respect to nominally-zero elements.
- [[concepts/gain-sensitivity-product]] — Moschytz's measure, equivalent to S_B^F with B = -1/A.
- [[concepts/network-function]]
- [[concepts/poles-and-zeros]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 5 — Sensitivities
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/08-chapter-5-sensitivities.txt`
- Authors: Jiri Vlach, Kishore Singhal
