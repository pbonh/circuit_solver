---
title: SystemML (Apache)
type: entity
id: entities/systemml
tags:
- machine-learning
- big-data
- sparse-matrix
- declarative
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
---

## Overview

SystemML (Ghoting et al., ICDE 2011; Boehm et al., PVLDB 2016) began as an IBM Research project in 2010 and became an active Apache incubator project (`systemml.apache.org`). It takes a declarative approach to machine learning and graph analytics: users write algorithms in an R-like (DML) or Python-like (PyDML) scripting language, and the system compiles, optimizes, and executes plans across single-node, MapReduce, and Spark backends.

## Characteristics

- DML and PyDML scripting languages with linear algebra, statistics, control flow, and UDFs.
- Compiler stages: parser (lexical/syntactic + ML-specific dimension checks) → high-level operator (HOP) DAG with rule-based + cost-based rewrites → low-level operator (LOP) DAG selecting CP/MR/Spark backend → runtime program with composite-MR piggybacking.
- Matrix blocking with dynamic per-block layout (sparse/dense), specialized multiplication kernels for every sparse/dense combination, and optional lightweight database compression on blocks.
- Hybrid runtime: mixes CP (single-node), MR, and Spark instructions in one execution plan.
- Programmatic APIs: standalone, Hadoop Batch, Spark Batch, MLContext (Scala/Java/Python for Spark Shell, Jupyter, Zeppelin), and JMLC (embedded scoring).
- YARN-based resource elasticity; task parallelism for independent loop iterations; emphasis on numerical accuracy of statistics.

## Common Strategies

- Write a graph-analytics algorithm as a few lines of DML (PageRank is nine lines in the book).
- Use MLContext API to integrate SystemML matrix operations with Spark RDDs and DataFrames in interactive notebooks.
- Use JMLC for low-latency scoring inside Java services.
- Trust the cost-based optimizer with good size estimates; provide schema/sparsity hints if necessary.

## Related Entities

- [[entities/pegasus]]
- [[entities/gbase]]
- [[entities/apache-spark]]
- [[concepts/mapreduce]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
