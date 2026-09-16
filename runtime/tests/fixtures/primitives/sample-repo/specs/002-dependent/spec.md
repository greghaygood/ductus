---
status: planned
dependencies: [001-basic]
---

# 002 — Dependent Sample Feature

A fixture that depends on `001-basic`, exercising the `traverse-deps`
primitive.

## Motivation

Pin the dependency graph for primitive tests. The §runtime-boundary
section is referenced to give `resolve-anchor` something to chew on.

## Acceptance Criteria

- [ ] Dependency on `001-basic` resolves and reports `compatible: true`.
