---
title: 'Solving Ordinary Differential Equations II — Chapter VII: Differential-Algebraic
  Equations of Higher Index'
type: source
id: summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations
kind: publication
tags:
- ode
- dae
- stiff
- index-reduction
- multibody
- hamiltonian
- symplectic
- runge-kutta
- bdf
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
---

## Key Points

- General implicit DAEs F(u',u)=0 with singular ∂F/∂u' are classified by index. For linear constant-coefficient pencils Bu'+Au=d, the Weierstrass–Kronecker canonical form (Theorem 1.1) yields P A Q = diag(C, I), P B Q = diag(I, N) where N is block-nilpotent; the maximal nilpotency size is the index of nilpotency, equal to the differentiation index for these problems.
- The differentiation index di (Gear–Petzold 1983, 1984) equals the number of analytic differentiations needed to extract an explicit ODE u'=φ(u) (the "underlying ODE"). Index 1: y'=f(y,z), 0=g(y,z) with g_z invertible. Index 2: 0=g(y) alone, requires the hidden constraint g_y(y)f(y,z)=0 and the index-1 condition g_y f_z invertible. Index 3 (1.15): typified by constrained mechanical systems q'=u, M(q)u'=f-G^Tλ, 0=g(q) with G M^-1 G^T invertible.
- The perturbation index pi (Hairer–Lubich–Roche 1989, HLR89) measures sensitivity: pi=m if perturbations of order δ in the equation produce solution errors bounded by ‖δ‖, ..., ‖δ^{(m-1)}‖. The Lubich (1989) example M(y)y'=f(y) and Campbell–Gear's nilpotent-Jordan example (1.32) show that the two indices can differ arbitrarily.
- Control problems (1.39): y'=f(y,u) with cost functional ∫φ(y,u)dx leads via Pontryagin's minimum principle to a DAE y'=f, v'=-f_y^T v-φ_y^T, 0=B^T v+φ_u, of variable index depending on the regularity of D=φ_uu and B^T C B.
- Constrained mechanical systems admit three equivalent formulations: index-3 position level (1.46a-c), index-2 velocity level (1.46a,b,d) with G(q)u=0, and index-1 acceleration level (1.46e) using the augmented system. The Gear–Gupta–Leimkuhler (GGL) formulation (1.48) appends the velocity constraint with an extra multiplier μ to keep both constraints satisfied while integrating an index-2 system.
- Index reduction by differentiation (VII.2): differentiating constraints down to index-1 enables standard ODE codes but introduces the drift-off phenomenon — for constrained mechanical systems g(q(t)) grows quadratically and G(q)u linearly with t (Theorem 2.1, Eq. 2.6). Remedies: Baumgarte stabilisation (1972, replaces constraint by g̈+2αġ+β²g=0 with α=β>0), projection onto position and velocity manifolds (Eq. 2.10, 2.11), local state-space form (Potra–Rheinboldt 1990) using generalised coordinate partitioning (Wehage–Haug 1982) or tangent-space parametrisation. Numerical study shows velocity-stabilisation projection alone is as effective as combined position+velocity projection.
- Overdetermined DAE formulation (Eq. 2.25, Führer 1988) appends constraints at multiple derivative levels and treats the resulting system via least-squares with Lagrange multipliers (Führer–Leimkuhler 1991), yielding a method closely related to the GGL discretisation. Campbell's unstructured-higher-index approach (1989) builds the derivative-array system and constructs an underlying ODE via QR factorisation.
- Multistep methods for index-2 DAEs (VII.3): the BDF schemes converge with full order p for the differential variable y but only with O(h^{p-1}) (after k initialisation steps) for the algebraic variable z. The key tool is the projector P(x)=I-f_z(g_y f_z)^{-1} g_y, and the local error bound involves (I-P)δ_n in algebraic-variable components. Sectors where σ(ζ)-τ(ζ) has all roots inside the unit disc allow β-blocked Adams variants and difference-corrected BDF (Söderlind 1989) to extend the class of usable methods.
- Runge–Kutta methods for index-2 DAEs (VII.4): the ε-embedding method (4.1) gives Y_n=O(h^p), Z_n=O(h^{q+1}) for stiffly accurate methods. For methods with R(∞)≠0 the z-component diverges. Collocation methods at Radau IIA points achieve superconvergence O(h^{2s-1}) for y and O(h^s) for z. Projected RK methods (Ascher–Petzold 1991, Hairer–Wanner) restore the position constraint after each step at modest cost.
- Order conditions for index-2 DAE Runge–Kutta (VII.5): the set DAT_2 of differential-algebraic trees (with meagre and fat vertices) generalises the classical Butcher trees. The new trees yield additional order conditions; for stiffly accurate Radau IIA the inhomogeneous expansion in elementary differentials F_J(t) is governed by the modified α_{ij}=Σ a_{ik}w_{kj} weights with simplifying assumptions C(η), D(ζ) inherited from the underlying RK theory. RODAS satisfies these new conditions automatically (Sect. VI.4 link).
- Half-explicit methods (VII.6, HLR89; Brasey–Hairer 1993; Murua 1995; Arnold 1995): the differential variable y is advanced explicitly while z is determined implicitly via 0=g(Y_i); these methods are highly efficient for constrained mechanical systems because they only require linear systems of the form (M G^T; G 0) per stage (Eq. 6.17). Coupled with Dormand–Prince RK5(4) pairs, Murua's order-5 scheme is implemented in PHEM56. The GBS-type extrapolation (Lubich 1989, Eq. 6.18) achieves h²-expansion for index-2 problems with f linear in z.
- Multibody mechanism computation (VII.7): Andrews' 7-body squeezer mechanism with 3 loops and 6 algebraic constraints. The chapter provides a complete Fortran reference implementation of M(q), f(q,q'), g(q), G(q). Comparisons of half-explicit PHEM56, BDF (DASSL), Rosenbrock (RODAS), and IRK (RADAU5) for nonstiff and stiff variants reveal that half-explicit methods dominate the nonstiff regime; implicit codes are essential for stiff variants. The stiff variant uses a stiff spring and gives stress-test conditions.
- Symplectic methods for constrained Hamiltonian systems (VII.8): the constrained Hamiltonian system q'=H_p, p'=-H_q-G^Tλ, 0=g(q) has its flow symplectic on the constrained manifold M (Theorem 8.1). The first-order symplectic method (8.8a-e) requires a position projection after each step; the second-order symmetric SHAKE/RATTLE pair (Ryckaert–Ciccotti–Berendsen 1977; Andersen 1983) and Jay's generalisation (Eq. 8.19) integrate molecular-dynamics problems while preserving symplectic structure. The Lobatto IIIA-IIIB pair (Theorem 8.5, Jay 1994/96) gives high-order (2s-2) symplectic methods on M, with Lobatto IIIA in the role of (b_i, a_{ij}) and Lobatto IIIB as (b̂_i, â_{ij}). Composition methods (Yoshida 1990, Reich 1996) generate arbitrarily-high-order symplectic methods at cost 3^{k-1} applications of the basic method.
- Backward error analysis on manifolds: a symplectic integrator with step h applied to a constrained Hamiltonian system can be viewed as the exact flow of a modified Hamiltonian H̃ on a modified manifold M̃, with H̃-H=O(h^p) for a p-th order method. This explains the long-term energy near-conservation observed in symplectic-method numerical experiments (perturbed Kepler problem, Fig. 2.3).

