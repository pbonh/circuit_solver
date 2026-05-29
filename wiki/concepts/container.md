---
title: Container
type: claim
id: claim-container
tags:
- cloud
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
---

## Definition

A container is a lightweight, isolated runtime unit that packages application code together with its dependencies and OS-level libraries. Containers share the host kernel (unlike VMs, which virtualize the whole machine), making them faster to start and lighter on resources. Docker is the dominant implementation.

## How It Works

A container image is a layered filesystem snapshot. The container engine (Docker, containerd) starts a process in a namespace with isolated network, filesystem, and PID views. Orchestration platforms (Kubernetes, Apache Mesos) schedule containers onto cluster nodes.

## Key Parameters

- Image size.
- Resource limits (CPU, memory).
- Restart policy.

## When To Use

The default packaging unit for modern microservices.

## Risks & Pitfalls

- Image bloat slows pulls and increases attack surface.
- Containers share the host kernel; kernel vulnerabilities are container vulnerabilities.

## Related Concepts

- [[concepts/microservices]]
- [[concepts/infrastructure-as-code]]
- [[entities/kubernetes]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
