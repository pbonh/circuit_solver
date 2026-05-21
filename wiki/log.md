---
title: "Circuit Simulation Knowledge Base Log"
type: log
updated: 2026-05-18
---

# Activity Log

Append-only record of ingest events, refinements, and major reorganizations.

## Initial ingest of `raw/` corpus (2026-05-15)

Parallel-agent ingest of every textbook and paper under `raw/`. Each per-book agent ran the Ingest workflow chapter-by-chapter on the extracted `_txt/` files, creating one summary per chapter and concept/entity pages on first mention. Concurrency rule: read-then-write-if-missing on shared concept/entity pages (never modify existing). Final reconciliation merged per-book logs, rebuilt the index, and aggregated cross-source `## Sources` lists.

### advanced-symbolic-analysis-for-vlsi-systems

- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 00-preface: wrote summary; created 8 concepts, 1 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 01-acknowledgments: wrote summary; created 0 concepts, 4 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 03-part-i-fundamentals: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 04-1-introduction: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 05-2-symbolic-analysis-techniques-in-a-nutshell: wrote summary; created 6 concepts, 2 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 06-3-binary-decision-diagram-for-symbolic-analysis: wrote summary; created 6 concepts, 1 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 07-part-ii-methods: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 08-4-determinant-decision-diagrams: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 09-5-ddd-implementation: wrote summary; created 1 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 10-6-generalized-two-graph-theory: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 11-7-graph-pair-decision-diagram: wrote summary; created 1 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 12-8-hierarchical-analysis-methods: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors: wrote summary; created 2 concepts, 1 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 14-part-iii-applications: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 15-10-symbolic-moment-computation: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 advanced-symbolic-analysis-for-vlsi-systems 17-12-statistical-parallel-monte-carlo-analysis-on-gpus: wrote summary; created 2 concepts, 2 entities


### computer-methods-circuit-analysis-design

- 2026-05-15 computer-methods-circuit-analysis-design 00-cover-and-front-matter: wrote summary; created 0 concepts, 4 entities
- 2026-05-15 computer-methods-circuit-analysis-design 01-preface: wrote summary; created 0 concepts, 1 entities
- 2026-05-15 computer-methods-circuit-analysis-design 02-motivation: wrote summary; created 13 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 04-chapter-1-fundamental-concepts: wrote summary; created 22 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 05-chapter-2-network-equations-and-their-solution: wrote summary; created 16 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 06-chapter-3-graph-theoretic-formulation-of-network-equations: wrote summary; created 9 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 07-chapter-4-general-formulation-methods: wrote summary; created 8 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 08-chapter-5-sensitivities: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 09-chapter-6-computer-generation-of-sensitivities: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 10-chapter-7-network-functions-in-the-frequency-domain: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 11-chapter-8-large-change-sensitivity-and-related-topics: wrote summary; created 5 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 12-chapter-9-introduction-to-numerical-integration-of-differential-equations: wrote summary; created 8 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 13-chapter-10-numerical-laplace-transform-inversion: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 14-chapter-11-modeling: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 15-chapter-12-dc-solution-of-networks: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 17-chapter-14-digital-and-switched-capacitor-networks: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 18-chapter-15-introduction-to-optimization-theory: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 19-chapter-16-time-domain-sensitivities-and-steady-state: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 20-chapter-17-design-by-minimization: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 21-appendix-a-laplace-transforms: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 22-appendix-b-partial-fraction-decomposition-of-rational-functions: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 23-appendix-c-special-complex-integration-of-a-rational-function: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 24-appendix-d-program-for-network-analysis: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 25-appendix-e-sparse-matrix-solver: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 computer-methods-circuit-analysis-design 26-appendix-f-selected-mathematical-topics: wrote summary; created 4 concepts, 0 entities


### data-analysis-visualizations-python

- 2026-05-15 data-analysis-visualizations-python 01-about-the-author: wrote summary; created 2 concepts (machine-learning, data-mining), 0 entities
- 2026-05-15 data-analysis-visualizations-python 02-about-the-technical-reviewers: wrote summary; created 1 concept (simulation), 0 entities
- 2026-05-15 data-analysis-visualizations-python 03-introduction: wrote summary; created 1 concept (regular-expressions), 1 entity (spyder-ide)
- 2026-05-15 data-analysis-visualizations-python 04-chapter-1-introduction-to-data-science-with-python: wrote summary; created 4 concepts (lambda-function, linear-regression, correlation, missing-data-handling), 0 entities
- 2026-05-15 data-analysis-visualizations-python 05-chapter-2-the-importance-of-data-visualization-in-business-intelligence: wrote summary; created 2 concepts (business-intelligence, exploratory-data-analysis), 3 entities (plotly, pip, r-language)
- 2026-05-15 data-analysis-visualizations-python 06-chapter-3-data-collection-structures: wrote summary; created 1 concept (dataframe), 0 entities
- 2026-05-15 data-analysis-visualizations-python 07-chapter-4-file-i-o-processing-and-regular-expressions: wrote summary; created 1 concept (data-extraction), 0 entities
- 2026-05-15 data-analysis-visualizations-python 08-chapter-5-data-gathering-and-cleaning: wrote summary; created 1 concept (data-cleaning), 1 entity (beautiful-soup)
- 2026-05-15 data-analysis-visualizations-python 09-chapter-6-data-exploring-and-analysis: wrote summary; created 1 concept (data-aggregation), 0 entities
- 2026-05-15 data-analysis-visualizations-python 10-chapter-7-data-visualization: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 data-analysis-visualizations-python 11-chapter-8-case-studies: wrote summary; created 0 concepts, 0 entities


### ddia

- 2026-05-15 ddia 00-copyright: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 ddia 02-preface: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 ddia 03-part-i-foundations-of-data-systems: wrote summary; created 21 concepts, 8 entities (covers chapters 1-4)
- 2026-05-15 ddia 04-part-ii-distributed-data: wrote summary; created 11 concepts (others pre-existed via parallel agents), 7 entities (covers chapters 5-9)
- 2026-05-15 ddia 05-part-iii-derived-data: wrote summary; created 13 concepts (others pre-existed via parallel agents), 5 entities (covers chapters 10-12)
- 2026-05-15 ddia 08-about-the-author: wrote summary; created 0 concepts, 1 entity
- 2026-05-15 ddia: dangling-link sweep complete; 94 unique [[concepts/...]]/[[entities/...]] links across 5 summaries, all targets exist


### foundations-scalable-systems

- 2026-05-15 foundations-scalable-systems 00-cover: wrote summary
- 2026-05-15 foundations-scalable-systems 01-copyright: wrote summary
- 2026-05-15 foundations-scalable-systems 03-preface: wrote summary
- 2026-05-15 foundations-scalable-systems 04-part-i-the-basics: wrote summary covering chapters 1-4
- 2026-05-15 foundations-scalable-systems 05-part-ii-scalable-systems: wrote summary covering chapters 5-9
- 2026-05-15 foundations-scalable-systems 06-part-iii-scalable-distributed-databases: wrote summary covering chapters 10-13
- 2026-05-15 foundations-scalable-systems 07-part-iv-event-and-stream-processing: wrote summary covering chapters 14-16
- 2026-05-15 foundations-scalable-systems 09-about-the-author: wrote summary
- 2026-05-15 foundations-scalable-systems final-sweep: created 123 concept pages, 17 entity pages; zero dangling links remaining in summary outputs


