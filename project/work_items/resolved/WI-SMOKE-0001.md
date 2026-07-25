---
id: WI-SMOKE-0001
title: Automatable Screenshot Smoke Check for Demo Scenes
type: deliverable
status: resolved
priority: medium
owner: project maintainers
depends_on:
  - WI-DEMO-0001
related_design:
  - project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md
  - project/design/proposals/adopted/DP-0005-blasterites-tester-demo-and-visual-smoke.md
blocked: false
blocked_reason: null
resolution: Implemented and merged in PR #11 (commit 9c0f1ba) — freeze-frame harness mode, a GPU-free Rust determinism guard, and scripts/smoke (Playwright + tolerant reference comparison) that captures the deterministic tester frames at 4:3/wide/tall and skips cleanly on GPU-less environments.
---

# WI-SMOKE-0001: Automatable Screenshot Smoke Check for Demo Scenes

## Objective
- Turn the manual browser visual-smoke capture recorded in `EV-0009` into an automatable, repeatable screenshot check at known deterministic tester frames, so CI or a scripted run can guard the DP-0006 renderer against visual regressions.

## Scope
- Render the baseline, Blasterites tester (deterministic pre-impact and post-impact frames), and tuner scenes headlessly or through a scripted browser.
- Capture screenshots at fixed elapsed-time values (the deterministic tester frames, e.g. `t=2000ms` and `t=4000ms` within the 5600ms cycle) rather than sampling the live animation loop.
- Capture at non-4:3 browser/canvas sizes (wide and tall) in addition to exact 4:3, to exercise the `centered_4_3` letterbox/pillarbox path — the portion of DP-0006's manual-inspection validation that `EV-0009` did not cover.
- Provide a stable way to freeze a single frame at a chosen time (e.g. a query parameter on the demo harness or a dedicated capture entrypoint) so captures are reproducible.
- Compare captured frames against committed reference images and fail on meaningful divergence (not all-black, not all-white, geometry present).
- Wire the check into `scripts/` (and optionally CI) alongside the existing validation lanes.

## Evidence
- DP-0006: `project/design/proposals/adopted/DP-0006-vector-crt-renderer-migration.md`
- Manual capture this builds on: `project/evidence/EV-0009.md`
- Deterministic timing constants: `webgpu_vector_lib/src/lib.rs` (`BLASTERITES_CYCLE_MS`, `BLASTERITES_IMPACT_MS`)
- Demo harness and routing: `webgpu_vector_lib/web/index.html`, `scripts/demos`

## Acceptance Criteria
- A scripted command renders and captures the baseline, Blasterites tester (deterministic frames), and tuner scenes without manual browser interaction.
- Captures are deterministic and reproducible from committed reference frames.
- The check distinguishes expected output from an all-black, all-white, or error state.
- The check is invocable from `scripts/` and documented alongside the other validation commands.
- WebGPU adapter, browser, and OS information is recorded when available (as in `EV-0002`/`EV-0009`).

## Non-Goals
- Do not turn the Blasterites tester into a playable game or add game logic.
- Do not require pixel-perfect equality across GPUs/drivers; tolerate adapter-level variation while still catching gross regressions.
- Do not block the adopted DP-0006 direction on this work; it is a follow-up guard, not an adoption prerequisite.
- Do not make the screenshot check a hard CI gate: headless WebGPU is unavailable on GPU-less runners, so the check skips cleanly there. It stays out of `scripts/validate`.

## Required Changes
- `webgpu_vector_lib/web/index.html`: add a freeze-frame capture mode — `?frame` (optionally `?frame=<ms>` or `?t=<ms>`) renders exactly one deterministic frame at a fixed elapsed time and signals readiness via `window.__smokeReady` / `window.__smokeError`. Preserve all existing animate/baseline/tuner behavior when `?frame` is absent.
- `webgpu_vector_lib/src/lib.rs`: add a Rust unit test asserting the tester geometry is deterministic (same elapsed time → identical output) and animates (pre-impact `t=2000ms` differs from post-impact `t=4000ms`). This is the GPU-free regression guard that runs in `scripts/test` on every CI run.
- `webgpu_vector_lib/web/smoke.mjs`: a Playwright + pngjs script that drives headless Chromium over the freeze-frame routes at 4:3 / wide / tall viewports, captures the canvas via the compositor screenshot, and asserts structural properties (not all-black, not all-white, geometry present, pre- vs post-impact frames differ, non-4:3 letterbox margins dark). Skips cleanly (exit 0) when no WebGPU adapter is available.
- `scripts/smoke`: wrapper that builds the WASM package, serves Vite, runs `smoke.mjs`, and tears down.
- `webgpu_vector_lib/web/package.json`: add `playwright` and `pngjs` devDependencies and a `smoke` npm script.
- `webgpu_vector_lib/web/smoke/reference/`: committed reference frames (wide, tall) for human comparison, with a README.
- `.gitignore`: ignore `webgpu_vector_lib/web/smoke-out/`.
- Docs: `scripts/README.md` and top-level `README.md` document `scripts/smoke`.

## Validation
- `scripts/version`
- `scripts/format --check`
- `scripts/lint`
- `scripts/test` (includes the new determinism guard)
- `lrh validate`
- `scripts/smoke` on a WebGPU-capable environment (renders and asserts; skips with a clear notice when no adapter is available)
