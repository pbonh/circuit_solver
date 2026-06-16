---
title: "Sparse Direct LU Solver with Markowitz Ordering as Primary Linear Solver"
status: proposed
date: 2026-06-16
decision-makers:
  - circuit-solver-team
consulted: []
informed: []
---

# Sparse Direct LU Solver with Markowitz Ordering as Primary Linear Solver

## Context and Problem Statement

At each Newton-Raphson iteration in the transient analog solver, the Circuit Solver Delta must solve a sparse linear system: **Jₓ · Δx = −F(x)**, where **J** is the Jacobian (MNA matrix derived from circuit conductances/transconductances), **F** is the residual (Kirchhoff current/voltage equations), and **Δx** is the incremental solution update.

The linear solver is the computational bottleneck: it accounts for 60–90% of wall-clock time in typical circuit simulations. Choosing the wrong solver incurs either severe memory bloat (fill-in of the sparse matrix during factorization) or slow convergence / failed solves on ill-conditioned Jacobians (which arise naturally from near-degenerate devices like nearly-shorted resistors).

The decision determines not only performance but also robustness (does the solver handle every valid circuit without numerical breakdown?) and extensibility (can the solver be ported to GPU or distributed compute in future releases?).

## Decision Drivers

1. **Robustness under ill-conditioning**: Circuit matrices can be ill-conditioned (e.g., very small transistor gm, large resistor ratios, nearly-degenerate coupling). The solver must use pivoting and numerical stability techniques to avoid divergence or NaN results.
2. **Performance at O(n^1.5) scaling**: For planar VLSI circuits (which dominate modern design), the MNA matrix is sparse with O(n) nonzeros. A well-tuned sparse LU factorization achieves O(n^1.5) complexity, avoiding the O(n³) cost of dense methods.
3. **Correctness (numerical pivoting)**: SPICE simulators use Markowitz/minimum-degree pivoting to minimize fill-in during Gaussian elimination; direct factorization with pivoting is the only reliable way to maintain numerical stability.
4. **GPU/distributed extension**: Future optimization targets include GPU-accelerated sparse LU (cuSPARSE, NVIDIA libraries) and distributed factorization (for very large industrial circuits). The sparse direct path is well-supported by HPC libraries; iterative methods on the GPU are less mature.

## Considered Options

### Option 1: Sparse Direct LU with Markowitz/Minimum-Degree Ordering (SPICE Standard, KLU)
Factorize the Jacobian **J = LU** using Gaussian elimination with Markowitz ordering (or variants like minimum-degree, approximate minimum-degree). Markowitz ordering chooses pivot rows/columns to minimize the number of new nonzeros created during elimination, reducing memory and computation.

Store **L** and **U** in sparse format (compressed row/column). For each Newton iteration, perform a **forward-backward substitution** (O(nnz)) to solve the system.

**Pros:**
- SPICE standard for 50+ years; proven robustness in commercial tools.
- Markowitz ordering near-optimally minimizes fill-in for circuit matrices.
- Direct solution guarantees convergence (up to numerical precision) for all non-singular **J**.
- KLU (Kirchhoff Least Upper) reference implementation is publicly available, tested, and well-documented.
- Pivoting ensures numerical stability even for ill-conditioned matrices.
- Naturally extensible to GPU (cuSPARSE, rocsparse) and distributed (SuperLU_DIST).

**Cons:**
- Memory usage scales with fill-in; planar circuits → O(n^1.5) memory, but dense circuits or high-fill geometries → near O(n²).
- Factorization time can dominate if the matrix structure changes frequently (e.g., switches toggling on/off); requires re-factorization.
- Symbolic factorization (determining **L** and **U** sparsity patterns) adds one-time overhead per new circuit or topology.

### Option 2: Iterative GMRES with ILU Preconditioner
Use a Krylov-subspace iterative method (GMRES, BiCGSTAB) with an Incomplete LU (ILU) preconditioner to accelerate convergence. ILU discards small fill-in entries, trading accuracy for speed.