### graphs-in-vlsi

- 2026-05-15 graphs-in-vlsi 00-preface: wrote summary; created 7 concepts, 2 entities
- 2026-05-15 graphs-in-vlsi 01-acknowledgments: wrote summary; created 1 concept, 0 entities
- 2026-05-15 graphs-in-vlsi 03-about-the-authors: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 04-1-introduction: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 05-2-graph-fundamentals: wrote summary; created 12 concepts (bipartite-graph race-lost), 0 entities
- 2026-05-15 graphs-in-vlsi 06-3-graphs-in-vlsi-circuits-and-systems: wrote summary; created 16 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 07-4-synchronization-in-vlsi: wrote summary; created 11 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 08-5-circuit-analysis: wrote summary; created 13 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 09-6-effective-resistance-of-truncated-infinite-mesh-structures: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 10-7-effective-resistance-of-finite-grids: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 11-8-placement-of-on-chip-distributed-voltage-regulators: wrote summary; created 6 concepts, 1 entity
- 2026-05-15 graphs-in-vlsi 12-9-exploratory-methodology-for-power-delivery: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 14-11-qucts-single-flux-quantum-clock-tree-synthesis: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 15-12-conclusions: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 16-a-green-s-function-for-a-truncated-grid: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 17-b-uniqueness-based-on-boundary-conditions: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 graphs-in-vlsi 18-c-multilayer-routing-algorithm: wrote summary; created 0 concepts, 0 entities


### guide-to-graph-algorithms

- 2026-05-15 guide-to-graph-algorithms 01-preface: wrote summary; references 3 concepts, 1 entity
- 2026-05-15 guide-to-graph-algorithms 02-about-the-authors: wrote summary; references 1 concept, 1 entity
- 2026-05-15 guide-to-graph-algorithms 03-acknowledgments: wrote summary; references 0 concepts, 1 entity
- 2026-05-15 guide-to-graph-algorithms 04-graphs: wrote summary; references 20 concepts, 0 entities
- 2026-05-15 guide-to-graph-algorithms 05-algorithms: wrote summary; references 30 concepts, 0 entities
- 2026-05-15 guide-to-graph-algorithms 06-problem-formulations: wrote summary; references 8 concepts, 0 entities
- 2026-05-15 guide-to-graph-algorithms 07-recent-trends: wrote summary; references 50 concepts, 1 entity


### hairer-ode-ii

- 2026-05-15 hairer-ode-ii 00-front-matter: wrote summary; cited 5 concept/entity links
- 2026-05-15 hairer-ode-ii 01-preface: wrote summary; cited 17 concept/entity links
- 2026-05-15 hairer-ode-ii 03-chapter-iv-stiff-problems-one-step-methods: wrote summary; cited ~55 concept/entity links
- 2026-05-15 hairer-ode-ii 04-chapter-v-multistep-methods-for-stiff-problems: wrote summary; cited ~45 concept/entity links
- 2026-05-15 hairer-ode-ii 05-chapter-vi-singular-perturbation-problems: wrote summary; cited ~38 concept/entity links
- 2026-05-15 hairer-ode-ii 06-chapter-vii-differential-algebraic-equations: wrote summary; cited ~55 concept/entity links


### modeling-simulation-systems

- 2026-05-15 modeling-simulation-systems 00-preface: wrote summary; created 5 concepts, 3 entities
- 2026-05-15 modeling-simulation-systems 02-basic-concepts: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 03-1-modeling-and-simulation-of-systems-of-systems: wrote summary; created 4 concepts, 1 entity
- 2026-05-15 modeling-simulation-systems 04-2-devs-integrated-development-environments: wrote summary; created 7 concepts, 1 entity
- 2026-05-15 modeling-simulation-systems 05-3-system-entity-structure-basics: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 06-4-devs-natural-language-models-and-elaborations: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 07-5-specialization-and-pruning: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 08-6-aspects-and-multi-aspects: wrote summary; created 5 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 09-7-managing-inheritance-in-pruning: wrote summary; created 1 concept, 0 entities
- 2026-05-15 modeling-simulation-systems 10-8-automated-and-rule-based-pruning-and-experimental-execution: wrote summary; created 5 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 11-advanced-concepts: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 12-9-devs-simulation-protocol: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 13-10-dynamic-structure-agent-modeling-and-publish-subscribe: wrote summary; created 5 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 14-11-interest-based-information-exchange-mappings-and-models: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 15-12-languages-for-constructing-devs-models: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 16-applications: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 17-13-flexible-modeling-support-environments: wrote summary; created 6 concepts, 1 entity
- 2026-05-15 modeling-simulation-systems 18-14-service-based-software-systems: wrote summary; created 4 concepts, 1 entity
- 2026-05-15 modeling-simulation-systems 19-15-cloud-system-simulation-modeling: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 20-16-model-development-and-execution-process-with-repositories-validation-and-verification: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 21-17-modeling-and-simulation-of-living-systems-as-systems-of-systems: wrote summary; created 6 concepts, 1 entity
- 2026-05-15 modeling-simulation-systems 22-18-activity-based-implementations-of-systems-of-systems: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 modeling-simulation-systems 23-19-devs-support-for-markov-modeling-and-simulation: wrote summary; created 5 concepts, 0 entities


### prototyping-python-dashboards

- 2026-05-15 prototyping-python-dashboards 01-about-the-author: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 prototyping-python-dashboards 02-about-the-technical-reviewer: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 prototyping-python-dashboards 03-acknowledgments: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 prototyping-python-dashboards 04-introduction: wrote summary; created 4 concepts (dashboard, data-visualization, reactive-programming, prototyping), 4 entities (atads-dataset, nginx, gunicorn, spyder-ide referenced existing)
- 2026-05-15 prototyping-python-dashboards 05-chapter-1-working-with-python: wrote summary; created 4 concepts (list, dictionary, series, object-oriented-design), 0 entities
- 2026-05-15 prototyping-python-dashboards 06-chapter-2-reactive-programming-with-plotly-and-dash: wrote summary; created 2 concepts (callback, python-decorator), 1 entity (dash)
- 2026-05-15 prototyping-python-dashboards 07-chapter-3-working-with-online-data: wrote summary; created 3 concepts (screen-scraping, csv, regular-expression), 2 entities (selenium, chromedriver)
- 2026-05-15 prototyping-python-dashboards 08-chapter-4-planning-the-dashboard-prototype: wrote summary; created 3 concepts (regression, polynomial, time-series), 1 entity (flask)
- 2026-05-15 prototyping-python-dashboards 09-chapter-5-our-first-dashboard: wrote summary; created 3 concepts (smoothing, css, css-grid), 0 entities
- 2026-05-15 prototyping-python-dashboards 10-chapter-6-dashboard-enhancements: wrote summary; created 3 concepts (fft, spectrum, standard-deviation), 0 entities
- 2026-05-15 prototyping-python-dashboards 11-chapter-7-hosting-an-application-on-a-unix-server: wrote summary; created 2 concepts (wsgi, virtual-environment), 3 entities (uwsgi, ufw, ubuntu)
- 2026-05-15 prototyping-python-dashboards 12-chapter-8-deploying-your-project-as-a-unix-service: wrote summary; created 2 concepts (reverse-proxy, systemd-service), 3 entities (systemd, letsencrypt, fail2ban)
- 2026-05-15 prototyping-python-dashboards 13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables: wrote summary; created 0 concepts, 1 entity (bts-t100-dataset)
- 2026-05-15 prototyping-python-dashboards 14-chapter-10-creating-a-web-portal: wrote summary; created 2 concepts (html, web-portal), 3 entities (wordpress, mysql, chrome-developer-tools, avopsinsight)
- 2026-05-15 prototyping-python-dashboards 15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis: wrote summary; created 1 concept (modeling), 0 entities
- 2026-05-15 prototyping-python-dashboards 16-chapter-12-afterword: wrote summary; created 0 concepts, 1 entity (kubernetes)
- 2026-05-15 prototyping-python-dashboards 17-appendix-a-utilities-for-managing-atads-data: wrote summary; created 1 concept (cron), 0 entities


