---
title: 'Computer Methods for Circuit Analysis and Design — Chapter 1: Fundamental
  Concepts'
type: source
id: source-computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts
kind: derived-summary
tags:
- foundational
- analog
- dc
- ac
- transient
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
---

## Key Points

- Reviews the assumed background: basic R, L, C elements, independent sources, ports and terminals, dependent sources (transducers), elementary two-ports, Thevenin/Norton transformations, network scaling, network functions, and time-domain response via Laplace inversion.
- Defines the linear time-invariant resistor (v = Ri), capacitor (i = C dv/dt) and inductor (v = L di/dt). Non-linear (current-controlled, voltage-controlled) and time-varying variants are mentioned.
- Introduces the Laplace transform of capacitor and inductor equations: for capacitor I = sCV - CV0, for inductor V = sLI - LI0. Initial conditions are equivalently represented as parallel/series independent sources (impulse or step form).
- Defines impedance Z = sL or 1/sC and admittance Y = 1/sL or sC. Provides constitutive equations in admittance and impedance descriptions.
- Distinguishes ports (paired terminals) from terminals. Networks of n terminals vs. n ports; four-terminal/two-port equivalence.
- Defines four canonical transducers in matrix form: VVT (voltage gain mu), VCT (transconductance g), CVT (transresistance r), CCT (current gain alpha).
- Elementary two-ports: ideal transformer, gyrator (realizable from two VCTs or CVTs), convertor (PIC, NIC), mutually coupled inductors with coefficient of coupling k = M / sqrt(L1 L2), 0 <= k <= 1.
- Nullator (V=0, I=0) and norator (no constitutive equation) — when combined, form a nullor, equivalent to an ideal operational amplifier or ideal transistor/tube.
- Thevenin/Norton equivalents: a voltage source E in series with Rs ↔ current source J = E/Rs in parallel with Rs. Generalized procedure: open- or short-circuit the load, deactivate sources (shorts/opens, leaving transducers intact), apply unit test source to get Rs.
- Network scaling: impedance scaling (divide all impedances by k: R→R/k, L→L/k, C→Ck) and frequency scaling (omega_s = omega_d/omega_0). Self-consistent unit sets (Standard, Audio, VHF, UHF) reduce overflow risk in computer arithmetic.
- Network functions for zero-initial-condition networks: Zin, Yin, Tv, Ti, Ztr, Ytr. These are rational functions in s with zeros and poles in the complex plane; the K, z_i, p_i triple defines the function up to constant.
- Amplitude |F(jw)|, phase phi(w), and group delay tau(w) = -d phi/d omega. Closed-form expressions of these in terms of (alpha_i, beta_i) coordinates of zeros and (gamma_i, delta_i) of poles.
- Time-domain response via partial-fraction expansion and inverse Laplace transform: simple-pole formula L^{-1}{K/(s-p)} = K e^{pt}; multiple-pole formula K t^{m-1}/(m-1)! e^{pt}; complex-conjugate pole pairs give 2 e^{ct}(A cos d t - B sin d t).
- Chapter 1 closes by noting that finding poles by polynomial root-finding becomes impractical for order >= 3 and motivates the numerical time-domain methods of Chapters 9, 10, 13.

## Relevant Concepts

- [[concepts/resistor]] — Linear time-invariant Ohm's-law element; nonlinear and time-varying generalizations.
- [[concepts/capacitor]] — q = Cv, i = C dv/dt; Laplace-domain initial-condition replacement.
- [[concepts/inductor]] — phi = Li, v = L di/dt; Laplace-domain initial-condition replacement.
- [[concepts/independent-voltage-source]] — Ideal source maintaining prescribed v independent of current.
- [[concepts/independent-current-source]] — Ideal source maintaining prescribed i independent of voltage.
- [[concepts/laplace-transform]] — Tool for converting differential constitutive equations to algebraic relations.
- [[concepts/impedance-admittance]] — Z = V/I, Y = I/V in the Laplace domain.
- [[concepts/port-terminal]] — Two views of multi-pole networks; ports are paired terminals.
- [[concepts/dependent-source]] — VVT, VCT, CVT, CCT generalized in two-port matrix form.
- [[concepts/ideal-transformer]] — Two-port with V1 = n V2, I1 = -I2/n.
- [[concepts/gyrator]] — Two-port realizable from two VCTs (or two CVTs).
- [[concepts/convertor]] — PIC and NIC two-ports generalizing the transformer.
- [[concepts/mutually-coupled-inductors]] — Two-port with mutual inductance M and coupling coefficient k.
- [[concepts/nullator-norator]] — Pathological elements; together form a nullor.
- [[concepts/operational-amplifier]] — Ideal OPAMP equivalent to a nullor.
- [[concepts/thevenin-norton-equivalents]] — Equivalent source transformations.
- [[concepts/network-scaling]] — Impedance and frequency scaling for practical computation.
- [[concepts/network-function]] — Rational function in s relating input and output of a linear circuit.
- [[concepts/poles-and-zeros]] — Roots of denominator and numerator of a network function.
- [[concepts/amplitude-phase-group-delay]] — Frequency-domain response characteristics.
- [[concepts/partial-fraction-expansion]] — Decomposition used for inverse Laplace transform.
- [[concepts/dirac-impulse]] — Distributional source used to represent initial conditions and excite networks.

## Source Metadata

- Source type: book chapter
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: 1 — Fundamental Concepts
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt`
- Authors: Jiri Vlach, Kishore Singhal