**Pros:**
- Memory usage controlled by ILU fill-in threshold; can be tuned for memory-constrained environments.
- Factorization step is faster than direct LU on some geometries (high-fill circuits).
- Naturally parallelizable (sparse matrix-vector product, Gram-Schmidt orthogonalization).

**Cons:**
- Convergence depends critically on **J** conditioning and ILU quality; poorly conditioned Jacobians (e.g., near-degenerate circuits) cause stalling or divergence even with preconditioning.
- Iterative methods require tuning (fill-in level, restart frequency, stopping tolerance); no universal "set and forget" parameters.
- Rounding errors accumulate; harder to debug why a solve failed.
- Not standard in SPICE; less proven for general circuit matrices. Industrial experience is limited.
- GPU implementation is less mature than direct LU (cuSPARSE has strong direct support; iterative + preconditioner = custom kernel integration).

### Option 3: Dense LU (Direct on Full Matrix)
Factor the entire Jacobian as a dense matrix using standard LAPACK routines (getrf/getrs for LU, getri for inversion).

**Pros:**
- Simple implementation; LAPACK is bulletproof and ubiquitous.
- Numerically very stable; LAPACK's LU includes full pivoting.

**Cons:**
- O(n³) factorization and O(n²) memory for even small circuits.
- Infeasible for circuits with >1000 nodes; typical modern VLSI is 10K–100K+ nodes.
- Ignores sparsity entirely; wastes computation on zero entries.
- Only acceptable as a fallback for tiny subcircuits or schematic-level designs.

### Option 4: Cholesky Factorization (For Symmetric PD Systems)
If the circuit is passive and certain simplifications are made, the Jacobian becomes symmetric positive-definite. Cholesky factorization is faster and more numerically stable than LU.

**Pros:**
- Theoretically most stable for symmetric PD matrices.
- Faster factorization than LU (half the operations).

**Cons:**
- **MNA Jacobians are not symmetric PD in general**. Nonlinear devices (diodes, transistors) and capacitive coupling introduce asymmetry. Cholesky fails or requires artificial symmetrization, which breaks physical correctness.
- Only applicable to very restricted circuit classes (passive RLC networks), defeating the purpose of a general simulator.

## Decision Outcome

**Chosen option: Sparse direct LU with Markowitz/minimum-degree ordering as primary; iterative GMRES+ILU as optional fallback for very large well-conditioned circuits.**

The decision is driven by the overwhelming evidence from 50 years of SPICE practice: direct LU with intelligent ordering is the workhorse linear solver for circuit simulation. Markowitz ordering near-optimally minimizes fill-in for the sparse, planar graphs typical of VLSI.

For a first release, the primary solver is sparse direct LU. As circuit sizes grow and if memory becomes critical, an optional iterative fallback (GMRES+ILU) can be exposed as a compile-time or runtime flag. However, iterative methods require careful tuning and are not a drop-in replacement; they are only viable for specific circuit classes (well-conditioned, power-grid-like).

## Consequences

- **Memory scales with fill-in**: For planar VLSI circuits (typical transistor layouts), fill-in is O(n^1.5), so memory scales similarly. Dense or highly connected circuits (e.g., analog mixed-signal with many bias networks) can see higher fill-in; memory usage should be monitored in production.
- **Factorization cost on topology changes**: If the circuit topology changes (e.g., switches toggling, parametric sweeps), the sparsity pattern of **J** may change, requiring re-symbolic-factorization. This is amortized over multiple Newton iterations within a time step, but can be a bottleneck for rapidly switching circuits.
- **GPU acceleration is a clear path**: cuSPARSE (NVIDIA) and rocSPARSE (AMD) provide highly optimized sparse LU and triangular solve routines. A future optimization target is GPU-resident **L** and **U** matrices and offloading the inner loop to GPU, avoiding repeated host–device transfers.
- **Distributed factorization for very large circuits**: SuperLU_DIST and similar libraries support distributed sparse LU on multi-node clusters. If the Circuit Solver Delta is deployed for industrial-scale simulation (100K+ nodes), distributed factorization is a natural extension.
- **Iterative solver exposure**: If memory or factorization time becomes a bottleneck in practice, expose GMRES+ILU as an optional solver. Mark it `experimental` until robustness on diverse circuits is demonstrated.