### python-data-analysts-toolkit

- 2026-05-15 python-data-analysts-toolkit 01-about-the-author: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 02-about-the-technical-reviewer: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 03-acknowledgments: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 04-introduction: wrote summary; created 5 concepts, 6 entities
- 2026-05-15 python-data-analysts-toolkit 05-chapter-1-getting-familiar-with-python: wrote summary; created 6 concepts, 3 entities
- 2026-05-15 python-data-analysts-toolkit 06-chapter-2-exploring-containers-classes-and-objects: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 07-chapter-3-regular-expressions-and-math-with-python: wrote summary; created 3 concepts, 2 entities
- 2026-05-15 python-data-analysts-toolkit 08-chapter-4-descriptive-data-analysis-basics: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 09-chapter-5-working-with-numpy-arrays: wrote summary; created 8 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 10-chapter-6-prepping-your-data-with-pandas: wrote summary; created 7 concepts, 2 entities
- 2026-05-15 python-data-analysts-toolkit 11-chapter-7-data-visualization-with-python-libraries: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 python-data-analysts-toolkit 12-chapter-8-data-analysis-case-studies: wrote summary; created 1 concept, 1 entity
- 2026-05-15 python-data-analysts-toolkit 13-chapter-9-statistics-and-probability-with-python: wrote summary; created 13 concepts, 1 entity


### rust-book

- 2026-05-15 rust-book 00-foreword: wrote summary; created 4 concepts, 2 entities
- 2026-05-15 rust-book 01-introduction: wrote summary; created 5 concepts, 1 entity
- 2026-05-15 rust-book 02-chapter-1-getting-started: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 rust-book 03-chapter-2-programming-a-guessing-game: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 rust-book 04-chapter-3-common-programming-concepts: wrote summary; created 6 concepts, 0 entities (+1 rust-control-flow rename)
- 2026-05-15 rust-book 05-chapter-4-understanding-ownership: wrote summary; created 9 concepts, 0 entities
- 2026-05-15 rust-book 06-chapter-5-using-structs-to-structure-related-data: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 rust-book 07-chapter-6-enums-and-pattern-matching: wrote summary; created 3 concepts, 0 entities
- 2026-05-15 rust-book 08-chapter-7-managing-growing-projects-with-packages-crates-and-modules: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 rust-book 09-chapter-8-common-collections: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 rust-book 10-chapter-9-error-handling: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 rust-book 11-chapter-10-generic-types-traits-and-lifetimes: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 rust-book 12-chapter-11-writing-automated-tests: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 rust-book 13-chapter-12-an-i-o-project-building-a-command-line-program: wrote summary; created 4 concepts, 0 entities
- 2026-05-15 rust-book 14-chapter-13-functional-language-features-iterators-and-closures: wrote summary; created 2 concepts, 0 entities
- 2026-05-15 rust-book 15-chapter-14-more-about-cargo-and-crates-io: wrote summary; created 5 concepts, 0 entities
- 2026-05-15 rust-book 16-chapter-15-smart-pointers: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 rust-book 17-chapter-16-fearless-concurrency: wrote summary; created 6 concepts, 0 entities
- 2026-05-15 rust-book 18-chapter-17-object-oriented-programming-features-of-rust: wrote summary; created 7 concepts, 0 entities
- 2026-05-15 rust-book 19-chapter-18-patterns-and-matching: wrote summary; created 1 concept, 0 entities
- 2026-05-15 rust-book 20-chapter-19-advanced-features: wrote summary; created 10 concepts, 0 entities
- 2026-05-15 rust-book 21-chapter-20-final-project-building-a-multithreaded-web-server: wrote summary; created 1 concept, 0 entities
- 2026-05-15 rust-book 22-appendix-a-keywords: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 rust-book 23-appendix-b-operators-and-symbols: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 rust-book 24-appendix-c-derivable-traits: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 rust-book 25-appendix-d-useful-development-tools: wrote summary; created 1 concept (clippy), 0 entities
- 2026-05-15 rust-book 26-appendix-e-editions: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 rust-book 27-appendix-f-translations-of-the-book: wrote summary; created 0 concepts, 0 entities
- 2026-05-15 rust-book 28-appendix-g-how-rust-is-made-and-nightly-rust: wrote summary; created 1 concept (rust-release-process), 1 entity (rust-project)


### simulation-whitepaper-v1

- 2026-05-15 simulation-whitepaper-v1 kundert-bctm98-simulation-tutorial: wrote summary `summaries/kundert-bctm98-simulation-tutorial.md`; created 27 concepts (newton-raphson-method, dc-analysis, homotopy-method, gmin-stepping, source-stepping, pseudo-transient-analysis, nodeset, ac-analysis, noise-analysis, small-signal-analysis, transient-analysis, integration-method, forward-euler, backward-euler, trapezoidal-rule, gear-bdf, stiff-circuit, local-truncation-error, numerical-damping, charge-conservation, fourier-analysis, timing-simulation, mixed-level-simulation, top-down-design, signal-flow-model, conservative-model, ahdl-mshdl); created 5 entities (spectre, verilog-ams, vhdl-ams, ken-kundert, cadence); concepts/modified-nodal-analysis and entities/spice already existed (created by parallel agents) and were not modified per concurrency rule.


### systems-big-graph-analytics

- 2026-05-15 systems-big-graph-analytics 01-1-introduction: wrote summary; created 4 concepts, 1 entity
- 2026-05-15 systems-big-graph-analytics 02-part-i-think-like-a-vertex: wrote summary; created 11 concepts, 15 entities
- 2026-05-15 systems-big-graph-analytics 03-part-ii-think-like-a-graph: wrote summary; created 7 concepts, 6 entities
- 2026-05-15 systems-big-graph-analytics 04-part-iii-think-like-a-matrix: wrote summary; created 7 concepts, 5 entities


### sze-physics-semiconductor-devices

