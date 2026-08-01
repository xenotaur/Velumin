---
id: WI-ARCH-0004
title: Resolve DP-0002 Phase 2/Phase 3 Sequencing Circularity by Reordering the Negotiation Bullet
type: operation
status: proposed
priority: low
owner: project maintainers
assigned_agents: []
related_focus:
  - FOCUS-RENDER-0001
related_roadmap:
  - ROADMAP-CORE
related_workstreams: []
related_design:
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md
depends_on: []
blocked_by: []
blocked: false
blocked_reason: null
resolution: null
expected_actions:
  - edit_file
forbidden_actions:
  - force_push
  - delete_branch
  - implement_phase_3_desktop
  - implement_native_surface_acquisition
  - change_rendered_output
acceptance:
  - DP-0002 Phase 2's milestone list no longer includes desktop-side adapter/capability negotiation as one of its own bullets
  - DP-0002 Phase 3's milestone list explicitly includes desktop-side adapter/capability negotiation as one of its own bullets
  - The "Unresolved sequencing circularity" note in DP-0002 is resolved, replaced with a brief historical pointer to the decision log
  - project/memory/decision_log.md records the reordering decision and rationale
  - lrh validate reports 0 errors
  - No code files are touched
required_evidence:
  - lrh_validate
artifacts_expected:
  - project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md (updated)
  - project/memory/decision_log.md (updated)
  - project/roadmap/roadmap.md (updated)
  - project/focus/current_focus.md (updated)
  - project/status/current_status.md (updated)
  - project/context/humans.md (updated)
---

# WI-ARCH-0004: Resolve DP-0002 Phase 2/Phase 3 Sequencing Circularity by Reordering the Negotiation Bullet

## Summary
Resolve the DP-0002 Phase 2/Phase 3 sequencing circularity flagged during `WI-ARCH-0003`'s review by splitting Phase 2's "adapter/capability negotiation for web and desktop" bullet: keep the web-side half (already done) in Phase 2, and move the desktop-side half into Phase 3, where it structurally belongs since it depends on Phase 3's own native-surface deliverable. Design-doc reconciliation only — no code changes.

## Problem / Context
`WI-ARCH-0003`'s review (2026-08-01) surfaced a real structural inconsistency in DP-0002's own milestone list: Phase 2 lists "explicit adapter/capability negotiation for web and desktop" as one of its own bullets, but achieving the desktop half requires a native surface/adapter acquisition path — exactly what Phase 3 ("Native `winit` Shell") would build — and Phase 3 is sequenced *after* Phase 2 in the current list. As scoped today, that one Phase 2 bullet can never complete before Phase 3 exists, while Phase 3 does not depend on that bullet completing first. This was acknowledged in an inline note added to DP-0002 during that review but deliberately left unresolved, naming two options: scope a minimal native-surface acquisition seam as its own Phase 2 slice, or explicitly merge/reorder the bullet into Phase 3. The maintainer has chosen the second, lighter option — this item does not build any native-surface code; it only makes the phase list internally consistent by moving the bullet to the phase it structurally belongs to.

### Duplication search
- In-repo: No existing implementation found. `WI-ARCH-0003` (resolved) flagged the circularity but explicitly left it unresolved ("Not resolved here.").
- Sibling repos: None identified.
- External libraries: None identified.
- Recommendation: Proceed.

### Demand search
- Work items: `WI-ARCH-0003` (resolved) is the source of this item; no other work item addresses this circularity.
- Proposals: DP-0002 (proposed) carries the unresolved note directly (Phase 2 milestone section).
- Backlog: No matching entries.
- Recommendation: No action.

## Scope
- Move the desktop-side half of the negotiation bullet from DP-0002 Phase 2 to Phase 3.
- Resolve the "Unresolved sequencing circularity" note, replacing it with a brief pointer to the decision log.
- Reassess and correct DP-0002 Phase 2's overall milestone status label now that its remaining bullets are all done.
- Sweep every other control-plane file that currently describes desktop-side negotiation as "part of Phase 2" and update it to match.
- Record the decision and rationale in `project/memory/decision_log.md`.
- Do not write any code, tests, or native-surface acquisition logic — this item is a documentation reconciliation only.

