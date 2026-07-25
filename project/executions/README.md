# Execution Records

Execution records under `project/executions/<WORK_ITEM_OR_AD_HOC>/` are
**point-in-time logs** of a single prompt's execution. Each record captures
what a prompt did, when, against which PR and commit, and its execution
lifecycle `status` (`in_progress` → `landed`).

## Point-in-time convention

Execution records are **not retro-reconciled** when project state later
changes. A record's `status` tracks the *execution's* lifecycle, not the
lifecycle of the artifacts it touched. Statements in a record's body describe
the world **as of that execution** — for example, a record may say a design
proposal was `proposed` even though the proposal was later adopted.

This is deliberate: rewriting a landed execution log would destroy the audit
trail. When reconciliation notes elsewhere say "the full control-plane set was
updated," that refers to the authoritative and derived status artifacts
(`status/`, `focus/`, `roadmap/`, `design/`, `context/`, and the relevant
proposals and work items) — **not** to the historical execution logs, which are
intentionally left as written.

To find the current state of any artifact, read the authoritative artifact
itself, not an execution record that references it.
