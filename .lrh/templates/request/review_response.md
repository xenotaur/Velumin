# PR Review Response Request

Review URL: {{REVIEW_URL}}

Unresolved threads: {{UNRESOLVED_THREADS}}

Use repository-root `REVIEWS.md` as the maintenance source for Velumin's review
protocol. Respect `AGENTS.md`, `STYLE.md`, and any relevant README files in
affected directories.

For each unresolved thread:

1. Quote the reviewer concern in one concise sentence.
2. Check whether the issue is still present on the current branch.
3. Check whether the concern is valid for Velumin's Rust/WASM/WebGPU direction.
4. Check whether the fix is feasible and belongs in the current PR.
5. State the code, documentation, shader, demo, or validation change made, or
   explain why no change is needed.
6. Provide exact file path(s) and line-level evidence to cite in the reply.
7. End with suggested final reviewer-facing response text.

Velumin-specific guardrails:

- Keep review fixes narrow, reviewable, and directly tied to actionable feedback.
- Do not rewrite unrelated Rust, JavaScript, WGSL, scripts, demos, or project
  control-plane documents.
- Preserve the retro vector-graphics goal and the browser WebGPU baseline.
- Treat rendering, shader, WebGPU, WASM, Vite, and demo-entrypoint changes as
  requiring extra care because they may affect visible browser behavior.
- Preserve uncertainty markers in `project/` unless maintainers explicitly
  resolve them.

Preferred task-phase validation from the repository root:

```sh
scripts/version
scripts/format --check
scripts/lint
scripts/test
```

Run `scripts/baseline` or `scripts/validate` when Rust/WASM/Vite browser build
behavior, renderer setup, shaders, demos, validation scripts, CI, or browser
harness behavior may be affected.

Do not routinely run `scripts/develop` during ordinary review repair. Use it for
setup/bootstrap or when the review issue is specifically about setup.

If `scripts/version` reports missing or mismatched required tools, or if
canonical commands fail because dependencies or tools are absent, stop formatter,
linter, or test debugging and report a setup/bootstrap mismatch rather than a
code regression.

Do not mention Python-specific tools such as Black, Ruff, pytest, or Python
import failures unless the affected repository code actually uses them.
