---
title: Network Time Protocol (NTP)
type: claim
id: claim-ntp
tags:
- distributed-systems
- networking
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/04-part-i-the-basics.txt
confidence:
  base: 0.65
---

## Definition

The Network Time Protocol (NTP) is a hierarchically organized internet protocol for synchronizing computer clocks to within a few milliseconds (LAN) or tens of milliseconds (WAN) of a reference time source.

## How It Works

A small number of stratum-0 reference clocks (GPS, atomic) feed ~300 root servers; about 20,000 stratum-2 servers synchronize from those, and so on through up to 15 levels. Clients exchange UDP messages with NTP servers, estimate transit time, and reset the local time-of-day clock — sometimes backward.

## Key Parameters

- Polling interval.
- Maximum allowed offset before correction.
- Preferred stratum.

## When To Use

Effectively required on every networked machine. Successors like Chrony provide higher accuracy; AWS provides the Time Sync Service.

## Risks & Pitfalls

- Clocks can jump backward; intervals computed across an NTP step can be negative.
- LAN-millisecond accuracy is still inadequate for ordering distributed events.

## Related Concepts

- [[concepts/clock-drift]]
- [[concepts/logical-clock]]
- [[concepts/truetime]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
