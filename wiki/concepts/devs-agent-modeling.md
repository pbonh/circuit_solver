---
title: DEVS Agent Modeling
type: claim
id: claim-devs-agent-modeling
tags:
- simulation
- modeling
- devs
- agent-based
- dynamic-structure
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt
confidence:
  base: 0.65
---

## Definition

DEVS Agent Modeling uses dynamic-structure DEVS to encode agents — DEVS atomic models that receive "Actor" objects (smart objects) as inputs, add them as peer components in the parent coupled model, interact through dynamically established couplings, and release them on output ports.

## How It Works

An Agent's external event for receiving an Actor invokes `addChildModel(actor)` on its parent and `addCoupling(...)` for both directions of the conversation (Agent.outHello ↔ Actor.inHello, etc.). The Agent and Actor exchange a Hello/Hi/GoodBye/Bye sequence. On Bye, the Agent issues `output.add(outActor, storedActor)` and on the subsequent internal event invokes `removeChildModel(actor)`, returning the parent to its initial structural state.

## Key Parameters

- Agent state machine for interaction phases (waitForActor, sendHello, waitForHi, sendGoodBye, waitForBye, sendActor)
- Actor smart-object class with input/output ports
- Dynamic-structure add/remove invocations
- Interaction time parameter

## When To Use

- Healthcare / care-center modeling where actors visit multiple agents
- Mobile-agent simulations
- Workflow scenarios with stateful objects flowing through processors

## Risks & Pitfalls

- Actor state must survive the addChild/removeChild lifecycle
- Coupling additions must reference current parent's name
- Agents at different levels of the hierarchy require care to avoid name collisions

## Related Concepts

- [[concepts/dynamic-structure-devs]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
