---
id: GUARDRAILS-COST
title: Cost Guardrails
status: active
owner: project maintainers
---

# Cost Guardrails

## Development Cost
- Prefer local builds, tests, and static inspection before adding heavier tooling.
- Avoid introducing dependencies unless they clearly support the project goal and fit existing architecture.
- Keep graphics validation practical for contributors working on local machines.

## Infrastructure Cost
- No paid services, hosted infrastructure, analytics, or external monitoring should be added without explicit approval.
- CI expansion should be incremental and justified by validation needs.

## Unknowns / TODO
- TODO: Confirm acceptable CI/runtime cost budget if automated browser or GPU validation is added.

