# GitHub Actions workflows

This directory holds the CI/CD workflows for the `circuit_solver`
repository. Per tasks.md item #71, the CI pipeline gates every push
to `main` and every pull request targeting `main`.

## Workflows

| File     | Trigger                           | Purpose                                                                                                                                                          |
| -------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ci.yml` | `push` to `main`, `pull_request`  | Run the same preflight gates the scientia integrator runs locally: fmt, clippy `-D warnings`, workspace tests, conformance harness, doc `-D warnings`, maturin develop. |

## Jobs in `ci.yml`

The pipeline is fanned out into six jobs that run in parallel
(fmt, clippy, test, conformance, docs, maturin-develop). The jobs
have no inter-job dependencies; a failure in one does not stop the
others from reporting, so a PR author sees the full picture on a
single run rather than re-pushing after each fix.

| Job              | Command                                                                          | Gates                                                                                          |
| ---------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `fmt`            | `cargo fmt --all -- --check`                                                     | Formatting drift                                                                               |
| `clippy`         | `cargo clippy --workspace --all-targets --locked -- -D warnings`                 | Lint cleanliness (`pedantic = warn` at workspace level, escalated to `deny` here)              |
| `test`           | `cargo test --workspace --all-targets --locked` + `cargo test --doc --workspace` | Every unit test + scenario integration test + every rustdoc doctest                            |
| `conformance`    | `cargo test -p conformance-harness --all-targets --locked`                       | ADR-0008 max(rel, abs) tolerance envelope against ngspice golden references (tasks.md #62)     |
| `docs`           | `cargo doc --workspace --no-deps --locked` with `RUSTDOCFLAGS=-D warnings`       | rustdoc warnings (broken links, missing docs on public items per crate-level config, etc.)     |
| `maturin-develop`| `maturin develop --locked` + `python -c "import circuit_solver"`                 | The PyO3 extension still builds and loads under abi3-py39 (design.md "PyO3 distribution path") |

## What's intentionally *not* here

- **Wheel production.** `maturin build --release` and the associated
  multi-Python / multi-platform wheel matrix are owned by
  tasks.md #72 and (a future) release-tagged workflow. The
  `maturin-develop` job here only verifies that the *develop*
  path is green so contributors are not surprised by failures on
  their machine after a PR merges.
- **macOS / Windows runners.** v1 ships on Linux first. The
  `.cargo/config.toml` carries `-L` paths for macOS-brew and
  `/usr/local/lib` so a developer on macOS can compile and test
  locally, but CI gates only on `ubuntu-latest` for now. Adding a
  macOS job is a one-line `strategy.matrix.os` change once we have
  bandwidth to triage SuiteSparse install variants on macOS
  runners.
- **Release / publish.** The crate is `publish = false` per
  ADR-0010 (unstable v1 API). No release workflow is needed until
  ADR-0010 is superseded.

## Local equivalents

Every CI gate has a one-liner equivalent that runs on a developer
laptop. Run them before opening a PR to avoid the CI round-trip:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --doc --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
maturin develop --locked && python -c "import circuit_solver"
```

The integrator's per-task preflight already runs the first five;
the maturin one is what tasks.md #71 adds to the gating contract.
