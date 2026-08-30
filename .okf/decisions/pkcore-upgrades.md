---
type: Playbook
title: Upgrading the pkcore dependency
description: What to check when bumping pkcore, plus the record of the 0.11.0 upgrade.
tags: [upgrade, dependency, pkcore, changelog]
timestamp: 2026-08-30T00:00:00Z
---

# The checklist

A green build is **not** sufficient evidence that an upgrade is safe. Upstream
`pkcore` ships changes in three flavours, and only one of them fails to compile.

1. **Read upstream's changelog first.** It is vendored in the cargo registry:
   `~/.cargo/registry/src/*/pkcore-<version>/CHANGELOG.md`. Read every section
   between the old version and the new one.
2. **Compile breaks** — the build tells you. Fix them.
3. **Silent behaviour changes** — nothing fails. Grep `src/` for the affected
   upstream path and decide whether this wrapper reaches it. These are the
   dangerous ones.
4. **Feature-flag changes** — check `Cargo.toml`'s `features = [...]` list
   against upstream's new defaults.
5. **Deprecations** — grep `src/` for the deprecated names.
6. **New APIs** — note what is now available but unwrapped, so the gap is
   visible instead of forgotten.

Then run `make ayce` and record the result in `CHANGELOG.md` under a heading
named for the new version — see [the changelog rule](/decisions/changelog-heading.md).

# Record: 0.8.0 → 0.11.0

Skipped the 0.9 and 0.10 lines. **No code changes were needed**; all 228 tests
pass.

| Upstream change | Effect here |
|---|---|
| `store` and `terminal` became **non-default** features | None. `Cargo.toml` already requested `features = ["store"]`, which is what `IndexCardMap`, `SevenFiveBCM` and `HUPResult` need. |
| Combinatorics signatures moved to `impl Iterator` | None. No third-party type was named in this wrapper. |
| `EquityOptions::max_samples` default 100,000 → 25,000 | None — **but this is the silent one.** This wrapper never calls `analysis::equity`, so no exposed method changed its answer. Verified by grep, not by the build. |
| `TableManager` and `TableEvent` deprecated | None. Never wrapped. |
| `Card` deserialization now rejects unparseable indices | None. |

# Available upstream, not wrapped here

- `Table::snapshot` / `Table::restore` and `PokerSession::snapshot` /
  `PokerSession::restore` — postcard byte-identical mid-hand save and resume.
- `Table::showdown` and `Table::audit_chip_total` — the finer tier under
  `end_hand`, letting a UI render the result before the table resets.
- The Pluribus **write** half from 0.10.0 (`Unumable`, `Pluribus::write_log`).

See [table and session](/api/table-session.md).

# Related

- [Version lockstep](/decisions/version-lockstep.md)
- [Build and test](/operations/build-and-test.md)

# Citations

[1] `pkcore` CHANGELOG, versions 0.10.0 and 0.11.0
[2] `CHANGELOG.md` in this repository, `## [0.11.0]` section
