---
type: Playbook
title: Release and publishing
description: A pushed v-tag drives the PyPI publish; make release is the gate that checks the preconditions.
tags: [release, ci, pypi, maturin]
timestamp: 2026-08-30T00:00:00Z
---

# The mechanism

`.github/workflows/publish.yml` triggers on a pushed tag matching `v*`. Nothing
else publishes. There is no manual upload path.

The workflow builds in parallel, then publishes once:

| Job | Targets |
|---|---|
| `linux` | `x86_64`, `aarch64` (manylinux auto) |
| `macos` | `x86_64`, `aarch64` |
| `windows` | `x86_64` |
| `sdist` | source distribution |
| `publish` | needs all four; downloads `wheels-*` artifacts and uploads |

Publishing uses **PyPI trusted publishing** — `permissions: id-token: write` and
the `pypi` GitHub environment. There is no API token stored in the repository.
Upload runs with `--skip-existing`, so a re-run is safe.

# Cutting a release

```bash
make release
```

That target refuses to run unless all of the following hold:

1. `Cargo.toml`'s version parses.
2. The working tree has **no uncommitted changes**.
3. The tag `v<version>` does not already exist.
4. `CHANGELOG.md` contains a `## [<version>]` heading.

Only then does it create the annotated tag and push it to `origin`.

> **Rule 4 is why this repository does not keep an `## [Unreleased]` section.**
> The newest changelog heading is always the current version, written at bump
> time. See [the changelog rule](/decisions/changelog-heading.md). That keeps
> `make release` runnable at any commit instead of needing a separate
> promotion step first.

# CI on every change

`.github/workflows/ci.yml` runs on pushes and PRs against `main`: Python 3.13,
stable Rust, cached `~/.cargo` and `target/`, then `maturin develop` and
`pytest`. It mirrors `make test`.

Note the gap: `pyproject.toml` declares `requires-python = ">=3.8"`, but CI only
exercises **3.13**. The wheels are built with `--find-interpreter`, so the
released artifacts cover more than the tests do.

# Related

- [Build and test](/operations/build-and-test.md)
- [Version lockstep](/decisions/version-lockstep.md)
