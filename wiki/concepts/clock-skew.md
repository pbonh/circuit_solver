---
title: "Clock Skew"
type: concept
tags: [distributed-systems, well-established, time, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Clock skew is the difference between the time reported by clocks on different machines (or by different clocks on the same machine). Quartz oscillators drift relative to each other, network time synchronization (NTP) has bounded accuracy, and time-of-day clocks can jump backward when corrected. This makes timestamp-based ordering unsafe across nodes.

## How It Works

- **Time-of-day clocks** (wall-clock): `clock_gettime(CLOCK_REALTIME)`. Synchronized with NTP. Can jump backward, skip seconds for leap-second handling, drift, or be wildly off if misconfigured.
- **Monotonic clocks**: `clock_gettime(CLOCK_MONOTONIC)`. Guaranteed only to move forward; absolute value is meaningless. Use for measuring durations.
- **NTP synchronization**: best-case ~tens of milliseconds over the internet, 1–2 ms in LAN. Google assumes 200 ppm drift.
- **Spanner's TrueTime**: GPS/atomic clocks plus an explicit `[earliest, latest]` uncertainty interval. Commits wait out the uncertainty to keep timestamps causally ordered.
- Skew can cause LWW to silently drop writes from a slow-clock node and break leases / leader-fencing checks based on wall-clock time.

## Key Parameters

- NTP synchronization interval.
- Acceptable skew tolerance.
- Use of GPS/atomic time sources for low-skew applications (MiFID II requires 100 μs of UTC).

## When To Use

Awareness: never use time-of-day clocks for ordering events across nodes. Use monotonic clocks for durations and logical clocks (Lamport, version vectors) for ordering. Use synchronized clocks only when paired with uncertainty intervals (TrueTime).

## Risks & Pitfalls

- LWW conflict resolution on clock-skewed nodes loses data.
- Lease expiry compared to wall clock can fail spectacularly if the clock jumps.
- VM live migration can pause a clock relative to the rest of the cluster.
- Mobile / embedded devices may have arbitrarily wrong clocks; never trust them.

## Related Concepts

- [[concepts/lamport-timestamp]]
- [[concepts/version-vector]]
- [[concepts/total-order-broadcast]]
- [[concepts/linearizability]]
- [[concepts/fencing-token]]
- [[entities/spanner]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
