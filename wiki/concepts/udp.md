---
title: User Datagram Protocol (UDP)
type: claim
id: concepts/udp
tags:
- networking
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

UDP is a connectionless, unreliable, datagram-oriented transport-layer protocol of the Internet Protocol suite. It provides minimal overhead and at-most-once delivery semantics.

## How It Works

Each datagram is sent independently with no handshake, sequencing, acknowledgment, or retransmission. The application must tolerate packet loss, duplication, and out-of-order delivery.

## Key Parameters

- Datagram size (MTU-bound to avoid fragmentation).
- Application-level retry policy.

## When To Use

Streaming media, VoIP, gaming, DNS queries, NTP, and other workloads where occasional loss is acceptable and low latency is critical.

## Risks & Pitfalls

- No congestion control: misbehaving apps can flood networks.
- Unreliable delivery; the app must handle loss explicitly.

## Related Concepts

- [[concepts/tcp]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