## Confirmation

Verification strategy:

1. **Factorization and solve correctness**: For each MNA matrix in the ISCAS-85 benchmark suite (10–20 circuits ranging from 100 to 5000+ nodes):
   - Perform LU factorization with Markowitz ordering.
   - Solve the system **Δx = L⁻¹U⁻¹ · F** (via forward/backward substitution).
   - Compute residual **r = J · Δx − (−F)** and verify **||r|| / ||F|| < 1e-10** (relative error).
   - Confirm that **||x_exact − x_solved||_∞ < 1e-10 · ||x_exact||_∞** (absolute accuracy on solution).

2. **Scaling verification**: Plot factorization time and memory usage vs. circuit size (number of nodes). Verify that factorization time scales sub-quadratically (ideally O(n^1.5)) and memory scales with fill-in (O(n^1.5) for planar).

3. **Robustness on ill-conditioned circuits**: Hand-craft small ill-conditioned test cases (e.g., resistor divider with 1 Ω and 1 GΩ in series). Confirm that the solver produces accurate solutions without numerical breakdown.

4. **Pivot analysis**: For each factorization, log the maximum pivot magnitude and minimum pivot magnitude. Verify that small pivots are avoided via Markowitz/minimum-degree ordering and partial pivoting.

5. **Regression suite**: Include all SPICE netlists (BJT amplifiers, CMOS logic, analog filters) in continuous integration. Solve MNA systems for each and ensure solution quality remains above threshold across releases.

## Pros and Cons of the Options

| Aspect | Sparse Direct LU | Iterative GMRES+ILU | Dense LU | Cholesky |
|--------|------------------|-------------------|----------|----------|
| **Robustness** | Excellent; pivoting | Good; depends on conditioning | Excellent; but memory | Not applicable (symmetric only) |
| **Memory usage** | O(n^1.5) planar; O(n²) worst-case | Controlled by ILU level | O(n²) always | O(n^1.5) (if applicable) |
| **Factorization time** | O(n^1.5) planar; proven in SPICE | O(n^1.5) arithmetic; but convergence not guaranteed | O(n³) | O(n^1.5); but limited scope |
| **Industry precedent** | SPICE standard for 50+ years | Emerging; SPICE adds as optional | Acceptable only for small circuits | Not used in SPICE; non-applicable |
| **Ill-conditioning handling** | Pivoting ensures stability | Requires tuning; can stall | Stable but impractical | Not applicable |
| **GPU acceleration** | Mature (cuSPARSE, rocSPARSE) | Less mature; custom kernels needed | Standard BLAS but O(n³) impractical | Standard BLAS but limited scope |
| **Distributed computing** | SuperLU_DIST; proven | Less mature | Not practical | Not applicable |
| **Tuning overhead** | Minimal (Markowitz is standard) | Requires ILU level, tolerance tuning | Minimal | N/A |

---

## Evidence

This decision is grounded in the following wiki evidence:
- [[vlsi-graph-methods]] — Graph algorithms for sparse matrix ordering (Markowitz, minimum-degree, AMD) in VLSI.
- [[computer-methods-circuit-analysis-design]] — Comprehensive treatment of sparse LU, Markowitz ordering, and numerical stability in circuit simulation.
- [[power-grid-analysis]] — Application of sparse direct LU to large-scale power-grid and on-chip power-delivery analysis.
- [[treewidth-and-graph-structure]] — Graph structure metrics (treewidth, planar graphs) that characterize sparse matrix fill-in and complexity.
