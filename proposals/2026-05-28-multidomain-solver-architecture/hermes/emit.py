#!/usr/bin/env python3
"""scientia-hermes-emit shim for circuit-solver-gamma.

Hermes v0.15.1 removed the `hermes kanban task create` verb; the scientia 1.0.0
CLI transport is broken. This script uses `hermes kanban create` (the current
verb) as a custom transport, with parent IDs wired AT creation time so cards
start as `blocked` (not `ready`) and the dispatcher cannot claim them before
their dependency links exist.

Fix history:
  v1 (original): used apply() — all cards created as `ready`, links wired after.
    Race: dispatcher claimed integrate cards before parent links were added.
  v2 (this): custom emit loop with --parent at creation; no separate links pass.
"""

from __future__ import annotations

import json
import subprocess
import sys

import scientia.paths as paths
from scientia.hermes.plan import ProfileModel

# --------------------------------------------------------------------------- #
# Constants                                                                    #
# --------------------------------------------------------------------------- #
CHANGE_ID = "2026-05-28-multidomain-solver-architecture"
BOARD = "circuit-solver-gamma"
PREFIX = "circuit-solver-gamma"

CHANGE_DIR = paths.change_dir(CHANGE_ID)
TASKS_MD = CHANGE_DIR / "tasks.md"
DESIGN_MD = CHANGE_DIR / "design.md"

# --------------------------------------------------------------------------- #
# Model configs (from development/config.yaml hermes.profiles)                #
# --------------------------------------------------------------------------- #
MODELS: dict[str, ProfileModel] = {
    f"{PREFIX}-implementer": ProfileModel(
        provider="fireworks",
        model="accounts/fireworks/models/glm-5p1",
        base_url="https://api.fireworks.ai/inference/v1",
        api_key_env="FIREWORKS_API_KEY",
        temperature=0.1,
        max_tokens=25344,
    ),
    f"{PREFIX}-reviewer": ProfileModel(
        provider="fireworks",
        model="accounts/fireworks/models/kimi-k2p6",
        base_url="https://api.fireworks.ai/inference/v1",
        api_key_env="FIREWORKS_API_KEY",
        temperature=0.3,
        max_tokens=32768,
    ),
    f"{PREFIX}-integrator": ProfileModel(
        provider="fireworks",
        model="accounts/fireworks/models/minimax-m2p7",
        base_url="https://api.fireworks.ai/inference/v1",
        api_key_env="FIREWORKS_API_KEY",
        temperature=0.0,
        max_tokens=24576,
    ),
    f"{PREFIX}-conflict-resolver": ProfileModel(
        provider="fireworks",
        model="accounts/fireworks/models/deepseek-v4-pro",
        base_url="https://api.fireworks.ai/inference/v1",
        api_key_env="FIREWORKS_API_KEY",
        temperature=0.0,
        max_tokens=131072,
    ),
}

# --------------------------------------------------------------------------- #
# CLI shim: hermes kanban create with --parent at creation time               #
# --------------------------------------------------------------------------- #

def _create_card(board: str, body: dict, parent_hermes_ids: list[str]) -> dict:
    """Create one card via `hermes kanban create`, wiring parents at creation time.

    Passing --parent IDs here (before the card is visible to the dispatcher)
    ensures the card starts as `blocked` until all parents complete.  The
    original v1 shim used a separate POST /links pass; by then the dispatcher
    had already claimed cards that had been created as `ready` with no parents.
    """
    body_text = body.get("body") or ""
    cmd = [
        "hermes", "kanban", "--board", board,
        "create", "--json",
        "--idempotency-key", body["idempotency_key"],
        "--body", body_text,
    ]
    for pid in parent_hermes_ids:
        cmd += ["--parent", pid]
    if body.get("assignee"):
        cmd += ["--assignee", body["assignee"]]
    if body.get("tenant"):
        cmd += ["--tenant", body["tenant"]]
    if body.get("workspace"):
        cmd += ["--workspace", body["workspace"]]
    if body.get("branch"):
        cmd += ["--branch", body["branch"]]
    if body.get("priority") is not None:
        cmd += ["--priority", str(body["priority"])]
    for skill in body.get("skills", []):
        cmd += ["--skill", skill]
    cmd.append(body["title"])  # positional
    result = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(result.stdout) if result.stdout.strip() else {}


