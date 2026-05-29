---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 7: Network Functions
  in the Frequency Domain'
type: source
id: source-computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain
kind: derived-summary
tags:
- foundational
- analog
- ac
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt
---

## Key Points

- A network function F(s) = N(s)/D(s) can be represented as a polynomial ratio (Eq. 7.1.4) or as a product of factors with poles and zeros (Eq. 7.1.5). The denominator D(s) is always det T(s).
- Two strategies for computer generation: (1) generate polynomial coefficients by interpolation, then find roots; (2) compute poles and zeros directly from the system matrix via the QZ algorithm.
- Polynomial interpolation: choose 2n+1 points s_i, evaluate D(s_i) = product of L diagonal entries, evaluate F(s_i) by forward/back substitution, compute N(s_i) = D(s_i) F(s_i). Solve the Vandermonde-style system X a = y for coefficients.
- Interpolation point choice critical for numerical stability: real-axis interpolation is ill-conditioned; equally-spaced points on the unit circle x_k = exp(2 pi k j / (n+1)) yield condition number K(X) = 1 (optimal). Real interpolation has condition numbers growing exponentially with polynomial degree.
- Discrete Fourier Transform (DFT) gives the coefficient solution: a_k = (1/(n+1)) sum_i y_i w^{-i k} where w = exp(2 pi j / (n+1)). Fast Fourier Transform (FFT) reduces cost to m(n+1) when n+1 = 2^m.
- Symbolic function generation algorithm (Section 7.5): pick n_0 ≥ degree (capacitors + inductors), interpolate on unit circle, factor T(s_i) at each point, recover N and D coefficients by DFT. Optimizations: use only upper-half-plane points; combine N and D into c_i = N + j D for one DFT call; compute determinant via sum-of-logs to avoid over/underflow.
- Cauer-parameter filter example: degree estimate that is too low gives wrong polynomial; increasing the estimate reveals the correct (lower) degree as superfluous terms become near-zero.
- Scaling is critical for high-order polynomials. On a 16-digit machine, numerical noise in the DFT is ~|max b_i| * 1e-16. The 9th-order Cauer filter required scaling so that max|b_i|/min|b_i| was minimized — otherwise five poles appeared in the right half-plane (incorrectly indicating instability).
- Root-finding methods: Newton-Raphson (local convergence, real-to-real or complex-to-complex initial estimate); Laguerre's method (better global properties, can converge to complex roots from real estimates, polynomial-specific); Jenkins-Traub algorithm [3, 4] for higher accuracy.
- Root refinement: after polynomial root-finding gives an approximate z or p, iterate Newton-Raphson on the system equations directly. For a zero: solve TX = W and T^T X^a = -d at z^k; update z^{k+1} = z^k - F(z^k)/((X^a)^T C X). For a pole: factor T(p^k) = LU; l_nn ≈ 0; solve L^T y = l_nn e_n and U z = e_n; update p^{k+1} = p^k - l_nn / (y^T C z). This is much more accurate than refining via polynomial.
- QZ algorithm (Moler-Stewart, 1973; EISPACK [6]): direct generalized-eigenvalue solver for det(s C + G) = 0, giving generalized eigenvalues as pairs (alpha_i, beta_i) with eigenvalue alpha_i/beta_i. Computes poles to high accuracy without polynomial intermediate. Numerator zeros are obtained by augmenting T with an extra row and column for the output: M(s) = det T_M, computed by a second QZ call.
- Trade-off: QZ algorithm is robust and needs no scaling, but is O(n^3) on dense matrices and loses circuit sparsity. Interpolative approach exploits sparsity and is faster for large networks (60% of QZ time in the 9th-order Cauer example), but requires careful scaling.
- Sensitivity of roots to polynomial coefficients (Eq. 7.7.2): closely-spaced roots and roots with |x| >> 1 or |x| << 1 have high sensitivity. This further motivates direct system-matrix methods (QZ) for poles and zeros at the boundary of numerical precision.

## Relevant Concepts

- [[concepts/network-function]] — Already covered.
- [[concepts/symbolic-function-generation]] — DFT-based polynomial-coefficient extraction.
- [[concepts/dft-fft]] — Discrete and fast Fourier transform.
- [[concepts/interpolation-condition-number]] — Numerical stability of polynomial fitting.
- [[concepts/qz-algorithm]] — Generalized eigenvalue solver for poles and zeros.
- [[concepts/laguerre-method]] — Globally convergent polynomial root finder.
- [[concepts/root-refinement]] — Newton-Raphson on system matrix to correct polynomial-derived approximations.
- [[concepts/newton-raphson-method]] — Already covered; used here for root refinement.
- [[concepts/poles-and-zeros]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 7 — Network Functions in the Frequency Domain
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt`
- Authors: Jiri Vlach, Kishore Singhal
