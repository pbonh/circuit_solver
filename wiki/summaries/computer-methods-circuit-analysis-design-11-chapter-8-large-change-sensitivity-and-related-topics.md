---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 8: Large Change
  Sensitivity and Related Topics'
type: source
id: source-computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics
kind: derived-summary
tags:
- foundational
- analog
- sensitivity
- sparse-matrix
- fault-analysis
- well-established
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt
---

## Key Points

- Large change sensitivity answers: given a network solved at nominal element values, can we obtain the exact solution after arbitrary changes in m elements without re-solving the entire system? Yes — by exploiting a low-rank matrix update.
- The modified system: (T_0 + P delta Q^T) X = W, where P and Q are n x m incidence-like matrices and delta = diag(delta_i) holds element perturbations. After re-arrangement (Eq. 8.1.16), only an m x m system (delta^{-1} + F) z = W_hat needs solving, where F = Q^T T_0^{-1} P. The output is F = F_0 - d_hat^T z.
- The augmented matrix F_hat = [F W_hat; d_hat^T F_0] of size (m+1) x (m+1) is the central data structure. It is computed once via m+1 forward/back substitutions on the nominal LU factorization. All subsequent perturbation queries use only F_hat.
- When only m_a < m elements are actually perturbed (delta_i != 0 for i in subset), the system reduces further to size m_a (Eqs. 8.1.24-25), using the corresponding rows/columns of F_hat.
- Differential sensitivity through the F matrix (Section 8.2): adjoint formulation gives (delta^{-1} + F^T) z^a = d. After m+1 preprocessing solves on the nominal system, both large-change solutions and first-order sensitivities for arbitrarily perturbed configurations cost only an m_a-sized solve each. The formula dF/d delta_i = y_i y_i^a falls out directly when only the variable-element sensitivities are wanted.
- Fault analysis (Section 8.3): single-fault investigations sweep delta_i over (-G, infinity); open-circuit corresponds to delta = -G, short-circuit to delta^{-1} = 0. Both extremes are handled by Eq. 8.3.1 without re-formulation. Fault directories are built by tabulating F(delta_i) loci for each component. Multi-fault analyses are restricted to one- or two-element faults in practice.
- Zero-pivot handling in sparse factorization (Section 8.4): when a zero pivot is encountered at step i during LU factorization, add e_i e_i^T to A, continue factorizing the modified matrix A_m, and treat the modifications as low-rank updates of the same Section-8.1 form. The original solution is recovered via A_m x = b - P z with z from (-I + F) z = b_hat.
- Symbolic analysis (Section 8.5): the network function F(delta_1, ..., delta_m) is multilinear in each delta_i. Coefficients of the numerator N and denominator D in delta are obtained from subdeterminants of F_hat:
  - dD/d(delta_{i1} ... delta_{il}) = det T_0 * det F(i1, ..., il) — retain those rows/columns of F.
  - dN/d(delta_{i1} ... delta_{il}) = det T_0 * det F_hat(i1, ..., il, m+1) — retain those rows/columns plus the last (W and d augmented) row and column.
- Bilinearity: at fixed frequency F is bilinear in each delta_i, with all coefficients available from subdeterminants of F_hat. With m variables, the numerator and denominator each have 2^m coefficients in the multilinear expansion.
- The symbolic method extends to keep s as a variable too, by combining with the Chapter 7 unit-circle interpolation.
- All the methods of Chapter 8 require specialized programming but offer dramatic savings for design-iteration loops over element variations. The Appendix D program implements the Section 8.5 symbolic method.

## Relevant Concepts

- [[concepts/large-change-sensitivity]] — Exact solution after non-infinitesimal element changes.
- [[concepts/low-rank-matrix-update]] — Sherman-Morrison-Woodbury-style update underlying the technique.
- [[concepts/fault-analysis]] — Determining failed component from response measurements.
- [[concepts/zero-pivot-handling]] — Sparse-factorization recovery technique using rank-one modifications.
- [[concepts/symbolic-analysis]] — Already covered; this chapter gives the F-matrix-based generation.
- [[concepts/multilinear-function]] — Network function is bilinear in each variable element at fixed frequency.
- [[concepts/sensitivity-analysis]]
- [[concepts/adjoint-method]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 8 — Large Change Sensitivity and Related Topics
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt`
- Authors: Jiri Vlach, Kishore Singhal
