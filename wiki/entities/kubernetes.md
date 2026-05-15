---
title: "Kubernetes"
type: entity
tags: [deployment, orchestration, cloud]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/16-chapter-12-afterword.txt"]
confidence: low
---

## Overview

Kubernetes is an open-source container-orchestration platform that automates deployment, scaling, and management of containerized applications. The book mentions it (along with Google's offerings) as a path to scaling dashboards that achieve wide demand.

## Characteristics

- Declarative configuration via YAML manifests describing desired state.
- Pods, Deployments, Services, Ingress, and ConfigMaps are core primitives.
- Built-in horizontal pod autoscaling and rolling updates.
- Multi-cloud and on-premises deployment options.

## Common Strategies

- Containerize a dashboard application via Docker.
- Deploy as a Kubernetes Deployment with multiple replicas and a Service for load balancing.
- Use Ingress controllers to terminate TLS and route traffic to the right Service.
- Reserve Kubernetes for genuinely high-demand workloads — most academic dashboards do not need it.

## Related Entities

- [[entities/nginx]]
- [[entities/gunicorn]]
- [[entities/dash]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
- [[summaries/prototyping-python-dashboards-16-chapter-12-afterword]]
