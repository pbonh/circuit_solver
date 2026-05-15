---
title: "Systems for Big Graph Analytics — Part III: Think Like a Matrix (PEGASUS, GBASE, SystemML) and Conclusions"
type: summary
tags: [graph, distributed-systems, big-data, sparse-matrix, analytics, mapreduce, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt"]
confidence: high
---

## Key Points

- A graph can be represented by its adjacency matrix A (|V|×|V|) or its incidence matrix B (|V|×|E|); many graph operations (neighbor lookup, induced subgraph, k-hop reachability, ego networks) translate to matrix-vector or matrix-matrix products.
- Algebraic graph theory underpins this view; the chapter cites Biggs and Godsil-Royle as background references.
- PEGASUS (CMU, pre-Pregel) is a MapReduce-based framework that models each iteration as a generalized matrix-vector multiplication parameterized by three user-defined operators: `combine2(M[i][j], v[j])`, `combineAll(intermediate values)`, and `assign(new vertex value)`. PageRank and Hash-Min are expressible.
- PEGASUS partitions M into b×b submatrices and v into b-element blocks; co-clustering and repeated diagonal-block products reduce iteration count.
- GBASE (IBM System G) supports both global queries and "targeted" queries that touch only a subgraph; it exposes built-in graph operations rather than custom APIs. Each operation reduces to one or a few exact matrix-vector multiplications on either A/Aᵀ or Bᵀ.
- GBASE stores reordered blocks compressed with GZip (less than 2% of original size in reported experiments) and uses a grid placement to balance the cost of in-neighbor and out-neighbor queries (O(√n) files read).
- SystemML (IBM, now Apache) is the most user-friendly and active of the three; it offers a declarative R-like DML (and PyDML) language, compiling scripts into hybrid runtime plans across in-memory single-node (CP), MapReduce, and Spark backends.
- SystemML's optimizer: parser → high-level operator (HOP) DAG with rule-based and cost-based rewrites → low-level operator (LOP) DAG with backend-specific physical operators → runtime program with MR/Spark instruction piggybacking.
- SystemML's MatrixBlock library chooses block layout (sparse/dense), specialized multiplication kernels (sparse×sparse, sparse×dense, etc.), and can run linear algebra directly on compressed blocks; YARN integration provides resource elasticity, task parallelism for independent loop iterations, and emphasis on numerical accuracy.
- Comparison: PEGASUS uses square blocks with node-clustering preprocessing; GBASE uses general rectangular compressed blocks with clustering and grid placement; SystemML uses general rectangular blocks with no clustering preprocessing and per-block dynamic layout.
- Matrix-based vs. vertex-centric tradeoff: matrix-based systems are intuitive for analysts comfortable with R/MATLAB linear algebra and integrate well with ETL and ML pipelines, but cannot easily express per-vertex activity tracking, so each iteration recomputes the full matrix even when few elements change. Vertex-centric systems implement dedicated graph runtimes and keep state in memory across iterations.
- Future research directions (Chapter 8): more vertex-centric algorithms (currently fewer than system papers); computation-intensive frameworks like G-thinker for high-complexity workloads; native big-matrix/tensor systems beyond simple operations (e.g., decompositions).

## Relevant Concepts

- [[concepts/matrix-based-graph-analytics]] — the central paradigm of Chapter 7.
- [[concepts/adjacency-matrix]] — primary matrix representation of a graph.
- [[concepts/incidence-matrix]] — vertex-edge matrix used by GBASE for edge-result queries.
- [[concepts/generalized-matrix-vector-multiplication]] — PEGASUS's programming abstraction parameterized by combine2/combineAll/assign.
- [[concepts/declarative-machine-learning-language]] — SystemML's R/Python-like scripting interface (DML/PyDML).
- [[concepts/hybrid-runtime-execution]] — SystemML's strategy of mixing in-memory and distributed plans.
- [[concepts/matrix-blocking]] — splitting matrices into blocks for I/O and processing efficiency.
- [[concepts/algebraic-graph-theory]] — foundational mathematical view that motivates matrix-based systems.
- [[entities/pegasus]] — CMU's MapReduce-based peta-scale graph mining system.
- [[entities/gbase]] — IBM's MapReduce graph system with built-in operations and compressed grid storage.
- [[entities/systemml]] — Apache project for declarative machine learning on MR/Spark, including graph analytics.
- [[concepts/mapreduce]] — the underlying batch dataflow engine for PEGASUS and GBASE.
- [[entities/apache-spark]] — alternative SystemML backend; also hosts GraphX.

## Source Metadata

- Source type: book chapters
- Book title: Systems for Big Graph Analytics
- Chapters: 7 (Matrix-Based Graph Systems), 8 (Conclusions and Future Research)
- File: raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt
- Authors: Da Yan, Yingyi Bu, Yuanyuan Tian, Amol Deshpande (2017, Springer)
