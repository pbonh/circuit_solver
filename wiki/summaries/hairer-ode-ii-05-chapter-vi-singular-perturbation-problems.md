---
title: 'Solving Ordinary Differential Equations II — Chapter VI: Singular Perturbation
  Problems and Index 1 Problems'
type: source
id: source-hairer-ode-ii-05-chapter-vi-singular-perturbation-problems
kind: derived-summary
tags:
- ode
- dae
- stiff
- singular-perturbation
- numerical-integration
- rosenbrock
- runge-kutta
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-vi-singular-perturbation-problems.txt
---

## Key Points

- Singular perturbation problems (SPP) take the form y'=f(y,z), εz'=g(y,z); when ε→0 they degenerate into the differential-algebraic equation y'=f(y,z), 0=g(y,z), called the reduced system or index-1 DAE. The Jacobian g_z must be invertible (assumption 1.7), giving a locally unique solution manifold z=G(y) and a state-space form y'=f(y,G(y)). Examples include van der Pol's equation in Lienard coordinates (Dorodnicyn 1947), chemical kinetics with fast/slow reactions, and method-of-lines parabolic discretisations.
- The ε-embedding method applies an implicit RK or multistep formula to the full SPP and then sets ε=0, producing a method directly applicable to the DAE; for invertible RK matrix A this requires only one matrix-vector elimination using w_{ij}=(A^{-1})_{ij}. The state-space-form method enforces g(y_{n+1},z_{n+1})=0 directly. For stiffly accurate methods (a_{si}=b_i) the two approaches coincide; Griepentrog–März call these IRK(DAE).
- Problems of the form Mu'=φ(u) with constant (possibly singular) M are equivalent to the semi-explicit form via Gaussian-elimination decomposition M=S·diag(I,0)·T (Eq. 1.19); the diagram (1.23) shows that the ε-embedding RK method commutes with this transformation, so RADAU5 with its M-option handles implicit-DAE problems like the transistor amplifier (Eq. 1.14) directly using the same numerical machinery.
- Convergence of Runge–Kutta on index-1 DAEs (Theorem 1.1): if stiffly accurate, both y- and z-components are O(h^p); otherwise z achieves only min(p,q+1) if -1≤R(∞)<1, or min(p-1,q) if R(∞)=+1; if |R(∞)|>1 the z-iteration diverges. Order reduction in the algebraic variable is governed by the stage order q.
- BDF and general multistep on index-1 DAEs (Theorem 2.1, Gear 1971): if the method is stable at 0 and at ∞, the global error is O(h^p) for both components. Lubich's Theorem 2.2 establishes stiffness-uniform error bounds h^p∫‖y^{(p+1)}‖ + (h+ρ^n)·z-component-starting-error + εh^p max‖z^{(p+1)}‖ for A(α)-stable, strictly stable-at-infinity multistep methods on sectorial SPP problems.
- Epsilon expansions (Vasil'eva 1963, VI.3): the smooth solution admits y(x)=Σε^j y_j(x), z(x)=Σε^j z_j(x); for general initial conditions one adds boundary-layer terms ε^j η_j(x/ε), ε^j ζ_j(x/ε) decaying like e^{-κξ}. Theorem 3.2 gives a rigorous remainder estimate O(ε^{N+1}) under the stability hypothesis μ(g_z)≤-1.
- Runge–Kutta solutions of SPPs admit a similar ε-expansion (Hairer–Lubich–Roche 1988) where the ε^j coefficients y_n^j, z_n^j are the RK solutions of the j-th system in the cascade (3.4); the diagram (3.26) commutes. The Δy^v, Δz^v errors satisfy the order-reduction estimate of Theorem 3.4: stage order q ⇒ Δy_n^v=O(h^{q+2-v}), Δz_n^v=O(h^{q+1-v}) for v≤q+1.
- Rosenbrock methods for DAEs (VI.4): order conditions involve a richer tree class DAT_y, DAT_z, LDAT (differential-algebraic trees) with both meagre and fat vertices. A Rosenbrock method is "stiffly accurate" if both a_{si}+γ_{si}=b_i and α_s=1, which makes the last stages equivalent to a simplified Newton iteration on the algebraic part (Schneider 1991); R(∞)=0 follows automatically.
- RODAS construction: s=6, order 4(3) embedded, stiffly accurate (Eq. 4.39), with both Y_1 and Y_1-hat being stiffly accurate. Free parameters γ=0.25, α_2=0.386, α_3=0.21, α_4=0.63 yield A-stability and small error constants. RODAS5 (Di Marzo 1992) extends to order 5 with s=8 stages. Inconsistent initial values introduce additional order conditions Σb_iω_{ij}=1 ensuring R(∞)=0 dampens the algebraic-component drift δ.
- Extrapolation of linearly implicit Euler for DAEs (VI.5): Deuflhard–Hairer–Zugck (1987) prove a perturbed asymptotic expansion y_i-y(x_i) = Σh^j(a_j(x_i)+α_i^j) + O(h^{M+1}), with localised perturbation terms α_i^j, β_i^j supported near initial values (Tables 5.1–5.2). The extrapolation tableau orders r_jk, s_jk (Tables 5.3–5.4) show differential-algebraic orders that grow more slowly than classical orders due to retained perturbations. For SPP problems (Theorem 5.6) ε^2 perturbations T_jj(H/ε)·b_2(0) appear, decaying exponentially for H/ε→∞ — the basis for SEULEX's effectiveness on stiff and DAE problems. Dense output (Hairer–Ostermann 1990) uses Hermite interpolation with extrapolated derivatives only at the right end, avoiding amplification of boundary-layer perturbations.
- Quasilinear problems C(y)y'=f(y) (VI.6): when C(y) has constant rank m<n, the problem represents a quasilinear DAE. The consistency condition T_2(y)f(y)=0 plus invertibility of (B'/(T_2 f)') in (6.13) gives a locally unique solution (Lemma 6.1). Lemma 6.2 establishes that C(y)+λ(f'(y_0)-f̄(y_0,y'_0)) is invertible for small λ≠0 — the key to RK/multistep/Rosenbrock feasibility on quasilinear DAEs. Examples include the moving-finite-element method (K. Miller–R.N. Miller 1981) applied to Burgers' equation, where the mass matrix C(y) becomes singular near inflection points.
- Numerical treatments of C(y)y'=f(y): transformation to semi-explicit form y'=z, 0=C(y)z-f(y) makes the index-1 framework applicable; Deuflhard–Nowak's LIMEX uses a modified linearly-implicit Euler discretisation (Eq. 6.20) approximating C(y_i)C(y_0)^{-1}≈I; alternatively the semi-explicit discretisation (Eq. 6.23) follows directly. Both admit perturbed asymptotic expansions justifying extrapolation (Lubich 1989).

## Relevant Concepts

- [[concepts/singular-perturbation-problem]] — Defining problem class of the chapter.
- [[concepts/differential-algebraic-equation]] — Index-1 DAE as limit ε→0.
- [[concepts/index-1-dae]] — Semi-explicit case with invertible g_z.
- [[concepts/state-space-form]] — Ordinary ODE y'=f(y,G(y)) obtained from index-1 DAE.
- [[concepts/epsilon-embedding-method]] — Apply numerical method to full SPP then set ε=0.
- [[concepts/reduced-system]] — Limit ε=0 of the SPP.
- [[concepts/stiffly-accurate-method]] — a_{si}=b_i for RK; (4.39) variant for Rosenbrock.
- [[concepts/stage-order]] — q=min C(η); governs DAE order reduction.
- [[concepts/order-reduction]] — Effective-order loss on DAEs and SPPs.
- [[concepts/runge-kutta-method]] — Applied to DAEs and SPPs.
- [[concepts/linear-multistep-methods]] — Applied to DAEs and SPPs (Gear 1971).
- [[concepts/rosenbrock-method]] — Stiffly accurate variants for DAEs (RODAS).
- [[concepts/extrapolation-method]] — Linearly implicit Euler/midpoint extrapolation (SEULEX/SODEX).
- [[concepts/linearly-implicit-euler]] — Basic discretisation for extrapolation on DAEs.
- [[concepts/boundary-layer]] — Exponentially decaying transient term in SPP solutions.
- [[concepts/asymptotic-expansion]] — Power-series expansion in ε for smooth solutions.
- [[concepts/perturbed-asymptotic-expansion]] — Localised perturbations supported near initial values (Deuflhard–Hairer–Zugck 1987).
- [[concepts/dense-output]] — Continuous numerical solution; Hairer–Ostermann's right-end Hermite construction.
- [[concepts/quasilinear-dae]] — C(y)y'=f(y) with singular constant-rank C.
- [[concepts/method-of-lines]] — Source of SPP via parabolic-PDE space discretisation.
- [[concepts/moving-finite-elements]] — K. Miller–R.N. Miller adaptive method.
- [[concepts/transistor-amplifier]] — Classic DAE example with singular M = mass matrix.
- [[concepts/implicit-function-theorem]] — Underpins z=G(y) under invertible g_z.
- [[concepts/logarithmic-norm]] — Used in stability assumption μ(g_z)≤-1.
- [[concepts/holomorphic-semigroup]] — Sectorial assumption (2.5) for Lubich's Theorem 2.2.
- [[concepts/differential-algebraic-tree]] — DAT_y, DAT_z, LDAT for Rosenbrock-DAE order conditions.
- [[concepts/simplified-newton-iteration]] — Last stages of stiffly accurate Rosenbrock methods.
- [[concepts/van-der-pol-equation]] — Recurring stiff-SPP example in Lienard form.
- [[concepts/brusselator]] — Diffusion-reaction stiff PDE example.
- [[concepts/burgers-equation]] — Quasilinear-PDE example for moving FE.
- [[entities/radau5]] — Stiffly accurate IRK code with implicit-DAE support.
- [[entities/rodas]] — Stiffly accurate Rosenbrock code constructed in VI.4.
- [[entities/rodas5]] — Order-5 extension (Di Marzo 1992).
- [[entities/seulex]] — Extrapolated linearly-implicit Euler code.
- [[entities/limex]] — Deuflhard–Nowak quasilinear-DAE extrapolation code.
- [[entities/christian-lubich]] — Convergence theorems for DAE/SPP.
- [[entities/ernst-hairer]] — Co-author; epsilon-expansion theorems.
- [[entities/peter-deuflhard]] — Extrapolation code lineage.
- [[entities/c-w-gear]] — BDF for DAEs (1971).
- [[entities/anatolii-vasileva]] — Boundary-layer asymptotic expansions.

## Source Metadata

- Source type: book chapter
- Book title: *Solving Ordinary Differential Equations II — Stiff and Differential-Algebraic Problems*
- Chapter: VI — Singular Perturbation Problems and Index 1 Problems (Sections VI.1–VI.6)
- File path: `raw/solving_ordinary_differential_equations_ii/_txt/05-chapter-vi-singular-perturbation-problems.txt`
- Authors: E. Hairer, G. Wanner
