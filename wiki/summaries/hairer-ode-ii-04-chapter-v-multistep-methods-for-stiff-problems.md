---
title: "Solving Ordinary Differential Equations II — Chapter V: Multistep Methods for Stiff Problems"
type: summary
tags: [ode, stiff, bdf, multistep, numerical-integration, transient, a-stability, g-stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt"]
confidence: high
---

## Key Points

- BDF (backward differentiation formulas), introduced by Curtiss & Hirschfelder (1952) and popularised by Gear (1971), are historically the first methods proposed for stiff ODEs and remain the most widely used. Their root-locus curves (V.1) cover progressively smaller regions of the imaginary axis; only k=1 and k=2 are A-stable, k≤6 are zero-stable, and k≥7 are inconsistent/unstable.
- The Adams family (explicit and implicit) gives tiny stability regions in the left half plane, completely unsuitable for stiff problems; using a predictor–corrector (PECE) scheme further shrinks the region (Chase 1962, Crane–Klopfenstein 1965, Krogh 1966). Nystrom and Milne–Simpson methods reduce to single-point or imaginary-axis-interval stability domains (Hamming 1959), exhibiting "weak instability" first analysed by Dahlquist (1951).
- The second Dahlquist barrier (Theorem 1.4, Dahlquist 1963): any A-stable linear multistep method has order p ≤ 2 and, at order 2, error constant C ≤ -1/12 with the trapezoidal rule as the unique extremal. Proved via the Riesz–Herglotz characterisation of Re(ρ(ζ)/σ(ζ)) > 0 on |ζ|>1.
- A(α)-stability (Widlund 1967), stiff stability (Gear 1971), Ao-stability (Cryer 1973), A-stability (Nevanlinna's joke 1979) — weakened conditions that admit higher orders. For BDF, the A(α)-stability angles are 90°, 90°, 86°, 73°, 52°, 18° for k=1..6. Theorem 2.2 (Grigorieff–Schroll 1978) shows A(α)-stable k-step methods of order k exist for every α<π/2 and every k.
- High-order A(α)-stable methods cannot be both practically accurate: the Jeltsch–Nevanlinna accuracy barrier (Theorem 2.6) bounds the L∞ Fourier-Peano kernel of any method whose stability region contains a tangent disc D_r by a constant times r^(p-1)/(p-2)! showing the second Dahlquist barrier fundamentally cannot be broken without paying a price in error constants.
- Generalised multistep methods (V.3) sidestep the barrier: Enright's second-derivative methods Σαᵢy_{n+i} = hΣβᵢf_{n+i} + h²γₖg_{n+k} achieve order k+2, A-stable for k=1,2 and stiffly stable up to k=7. Second-derivative BDF (SDBDF) goes up to order 11 (stiffly stable to k=9). Blended methods (Skeel & Kong 1977) combine Adams and BDF with weight -hJγ to handle both stiff and nonstiff regimes. Cash's extended BDF methods (1980, 1983 modified MEBDF) use a "super-future" point at x_{n+k+1} and a three-stage predictor–corrector–corrector scheme A-stable to p=4, stiffly stable to p=9.
- Multistep collocation methods of Radau type (Guillou–Soulé 1969, Lie–Nørsett 1989) build a degree-s+k-1 spline polynomial fitting k past values and s collocation conditions; choosing the c_i via Krylov's equations gives order 2s+k-2, A-stable for k=1,2 (orders 5,6) and A(α)-stable up to p=20.
- Order star theory on Riemann surfaces (V.4) extends the IV.4 finger-counting machinery to multi-valued characteristic functions Q(μ,ζ). The Daniel & Moore conjecture (proved Theorem 4.4): an A-stable s-stage RK or s-derivative multistep method has order p ≤ 2s; equality requires error constant satisfying (-1)^s C ≥ s!s!/((2s)!(2s+1)!) achieved by the diagonal Padé. Property C (Jeltsch–Nevanlinna 1982) generalises the comparison machinery to methods whose principal root carries the entire instability, yielding stability-comparison theorems (no method's scaled domain can strictly contain another's).
- General linear methods (4.26): U_{n+1} = AU_n + hB̄f(V_n), V_n = ĀU_n + hBf(V_n); stability governed by S(μ) = Ā + μB̄(I - μB̃)^-1B. The number of "poles representing numerical work" equals the rank of B̃, preserving the Daniel–Moore barrier for the entire general-linear class.
- G-stability (Dahlquist 1975/76, V.6): the one-leg method companion (Σα_iy_{n+i} = hf(x̄, Σβ_iy_{n+i})) is contractive in a quadratic Lyapunov norm ‖·‖_G with symmetric positive-definite G iff the underlying linear method is A-stable (Dahlquist's equivalence theorem, proved via Riesz–Herglotz). Algebraic stability for general linear methods (V.9, Burrage–Butcher 1979) requires a block-positive-definite condition generalising IV.12.
- The Kreiss matrix theorem (V.7): the resolvent condition ‖(zI-A)^-1‖ ≤ C/(|z|-1) for |z|>1 is equivalent to power-boundedness ‖A^n‖ ≤ M, with M proportional to kC for k×k matrices (LeVeque–Trefethen 1984 proof). Combined with Lemma 7.3–7.5, this controls the companion matrix C(μ) uniformly over the stability domain.
- Multiplier technique (Nevanlinna–Odeh 1981, V.8): instead of taking inner product of the error recursion with Δy_m, take it with Σμ_{m-j}Δy_j for a rational multiplier μ(ζ); the modified scheme (ρ̃,σ̃) = (ρτ, σς)/x must be A-stable, granting G-stability convergence proofs to A(α)-stable methods (BDF k=2..6 all admit such η-multipliers). Yields global error bounds for one-sided Lipschitz problems satisfying multiplier-modified contractivity (8.22).
- Discrete variation of constants (Crouzeix–Raviart 1976, Lubich 1988–91, V.7): the global error obeys e_m = Σ r_{m-j}(μ)d_j where r_j(μ) is the discrete resolvent — coefficients of (δ(ζ)-μ)^-1ζ^k/σ(ζ^-1). Combined with the Kreiss decay estimates this gives O(h^p) convergence for holomorphic-semigroup (sectorial) problems including parabolic PDE method-of-lines discretisations (Theorem 7.10), with order-reduction analysis for the Prothero–Robinson model showing e_m = O(|λ|^-1 h^{p-1}) for sectorial linear systems.

## Relevant Concepts

- [[concepts/linear-multistep-methods]] — Defining family of the chapter.
- [[concepts/gear-bdf]] — Foundational stiff multistep family (Curtiss–Hirschfelder, Gear).
- [[concepts/adams-method]] — Explicit and implicit Adams families discussed for comparison.
- [[concepts/predictor-corrector-method]] — PECE scheme and its stability degradation.
- [[concepts/nystrom-method]] — Explicit midpoint and Milne–Simpson; weak instability.
- [[concepts/one-leg-method]] — Dahlquist's nonlinear-stable reformulation.
- [[concepts/g-stability]] — Quadratic Lyapunov contractivity for one-leg methods.
- [[concepts/dahlquist-barrier]] — Order ≤ 2 for A-stable LMS.
- [[concepts/daniel-moore-conjecture]] — Order ≤ 2s for A-stable s-stage RK/s-derivative MS (proved).
- [[concepts/a-stability]] — Used throughout for comparison and barrier theorems.
- [[concepts/a-alpha-stability]] — Widlund's sector relaxation.
- [[concepts/stiff-stability]] — Gear's hybrid concept.
- [[concepts/ao-stability]] — Cryer's negative-real-axis stability.
- [[concepts/stability-region]] — Defined via characteristic equation ρ(ζ) - μσ(ζ).
- [[concepts/root-locus-curve]] — Image of the unit circle under μ = ρ(ζ)/σ(ζ).
- [[concepts/order-star]] — Extended to Riemann surfaces in V.4.
- [[concepts/riemann-surface]] — Algebraic-function setting for multi-valued characteristic equations.
- [[concepts/property-c]] — Jeltsch–Nevanlinna comparison-of-stability-domains tool.
- [[concepts/enright-method]] — Second-derivative multistep methods.
- [[concepts/sdbdf-method]] — Second-derivative BDF.
- [[concepts/blended-multistep-method]] — Skeel–Kong combined Adams/BDF.
- [[concepts/extended-bdf-method]] — Cash's EBDF / MEBDF using super-future points.
- [[concepts/multistep-collocation]] — Spline-based generalisations including Radau-type multistep.
- [[concepts/general-linear-method]] — Butcher's unifying RK+MS framework.
- [[concepts/peano-kernel]] — Sharpest measure of multistep truncation error.
- [[concepts/kreiss-matrix-theorem]] — Power-boundedness via resolvent condition.
- [[concepts/multiplier-technique]] — Nevanlinna–Odeh convergence tool for A(α)-stable LMS.
- [[concepts/discrete-variation-of-constants]] — Lubich's discrete resolvent approach.
- [[concepts/holomorphic-semigroup]] — Sectorial-operator analysis underpinning parabolic-PDE convergence.
- [[concepts/error-constant]] — Quantitative measure of method accuracy in (1.19) and (4.16).
- [[concepts/order-reduction]] — Loss of effective order on stiff problems.
- [[concepts/trapezoidal-rule]] — Unique extremal A-stable LMS of order 2.
- [[concepts/backward-euler]] — BDF1; baseline L-stable method.
- [[concepts/dahlquist-test-equation]] — Underlying y'=λy used throughout.
- [[concepts/one-sided-lipschitz-condition]] — Setting for nonlinear convergence theorems.
- [[concepts/method-of-lines]] — Parabolic-PDE source addressed by sectorial Theorem 7.10.
- [[entities/germund-dahlquist]] — Founder of multistep stability theory.
- [[entities/c-w-gear]] — Author of the foundational 1971 book and stiff-stability concept.
- [[entities/charles-curtiss]] — Co-introducer of BDF.
- [[entities/heinz-otto-kreiss]] — Originator of the resolvent theorem.
- [[entities/john-butcher]] — General-linear-method framework.
- [[entities/christian-lubich]] — Discrete variation-of-constants for parabolic problems.
- [[entities/olavi-nevanlinna]] — Multiplier technique co-discoverer.
- [[entities/jeff-cash]] — Extended-BDF (EBDF/MEBDF) developer.
- [[entities/syvert-norsett]] — Multistep collocation co-developer.
- [[entities/peter-deuflhard]] — Stiff-extrapolation lineage referenced for codes.
- [[entities/lsode]] — Hindmarsh's BDF code mentioned for numerical comparisons in V.5.

## Source Metadata

- Source type: book chapter
- Book title: *Solving Ordinary Differential Equations II — Stiff and Differential-Algebraic Problems*
- Chapter: V — Multistep Methods for Stiff Problems (Sections V.1–V.9)
- File path: `raw/solving_ordinary_differential_equations_ii/_txt/04-chapter-v-multistep-methods-for-stiff-problems.txt`
- Authors: E. Hairer, G. Wanner