- 2026-05-15 sze-physics-semiconductor-devices 00-preface: wrote summary; created 1 entity (sze-physics-semiconductor-devices-book), referenced concepts (semiconductor-device, mosfet, igbt)
- 2026-05-15 sze-physics-semiconductor-devices 02-introduction: wrote summary; created concepts (semiconductor-device, mosfet, p-n-junction, bipolar-junction-transistor, schottky-barrier, mis-capacitor, heterojunction, thyristor, igbt) and entity (semiconductor-device-family)
- 2026-05-15 sze-physics-semiconductor-devices 03-part-i-semiconductor-physics: wrote short part-divider summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices 04-chapter-1-physics-and-properties-of-semiconductors-a-review: wrote summary; created 17 concepts (energy-band-structure, bandgap, effective-mass, fermi-dirac-distribution, carrier-concentration, donor-acceptor-doping, carrier-mobility, drift-diffusion-equation, einstein-relation, hall-effect, impact-ionization, shockley-read-hall-recombination, carrier-lifetime, thermionic-emission, quantum-mechanical-tunneling, space-charge-limited-current, poisson-equation, continuity-equation, quantum-well), 2 entities (silicon, gallium-arsenide)
- 2026-05-15 sze-physics-semiconductor-devices 05-part-ii-device-building-blocks: wrote short part-divider summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices 06-chapter-2-p-n-junctions: wrote summary; created 8 concepts (depletion-region, built-in-potential, junction-capacitance, shockley-diode-equation, avalanche-breakdown, zener-breakdown, reverse-recovery, varactor-diode, p-i-n-diode)
- 2026-05-15 sze-physics-semiconductor-devices 07-chapter-3-metal-semiconductor-contacts: wrote summary; created 3 concepts (ohmic-contact, image-force-lowering, fermi-level-pinning), 1 entity (silicide)
- 2026-05-15 sze-physics-semiconductor-devices 08-chapter-4-metal-insulator-semiconductor-capacitors: wrote summary; created 7 concepts (threshold-voltage, inversion-layer, flatband-voltage, interface-traps, oxide-charge, fowler-nordheim-tunneling, dielectric-breakdown), 1 entity (silicon-dioxide)
- 2026-05-15 sze-physics-semiconductor-devices 09-chapter-6-mosfets: wrote summary; created 8 concepts (subthreshold-conduction, short-channel-effects, hot-carrier-effects, dennard-scaling, finfet, silicon-on-insulator, floating-gate-memory, single-electron-transistor, cmos-logic, poly-silicon-gate)
- 2026-05-15 sze-physics-semiconductor-devices 10-chapter-7-jfets-mesfets-and-modfets: wrote summary; created 4 concepts (jfet, mesfet, modfet, two-dimensional-electron-gas)
- 2026-05-15 sze-physics-semiconductor-devices 11-part-iv-negative-resistance-and-power-devices: wrote short part-divider summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices 12-chapter-8-tunnel-devices: wrote summary; created 3 concepts (tunnel-diode, resonant-tunneling-diode, negative-differential-resistance)
- 2026-05-15 sze-physics-semiconductor-devices 13-chapter-9-impatt-diodes: wrote summary; created 1 concept (impatt-diode)
- 2026-05-15 sze-physics-semiconductor-devices 14-chapter-10-transferred-electron-and-real-space-transfer-devices: wrote summary; created 1 concept (transferred-electron-device)
- 2026-05-15 sze-physics-semiconductor-devices 15-chapter-11-thyristors-and-power-devices: wrote summary; created 1 concept (junction-termination)
- 2026-05-15 sze-physics-semiconductor-devices 16-part-v-photonic-devices-and-sensors: wrote short part-divider summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices 17-chapter-12-leds-and-lasers: wrote summary; created 5 concepts (light-emitting-diode, semiconductor-laser, radiative-recombination, population-inversion, quantum-cascade-laser, vcsel), 1 entity (indium-phosphide)
- 2026-05-15 sze-physics-semiconductor-devices 18-chapter-13-photodetectors-and-solar-cells: wrote summary; created 6 concepts (photodiode, avalanche-photodiode, solar-cell, charge-coupled-device, photoconductor, quantum-well-infrared-photodetector)
- 2026-05-15 sze-physics-semiconductor-devices 19-chapter-14-sensors: wrote summary; created 4 concepts (semiconductor-sensor, piezoresistivity, ion-sensitive-fet, thermistor)
- 2026-05-15 sze-physics-semiconductor-devices 20-appendix-a-list-of-symbols: wrote short reference summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices 21-appendix-e-properties-of-important-semiconductors: wrote short reference summary; 0 new concepts/entities
- 2026-05-15 sze-physics-semiconductor-devices: dangling-link sweep passed (0 missing concept/entity targets in 21 summaries)


## Lint sweep (2026-05-15)

Ran the full Lint workflow (per `AGENTS.md`). Inventory: 1152 concepts, 183 entities, 207 summaries, 0 syntheses; no specs, ADRs, contexts, context-maps, vision, grills, or architecture pages yet (R&D pipeline is at the ingest stage only).

### Auto-fixed

- **Bad wiki-link path in scaffold examples.** `flashcards/example.md` and `presentations/example.md` cited `[[wiki/index.md]]` (the `wiki/` prefix is wrong — wiki links are relative to wiki root). Rewrote both to `[[index]]` and updated their `sources:` frontmatter from `"wiki/index.md"` to `"index.md"`.
- **Orphan example pages.** `flashcards/example.md` and `presentations/example.md` had zero inbound links because the index's Flashcards / Presentations tables contained only HTML-comment placeholder rows. Added real rows so the examples are now discoverable from the index. Orphan count: 2 → 0.

### Categorized findings

- **Orphans:** 0 remaining (was 2 before auto-fix).
- **Sections:** 0 concept / entity / summary pages missing required sections (1152 + 183 + 207 spot-checked). Frontmatter `type:` and `confidence:` present on every page.
- **Pipeline compliance:** `wiki/.pipeline.yaml` is present but the pipeline beyond Ingest has not been started — no vision, no contexts, no context-maps, no grills, no architecture, no ADRs, no specs. None of the spec/ADR/architecture lint checks have anything to evaluate. Two infrastructure artifacts are not green: `project_init` still has the CUSTOMIZE marker in `project/README.md`, and `board_bound` still has the CUSTOMIZE marker in `kanban/board.yaml` with an empty `board:` slug. `scripts/check-prereqs.sh` was not found in the repo, so the manifest's gate semantics could not be exercised — slash commands relying on it would currently no-op (back-compat path).
- **Mermaid:** skipped — `mmdc` not on `PATH` and there are no architecture pages anyway.
- **ADR backlinks:** skipped — no ADRs and no architecture pages.
- **Triage:** skipped — no triage cross-link annotations exist yet.
- **Hermes-skipped:** Kanban-board drift, kanban↔spec backlinks, handoff completeness, and deprecated-ADR active-task checks all skipped — `hermes` not on `PATH`. `kanban/board.yaml` is also still in its unbound CUSTOMIZE state, so even with Hermes the board-drift check would no-op.

### Issues requiring human judgment (not auto-fixed)

