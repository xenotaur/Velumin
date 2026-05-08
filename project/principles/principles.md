---
id: PRINCIPLES-CORE
title: Project Principles
status: active
owner: project maintainers
---

# Principles

## Product Principles
- Preserve Velumin's stated identity as a retro vector-graphics library.
- Prefer visual output and APIs that support retro game aesthetics inspired by arcade vector displays such as Asteroids, Star Castle, and Space War.
- Keep library behavior understandable for game developers who need predictable rendering primitives.

## Engineering Principles
- Ground changes in repository evidence and working code.
- Favor small, testable increments over broad rewrites.
- Protect the Rust/WASM/WebGPU boundary from unnecessary churn.
- Keep examples and runtime harnesses aligned with the library API they demonstrate.

## LRH Operating Principles
- Treat authoritative LRH artifacts as the source of project intent and execution state.
- Treat `context/` files as derived summaries only.
- Record uncertainty explicitly instead of turning assumptions into commitments.
- Prefer bootstrap-safe additions over source-code modification during LRH setup.

