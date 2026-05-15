---
title: "TrueTime"
type: concept
tags: [distributed-systems, consistency, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: medium
---

## Definition

TrueTime is Google's bounded-uncertainty time service. It uses satellite-connected GPS receivers and atomic clocks in each data center to provide timestamps with a known maximum clock skew (around 7 ms). It is the secret sauce that enables Cloud Spanner to deliver external (strict-serializable) consistency at global scale.

## How It Works

Every call to `TrueTime.now()` returns an interval `[earliest, latest]` representing the bounded uncertainty. Spanner commits transactions with a chosen timestamp and waits for `latest` to be in the past at all nodes ("commit wait") before releasing locks. This guarantees that any later transaction sees a strictly higher timestamp.

## Key Parameters

- Clock skew bound (~7 ms in production).
- Commit-wait duration.

## When To Use

Specialized hardware in major cloud provider data centers; not available outside Google. Open-source variants (HLCs, NTP-bound clocks) approximate the idea with weaker guarantees.

## Risks & Pitfalls

- Requires custom hardware in every data center.
- Larger uncertainty bounds directly slow transactions.

## Related Concepts

- [[concepts/clock-drift]]
- [[concepts/linearizability]]
- [[concepts/strong-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
