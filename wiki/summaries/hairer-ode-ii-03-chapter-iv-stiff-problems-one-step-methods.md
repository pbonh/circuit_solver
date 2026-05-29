---
title: 'Solving Ordinary Differential Equations II — Chapter IV: Stiff Problems –
  One-Step Methods'
type: source
id: source-hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods
kind: derived-summary
tags:
- ode
- stiff
- runge-kutta
- rosenbrock
- numerical-integration
- transient
- a-stability
- l-stability
- b-stability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt
---

## Key Points

- Stiff equations are problems where certain implicit methods (especially BDF) outperform explicit ones tremendously; classical examples are Robertson's chemical kinetics, van der Pol with large mu, the Brusselator with method-of-lines diffusion, and the elastic-beam Lagrangian system. Stiffness depends not only on the Jacobian eigenvalues but on dimension, smoothness of the solution, integration interval, and initial conditions.
- For explicit Runge–Kutta methods applied to y'=Jy, the linearised behaviour is governed by R(z)=R(hλ), and stability requires hλ to lie in S={z:|R(z)|≤1}; stiff problems have eigenvalues so far in the left half-plane that explicit methods need impractically tiny steps. Automatic stiffness detection (Shampine–Hiebert second error estimator; dominant-eigenvalue power-method estimate) lets nonstiff codes warn about stiffness.
- PI step-size control (Gustafsson, Lundh, Söderlind 1988) replaces the standard I-controller with proportional-integral feedback h_{n+1}=h_n·(Tol/err_n)^α·(err_{n-1}/err_n)^β; this damps the ragged step-size oscillations that plague explicit codes near the stability border and reduces step rejections.
- Stabilised explicit RK methods (Chebyshev-type) construct R(z) from shifted/damped Chebyshev polynomials to give large negative-real stability domains (Lebedev's DUMKA, van der Houwen–Sommeijer's RKC, Abdulle–Medovikov's ROCK4); these are efficient for mildly stiff, large-dimension PDE-derived systems with eigenvalues clustered near the negative real axis.
- A-stability (Dahlquist 1963): S⊇C^-; L-stability adds R(∞)=0 to damp stiff modes quickly (Ehle 1969); A(α)-stability covers a sector |arg(-z)|<α (Widlund 1967); stiffly accurate methods (a_{si}=b_i, Prothero–Robinson) automatically have R(∞)=0 and are essential for DAEs and singular perturbation problems.
- Order stars (Wanner, Hairer, Nørsett 1978): comparing |R(z)| to |e^z| rather than to 1 yields a topological tool that proves classical results (Dahlquist barrier, Daniel–Moore conjecture, real-pole order bounds, Padé A-stability) via finger-counting arguments. The (k,j) Padé approximation R_{kj} is A-stable iff k≤j≤k+2 (Ehle's conjecture).
- Implicit RK families based on simplifying assumptions B(p), C(η), D(ζ) (Butcher 1964): Gauss methods (order 2s, (s,s)-Padé, A-stable), Radau IA & Radau IIA (order 2s-1, A- and L-stable), Lobatto IIIA, IIIB, IIIC (order 2s-2). The W-transformation using shifted Legendre polynomials gives a unified construction tool and reduces A-stability to continued-fraction positivity arguments.
- SDIRK methods have a single repeated diagonal γ allowing one LU-decomposition per step; tractable algebraic order conditions reduce to simple polynomials in γ. The 5-stage L-stable order-4 SDIRK4 method (γ=1/4 with c'_2=1/2, c'_3=3/5, rational coefficients) plus continuous embedded third-order error estimator forms a usable code.
- Rosenbrock (linearly implicit RK) methods replace nonlinear systems by linear systems with matrix I-hγJ, where J=f'(y_0); order conditions involve trees with vertex-marking rules (β_{ij}=α_{ij}+γ_{ij}); methods of Kaps–Rentrop, Shampine, Veldhuizen, GRK4T and stiffly accurate RODAS (built for DAEs) all stem from this framework. W-methods relax J to an arbitrary approximation A but pay with many more order conditions (tree-set TW).
- Practical RADAU5 implementation: simplified Newton with starting values from previous-step interpolation polynomial, complex-arithmetic 2n×2n block solve (factor ~5 reduction over plain 3n×3n), Hessenberg pre-transformation for large dense Jacobians, error estimator (I-hγJ)^-1 form to keep err bounded as hλ→∞, and Gustafsson predictive (PI-flavoured) step controller log C_{n+1}-log C_n constant.
- Extrapolation methods for stiff problems use linearly implicit Euler or linearly implicit midpoint rule as base scheme (Bader & Deuflhard 1983); the Aitken–Neville tableau on h^2 (midpoint) or h (Euler) expansions yields adaptive-order codes SODEX and SEULEX; smoothing (Gragg/Lindberg) is needed to recover L-stability.
- B-stability (Butcher 1975), based on the one-sided Lipschitz condition (f(x,y)-f(x,z), y-z)≤ν‖y-z‖², generalises A-stability to nonlinear problems. Algebraic stability ((b_i≥0) and M=BA+A^TB-bb^T non-negative definite, Burrage–Butcher / Crouzeix 1979) is sufficient for B-stability and, for S-irreducible methods, equivalent (Hundsdorfer–Spijker 1981, with Kirszbraun's extension and Schoenberg's geometric proof). Gauss, Radau IA, Radau IIA, Lobatto IIIC are algebraically stable; Lobatto IIIA and IIIB are not.
- The error growth function φ_R(x) (linear) and φ_B(x) (nonlinear) measure contraction rates; both are superexponential for A- resp. B-stable methods (Hairer–Zennaro 1996), giving asymptotic stability bounds. The threshold factor (Spijker 1985, Bolley–Crouzeix, Kraaijevanger 1986) characterises contractivity in ‖·‖_∞ and ‖·‖_1 via absolute monotonicity (Bernstein 1928).
- Existence and uniqueness of IRK solutions hinge on the coercivity α_0(A^-1) (Crouzeix–Raviart 1980; Dekker 1984): under hν<α_0(A^-1) the simplified Newton system has a unique solution and perturbation estimates ‖Δg‖≤‖A^-1‖/(α_0(A^-1)-hν)‖δ‖ hold. Explicit values are computed for Gauss, Radau IA/IIA, Lobatto IIIC. The Lobatto IIIC methods with s≥3 satisfy α_0(A^-1)=0 but still have unique solutions for problems with μ(J)≤0 (Liu & Kraaijevanger 1988).
- Order reduction (Prothero–Robinson 1974) is the phenomenon that the effective order on stiff problems collapses to the stage order q=min(p, max C(η)). B-convergence (Frank, Schneid, Ueberhuber 1981) is convergence with constants independent of stiffness; algebraically stable methods of stage order q are B-convergent of order q (Theorem 15.3). Lobatto IIIB cannot be B-convergent (singular A leads to unbounded local error); Rosenbrock methods need new condition Σb_iω_{ij}α_j=1 (or m_i=b_i analogue of a_{si}=b_i) to escape O(h²) order reduction on stiff non-autonomous problems.

## Relevant Concepts

- [[concepts/stiff-circuit]] — Central object of the entire chapter.
- [[concepts/runge-kutta-method]] — Subject family.
- [[concepts/implicit-runge-kutta]] — Backbone for stiff one-step methods.
- [[concepts/explicit-runge-kutta]] — Discussed for stiffness detection and stabilisation.
- [[concepts/rosenbrock-method]] — Linearly implicit RK; Sect. IV.7.
- [[concepts/w-method]] — Rosenbrock variant with inexact Jacobian.
- [[concepts/sdirk-method]] — Singly diagonally-implicit RK with common γ on the diagonal.
- [[concepts/dirk-method]] — Diagonally implicit RK.
- [[concepts/extrapolation-method]] — SODEX / SEULEX based on Bader–Deuflhard linearly-implicit midpoint and Euler.
- [[concepts/dahlquist-test-equation]] — y'=λy underpinning A-stability and R(z).
- [[concepts/stability-function]] — R(z); polynomial for explicit, rational for implicit RK.
- [[concepts/stability-domain]] — S={z:|R(z)|≤1}; basis for explicit-method step bounds.
- [[concepts/a-stability]] — S⊇C^-; Dahlquist 1963.
- [[concepts/l-stability]] — A-stable plus R(∞)=0; Ehle 1969.
- [[concepts/a-alpha-stability]] — Sector stability; Widlund 1967.
- [[concepts/stiffly-accurate-method]] — a_{si}=b_i; Prothero–Robinson 1974.
- [[concepts/order-star]] — Topological tool, Wanner–Hairer–Nørsett 1978.
- [[concepts/pade-approximation]] — Optimal rational approximations to e^z.
- [[concepts/gauss-method]] — Order 2s collocation at shifted Legendre roots.
- [[concepts/radau-iia-method]] — L-stable order 2s-1 method underlying RADAU5.
- [[concepts/radau-ia-method]] — Companion left-Radau method.
- [[concepts/lobatto-iiic-method]] — Algebraically stable Lobatto family.
- [[concepts/lobatto-iiia-method]] — Collocation Lobatto; not B-stable.
- [[concepts/lobatto-iiib-method]] — Discrete-derivative Lobatto; not B-stable.
- [[concepts/collocation-method]] — Subclass characterised by C(s); Wright 1970.
- [[concepts/butcher-simplifying-assumptions]] — B(p), C(η), D(ζ); order analysis.
- [[concepts/w-transformation]] — Legendre-polynomial basis for RK construction; Hairer–Wanner 1981.
- [[concepts/simplified-newton-iteration]] — Reuse of Jacobian in implicit-RK solvers.
- [[concepts/pi-step-size-control]] — Gustafsson PI controller for adaptive integration.
- [[concepts/predictive-step-size-control]] — Gustafsson 1994 predictive variant.
- [[concepts/chebyshev-method]] — Stabilised explicit RK; DUMKA, RKC, ROCK4.
- [[concepts/automatic-stiffness-detection]] — Shampine–Hiebert and power-method estimators.
- [[concepts/one-sided-lipschitz-condition]] — Foundation for nonlinear stability theory.
- [[concepts/b-stability]] — Nonlinear contractivity; Butcher 1975.
- [[concepts/algebraic-stability]] — b_i≥0 and M non-negative definite; equivalent to B-stability for S-irreducible methods.
- [[concepts/an-stability]] — Stability for scalar non-autonomous linear problems.
- [[concepts/contractivity]] — Numerical analogue of ‖y(x)-z(x)‖ non-increasing.
- [[concepts/error-growth-function]] — φ_R(x) for linear, φ_B(x) for nonlinear problems.
- [[concepts/threshold-factor]] — Contractivity in ‖·‖_∞ and ‖·‖_1; Kraaijevanger.
- [[concepts/absolutely-monotonic-function]] — Bernstein characterisation linking R(z) to contractivity.
- [[concepts/von-neumann-theorem]] — Bound on ‖R(A)‖ via logarithmic norm.
- [[concepts/order-reduction]] — Stage-order collapse on stiff problems; Prothero–Robinson 1974.
- [[concepts/b-convergence]] — Stiffness-independent global error; Frank–Schneid–Ueberhuber 1981.
- [[concepts/stage-order]] — min(p, max C(η)); governs B-convergence.
- [[concepts/trapezoidal-rule]] — A-stable but not L-stable; limited B-convergence.
- [[concepts/backward-euler]] — Workhorse L-stable first-order method.
- [[concepts/coercivity-coefficient]] — α_0(A^-1) controlling existence and perturbation bounds.
- [[concepts/method-of-lines]] — Source of stiffness via diffusion discretisation.
- [[entities/radau5]] — Reference code based on 3-stage Radau IIA.
- [[entities/sdirk4]] — Hairer–Wanner SDIRK order-4 code.
- [[entities/rodas]] — Stiffly accurate Rosenbrock code for DAEs.
- [[entities/ros4]] — Rosenbrock order-4 code (multiple coefficient sets).
- [[entities/seulex]] — Extrapolation code based on linearly implicit Euler.
- [[entities/sodex]] — Extrapolation code based on linearly implicit midpoint (equivalent to Bader–Deuflhard METAN1).
- [[entities/dumka]] — Lebedev's Chebyshev-extended explicit-RK code.
- [[entities/rkc]] — Sommeijer Runge–Kutta–Chebyshev code.
- [[entities/dopri5]] — Reference explicit-RK code used as stiffness comparator.
- [[entities/germund-dahlquist]] — Founder of A-stability and barrier theorems.
- [[entities/john-butcher]] — Order theory, B-stability, simplifying assumptions.

## Source Metadata

- Source type: book chapter
- Book title: *Solving Ordinary Differential Equations II — Stiff and Differential-Algebraic Problems*
- Chapter: IV — Stiff Problems – One-Step Methods (Sections IV.1–IV.15)
- File path: `raw/solving_ordinary_differential_equations_ii/_txt/03-chapter-iv-stiff-problems-one-step-methods.txt`
- Authors: E. Hairer, G. Wanner
