---
type: Decision
title: Version lockstep with pkcore
description: pkcore.py's own version always equals the pkcore version it wraps — a hard project rule.
tags: [versioning, policy, semver]
timestamp: 2026-08-30T00:00:00Z
---

# The rule

> `pkcore.py`'s version in `Cargo.toml` must always match the `pkcore` dependency
> version. When `pkcore` is bumped, bump `pkcore.py`'s own `version` field to the
> same value in the same change.

This is recorded in the repository's `CLAUDE.md` and is not negotiable per
change.

# Why

The package is a **pure binding layer** — it adds no poker behaviour of its own
([binding layer](/architecture/binding-layer.md)). So its own semver has nothing
independent to describe. Matching the upstream version makes one question
answerable from the version string alone: *which pkcore am I actually talking
to?*

The cost is that this package cannot express its own patch releases. A
binding-only fix has to wait for, or borrow, an upstream number. That trade has
been accepted.

# Consequences

- Version numbers here are **not contiguous**. This repo has jumped 0.2.1 → 0.7.0
  and 0.9.0 → 0.11.0, because it follows upstream rather than counting its own
  releases.
- `pyproject.toml` uses `dynamic = ["version"]`, so maturin reads the version
  out of `Cargo.toml`. There is exactly one place to edit.
- A bump touches: `Cargo.toml` `[package] version`, `Cargo.toml` `pkcore`
  dependency, `Cargo.lock` (via a build), and `CHANGELOG.md`'s newest heading —
  see [the changelog rule](/decisions/changelog-heading.md).

# How to bump

```bash
# 1. edit both version strings in Cargo.toml
# 2. rebuild so Cargo.lock updates and the API is checked
make build && make test
# 3. write the CHANGELOG heading as ## [<version>] - <YYYY-MM-DD>
```

See [pkcore upgrades](/decisions/pkcore-upgrades.md) for what to check while
doing it, and [release](/operations/release.md) for shipping it.
