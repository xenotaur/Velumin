---
execution_id: 2026_07_25_14_09_24_DP_0007_DISPLAY_PRESET_API_REVIEW
prompt_id: PROMPT(AD_HOC:DP_0007_DISPLAY_PRESET_API_REVIEW)[2026-07-25T14:04:51-04:00]
work_item: AD_HOC
status: landed
rerun_of: 
pr: https://github.com/xenotaur/Velumin/pull/12
commit: 858907863e04a3733ae85459ef14dc1a267b3648
created_at: 2026-07-25T14:09:24-04:00
agent: claude_app
instruction_source: https://github.com/xenotaur/Velumin/pull/12
session_transcript: claude-app:10b17519-5c53-4a2a-be87-5686ece435f9
---

# Summary

Review response for PR #12 (DP-0007 proposal). Addressed nine reviewer threads
(codex + copilot). PR #12 is a proposal draft authored outside `/lrh-implement`,
so `rerun_of` is intentionally empty.

# Result

All nine threads triaged present, valid, in-scope; fixed, none skipped:

- codex `r3650668633`: the proposal's "adding presets stays backward-compatible"
  claim conflicts with a plain public enum (exhaustive matches break). Marked the
  proposed `VectorDisplayPreset` `#[non_exhaustive]` and noted the closed-set
  alternative.
- copilot `r3650672543`: fixed the "EV-0009, via WI-SMOKE-0001" wording — EV-0009
  is the manual capture; WI-SMOKE-0001 automated it.
- The remaining seven threads (codex `r3650668634`; copilot `r3650672493`,
  `r3650672508`, `r3650672528`, `r3650672558`, `r3650672578`, `r3650672590`) all
  flagged the same internally-contradictory drift: the DP-0007 edits left "Land
  WI-SMOKE-0001" actions and "non-4:3 not yet visually verified / no automated
  check" claims even though WI-SMOKE-0001 is resolved and `scripts/smoke`
  captures 4:3/wide/tall. Reconciled the full set — status/current_status.md,
  focus/current_focus.md, roadmap/roadmap.md, design/design.md, context/agents.md,
  and the adopted DP-0006 body — so WI-SMOKE-0001 reads as done, the non-4:3
  letterbox path reads as covered by the smoke check, and only the non-default
  preset capture and DP-0007 remain open. This drift predated PR #12 (from the
  DP-0006 adoption PR) and should have been reconciled at WI-SMOKE-0001 closeout.

# Validation

- `lrh validate` — 0 errors, 0 warnings.
- Rust/WASM lanes not applicable: this change touches only `project/` markdown.

# Follow-up

- Update `session_transcript` from `pending` to `claude-app:<session-id>` after
  the session ends.