## Required Changes
1. In `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md`'s Phase 2 section: narrow the "Add explicit adapter/capability negotiation for web and desktop" bullet to web-side only (mark it done, since the web-side capability checks already exist in `WebGPU::create_with_preset`), and remove the desktop-side clause and the "Unresolved sequencing circularity" note, replacing the note with one sentence pointing to the new `project/memory/decision_log.md` entry.
2. In the same file's Phase 3 section: add a new bullet for desktop-side adapter/capability negotiation, with a note that it moved here from Phase 2 on 2026-08-01 to resolve the sequencing circularity.
3. In the same file's Phase 2 heading: update the status label from "Partially done (browser-only)" to "Done (browser-only)" — accurate once the negotiation bullet is narrowed to its already-done web-side half, since the other two Phase 2 bullets (DP-0001 `wgpu` upgrade, reusable renderer state) are already marked done.
4. Add a `project/memory/decision_log.md` entry ("2026-08-01: Resolve DP-0002 Phase 2/Phase 3 sequencing circularity by reordering") recording the decision, rationale (this is a phase-list correction, not a Phase 3 workstream selection or any native-surface commitment), and follow-ups (Phase 3 still needs its own explicit selection before any of its bullets, including the moved one, can be worked).
5. Sweep and update the following files, all of which currently describe desktop-side negotiation as "part of Phase 2" or "Phase 2's remaining bullet" (verified via `grep` before drafting this item — 8 occurrences total): `project/roadmap/roadmap.md` (1), `project/focus/current_focus.md` (3), `project/status/current_status.md` (3), `project/context/humans.md` (1). Reword each to reflect that Phase 2 is now done (browser-only) and desktop-side negotiation is a Phase 3 bullet awaiting that phase's own selection — not "Phase 2's remaining bullet" needing a work item of its own.

## Non-Goals
- Do not implement any native `winit` entry point or native surface/adapter acquisition code — that is Phase 3's actual deliverable, gated on its own explicit workstream selection.
- Do not select DP-0002 Phase 3 as an active workstream — moving a bullet into Phase 3's milestone list is a documentation correction, not a selection decision.
- Do not change any rendered visual output or touch any code file.
- Do not scope or imply a "minimal native-surface acquisition seam" — that was the heavier alternative the maintainer explicitly did not choose.

## Acceptance Criteria
- DP-0002 Phase 2's milestone list no longer includes desktop-side adapter/capability negotiation as one of its own bullets.
- DP-0002 Phase 3's milestone list explicitly includes desktop-side adapter/capability negotiation as one of its own bullets, noting it moved from Phase 2.
- The "Unresolved sequencing circularity" note is resolved, replaced with a pointer to the decision log entry.
- DP-0002 Phase 2's status label reads "Done (browser-only)".
- `project/memory/decision_log.md` records the decision and rationale.
- All 8 swept references in `roadmap.md`, `current_focus.md`, `current_status.md`, and `humans.md` are consistent with the reorder.
- `lrh validate` reports 0 errors.
- No file under `webgpu_vector_lib/` or `velumin-core/` is touched.

## Validation
- `lrh validate`

## Risk Notes
- Marking Phase 2 "Done (browser-only)" could be misread as "DP-0002 is fully resolved" — mitigated by keeping the "(browser-only)" qualifier and DP-0002's overall `status: proposed` unchanged, so the distinction between phase-level and proposal-level status stays clear.
- This item resolves a documentation inconsistency, not the underlying product question of when/how desktop support actually gets built — Phase 3 remains genuinely unstarted and unselected after this item.

## Related Workstream and Designs
- Design: `project/design/proposals/proposed/DP-0002-cross-platform-renderer-architecture.md` (Phase 2 and Phase 3 milestone sections)
- Prior work: `WI-ARCH-0003` (PR #22, resolved) — flagged this circularity during its own review
