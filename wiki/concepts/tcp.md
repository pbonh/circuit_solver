---
title: "Transmission Control Protocol (TCP)"
type: concept
tags: [networking, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt"]
confidence: high
---

## Definition

TCP is a connection-oriented, stream-oriented, reliable transport-layer protocol of the Internet Protocol suite. It provides ordered, error-checked delivery of byte streams between two hosts on top of IP.

## How It Works

A three-way handshake (SYN, SYN-ACK, ACK) establishes a connection. Sender breaks data into packets up to 65,535 bytes, attaching sequence numbers; the receiver acknowledges cumulatively, triggering retransmission of unacknowledged packets. TCP also provides flow control and congestion control.

## Key Parameters

- Connection timeout / keep-alive interval.
- Send/receive buffer sizes.
- Congestion-control algorithm (CUBIC, BBR).

## When To Use

The default transport for HTTP, HTTPS, and most internet application protocols. Use UDP instead when low latency outweighs reliability (streaming media, gaming, VoIP).

## Risks & Pitfalls

- Connection-establishment cost is significant; reuse connections.
- Head-of-line blocking inside a single TCP stream.
- High latency on long fat networks unless tuned.

## Related Concepts

- [[concepts/udp]]
- [[concepts/rpc]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
