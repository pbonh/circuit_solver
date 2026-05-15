---
title: "Circuit Simulation Knowledge Base Log"
type: log
updated: 2026-05-15
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
