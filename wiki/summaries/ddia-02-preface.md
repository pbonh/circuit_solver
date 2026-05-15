---
title: "Designing Data-Intensive Applications — Preface"
type: summary
tags: [foundational, well-established, distributed-systems, preface]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/02-preface.txt"]
confidence: high
---

## Key Points

- Defines a **data-intensive application** as one whose primary challenge is the quantity, complexity, or rate of change of data — as opposed to compute-intensive applications limited by CPU.
- Identifies the forces driving the explosion of data systems: web-scale companies (Google, Amazon, Facebook, LinkedIn, etc.), agile business needs, FOSS adoption, multi-core/networked hardware, IaaS democratizing distributed systems, and rising availability expectations.
- Argues the book's purpose is to dig past buzzwords ("NoSQL!", "Big Data!", "CAP theorem!", "MapReduce!") to enduring principles, so readers can choose appropriate tools and reason about trade-offs.
- States the intended audience: backend/server-side software engineers, architects, and technical managers familiar with relational databases and SQL.
- Outlines the three parts of the book: Part I (foundations: reliability, scalability, maintainability, data models, storage engines, encoding), Part II (distributed data: replication, partitioning, transactions, problems with distributed systems, consistency and consensus), Part III (derived data: batch processing, stream processing, building reliable scalable maintainable applications).
- Has a bias toward FOSS for understandability and to avoid vendor lock-in, but discusses proprietary systems where appropriate.
- Avoids the underdefined "Big Data" term in favor of clearer distinctions like single-node vs distributed, online/interactive vs offline/batch.

## Relevant Concepts

- [[concepts/reliability]] — One of the three core book themes.
- [[concepts/scalability]] — One of the three core book themes.
- [[concepts/maintainability]] — One of the three core book themes.

## Source Metadata

- Source type: book front matter (preface)
- Book title: Designing Data-Intensive Applications
- Author: Martin Kleppmann
- File path: `raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/02-preface.txt`
- Publisher: O'Reilly Media, March 2017
