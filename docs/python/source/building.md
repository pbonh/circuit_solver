# Building this site

This site is built with [mkdocs] + the [material theme] + the
[mkdocstrings] Python handler. The mkdocstrings handler is wired
up but **disabled by default** so the site renders without
requiring an importable `circuit_solver` extension.

## Prerequisites

```bash
pipx install mkdocs                # or: pip install --user mkdocs
pip install --user                 \
    mkdocs-material                \
    mkdocstrings[python]           \
    pymdown-extensions
```

Python ≥ 3.9 is required (matches the `abi3-py39` floor of the
`circuit-solver-py` crate).

## Building (hand-curated content only)

The default configuration renders the hand-curated reference pages
under `docs/python/source/reference/`. These pages mirror the
`///` docstrings in `crates/circuit-solver-py/src/` verbatim.

From the repo root:

```bash
cd docs/python
mkdocs serve                       # live-reload at http://127.0.0.1:8000/
# or:
mkdocs build                       # one-shot build into target/docs/python/
```

The build output goes to `target/docs/python/` (set in
`mkdocs.yml`) so it's caught by the existing `**/target/` gitignore
entry and does not pollute the repo.

## Building with autodoc (requires a built extension)

To populate the reference pages from the *live* `circuit_solver`
extension's `__doc__` strings (rather than the hand-curated
markdown), do the following:

1. Build the extension with [maturin] in develop mode:

   ```bash
   pip install --user maturin
   cd crates/circuit-solver-py
   maturin develop --release
   ```

   This installs `circuit_solver` into the active Python
   environment. (The full wheel build is `tasks.md` #72; this is
   the develop-mode shortcut.)

2. Enable mkdocstrings in `docs/python/mkdocs.yml`:

   ```yaml
   plugins:
     - search
     - mkdocstrings:
         enabled: true       # ← flip this
         default_handler: python
         …
   ```

3. Replace the body of any reference page with a `::: <symbol>`
   directive, e.g.:

   ```markdown
   # CircuitBuilder

   ::: circuit_solver.CircuitBuilder
       options:
         show_root_heading: true
         members_order: source
   ```

4. Re-run `mkdocs serve`.

The hand-curated pages stay valid as the authoritative reference for
anyone reading the docs without a built extension (e.g. on
documentation hosting that does not run a Rust toolchain).

## CI integration

The full CI pipeline (`tasks.md` #71) will run:

```bash
cd docs/python && mkdocs build --strict
```

as a non-behavioral preflight. `--strict` fails the build on any
broken internal link or unrecognised mkdocs warning, catching docs
rot without exercising the simulator.

[mkdocs]: https://www.mkdocs.org/
[material theme]: https://squidfunk.github.io/mkdocs-material/
[mkdocstrings]: https://mkdocstrings.github.io/
[maturin]: https://www.maturin.rs/
