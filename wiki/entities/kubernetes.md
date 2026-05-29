---
title: Kubernetes
type: entity
id: entity-kubernetes
tags:
- deployment
- orchestration
- cloud
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/16-chapter-12-afterword.txt
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Overview

Two cited sources both touch Kubernetes only at the level of named-tool deployment guidance:
- Prototyping Python Dashboards, afterword: "The easiest answer is to use a technology like Kubernetes (Google also offers this kind of capability) that can load balance and scale your dashboard to serve many users."
- Foundations of Scalable Systems, Ch. 16: "Containers are typically utilized in concert with a cluster management platform such as Kubernetes or Apache Mesos. These orchestration platforms provide APIs for you to control how, when, and where your containers execute. They make it possible to automate your deployment of containers to support varying system loads using autoscaling and simplify the management of deploying multiple containers across multiple nodes in a cluster."

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