1. **Stub pages.** 6 reconciliation stubs (`To be expanded` content, `sources: ["raw/_reconciled"]`, `confidence: low`) exist and are cited from real pages — they should be filled or removed:
   - `concepts/concept-name.md` — pure schema placeholder cited only from `index.md` and `journal/template.md`. Recommend **deleting** (and dropping the index row + template's example link) once the template can use a different exemplar.
   - `concepts/display-trait.md` — cited from `concepts/debug-trait.md`. Has a clear home in the Rust Book corpus (`summaries/rust-book-20-chapter-19-advanced-features.md` references `Display`).
   - `concepts/raii.md` — cited from `concepts/drop-trait.md`. Rust Book corpus covers this (ownership / Drop chapter).
   - `concepts/raw-pointers.md` — cited from `concepts/unsafe-rust.md` and `concepts/ffi.md`. Rust Book chapter 19 covers this.
   - `entities/ngspice.md` — cited from `entities/spice.md` and `entities/hspice.md`. Plenty of context in the SPICE family pages.
   - `entities/opencl.md` — cited from `entities/nvidia-cuda.md`. CUDA/GPU material exists in the summaries.
   - Next step: run `/wiki-refine` (or a small targeted `/wiki-ingest` on the citing pages) to populate these from existing sources rather than fabricating.
2. **Duplicate concept pages — different slugs, same concept.** Parallel-agent ingest produced both:
   - `concepts/backward-euler.md` *and* `concepts/backward-euler-method.md`
   - `concepts/forward-euler.md` *and* `concepts/forward-euler-method.md`
   - `concepts/topological-sort.md` *and* `concepts/topological-sorting.md`
   Each pair describes the same concept from two different source books. Recommendation: pick the canonical slug (probably the `-method` variant for Euler, since it matches the wider numerical-integration concept family; `topological-sort` is shorter), merge `## Sources` + `## How It Works` + cross-links into the survivor, redirect the citing pages, and delete the loser.
3. **Concept/Entity slug collisions.** Three slugs exist as *both* a concept page and an entity page:
   - `raft.md` — concept = "Raft" (algorithm), entity = "Raft Consensus Algorithm". Same idea, different page-type framing.
   - `paxos.md` — same pattern.
   - `mapreduce.md` — concept = "MapReduce" (the algorithm), entity = "MapReduce" (the system). This one might legitimately want both, but the slug collision will confuse `[[raft]]`-style links (Obsidian resolves to whichever directory it picks first).
   Recommendation: keep one page per slug. For Raft / Paxos, fold the entity into the concept page's `## How It Works` and delete the entity. For MapReduce, rename the entity to `entities/mapreduce-framework` (or similar) to disambiguate.
4. **`concepts/concept-name.md` is referenced from `journal/template.md` as a literal example.** The template should either point at a real concept (e.g. `[[concepts/nodal-analysis]]`) or use an explicit `<placeholder>` syntax to avoid teaching new users that a `concept-name` page exists.
5. **No syntheses pages.** `wiki/syntheses/` is empty. With 1152 concepts spanning circuit simulation, graph algorithms, distributed systems, semiconductor physics, ODE numerical-integration, and Rust language features, there is enormous synthesis opportunity (the AGENTS vision explicitly calls out unifying analog / digital / mixed-signal simulation under a single graph + solver framing). Recommend running `/wiki-query` on cross-cutting questions like *"how does graph partitioning in big-graph systems compare to circuit branch-tearing"* and dropping the resulting syntheses.
6. **Low-confidence cluster.** 235 pages (15 %) are at `confidence: low`; about half are ODE-numerical-integration concepts ingested from Hairer/Wanner *Solving ODE II* but never fleshed out (`a-alpha-stability`, `algebraic-stability`, `an-stability`, `ao-stability`, `b-convergence`, …). The source material exists under `raw/solving_ordinary_differential_equations_ii/` — running `/wiki-ingest` more deeply on those chapters would lift confidence across that whole cluster.

### Suggested next sources / topics

- **Run `/wiki-project-init`** to remove the `CUSTOMIZE` marker from `project/README.md` (declare language = Rust, build/test commands, entry point) so the `project_init` pipeline artifact flips green.
- **Run `/wiki-kanban-board <slug>`** once the board has been created in Hermes — this will populate `kanban/board.yaml`'s `board:` / `profiles:` fields and flip `board_bound` green. Without this, no `/wiki-kanban-emit` or `/wiki-kanban-ingest` invocation can succeed.
- **Run `/wiki-strategy <topic>`** for the differentiating R&D theme (the AGENTS preamble says "unified view of analog, digital, and mixed-signal circuit simulation" — a good first vision page). That unblocks the whole strategy → grill → architecture → ADR → spec chain.
- **Install `mmdc` (`npm i -g @mermaid-js/mermaid-cli`)** so future architecture-page Mermaid blocks are validated by lint.
- **Install `hermes`** and bind the kanban board so the round-trip checks (board drift, handoff completeness, kanban↔spec backlinks) can run.
- **Ingest `raw/solving_ordinary_differential_equations_ii/`** chapter-by-chapter to lift the ODE-stability low-confidence cluster.

### Summary line

Auto-fixed: 2 dangling links + 2 orphan example pages (now in index). Outstanding: 6 stubs, 3 same-concept-different-slug duplicates, 3 concept/entity slug collisions, an empty syntheses corpus, 235 low-confidence pages, and two pipeline-infrastructure artifacts (`project_init`, `board_bound`) still carrying their CUSTOMIZE markers. Hermes-dependent and Mermaid-dependent checks skipped cleanly (tools not on `PATH`).


## Duplicate-page merges (2026-05-15)

Resolved the six duplicates flagged by the prior lint sweep. For each pair, content from both pages was merged into the surviving canonical page (preserving every distinct claim, source citation, and cross-link), inbound links were rewritten across the wiki, the loser file was deleted, and the index was reconciled.

### Same-concept / different-slug merges (concept ↔ concept)

| Survivor | Removed | Notes |
|----------|---------|-------|
| `concepts/backward-euler` | `concepts/backward-euler-method` | Survivor had 12 inbound, loser had 7. Merged numerical-analysis rigor from the loser (truncation-error coefficient c₂ = +1/2, linear-system form `(I − hA) x_{n+1} = x_n + h w_{n+1}`, A-stability geometry) into the analog-simulation framing of the survivor. Combined sources from both books (Kundert + Hairer/Wanner + Vlach/Singhal). Tags now `analog, transient, numerical-integration, foundational, well-established`. |
| `concepts/forward-euler` | `concepts/forward-euler-method` | Survivor had 10 inbound, loser had 5. Merged stability-region geometry (unit disk centred at −1) and predictor-corrector usage from the loser into the timing-simulation framing of the survivor. Both source books cited. |
| `concepts/topological-sort` | `concepts/topological-sorting` | Tie at 5 inbound each; picked the shorter, more common slug. Merged the DFS-based variant, tie-breaking discussion (queue vs stack vs priority queue), and the betweenness-NP-completeness caveat from `topological-sorting` into `topological-sort`. |

### Concept ↔ entity slug-collision merges (concept survives)

| Survivor | Removed | Notes |
|----------|---------|-------|
| `concepts/raft` | `entities/raft` | Algorithms are concepts in this wiki. Merged the entity page's implementation details (etcd, Consul, CockroachDB, TiKV, RethinkDB; pre-vote optimization; joint-consensus membership change; snapshotting) and DDIA's safety-property framing (log-matching, term numbers, randomized timeouts) into the concept page. Cross-links to `[[entities/etcd]]`, `[[entities/zookeeper]]`, `[[entities/cockroachdb]]`, `[[entities/yugabytedb]]`, `[[entities/neo4j]]`, `[[entities/spanner]]` added under Related Concepts. Confidence high. |
| `concepts/paxos` | `entities/paxos` | Same approach. Merged DDIA's three-role framing (proposers, acceptors, learners), the two-phase Prepare/Accept structure, ballot-number ordering, Multi-Paxos amortization, and the EPaxos/Mencius leaderless variants. FLP-impossibility / liveness caveat from the entity page preserved. Cross-links to `[[entities/zookeeper]]`, `[[entities/spanner]]`, `[[entities/etcd]]`. Confidence bumped medium → high. |
| `concepts/mapreduce` | `entities/mapreduce` | Concept survives because the same slug was both. Preserved the entity page's big-graph systems context (PEGASUS, GBASE, SystemML, nscale, HDFS materialization cost) alongside the concept page's DDIA framing (purity requirement, fault tolerance via re-execution, surface compilation from SQL/Hive/Pig). Pregel's anti-pattern motivation noted. Confidence bumped medium → high. |

### Mechanics

- For each pair: rewrote every `[[<loser>]]` and `[[<loser>|<display>]]` occurrence to point at the survivor via `sed -i` across every `*.md` (except the loser file itself). Verified zero remaining citers of any loser slug before deleting.
- Deleted 6 files: `concepts/backward-euler-method.md`, `concepts/forward-euler-method.md`, `concepts/topological-sorting.md`, `entities/raft.md`, `entities/paxos.md`, `entities/mapreduce.md`.
- Reconciled `wiki/index.md`: removed 6 duplicate rows (3 concept-table duplicates produced by the sed pass, 3 entity-table rows whose targets no longer exist), updated tags + confidence on the 6 surviving rows.
- Statistics updated: **Concepts 1152 → 1149, Entities 183 → 180**.

### Verification

- Inbound-link rewrite: 0 remaining citers of any of the 6 loser slugs anywhere in the wiki.
- Dangling-link sweep: clean (the one regex hit, ``\[\[wiki/index.md\]\]`` in this log file's prior section, is a backtick-quoted code example, not a real link).
- Orphan sweep: clean.
- Required-section schema: every survivor still satisfies its page-type schema (`## Definition`, `## How It Works`, `## Key Parameters`, `## When To Use`, `## Risks & Pitfalls`, `## Related Concepts`, `## Sources`).

## 2026-05-15 — Hairer–Wanner low-confidence stub buildout

Promoted **all 126** `confidence: low` concept stubs derived from `raw/solving_ordinary_differential_equations_ii/` from boilerplate placeholders to substantive concept pages. Source: the four chapter summaries (`summaries/hairer-ode-ii-03..06-*.md`) — already at `confidence: high` and dense with method-specific detail — were the primary input; chapter txt files were consulted for specific entries where additional precision was needed.

### Scope

| Chapter | Concepts promoted | Confidence after |
|---------|-------------------|------------------|
| IV — Stiff Problems / One-Step Methods | 47 | medium (most) / high (core: A-stability, B-stability, Runge–Kutta, IRK, Radau IIA, stage order, order reduction, dense output, extrapolation, order star, stability function/region/domain, stiffly accurate, simplified Newton, PI / predictive step control, Rosenbrock, Chebyshev) |
| V — Multistep Methods for Stiff Problems | 22 | medium / high (Dahlquist barrier, Daniel–Moore conjecture, G-stability, one-leg method, error constant, root-locus curve, Kreiss matrix theorem) |
| VI — Singular Perturbation Problems | 18 | medium / high (singular-perturbation-problem, differential-algebraic-equation, index-1-dae, boundary-layer, asymptotic-expansion, ε-embedding, state-space form, van der Pol, Brusselator, transistor amplifier, method of lines) |
| VII — Differential-Algebraic Equations / Higher Index | 39 | medium / high (DAE indices and definitions, Weierstrass–Kronecker, drift-off, Baumgarte, GGL, projection-method-dae, half-explicit, projected RK, constrained mechanical / Hamiltonian systems, symplectic methods, SHAKE / RATTLE, Lobatto IIIA-IIIB pair, composition methods, backward error analysis on manifolds, multibody, squeezer, Kepler, pendulum DAE) |

### Page-by-page treatment

For each stub I overwrote the placeholder body with a real:
- `## Definition` — 2–4 sentences with the precise mathematical statement (test equation, condition, family).
- `## How It Works` — mechanics drawn from the chapter summaries, with named theorems (Burrage–Butcher, Wanner–Hairer–Nørsett, Hairer–Lubich–Roche, Vasil'eva, Dahlquist) and standard codes (RADAU5, RODAS, SEULEX, PHEM56, LIMEX, RKC) cited where they apply.
- `## Key Parameters` — concrete dimensionful quantities (orders p, q; sector angle α; γ for SDIRK; the matrix M of algebraic stability; etc).
- `## When To Use` — domain-specific use cases (stiff ODE, DAE, SPP, multibody, parabolic PDE).
- `## Risks & Pitfalls` — order reduction, drift-off, conditioning gotchas, regime-of-validity warnings.
- `## Related Concepts` — actual `[[concepts/...]]` / `[[entities/...]]` cross-links.

Confidence bumped from `low` to `medium` (concepts whose treatment is grounded in the chapter summary alone) or `high` (concepts already broadly covered in the surrounding wiki literature plus the Hairer–Wanner detail).

### Acceptance gate

- **Zero dangling links** — scanned every `[[…]]` reference in all 126 rewritten pages against the filesystem; one initial dangler (`concepts/spice-engine` in `transistor-amplifier`) was repointed to `[[entities/spice]]`. Re-scan returned 0 missing targets across 149 unique link slugs.
- `## Definition` / `## How It Works` / `## Key Parameters` / `## When To Use` / `## Risks & Pitfalls` / `## Related Concepts` / `## Sources` present on every page.
- All pages still cite the appropriate `summaries/hairer-ode-ii-*` summary under `## Sources`.

### Statistics impact

No new pages created; only the 126 existing stubs were rewritten in place. `wiki/index.md` rows pointing at these concepts continue to resolve. No new orphans.

## project-init: scaffolded project/ and kanban/ (2026-05-17)

Ran `/wiki-project-init`. Filled the customization marker in `project/README.md` (language = rust, python; build = `cargo build`; test = `cargo test`; entry = `python -m circuit_solver`). Updated `AGENTS.md` `## Implementation Workspace` coherently. Appended Python ignores (`*.egg-info/`, `.pytest_cache/`, `.mypy_cache/`, `.ruff_cache/`) to `project/.gitignore`. `kanban/` was already fully scaffolded; `kanban/board.yaml` left untouched (still carries its `<!--` marker for `/wiki-kanban-board`).

- `project/README.md` marker check: `grep -c '<!--'` = 0 ✓
- `AGENTS.md` marker check: `grep -c '<!--'` = 0 ✓
- `kanban/` subtree check: 6 expected files present ✓
- `kanban/board.yaml` untouched: `grep -c '<!--'` = 1 (still has marker) ✓

Next step: `/wiki-kanban-board <slug>` to bind a Hermes board and flip the `board_bound` artifact green.

## kanban-board: bound circuit-solver (2026-05-17)

Bound Hermes Kanban board `circuit-solver` to this wiki.

- **Board slug:** `circuit-solver` (created via `hermes kanban boards create`)
- **Workspace default:** `dir:./project`
- **Profiles:** `orchestrator` → `default`, `worker` → `default`, `reviewer` → `default`
- **Skill verification:** `default` has `kanban-orchestrator` and `kanban-worker` enabled
- **Smoke test:** `hermes kanban --board circuit-solver list` exits 0
- **Acceptance gates:**
  - `grep -c '<!--' kanban/board.yaml` → 0 ✓
  - `yq eval '.board' kanban/board.yaml` → `circuit-solver` ✓
  - `profiles.orchestrator/worker/reviewer` all non-empty ✓
  - `AGENTS.md` `## Kanban Board` section contains no `<!--` marker ✓

Manifest artifact `board_bound` is now satisfied; `kanban_emit` and `kanban_ingest` are ungated.

## Query synthesis: bounded contexts in circuit simulation (2026-05-17)

Ran Query workflow against the question: *what bounded contexts seem to exist in this domain?*
Read `wiki/index.md` and 14 representative concept pages across analog, digital, mixed-signal, symbolic, physical-design, and DEVS clusters.

**Synthesis created:** `[[syntheses/bounded-contexts-in-circuit-simulation]]`

**Implied bounded contexts identified (8):**
1. Netlist & Graph Representation
2. Device Modeling & Compact Models
3. Analog Numerical Solver Engine
4. Digital Logic & Verification
5. Mixed-Signal Integration
6. Symbolic Analysis & Model Order Reduction
7. VLSI Physical Design & EDA
8. Discrete-Event Simulation Framework

**Key insight:** The wiki has 1,149 concepts but zero explicit context/context-map pages. The concept graph reveals a shared kernel around `[[concepts/modified-nodal-analysis]]` / `[[concepts/branch-stamping]]`, a customer–supplier edge from Device Modeling to Analog Solver, and an anticorruption-layer role for Mixed-Signal Integration. False cognates already present in the corpus: "node", "model", "simulation", "conservative", "branch".

**Recommendation:** Run `/wiki-strategy circuit-simulation` to formalize these clusters into explicit `wiki/contexts/` pages and a context map.

**Acceptance gate:** Verified every `[[…]]` reference in the new synthesis page against the filesystem; 0 dangling links. `wiki/index.md` statistics bumped: Syntheses 0 → 1.

## Strategy workflow for circuit-solver (2026-05-17)

Ran Strategy workflow for topic `circuit-solver`. Read `wiki/index.md`, the synthesis `[[syntheses/bounded-contexts-in-circuit-simulation]]`, and 12 representative concept/entity pages (dc-analysis, ac-analysis, transient-analysis, modified-nodal-analysis, device-modeling, sparse-matrix, newton-raphson-method, spice, spectre, rust-language, python, integration-method).

**Distillation pass:** Core (load-bearing) contexts: numeric-solver, analysis-orchestration. Supporting contexts: netlist-graph, device-modeling, application-frontend.

**Pages created or updated:**
- `[[vision/circuit-solver]]` — Value proposition, scope, differentiation for the R&D effort.
- `[[contexts/netlist-graph]]` — Circuit structural representation; ubiquitous language around nodes, branches, elements.
- `[[contexts/device-modeling]]` — Semiconductor equations and linearization; stamp contract with solver.
- `[[contexts/numeric-solver]]` — MNA, sparse LU, Newton-Raphson, integration methods.
- `[[contexts/analysis-orchestration]]` — DC/AC/transient/noise control loops and unified result structure.
- `[[contexts/application-frontend]]` — Python API and CLI entry point.
- `[[context-maps/circuit-solver]]` — Translations table, false-cognate inventory, integration-pattern assignments for all five contexts.
- Stubs: `[[concepts/bounded-context]]`, `[[concepts/context-map]]`, `[[concepts/false-cognate]]` (DDD scaffolding needed by required sections).

**False cognates flagged:**
- Node (netlist-graph vertex vs. numeric-solver matrix variable)
- Model (netlist-graph string key vs. device-modeling equation set)
- Operating Point (global DC solution vs. per-device bias)
- Convergence (analysis-level success vs. Newton-iteration tolerance)

**Acceptance gate:** Scanned every `[[…]]` reference in new pages; created missing stubs; 0 dangling links. `wiki/index.md` statistics bumped: Concepts 1149 → 1152, Vision 0 → 1, Contexts 0 → 5, Context Maps 0 → 1.

**2026-05-17 — /wiki-grill circuit-solver**

Started grill on \`circuit-solver\`. Top-level decisions identified: 6 items (binding mechanism, sparse solver, graph assembly, mixed-signal coupling, NR convergence, model dispatch). Grilled Decision 1 (Rust-to-Python binding).
- Q1→A1: PyO3 in-process
- Q2→A2: Immutable \`Circuit\` graph + per-request mutable analysis state
- Q3→A3: Builder API via PyO3 (Rust-backed incremental construction, \`.freeze()\` to immutable)
- Decision recorded: PyO3 in-process extension, immutable graph via builder, per-request analysis options.
- Status: in progress. Forward stubs created: \`[[architecture/circuit-solver]]\`, \`[[specs/circuit-solver]]\`.
- Next: pick another top-level decision to continue grilling.

**2026-05-17 — /wiki-grill circuit-solver (completed)**

Grill for \`circuit-solver\` completed. All 6 top-level decisions resolved:
1. Rust-to-Python binding: PyO3 in-process, immutable Circuit via builder API, per-request analysis state
2. Sparse direct solver: russell (DC/Transient), faer (AC)
3. Graph-to-matrix: flatten once, full matrix built, analysis extracts sub-view
4. Mixed-signal coupling: optimistic sync, sparse checkpointing, shared scheduler
5. NR convergence: ΔI/ΔV primary + KCL guard triggered on claimed convergence
6. Device dispatch: closed enum for core models (diode, BJT, MOSFET)
- Status updated to \`done\`. Grill file: \`wiki/grills/circuit-solver.md\`.
- Forward stubs: \`wiki/architecture/circuit-solver.md\`, \`wiki/specs/circuit-solver.md\`
- Next step: /wiki-architecture circuit-solver

## 2026-05-17 — /wiki-architecture circuit-solver

Ran Architecture workflow for `circuit-solver`.

**Purpose:** *How does a SPICE netlist get parsed, flattened, stamped, and solved end-to-end, particularly in mixed-signal simulation environments?*

**C4 levels drawn:** Context + Container (default).

**Containers identified:**
1. Python Frontend (Python, PyO3)
2. Netlist Graph Builder (Rust)
3. Device Model Engine (Rust)
4. Numeric Solver Engine (Rust, russell, faer)
5. Analysis Orchestrator (Rust)
6. Mixed-Signal Scheduler (Rust)

**Decisions surfaced (5):**
1. PyO3 In-Process Binding with Immutable Circuit Graph
2. Hybrid Sparse Direct Solver Backend (russell + faer)
3. Two-Pass Graph Flattening with Per-Analysis Sub-Views
4. Optimistic Mixed-Signal Synchronization via Shared Scheduler
5. Closed Enum Device Model Dispatch

**Cross-link updates:** Added `## Architecture` sections to all 5 context pages (`netlist-graph`, `device-modeling`, `numeric-solver`, `analysis-orchestration`, `application-frontend`).

**Acceptance gate:** Zero dangling links — all 9 `[[…]]` references on `architecture/circuit-solver.md` resolve to existing files.

**Next step:** `/wiki-adr <decision title>` for each surfaced decision.

---

2026-05-17 — ADR-0001 opened: "PyO3 In-Process Binding with Immutable Circuit Graph". Status: proposed. ASR: "The simulator must expose an ergonomic, zero-copy Python API for interactive circuit construction and analysis while preserving Rust's ownership and memory-safety guarantees." Cross-linked: architecture/circuit-solver (surfaced bullet updated → ADR-0001), contexts/application-frontend, concepts/ownership, concepts/memory-safety, concepts/python, concepts/ffi. Stubs created: concepts/architecturally-significant-requirement, concepts/architectural-decision-record, concepts/system-architecture, concepts/quality-attributes. Index.md updated (Decisions table + Concepts count 1152→1156). Zero dangling links verified.

---

2026-05-17 — ADR-0002 opened: "Hybrid Sparse Direct Solver Backend (russell + faer)". Status: proposed. ASR: "The numeric solver must perform sparse direct LU factorization for all in-scope analysis types without crossing an FFI boundary to C/C++ legacy libraries, because memory safety is a core differentiator, and the analysis portfolio inherently requires both real-valued (DC/transient) and complex-valued (AC) solves." Cross-linked: architecture/circuit-solver (surfaced bullet updated → ADR-0002), contexts/numeric-solver, concepts/sparse-matrix, concepts/lu-decomposition, concepts/ffi, concepts/memory-safety. Index.md updated (Decisions table). Zero dangling links verified.

---

2026-05-17 — ADR-0003 opened: "Two-Pass Graph Flattening with Per-Analysis Sub-Views". Status: proposed. ASR: "The numeric solver must handle ground-reference and constraint-mask differences between operating-point and small-signal analyses without re-flattening the netlist graph." Cross-linked: architecture/circuit-solver (surfaced bullet updated → ADR-0003), contexts/netlist-graph, contexts/numeric-solver, contexts/analysis-orchestration, concepts/architecturally-significant-requirement. Index.md updated (Decisions count 1→3, Decisions table). Zero dangling links verified.

---

2026-05-17 — ADR-0004 opened: "Optimistic Mixed-Signal Synchronization via Shared Scheduler". Status: proposed. ASR: "The simulator must support mixed-signal co-simulation between the continuous-time analog solver and an external event-driven digital simulator, using optimistic time advancement with efficient rollback on digital mispredictions, while keeping the analog analysis-orchestration and digital kernel contexts decoupled (neither queries the other directly)." Cross-linked: architecture/circuit-solver (surfaced bullet updated → ADR-0004), contexts/analysis-orchestration, grills/circuit-solver, vision/circuit-solver, concepts/mixed-level-simulation. Index.md updated (Decisions count 3→4, Decisions table). Zero dangling links verified.

---

2026-05-17 — ADR-0005 opened: "Closed Enum Device Model Dispatch". Status: proposed. ASR: "Newton-Raphson stamp evaluation must run in a tight loop with zero-cost dispatch and cache-friendly data layouts." Cross-linked: architecture/circuit-solver (surfaced bullet updated → ADR-0005), contexts/device-modeling, contexts/numeric-solver, grills/circuit-solver, vision/circuit-solver, concepts/zero-cost-abstractions, concepts/static-dispatch, concepts/dynamic-dispatch, concepts/trait-objects, concepts/memory-safety. Index.md updated (Decisions count 4→5, Decisions table). Zero dangling links verified.

---

2026-05-18 — Spec emitted: `wiki/specs/circuit-solver.md` (slug: `circuit-solver`). Goal: v1 binary acceptance criteria for the unified analog / digital / mixed-signal simulator, validated against [[entities/ngspice]] (analog golden reference) and [[entities/icarus-verilog]] (digital golden reference) on the [[entities/sky130-pdk]] and [[entities/asap7-pdk]]. Stories: 5 (analog conformance, digital event-trace equivalence, three mixed-signal cosim circuits, Newton-Raphson convergence guard, PyO3 frontend contract). Scenarios: 13 fenced ` ```gherkin ` blocks including one `Scenario Outline` (six cells/testbenches across both PDKs). User-confirmed scope envelopes: lenient functional-correctness (5 % / 0.5 dB / 100 µV / 2 dB); ASAP7 restricted to gate-level digital per [[decisions/0005-closed-enum-device-model-dispatch|ADR-0005]] (BSIM-CMG analog deferred); event-trace equivalence (not byte-level VCD) as the digital metric; mixed-signal corpus = digital-driven analog load, comparator + DFF, level shifter. ADR citations in frontmatter: 0001–0005 (all currently `proposed` — recorded as a soft-gate decision-to-proceed under the spec's `## Sources` section). Stubs created (`confidence: low`): concepts/golden-reference, concepts/value-change-dump, concepts/event-trace-equivalence, concepts/global-interpreter-lock, entities/icarus-verilog, entities/sky130-pdk, entities/asap7-pdk, entities/pyo3, entities/russell, entities/faer. Index.md updated (Concepts 1156→1160, Entities 180→186, Specs row added with count 1, header `updated` field bumped to 2026-05-18). Zero dangling links verified.

---

2026-05-18 — ADRs 0001–0005 promoted from `proposed` to `accepted`. Files updated: `wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md`, `wiki/decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer.md`, `wiki/decisions/0003-two-pass-graph-flattening-with-per-analysis-sub-views.md`, `wiki/decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler.md`, `wiki/decisions/0005-closed-enum-device-model-dispatch.md` — `## Status` flipped to `accepted` and frontmatter `updated` bumped to 2026-05-18 in each. ADR bodies were left untouched, preserving write-once discipline ([[concepts/architectural-decision-record]]). Downstream impact: the Spec workflow's soft-gate on `wiki/specs/circuit-solver.md` is now satisfied; `/wiki-kanban-emit` can dispatch the spec under the P2 pipeline pattern. Index.md `## Decisions` table refreshed (all five rows now show `accepted | 2026-05-18`). Spec's `## Sources` ADR-status footnotes updated to reflect acceptance.

---

2026-05-18 — Kanban emit: `wiki/specs/circuit-solver.md` (slug: `circuit-solver`). Board: `circuit-solver`. Tenant: `default`. Workspace: `dir:/home/phillip/Boxes/Homes/RustDev/Code/github.com/pbonh/circuit_solver/project`. Skills: `wiki-maintainer`, `kanban-worker`. Profile mapping: orchestrator → `default`, worker → `default`, reviewer → `default`. Pattern: P2 pipeline (`worker` → `reviewer`). Parent task: `t_e3d1ebe9` (idempotency key `circuit-solver:0001+0002+0003+0004+0005:8f2bb9a98937aad9e3df672478aabd493d18d4cbdf8ef6ef3206726dfafb96b3`). Scenario tasks (12, all `worker` → `default`): `t_8ca9027e` (NMOS saturation), `t_0dac134b` (CS amp AC), `t_2c3eb911` (ring oscillator transient), `t_7c3875fb` (noise), `t_6ec40808` (digital event-trace), `t_99a70ad2` (digital-driven analog load), `t_69000f20` (comparator + DFF), `t_79ecbefd` (level shifter), `t_af0770ea` (hybrid convergence guard), `t_8e71a163` (homotopy fallback), `t_5931e2d9` (immutability / zero-copy), `t_a073ec79` (GIL release). Aggregator task: `t_10406b92` (reviewer → `default`, idempotency key `circuit-solver:0001+0002+0003+0004+0005:aggregator:8e0549487ea897c877fe5ae528812b963f1857313c5815babb587a2787d73abc`). ADR ids: `0001`, `0002`, `0003`, `0004`, `0005` (all `accepted`). Spec page updated with `## Kanban Tasks` section. Zero dangling links verified. Next step: `/wiki-kanban-ingest <aggregator-task-id>` after worker branches merge.
