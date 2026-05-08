# Style Guide

Velumin keeps style guidance minimal and delegates mechanical decisions to canonical Rust and web tooling.

## General
- Prefer small, scoped, reviewable changes.
- Prefer clarity over cleverness.
- Preserve existing behavior unless the task requires changing it.
- Avoid drive-by refactors, unrelated renames, and broad formatting churn.
- Keep project-control updates evidence-backed and consistent with `project/`.

## Rust
- Use `rustfmt` through `scripts/format`; do not manually fight formatter output.
- Use Clippy through `scripts/lint`; CI treats warnings as errors.
- Use the checked-in Rust toolchain in `rust-toolchain.toml`.
- For public Rust API design, consider the Rust API Guidelines, especially as DP-0002 and DP-0003 mature.

References:
- Rust Style Guide: https://doc.rust-lang.org/style-guide/
- rustfmt: https://github.com/rust-lang/rustfmt
- Clippy CI guidance: https://doc.rust-lang.org/stable/clippy/continuous_integration/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

## Web/WASM
- Keep browser harness changes narrow and easy to validate through `scripts/baseline`.
- Prefer standard wasm-pack, npm, and Vite behavior over custom wrappers.
- Keep generated outputs, caches, and local build artifacts out of source control.

## Scripts
- Scripts should remain thin repository entrypoints.
- Avoid moving core project behavior into shell scripts when it belongs in Rust code, package tooling, or documented LRH project-control artifacts.
- Pin setup tools when CI installs them so validation remains reproducible.