def _archive_card(board: str, hermes_id: str) -> None:
    subprocess.run(
        ["hermes", "kanban", "--board", board, "archive", hermes_id],
        capture_output=True, text=True, check=True,
    )


# --------------------------------------------------------------------------- #
# Custom emit loop (replaces scientia.hermes.apply.apply)                     #
# --------------------------------------------------------------------------- #

def _emit(plan, board: str, *, dry_run: bool, on_supersede: str = "archive") -> dict[str, str]:
    """Create/skip cards with parents wired at creation time; returns key→hermes_id."""
    from scientia.hermes import ledger, render

    old = ledger.load(plan.change_id)
    all_cards = ([plan.epic] + list(plan.cards)) if plan.epic is not None else list(plan.cards)
    entries = ledger.entries_for_plan(plan)

    # Carry over already-created IDs from a prior ledger
    for key, entry in entries.items():
        if key in old and old[key].hermes_id:
            entry.hermes_id = old[key].hermes_id
            entry.last_status = old[key].last_status

    if dry_run:
        return {
            c.key: (old[c.key].hermes_id if c.key in old and old[c.key].hermes_id else "(new)")
            for c in all_cards
        }

    # Running key→hermes_id map; populated as cards are created (topological order)
    id_map: dict[str, str] = {k: e.hermes_id for k, e in entries.items() if e.hermes_id}

    diff = ledger.diff(old, plan)
    created: set[str] = set()

    for card in all_cards:  # plan is topologically ordered — parents come before children
        entry = entries[card.key]
        if entry.hermes_id:  # already exists; skip
            id_map[card.key] = entry.hermes_id
            continue

        # All parents in card.parents are guaranteed to already have hermes IDs
        # because the plan is topologically ordered (parent cards come first).
        parent_ids = [id_map[pk] for pk in card.parents if pk in id_map]

        payload = render.task_payload(card, board)
        resp = _create_card(board, payload, parent_ids)
        entry.hermes_id = str(resp.get("id", ""))
        entry.last_status = "todo"
        id_map[card.key] = entry.hermes_id
        created.add(card.key)

    # Archive superseded cards from a prior emit (re-key scenario)
    if on_supersede == "archive":
        superseded = list(diff.removed) + [old_key for old_key, _ in diff.changed]
        for old_key in superseded:
            hid = old[old_key].hermes_id if old_key in old else None
            if hid:
                _archive_card(board, hid)

    ledger.record(plan.change_id, entries)

    return {k: e.hermes_id for k, e in entries.items() if e.hermes_id}


# --------------------------------------------------------------------------- #
# Main                                                                         #
# --------------------------------------------------------------------------- #

