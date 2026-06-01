//! Build script for `circuit-solver-py`.
//!
//! When the crate is built with the `extension-module` feature
//! disabled (i.e. for `cargo test --no-default-features`), the test
//! binary links directly against `libpython` and needs to be able to
//! find the matching `.so` at runtime. Embed the Python `LIBDIR`
//! reported by `pyo3-build-config` as an `rpath` so the test binary
//! is self-contained and `cargo test` works without the caller having
//! to set `LD_LIBRARY_PATH`.
//!
//! Under the `extension-module` feature the build script is a no-op:
//! the resulting `cdylib` is loaded by `CPython`, which provides the
//! symbols, and no static `libpython` link occurs.

fn main() {
    // The feature flag is exposed as `CARGO_FEATURE_<NAME>` (uppercase,
    // hyphen → underscore). When `extension-module` is on, the build
    // script has nothing to add.
    if std::env::var("CARGO_FEATURE_EXTENSION_MODULE").is_ok() {
        return;
    }

    // `pyo3_build_config::get` queries the host Python interpreter that
    // PyO3's build machinery would otherwise link against. The `lib_dir`
    // field is `Some(path)` whenever a shared `libpython` was located
    // (typical on Linux/macOS); embed that path as an `rpath` so the
    // test binary resolves `libpython3.X.so` at runtime without the
    // caller setting `LD_LIBRARY_PATH`.
    let cfg = pyo3_build_config::get();
    if let Some(lib_dir) = cfg.lib_dir.as_deref() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}");
    }
}
