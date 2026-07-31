# Decision Log

## 2026-05-07: Bootstrap LRH project directory

### Summary
- Created a standard LRH `project/` scaffold for Velumin because no top-level `project/` directory existed at bootstrap time.

### Decisions
- Treated the repository as classification `new`.
- Added only files under `project/`.
- Used the user's stated project direction as the primary goal source.
- Used README, Cargo metadata, Rust source, WGSL shader, and web harness files as repository evidence.
- Marked roadmap, API shape, validation strategy, browser support, and release strategy as uncertain.

### Rationale
- The request was explicitly a bootstrap request for LRH support.
- Repository evidence confirms Velumin identity and an early Rust/WASM/WebGPU vector-rendering implementation path.
- The available documentation is sparse, so conservative TODOs are safer than invented commitments.

### Uncertainty / Follow-ups
- Confirm the intended public API model.
- Confirm whether WebGPU is the long-term sole backend.
- Confirm build/test/demo commands and CI expectations.
- Confirm named ownership and review expectations.

### Status
- Accepted (Bootstrap Phase)

## 2026-07-30: Scope WI-ARCH-0001 without the webgpu_vector_lib rename

### Summary
- When scoping `WI-ARCH-0001` (extracting a `velumin-core` crate per DP-0002
  Phase 1), the maintainer chose to keep the crate name `webgpu_vector_lib`
  for this work item rather than rename it to `velumin` in the same change.

### Decisions
- `WI-ARCH-0001` extracts `velumin-core` as a new workspace member but does
  not rename the existing `webgpu_vector_lib` crate.
- This is a scoping decision for `WI-ARCH-0001` only — it does **not**
  resolve DP-0002's own open question ("Decide whether to rename
  `webgpu_vector_lib` to `velumin`"), which remains `undecided`, and does not
  resolve the `project/design/design.md` TODO on package identity. Both stay
  open for a future, separate decision.

### Rationale
- Keeping the existing crate name in this change limits blast radius (no
  `Cargo.toml`/`package.json`/CI path renames) while still extracting the new
  `velumin-core` member under its target name.
- A full rename is a separable, larger decision (affects publishing, CI,
  docs) better made on its own rather than bundled into a structural
  extraction refactor.

### Uncertainty / Follow-ups
- Whether/when to rename `webgpu_vector_lib` to `velumin` remains open; see
  DP-0002 Phase 1 and `project/design/design.md`'s TODO.

### Status
- Accepted (scoped to WI-ARCH-0001 only)

## 2026-07-31: Select DP-0002 Phase 2 as the active next workstream

### Summary
- The maintainer selected DP-0002 Phase 2 ("Modern Shared `wgpu` Renderer")
  as the active next workstream, following Phase 1's completion in
  `WI-ARCH-0001` (PR #17).

### Decisions
- DP-0002 Phase 2 — introducing a reusable renderer state that can render to
  any supported `wgpu::Surface`, plus explicit desktop-side adapter/
  capability negotiation — is now the selected direction for upcoming
  rendering-architecture work.
- This decision records selection only. No work item has been scoped for
  Phase 2 yet, and no code has changed; scoping a concrete work item is a
  separate, later step.
- DP-0002 as a whole remains `status: proposed` (not adopted) — selecting one
  phase as the active workstream is narrower than adopting the full proposal,
  consistent with how Phase 1 was treated.
- DP-0003 and DP-0002 Phase 3 (native `winit` shell) remain unselected,
  proposed follow-up directions.

### Rationale
- Phase 1 (`WI-ARCH-0001`) extracted the platform-neutral type crate
  (`velumin-core`) and left the renderer/browser-adapter split for a later
  phase, reasoning that Phase 3 (native `winit`) should exist first to
  validate that boundary. Phase 2 itself, however, is a prerequisite for
  Phase 3 to exist at all — a surface-agnostic renderer state has to exist
  before a native frontend can consume it — so selecting Phase 2 now moves
  the concrete next step forward without contradicting Phase 1's deferral of
  the deeper renderer/adapter crate split.

### Uncertainty / Follow-ups
- The exact crate boundary for Phase 2's renderer state (e.g. whether it
  lands inside `webgpu_vector_lib` first or as a new `velumin-renderer-wgpu`
  member) is not decided here; that is scope work for the first Phase 2 work
  item.
- Phase 3 (native `winit` shell) still requires its own explicit selection
  before any native-desktop work begins, per the existing focus non-goal.

### Status
- Accepted