def main(dry_run: bool = True) -> None:
    from scientia.hermes.parse import parse_tasks, parse_design
    from scientia.hermes.plan import Routing, PlanOptions, build_plan
    from scientia.hermes.validators import validate_plan, validate_routing, ownership_smells
    from scientia.hermes.preflight import check as preflight_check
    from scientia.hermes.board import resolve_board
    from scientia.hermes import ledger

    # 1. Parse
    tasks_text = TASKS_MD.read_text()
    design_text = DESIGN_MD.read_text()
    tasks = parse_tasks(tasks_text)
    c4, comp_map, contracts = parse_design(design_text)
    print(f"  Parsed {len(tasks)} tasks, {len(c4)} C4 diagrams, {len(contracts)} contracts")

    # 2. Resolve board
    board = resolve_board("")
    assert board == BOARD, f"unexpected board: {board}"

    # 3. Routing
    routing = Routing(
        default_implementer=f"{PREFIX}-implementer",
        default_reviewer=f"{PREFIX}-reviewer",
        default_integrator=f"{PREFIX}-integrator",
        resolver=f"{PREFIX}-conflict-resolver",
        board=board,
        tenant=CHANGE_ID,
        profile_models=MODELS,
    )

    # 4. Options
    adr_contracts = frozenset({
        "netlist.CircuitGraph",   # ADR-0001
        "numeric.StampInterface", # ADR-0002
        "netlist.FlattenedView",  # ADR-0003
        "devices.DeviceModel",    # ADR-0005
        "digital.DigitalKernel",  # ADR-0006
    })
    options = PlanOptions(
        pipeline="impl-review-integrate",
        emit_epic=True,
        workspace="worktree",
        max_parallel_per_file_group=2,
        conflict_prevention=True,
        adr_contracts=adr_contracts,
    )

    # 5. Build plan
    try:
        plan = build_plan(CHANGE_ID, tasks, c4, comp_map, contracts, routing, options)
    except Exception as e:
        print(f"FATAL build_plan error: {type(e).__name__}: {e}", file=sys.stderr)
        sys.exit(1)

    all_cards = ([plan.epic] + list(plan.cards)) if plan.epic is not None else list(plan.cards)
    print(f"  Plan: {len(all_cards)} cards (1 epic + {len(plan.cards)} work cards)")

    # 6. Validate
    known_profiles = set(MODELS.keys())
    plan_errors = validate_plan(plan, known_profiles=known_profiles)
    routing_errors = validate_routing(routing, tasks, known_profiles=known_profiles,
                                      pipeline="impl-review-integrate")
    for w in ownership_smells(tasks, comp_map):
        print(f"  WARNING (ownership smell): {w}")
    if plan_errors or routing_errors:
        for e in plan_errors + routing_errors:
            print(f"  ERROR: {e}", file=sys.stderr)
        sys.exit(1)

    # 7. Preflight
    result = preflight_check(
        plan,
        require_gateway=True,
        rest_base="http://127.0.0.1:8787/api/plugins/kanban",
        known_profiles=known_profiles,
    )
    for w in result.warnings:
        print(f"  PREFLIGHT WARNING: {w}")
    if not result.ok:
        for e in result.errors:
            print(f"  PREFLIGHT ERROR: {e}", file=sys.stderr)
        sys.exit(1)
    print("  Preflight: OK")

    # 8. Emit (custom loop — parents wired at creation, no separate links pass)
    old = ledger.load(CHANGE_ID)
    id_map = _emit(plan, board, dry_run=dry_run)

    # 9. Report
    old_after = ledger.load(CHANGE_ID)
    diff = ledger.diff(old, plan)
    if dry_run:
        n_new = sum(1 for v in id_map.values() if v == "(new)")
        n_exist = sum(1 for v in id_map.values() if v != "(new)")
        print(f"\n  [DRY RUN] would create: {n_new}, already exist: {n_exist}")
        print(f"  changed: {len(diff.changed)}, removed: {len(diff.removed)}")
        if n_new:
            sample = [(k, v) for k, v in id_map.items() if v == "(new)"][:5]
            print(f"\n  Sample new (first 5):")
            for k, _ in sample:
                print(f"    {k}")
    else:
        n_created = sum(1 for k in id_map if k not in old)
        n_exist = sum(1 for k in id_map if k in old)
        print(f"\n  Results: created={n_created}, existing={n_exist}, "
              f"changed={len(diff.changed)}, removed={len(diff.removed)}")


if __name__ == "__main__":
    dry = "--apply" not in sys.argv
    print("=== DRY RUN ===\n" if dry else "=== APPLY ===\n")
    main(dry_run=dry)
