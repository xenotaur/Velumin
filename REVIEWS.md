# Review Protocol

Canonical protocol for addressing pull request review feedback in Velumin.

## 1. Triage Each Comment
For each reported review comment or issue:

1. Presence check: confirm the issue is still present on the current branch.
2. Validity check: confirm the concern is valid and worth addressing.
3. Feasibility check: confirm the fix belongs in the current change.

If any check fails, explain why and do not force an unrelated code change.

## 2. Keep Fixes Scoped
- Fix current, actionable review feedback only.
- Group overlapping comments by behavior or file.
- Do not add broad refactors, formatting churn, or unrelated cleanup.
- Do not reply on GitHub or resolve review threads unless explicitly asked.

## 3. Validate With Repository Entrypoints
Use canonical repository commands rather than ad hoc tool invocations.

Preferred task-phase sequence:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
```

Also run `scripts/baseline` or `scripts/validate` when Rust/WASM/Vite build behavior, scripts, CI, or browser harness behavior may be affected.

Do not routinely run `scripts/develop` during ordinary review repair. Use it for setup/bootstrap or when the issue is specifically about setup.

## 4. Setup Mismatch Handling
- If `scripts/version` reports missing or mismatched required tools, stop formatter/linter debugging and report a setup/bootstrap mismatch.
- If canonical commands fail because dependencies or tools are absent, report setup/bootstrap mismatch rather than a code regression.
- If `scripts/develop` fails due to installer, cache, network, or package-index issues, treat that as setup evidence unless the reviewed change explicitly concerns setup.

## 5. Evidence
Before claiming drift, non-reproducibility, or pre-existing unrelated failure, collect current-branch evidence:

```sh
git rev-parse HEAD
git status --short
```

Report exactly which canonical commands were run and whether any closest-equivalent command was used because a canonical entrypoint was unavailable.
