---
title: "Computer Methods for Circuit Analysis and Design — Chapter 6: Computer Generation of Sensitivities"
type: summary
tags: [foundational, analog, sensitivity, ac, sparse-matrix, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt"]
confidence: high
---

## Key Points

- Two basic sensitivity problems: (1) sensitivity of all variables X to one parameter h (sensitivity-network method); (2) sensitivity of one scalar output to many parameters (adjoint / transpose-system method). The second is the dominant practical case.
- Sensitivity network method: differentiating TX = W gives T (dX/dh) = -(dT/dh) X + dW/dh. Once T is factored, each parameter requires one extra forward/back substitution.
- Adjoint / transpose-system method: for scalar output phi = d^T X, the sensitivity is d phi / d h = (X^a)^T [-(dT/dh) X + dW/dh], where the adjoint vector X^a solves T^T X^a = -d. This requires only TWO solves total (the direct T X = W and the adjoint T^T X^a = -d), independent of the number of parameters.
- The same LU factorization of T is used for both direct and adjoint solves: T = LU implies T^T = U^T L^T, and SOLVET (Fig. 6.2.1) reuses the L and U stored in A by SOLVE to solve A^T x = b. Forward sub uses U^T, back sub uses L^T.
- For network applications: sources produce sparse W; outputs produce sparse d. With nodal/MNA/tableau formulations, dT/dh is a rank-one outer product s^v (e_i - e_j)(e_k - e_l)^T for v=0 (resistive) or v=1 (reactive); the sensitivity formula reduces to dphi/dh = s^v (x_i^a - x_j^a)(x_k - x_l) — at most two multiply-subtract operations per parameter, instead of the ~n^2 operations needed for a dense (X^a)^T (dT/dh) X product.
- Sensitivity to a current source j: dphi/dh = x_i^a - x_j^a (just a vector entry difference).
- Sensitivity to OPAMP gain (via B = -1/A): incorporated as an additional row and column in the MNA matrix, with B set to zero numerically but tracked as a variable. dphi/dB = x_{m+1}^a x_{m+1} (a scalar product of two augmented entries). Sensitivity to OPAMP output resistance R_0 has a similar form.
- Sensitivity to parasitics: an element with nominal value zero contributes a zero stamp to T (no effect on factorization), but dT/dh is nonzero. The transpose-system method gives the parasitic sensitivity without any special treatment — just include the zero element in the matrix layout.
- Sensitivity of amplitude/phase from a complex output phi = |phi| e^{j phi}: d|phi|/dh = |phi| Re(phi^{-1} dphi/dh); d phi(angle)/dh = Im(phi^{-1} dphi/dh). In dB: d alpha/dh|_{dB} = 8.686 Re(phi^{-1} dphi/dh) (Neper × 8.686 = dB).
- Sensitivity to frequency: dphi/d omega = j (X^a)^T C X (using s = j omega and T = G + sC). Equivalently, sum over reactive-element sensitivities. Group delay tau = -d phi(angle)/d omega is obtained as -Im(phi^{-1} dphi/d omega), computable with the same adjoint solve.
- Zero sensitivity: at a zero z of an output, dz/dh = -(dphi/dh)/(dphi/ds) = -(adjoint formula numerator) / ((X^a)^T C X). Computable for high-order polynomial zeros via numerical means, where symbolic root-finding is impossible.
- Pole sensitivity: at a pole p, T is singular and X, X^a cannot be computed directly. Instead, factor T(p) = LU; l_nn = 0 at the pole. Define X and X^a as solutions of UX = e_n and L^T X^a = l_nn e_n with x_n^a = 1; then (X^a)^T (dT/dh) X = d l_nn / dh, giving dp/dh = -(d l_nn / dh)/(d l_nn / ds). Permutation matrices accommodate pivoting.
- Q and omega_0 pole/zero sensitivity follow from dz/dh via the relations Q = (a^2 + b^2)^{3/2}/(2a) and omega_0 = sqrt(a^2 + b^2). Eq. 6.5.26 and 6.5.27 give compact formulas in terms of d(z conjugate-times-z)/dh.
- Temperature sensitivity: dphi/dT = sum_m (dphi/dE_m)(dE_m/dT), using chain rule. For linear temperature coefficient tau_m, dE_m/dT = E_m0 tau_m.
- Thevenin/Norton equivalents: one adjoint solve plus two simple inner products yields both the open-circuit voltage and the short-circuit current at any port.
- Noise analysis: each noise source W_i contributes phi_i = -(X^a)^T W_i. With m+1 sources (one signal, m noise sources), a single adjoint solve gives all phi_i. Output noise amplitude is sqrt(sum_i phi_i^2). For a conductance G, the source coefficient is sqrt(4 k T B G).
- Generalized scalar output psi = phi(X, h): the adjoint system becomes T^T X^a = -(d phi/d X)^T, computed after solving T X = W (the RHS now depends on X).
- Higher-order derivatives: d^2 phi/(d h_p d h_q) requires the adjoint vector and either repeated adjoint solves (one per output component) or direct sensitivity vectors dX/dh_p, dX/dh_q (one solve per parameter). Trade-off depends on whether the number of parameters or the matrix dimension is larger.
- Group delay compensation example: fourth-order Chebyshev pass-band response is given a flatter group delay by cascading all-pass network sections. The ninth-order Cauer filter (Section 4.10) sensitivities to OPAMPs 1, 3, 5, 7 are plotted; sensitivities to OPAMPs 2, 4, 6, 8 are identical.

## Relevant Concepts

- [[concepts/adjoint-method]] — Already covered; this chapter gives the algebraic-system formulation.
- [[concepts/sensitivity-network-method]] — Direct differentiation, one solve per parameter.
- [[concepts/transpose-system-method]] — Adjoint formulation in terms of T^T X^a = -d.
- [[concepts/tellegen-theorem]] — Original network-based justification for the adjoint approach.
- [[concepts/group-delay-sensitivity]] — Imaginary part of phase-frequency derivative via adjoint solve.
- [[concepts/noise-analysis]] — One adjoint solve covers all noise sources.
- [[concepts/higher-order-sensitivity]] — Second derivatives by repeated adjoint or direct application.
- [[concepts/pole-sensitivity-singular-matrix]] — Handling of singular T at a pole frequency via LU factor null-space.
- [[concepts/sensitivity-analysis]]
- [[concepts/lu-decomposition]]
- [[concepts/modified-nodal-analysis]]

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 6 — Computer Generation of Sensitivities
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/09-chapter-6-computer-generation-of-sensitivities.txt`
- Authors: Jiri Vlach, Kishore Singhal