## Relevant Concepts

- [[concepts/differential-algebraic-equation]] — Central object of the chapter.
- [[concepts/index-of-a-dae]] — Defining classification.
- [[concepts/differentiation-index]] — Gear–Petzold formulation.
- [[concepts/perturbation-index]] — HLR89 sensitivity measure.
- [[concepts/index-of-nilpotency]] — Linear-constant-coefficient setting.
- [[concepts/weierstrass-kronecker-form]] — Canonical form for regular matrix pencils.
- [[concepts/matrix-pencil]] — A+λB structure.
- [[concepts/index-2-dae]] — y'=f(y,z), 0=g(y); central case for analysis.
- [[concepts/index-3-dae]] — Constrained mechanical systems formulation.
- [[concepts/constrained-mechanical-system]] — Index-3 Lagrangian model.
- [[concepts/constrained-hamiltonian-system]] — Index-3 Hamiltonian formulation.
- [[concepts/control-problem-dae]] — Optimal control reformulated as DAE.
- [[concepts/euler-lagrange-equation]] — Mechanical-systems foundation.
- [[concepts/hidden-constraint]] — Differentiated algebraic constraint.
- [[concepts/drift-off]] — Constraint-violation growth under index reduction.
- [[concepts/baumgarte-stabilization]] — Damping the drift-off.
- [[concepts/projection-method-dae]] — Restoring constraints by projection.
- [[concepts/state-space-form]] — Local-coordinate ODE on the constraint manifold.
- [[concepts/generalized-coordinate-partitioning]] — Wehage–Haug parametrisation.
- [[concepts/tangent-space-parametrization]] — Potra–Rheinboldt local coordinates.
- [[concepts/ggl-formulation]] — Gear–Gupta–Leimkuhler stabilised index-2 system.
- [[concepts/overdetermined-dae]] — Adds extra differentiated constraints (Führer).
- [[concepts/derivative-array]] — Campbell's underlying-ODE construction.
- [[concepts/half-explicit-method]] — Explicit in y, implicit in z; ideal for constrained mechanics.
- [[concepts/projected-runge-kutta]] — RK plus projection for index 2/3 DAE.
- [[concepts/index-reduction]] — Differentiation-based simplification of higher-index DAE.
- [[concepts/differential-algebraic-tree]] — DAT_2, LDAT for order conditions.
- [[concepts/stiffly-accurate-method]] — Essential for DAE convergence.
- [[concepts/runge-kutta-method]] — Applied directly to DAEs.
- [[concepts/gear-bdf]] — Default DAE solver class (Gear 1971).
- [[concepts/extrapolation-method]] — GBS-style for index-2 DAEs (Lubich).
- [[concepts/symplectic-method]] — Geometry-preserving methods for Hamiltonian DAEs.
- [[concepts/symplectic-integrator]] — Class includes Lobatto IIIA-IIIB pair, SHAKE/RATTLE.
- [[concepts/shake-algorithm]] — Ryckaert–Ciccotti–Berendsen for molecular dynamics.
- [[concepts/rattle-algorithm]] — Andersen's velocity-completed SHAKE.
- [[concepts/lobatto-iiia-iiib-pair]] — Jay's high-order symplectic DAE integrator.
- [[concepts/composition-method]] — Yoshida-type higher-order construction.
- [[concepts/backward-error-analysis-manifolds]] — Tool for explaining symplectic-integrator long-term behaviour.
- [[concepts/manifold-differential-equation]] — Underlying geometric setting.
- [[concepts/runge-kutta-collocation]] — Used for high-order DAE solvers.
- [[concepts/lagrange-multiplier]] — λ in constrained systems.
- [[concepts/multibody-system]] — Application area (Schiehlen 1990).
- [[concepts/pendulum-equation]] — Recurring index-3 test problem.
- [[concepts/squeezer-mechanism]] — Andrews' seven-body benchmark.
- [[concepts/kepler-problem]] — Standard symplectic-method test problem.
- [[entities/radau5]] — Used for stiff DAE comparisons.
- [[entities/rodas]] — Stiffly accurate Rosenbrock for index-1 (and via index reduction higher index).
- [[entities/dassl]] — Petzold's BDF code referenced for DAE comparisons.
- [[entities/phem56]] — Murua's half-explicit DAE code.
- [[entities/seulex]] — Extrapolation code applicable to index-1 DAEs.
- [[entities/limex]] — Quasilinear-DAE extrapolation code (Deuflhard–Nowak).
- [[entities/c-w-gear]] — DAE numerical analysis pioneer.
- [[entities/linda-petzold]] — DAE convergence theory; differentiation index.
- [[entities/christian-lubich]] — Convergence theory; HLR89; extrapolation for DAE.
- [[entities/ernst-hairer]] — Co-author; convergence theorems and HLR89.
- [[entities/laurent-jay]] — Symplectic Lobatto IIIA-IIIB pair.
- [[entities/sebastian-reich]] — Composition methods on manifolds.
- [[entities/werner-schiehlen]] — Multibody-dynamics reference.

## Source Metadata

- Source type: book chapter
- Book title: *Solving Ordinary Differential Equations II — Stiff and Differential-Algebraic Problems*
- Chapter: VII — Differential-Algebraic Equations of Higher Index (Sections VII.1–VII.8)
- File path: `raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt`
- Authors: E. Hairer, G. Wanner
