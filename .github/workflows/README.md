# GitHub Actions Workflows

Velumin's CI is intentionally thin: workflows install or prepare tools, then call repository-owned scripts.

## `validate.yml`

`validate.yml` runs the DP-0004 validation workflow.

Triggers:

- pull requests;
- pushes to `main`;
- manual `workflow_dispatch`.

Permissions:

```yaml
permissions:
  contents: read
```

Job behavior:

1. Check out the repository.
2. Set up Node 24.
3. Enable npm caching keyed to `webgpu_vector_lib/web/package-lock.json`.
4. Run `scripts/develop`.
5. Run `scripts/validate`.

The workflow should not duplicate validation logic from the scripts. If the local validation contract changes, update scripts first, then keep the workflow as a small setup-and-dispatch wrapper.

## Tooling Decisions

- Rust is selected through the checked-in `rust-toolchain.toml`.
- `scripts/develop` ensures `rustfmt`, Clippy, and `wasm32-unknown-unknown` are present.
- `scripts/develop` runs `npm ci` so CI uses the checked-in npm lockfile.
- `scripts/develop` installs pinned `wasm-pack` version `0.13.1` when CI does not already have that exact version.

## Maintenance Notes

- Keep workflow permissions minimal unless a job explicitly needs more access.
- Prefer changing `scripts/` over embedding logic directly in YAML.
- Do not add browser visual smoke, cargo-deny, dependency review, Dependabot, or broader supply-chain policy to this workflow without a project work item.
- If CI fails in `scripts/develop`, first check setup/toolchain drift before treating it as a code regression.
