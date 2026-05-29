---
title: Systems for Big Graph Analytics — Introduction
type: source
id: summaries/systems-big-graph-analytics-01-1-introduction
kind: publication
tags:
- graph
- big-data
- graph-processing
- analytics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/01-1-introduction.txt
---

## Key Points

- Big graph datasets (online social networks, knowledge graphs) have driven a wave of systems research into distributed graph analytics platforms.
- The book is selective rather than comprehensive: it covers a few representative systems judged most important for beginners to learn the area quickly.
- Tutorials accompany the descriptions, illustrating how to use graph systems and how to build Big Data systems from scratch (API design, cross-machine objects, interaction with a distributed file system).
- The focus is on computation models for graph analytics rather than data storage; graph databases like TITAN are explicitly out of scope despite their importance.
- Queries on graphs are highly heterogeneous (random walks vs. graph matching), so a one-size-fits-all query model is not promising; computation and storage are largely independent.
- The book is organized in three parts: Part I "think like a vertex" (vertex-centric systems including Pregel-like), Part II "think like a graph" (block-centric and subgraph-centric), Part III "think like a matrix" (matrix-based systems).
- Future research directions are discussed in the final chapter (Chap. 8).
- Authors: D. Yan, Y. Bu, Y. Tian, A. Deshpande; published 2017 as a SpringerBriefs in Computer Science volume; an associated survey appears in Foundations and Trends in Databases.

## Relevant Concepts

- [[concepts/vertex-centric-programming]] — the dominant programming model for big graph systems, covered in Part I.
- [[concepts/block-centric-computation]] — Part II's alternative model that processes blocks of vertices instead of individual ones.
- [[concepts/subgraph-centric-computation]] — Part II's framework for computation-intensive graph mining problems.
- [[concepts/matrix-based-graph-analytics]] — Part III's approach: representing graphs as matrices and running linear algebra.
- [[entities/pregel]] — the pioneering Google system that introduced vertex-centric programming for big graphs.

## Source Metadata

- Source type: book chapter
- Book title: Systems for Big Graph Analytics
- Chapter: 1 — Introduction
- File: raw/SystemsForBigGraphAnalytics/_txt/01-1-introduction.txt
- Authors: Da Yan, Yingyi Bu, Yuanyuan Tian, Amol Deshpande (2017, Springer)
