---
title: Read Your Own Writes (RYOWs)
type: claim
id: claim-read-your-own-writes
tags:
- distributed-systems
- consistency
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

Read-Your-Own-Writes (RYOWs) consistency is a per-session guarantee that, after a client makes a write, any subsequent read by the same client returns the new value — regardless of replication lag visible to other clients. It is weaker than full strong consistency but addresses a common user-visible inconsistency.

## How It Works

In leader-follower systems, route the client's subsequent reads to the leader (default in MongoDB). In Neo4j, the client receives a "bookmark" after a write and presents it on subsequent reads so only sufficiently fresh replicas serve them. Causal-consistency frameworks generalize this idea.

## Key Parameters

- Bookmark/session-token TTL.
- Routing policy (leader vs. follower).

## When To Use

Profile updates, posting comments, any workflow where the user immediately revisits their own change.

## Risks & Pitfalls

- Other users still see eventual consistency.
- Routing reads to the leader can swamp it.

## Related Concepts

- [[concepts/eventual-consistency]]
- [[concepts/strong-consistency]]
- [[concepts/tunable-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
