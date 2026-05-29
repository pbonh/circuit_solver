---
title: Infrastructure as Code (IaC)
type: claim
id: concepts/infrastructure-as-code
tags:
- cloud
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Infrastructure as Code (IaC) is the practice of defining infrastructure (compute, networking, storage, security policies) in machine-readable templates that are version-controlled and applied programmatically. Tools include Terraform, AWS CloudFormation, Pulumi, and Ansible.

## How It Works

Authors write declarative templates describing the desired state. The IaC tool computes a diff against the actual state and applies changes idempotently. Templates live in version control alongside application code; changes go through pull-request review.

## Key Parameters

- Template language (HCL, YAML, code).
- State storage and locking.

## When To Use

Always for cloud deployments at any non-trivial scale.

## Risks & Pitfalls

- Drift between actual and templated state.
- Secrets in templates if not handled carefully.

## Related Concepts

- [[concepts/devops]]
- [[concepts/continuous-delivery]]
- [[concepts/container]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
