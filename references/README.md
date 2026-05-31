# references/ — scientia per-repo override directory

`scientia.paths.references_dir()` returns this directory *in place of* the
packaged `references/` whenever it exists, and resolves **both** `config.yaml`
**and every template** from here, with **no per-file fallback** to the package
(`scientia.templates.template_path`). So overriding config requires shipping a
*complete* `references/`: the override `config.yaml` **plus** all `*.md.tmpl`
templates. A config-only directory would shadow the package and break every
template-rendering stage.

## Contents

- `config.yaml` — the per-repo override (only the `hermes:` block differs from
  the bundle default; everything else inherits). This is the authoritative
  config the package reads.
- `*.md.tmpl` — mirrored verbatim from the installed `scientia` package so they
  resolve locally. They are copies, not symlinks, for portability across
  machines and checkouts.

## Maintenance — re-sync on upgrade

These templates were mirrored from **scientia 1.0.0**. They do not auto-update.
After upgrading the `scientia` package, re-copy the templates so this directory
does not silently serve stale versions:

```sh
PKG=$(python3 -c 'import scientia, pathlib; print(pathlib.Path(scientia.__file__).parent / "references")')
cp -p "$PKG"/*.md.tmpl references/
```

Then diff `config.yaml` against `"$PKG"/config.yaml` to pick up any new
recognized keys or schema changes before relying on the next pipeline run.
