#!/usr/bin/env python3
"""Emit the multidomain-solver change onto the Hermes kanban via a v0.15.1 CLI shim.

Why a shim: the scientia 0.2 bundle's scientia.hermes.apply transport targets a
`hermes kanban task create/update` CLI and an unauthenticated loopback REST API.
Current Hermes (v0.15.1) exposes neither — its surface is `hermes kanban create`
(positional title, no `task` subcommand) and an auth-gated dashboard. apply()
exposes a `transport=` seam (used by its own test suite); we inject a transport
that maps the three ops apply() issues onto the real v0.15.1 CLI.

Mapping:
  POST /tasks        -> hermes kanban [--board B] create "<title>" --body .. \
                          --idempotency-key K [--assignee/--tenant/--workspace/ \
                          --branch/--priority/--skill ..] --json   (parse {id})
  POST /links        -> hermes kanban link <parent_id> <child_id>
  PATCH /tasks/{id}  -> status=archived -> hermes kanban archive <id>
                        assignee=X       -> hermes kanban reassign <id> --to X

Per-card model overrides are NOT sent (v0.15.1 `create` has no --model-* flags);
the model lives on the assignee profile instead. Idempotent via the ledger.
"""
from __future__ import annotations

import json
import subprocess
import sys

import yaml

from scientia import paths
from scientia.hermes import board as boardmod, parse
from scientia.hermes.apply import apply
from scientia.hermes.plan import PlanOptions, ProfileModel, Routing, build_plan

CID = "2026-05-28-multidomain-solver-architecture"
DRY = "--apply" not in sys.argv


def make_cli_transport(verbose: bool = True):
    n = {"create": 0, "link": 0, "archive": 0}

    def call(method: str, path: str, body):
        body = body or {}
        if method == "POST" and path == "/tasks":
            cmd = ["hermes", "kanban"]
            if body.get("board"):
                cmd += ["--board", body["board"]]
            cmd += ["create", body["title"], "--body", body["body"],
                    "--idempotency-key", body["idempotency_key"], "--json"]
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
            out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
            resp = json.loads(out) if out.strip() else {}
            n["create"] += 1
            if verbose:
                print(f"  + create #{n['create']:>2} {resp.get('id','?'):<12} "
                      f"{body.get('assignee','-'):<12} {body['title'][:60]}")
            return {"id": resp.get("id")}
        if method == "POST" and path == "/links":
            subprocess.run(["hermes", "kanban", "link", str(body["parent"]),
                            str(body["child"])], capture_output=True, text=True, check=True)
            n["link"] += 1
            return {}
        if method == "PATCH" and path.startswith("/tasks/"):
            tid = path.rsplit("/", 1)[-1]
            if body.get("status") == "archived":
                subprocess.run(["hermes", "kanban", "archive", tid],
                               capture_output=True, text=True, check=True)
                n["archive"] += 1
            elif body.get("assignee"):
                subprocess.run(["hermes", "kanban", "reassign", tid, "--to", body["assignee"]],
                               capture_output=True, text=True, check=True)
            return {}
        raise ValueError(f"shim: unsupported op {method} {path}")

    call.counts = n
    return call


def build():
    cdir = paths.change_dir(CID)
    cfg = yaml.safe_load(open("development/config.yaml"))
    h = cfg["hermes"]
    tasks = parse.parse_tasks(open(cdir / "tasks.md").read())
    c4, comp_map, contracts = parse.parse_design(open(cdir / "design.md").read())

    def pm(d):
        if not d:
            return None
        return ProfileModel(provider=d.get("provider", "fireworks"), model=d.get("model", ""),
                            base_url=d.get("base_url"), api_key_env=d.get("api_key_env"),
                            temperature=d.get("temperature"), max_tokens=d.get("max_tokens"))

    profile_models = {k: pm(v.get("model")) for k, v in (h.get("profiles") or {}).items()}
    profile_models = {k: v for k, v in profile_models.items() if v}
    routing = Routing(
        default_implementer="implementer", default_reviewer="reviewer",
        default_integrator="integrator", resolver="conflict-resolver",
        epic_assignee="integrator", board=boardmod.resolve_board(h.get("board")),
        tenant=CID if h.get("tenant_strategy") == "change-id" else None,
        profile_models=profile_models, default_model=pm((h.get("models") or {}).get("default")),
    )
    options = PlanOptions(
        pipeline=h.get("pipeline", "impl-review-integrate"),
        emit_epic=bool(h.get("emit_epic", True)), workspace=h.get("workspace", "worktree"),
        max_parallel_per_file_group=int(h.get("max_parallel_per_file_group", 2)),
        conflict_prevention=bool(h.get("conflict_prevention", True)),
        adr_contracts=frozenset(c.name for c in contracts if c.ratified_by),
    )
    return build_plan(CID, tasks, c4, comp_map, contracts, routing, options)


def main():
    plan = build()
    print(f"plan: epic={'yes' if plan.epic else 'no'} cards={len(plan.cards)} board={plan.board}")
    if DRY:
        m = apply(plan, dry_run=True, backend="cli")
        new = sum(1 for v in m.values() if v == "(new)")
        print(f"DRY RUN ok: {len(m)} keys, {new} new. Re-run with --apply to emit.")
        return
    transport = make_cli_transport()
    print("EMITTING (real)…")
    id_map = apply(plan, dry_run=False, backend="cli", transport=transport,
                   on_supersede="archive", write_ledger=True)
    c = transport.counts
    print(f"\nDONE: created={c['create']} links={c['link']} archived={c['archive']} "
          f"| ledger ids={len(id_map)}")


if __name__ == "__main__":
    main()
