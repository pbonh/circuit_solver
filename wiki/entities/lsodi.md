---
title: LSODI
type: entity
id: entity-lsodi
tags:
- ode
- numerical-integration
- foundational
created: 2026-05-21
updated: 2026-05-21
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/06-chapter-vii-differential-algebraic-equations.txt
---

## Overview

LSODI (Hindmarsh 1980) is the implicit-form sister solver in the Livermore ODEPACK family, alongside [[entities/lsode]] and [[entities/dassl]]. It handles implicit ODEs and index-1 DAEs via BDF methods with Newton iteration.

## Characteristics

- Sister code to [[entities/lsode]] within the Livermore ODEPACK suite.
- Implicit-form multistep solver (BDF, orders 1–5) with variable step and order.
- Designed for implicit ODEs and index-1 differential-algebraic systems.

## Related Entities

- [[entities/lsode]] — explicit/implicit ODE sister solver in ODEPACK.
- [[entities/dassl]] — Petzold’s DAE code, often compared to LSODI in benchmarks.
