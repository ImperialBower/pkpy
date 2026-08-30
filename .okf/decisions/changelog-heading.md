---
type: Decision
title: The changelog heading is the version, never Unreleased
description: A version bump writes its own CHANGELOG heading in the same change; this repository keeps no Unreleased section.
tags: [changelog, versioning, policy, release]
timestamp: 2026-08-30T00:00:00Z
---

# The rule

> The newest heading in `CHANGELOG.md` must be the version being bumped to —
> never `## [Unreleased]`. A version bump writes its heading as
> `## [<version>] - <YYYY-MM-DD>` in the same change that edits `Cargo.toml`.

Recorded in the repository's `CLAUDE.md` alongside
[version lockstep](/decisions/version-lockstep.md).

# Why

`make release` refuses to tag unless `CHANGELOG.md` already contains
`## [<version>]` ([release](/operations/release.md), precondition 4). With an
`[Unreleased]` section at the top, that gate is **always** failing, and shipping
needs an extra manual promotion step performed from memory at exactly the wrong
moment — under release pressure.

Writing the heading at bump time removes the step. Any commit on `main` is
taggable as-is.

This is a deliberate departure from the Keep a Changelog convention, which does
keep an `[Unreleased]` section. That convention assumes the version number is
unknown until release. Here it is not: [version lockstep](/decisions/version-lockstep.md)
means the number is decided the moment the `pkcore` dependency is chosen, so
there is nothing to defer.

# What this means in practice

- **No empty `## [Unreleased]` section above the current version.** Work landed
  after a bump but before the tag goes under the current version's heading. The
  next bump renames that heading and dates it.
- **The date is the bump date**, not the tag date. They are usually the same day
  and the difference has never mattered.
- **Editing a released version's section is allowed** while it is still the
  newest heading, because a heading only becomes frozen once the next bump moves
  past it.

# Related

- [Version lockstep](/decisions/version-lockstep.md) — why the number is known early.
- [Release and publishing](/operations/release.md) — the gate this rule satisfies.
- [Upgrading the pkcore dependency](/decisions/pkcore-upgrades.md) — the bump checklist.
